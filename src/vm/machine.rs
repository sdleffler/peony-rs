use std::{collections::VecDeque, sync::Arc};

use vm::heap::{Address, DeepCopy, Heap, HeapError, Immediate, OfActivation, OfEnvironment, Type,
               Unpacked as UnpackedWord, Value, Word};

#[derive(Debug, Fail)]
pub enum ExecError {
    #[fail(display = "machine halted by instruction")]
    Halt,

    #[fail(display = "no such local")]
    LocalNotFound,

    #[fail(display = "expected non-empty value stack")]
    EmptyStack,

    #[fail(display = "heap error: {}", _0)]
    Heap(#[cause] HeapError),
}

impl From<HeapError> for ExecError {
    fn from(err: HeapError) -> Self {
        ExecError::Heap(err)
    }
}

/// Trait for "hooked" instructions: custom non-builtin instructions depending on custom word
/// layouts.
pub trait Hook<W: Word> {}

/// Trait for types usable as instructions.
pub trait Insn<W: Word>: Copy + Eq + 'static {
    type Hook: Hook<W>;

    fn unpack(self) -> Unpacked<Self::Hook>;
    fn pack(Unpacked<Self::Hook>) -> Self;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Local {
    scope: usize,
    index: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Unpacked<H> {
    /// Stop. Drop. Roll.
    Halt,

    /// Load a constant from the current template into a location.
    ///
    /// ``` ignore
    /// PushConstant(CONSTANT_ID)
    ///
    /// [..] => [.., c]
    /// ```
    PushConstant(usize),

    /// Get a local variable and push it onto the stack.
    ///
    /// ``` ignore
    /// Get(LOCAL)
    ///
    /// [..] => [.., t]
    /// ```
    PushLocal(Local),

    /// Pop a value off the stack and put it into a local variable.
    ///
    /// ``` ignore
    /// PopLocal(LOCAL)
    ///
    /// [.., t] => [..] (LOCAL set to t)
    /// ```
    PopLocal(Local),

    /// Save the current continuation and push it onto the continuation stack.
    ///
    /// ``` ignore
    /// SaveCurrentCont(RESUME)
    ///
    /// STACK => [] (STACK and registers copied to activation record w/ saved PC set to RESUME)
    /// ```
    SaveCurrentCont(usize),

    /// Pop the top activation record off of the continuation stack and load registers from it.
    /// Then, extend the stack from the bottom up with the saved stack segment in the activation
    /// record.
    ///
    /// ``` ignore
    /// LoadCurrentCont
    ///
    /// [.., t] => [STACK, t] (STACK and registers, including PC, loaded from activation record)
    /// ```
    LoadCurrentCont,

    /// "Hooked" instructions are non-builtins.
    Hook(H),
}

/// A function "template" contains a pointer to the function's code, along with a function-specific
/// pool of constants, stored as a single message heap.
#[derive(Debug)]
pub struct Template<W: Word> {
    label: usize,
    constants: Heap<W>,
    index: Vec<W>,
}

/// Programs have several important regions of data.
#[derive(Debug)]
pub struct Program<W: Word, I: Insn<W>> {
    templates: Vec<Template<W>>,
    code: Vec<I>,
}

impl<W: Word, I: Insn<W>> Program<W, I> {
    pub fn code(&self) -> &[I] {
        &self.code
    }
}

#[derive(Debug)]
pub struct Registers<W: Word> {
    /// The "environment" register.
    env: Address<W, OfEnvironment>,

    /// The "continuation stack" register.
    cs: Address<W, OfActivation>,

    /// The "escape continuation" register.
    ec: Address<W, OfActivation>,

    /// The "template" register.
    tm: usize,

    /// The program counter (a.k.a. instruction pointer).
    pc: usize,
}

#[derive(Debug)]
pub struct Machine<W: Word, I: Insn<W>> {
    registers: Registers<W>,
    program: Arc<Program<W, I>>,
    stack: VecDeque<W>,
    heap: Heap<W>,
}

pub enum Action {
    Next,
    JumpAbs(usize),
    JumpRel(isize),
    Halt,
}

impl<W: Word, I: Insn<W>> Machine<W, I>
where
    W::Tag: Type<W>,
{
    #[inline]
    pub fn fetch(&self) -> I {
        self.program.code[self.registers.pc]
    }

    #[inline]
    pub fn scope(&mut self, scope: usize) -> Result<Address<W, OfEnvironment>, ExecError> {
        let mut env = self.registers.env;
        for _ in 0..scope {
            let ptr = self.heap.get_mut(env).parent().pointer().unwrap();
            env = self.heap.address(ptr).unwrap();
        }
        Ok(env)
    }

    #[inline]
    pub fn read_loc(&mut self, location: Local) -> Result<W, ExecError> {
        let env = self.scope(location.scope)?;
        self.heap
            .get(env)
            .locals()
            .get(location.index)
            .cloned()
            .ok_or(ExecError::LocalNotFound)
    }

    #[inline]
    pub fn write_loc(&mut self, location: Local, word: W) -> Result<(), ExecError> {
        let env = self.scope(location.scope)?;
        let mut view_mut = self.heap.get_mut(env);
        let local_mut = view_mut
            .locals_mut()
            .get_mut(location.index)
            .ok_or(ExecError::LocalNotFound)?;
        *local_mut = word;
        Ok(())
    }

    const TRUNCATE_BITS: u32 = 64 - W::INT_SIZE;

    pub fn step(&mut self) -> Result<(), ExecError> {
        let action = match self.fetch().unpack() {
            Unpacked::Halt => Action::Halt,
            Unpacked::PushConstant(cst) => {
                let value = {
                    let Template {
                        constants, index, ..
                    } = &self.program.templates[self.registers.tm];
                    let root = index[cst];
                    let mut copy = DeepCopy::new(constants, &mut self.heap);
                    copy.root(root)?
                };
                self.stack.push_front(value);
                Action::Next
            }
            Unpacked::PushLocal(src) => {
                let value = self.read_loc(src)?;
                self.stack.push_front(value);
                Action::Next
            }
            Unpacked::PopLocal(dst) => {
                let value = self.stack.pop_front().ok_or(ExecError::EmptyStack)?;
                self.write_loc(dst, value)?;
                Action::Next
            }
            Unpacked::SaveCurrentCont(pc_resume) => {
                let addr = self.heap.alloc1::<OfActivation>(self.stack.len())?;

                let env = {
                    let env_addr = self.registers.env;
                    let env_ptr = env_addr.into_pointer();
                    Word::pack(UnpackedWord::Value(Value::Pointer(env_ptr)))
                };
                let cs = {
                    let cs_addr = self.registers.cs;
                    let cs_ptr = cs_addr.into_pointer();
                    Word::pack(UnpackedWord::Value(Value::Pointer(cs_ptr)))
                };
                let tm = {
                    let tm_raw = self.registers.tm as u64;
                    Word::pack(UnpackedWord::Raw(tm_raw))
                };
                let pc = Word::pack(UnpackedWord::Raw(pc_resume as u64));

                let mut view_mut = self.heap.get_mut(addr);
                *view_mut.env_mut() = env;
                *view_mut.cs_mut() = cs;
                *view_mut.tm_mut() = tm;
                *view_mut.pc_mut() = pc;

                {
                    let (front, back) = self.stack.as_slices();
                    view_mut.stack_mut()[..front.len()].copy_from_slice(front);
                    view_mut.stack_mut()[front.len()..].copy_from_slice(back);
                }

                self.stack.clear();
                self.registers.cs = addr;

                Action::Next
            }
            Unpacked::LoadCurrentCont => {
                let addr = self.registers.cs;
                let view = self.heap.get(addr);

                self.registers.env = self.heap.address(view.env().pointer().unwrap())?;
                self.registers.cs = self.heap.address(view.cs().pointer().unwrap())?;
                self.registers.tm = view.tm().raw().unwrap() as usize;

                self.stack.extend(view.stack().iter().cloned().rev());

                Action::JumpAbs(view.pc().raw().unwrap() as usize)
            }
            Unpacked::Hook(hook) => unimplemented!(),
        };

        match action {
            Action::Next => {
                self.registers.pc += 1;
                Ok(())
            }
            Action::JumpAbs(addr) => {
                self.registers.pc = addr;
                Ok(())
            }
            Action::JumpRel(offset) => {
                if offset >= 0 {
                    self.registers.pc += offset as usize;
                } else {
                    self.registers.pc -= offset.abs() as usize;
                }
                Ok(())
            }
            Action::Halt => Err(ExecError::Halt),
        }
    }
}
