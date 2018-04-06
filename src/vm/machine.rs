use std::{collections::VecDeque, sync::Arc};

use vm::heap::{Address, DeepCopy, Heap, HeapError, Immediate, OfActivation, OfCons, OfEnvironment,
               Type, Unpacked as UnpackedWord, Value, Word};

#[derive(Debug, Fail)]
pub enum ExecError {
    #[fail(display = "machine halted by instruction")]
    Halt,

    #[fail(display = "no such local")]
    LocalNotFound,

    #[fail(display = "expected non-empty value stack")]
    EmptyStack,

    #[fail(display = "empty continuation stack")]
    EmptyContStack,

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
    /// NB this does *not* save the program counter! The activation record's saved PC is set
    /// to `RESUME`.
    /// NB this will clear the stack!
    /// TODO does it *need* to clear the stack? Would it be better just to copy the stack?
    ///
    /// ``` ignore
    /// SaveCurrentCont(RESUME)
    ///
    /// STACK => []
    ///
    /// CONT := ACTIVATION(ENVT, CONT, TEMPLATE, RESUME, STACK)
    /// ```
    SaveCurrentCont(usize),

    /// Pop the top activation record off of the continuation stack and load registers from it.
    /// Then, extend the stack from the bottom up with the saved stack segment in the activation
    /// record.
    ///
    /// NB `LoadCurrentCont` will pop the delimiter stack if `CONT` is `Nil`, setting `CONT` to the
    /// saved value at the top of the delimiter stack. `CONT` is `Nil` and `DELIM` is `Nil`, then
    /// the VM will halt.
    ///
    /// ``` ignore
    /// LoadCurrentCont
    ///
    /// [..] => [.., ACTIVATION.STACK[..]]
    ///
    /// ENVT := ACTIVATION.ENVT
    /// CONT := ACTIVATION.CONT
    /// TEMPLATE := ACTIVATION.TEMPLATE
    /// PC := ACTIVATION.PC
    /// ```
    LoadCurrentCont,

    /// Push the contents of the "current continuation" register onto the value stack.
    ///
    /// There is no `PopCurrentCont`. The proper way to set the current continuation is to use a
    /// `Call`.
    ///
    /// NB `PopCurrentCont` will error if `CONT` is `Nil`!
    ///
    /// ``` ignore
    /// PushCurrentCont
    ///
    /// [..] => [.., CONT]
    /// ```
    PushCurrentCont,

    // /// Reset the current continuation, pushing it onto the delimiter stack.
    // ///
    // /// ``` ignore
    // /// ResetCurrentCont
    // ///
    // /// DELIM := Cons(CONT, DELIM)
    // /// CONT := Nil
    // /// ```
    //ResetCurrentCont,
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
    envt: Address<W, OfEnvironment>,

    /// The "continuation stack" register.
    ///
    /// This may be `None`, in which case indicating that the current continuation is to be popped
    /// from the delimiter stack.
    cont: Option<Address<W, OfActivation>>,

    /// The "delimiter stack" register.
    ///
    /// This may sometimes be `None`, in which case indicating that there is no delimiting scope.
    /// If the continuation stack is `None` and the delimiter stack is `None` and a continuation is
    /// loaded, then the machine halts and its final value is that of the stack.
    delim: Option<Address<W, OfCons>>,

    /// The "template" register.
    template: usize,

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
        let mut env = self.registers.envt;
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

    #[inline]
    fn save_current_cont(&mut self, resume: usize) -> Result<Action, ExecError> {
        let addr = self.heap.alloc1::<OfActivation>(self.stack.len())?;

        let env = Word::pack(UnpackedWord::pointer(self.registers.envt.into_pointer()));
        let tm = Word::pack(UnpackedWord::Raw(self.registers.template as u64));
        let pc = Word::pack(UnpackedWord::Raw(resume as u64));

        let cs = match self.registers.cont {
            Some(cs_addr) => Word::pack(UnpackedWord::pointer(cs_addr.into_pointer())),
            None => Word::pack(UnpackedWord::nil()),
        };

        let mut view_mut = self.heap.get_mut(addr);
        *view_mut.envt_mut() = env;
        *view_mut.cont_mut() = cs;
        *view_mut.template_mut() = tm;
        *view_mut.pc_mut() = pc;

        {
            let (front, back) = self.stack.as_slices();
            view_mut.stack_mut()[..front.len()].copy_from_slice(front);
            view_mut.stack_mut()[front.len()..].copy_from_slice(back);
        }

        self.stack.clear();
        self.registers.cont = Some(addr);

        Ok(Action::Next)
    }

    #[inline]
    fn pop_delim(&mut self) -> Result<Option<Address<W, OfActivation>>, ExecError> {
        match self.registers.delim {
            Some(addr) => {
                let view = self.heap.get(addr);

                let cont_addr = self.heap.address(view.head().pointer().unwrap())?;
                let maybe_delim_addr = match view.tail().pointer() {
                    Some(ptr) => Some(self.heap.address(ptr)?),
                    None => {
                        assert!(view.tail().is_nil());
                        None
                    }
                };

                self.registers.delim = maybe_delim_addr;

                Ok(Some(cont_addr))
            }
            None => Ok(None),
        }
    }

    #[inline]
    fn load_current_cont(&mut self) -> Result<Action, ExecError> {
        let addr = match self.registers.cont {
            Some(addr) => addr,
            None => match self.pop_delim()? {
                Some(addr) => addr,
                None => return Ok(Action::Halt),
            },
        };

        let view = self.heap.get(addr);
        self.registers.envt = self.heap.address(view.envt().pointer().unwrap())?;
        self.registers.cont = if view.cont().is_nil() {
            None
        } else {
            Some(self.heap.address(view.cont().pointer().unwrap())?)
        };
        self.registers.template = view.template().raw().unwrap() as usize;
        self.stack.extend(view.stack().iter().cloned().rev());

        Ok(Action::JumpAbs(view.pc().raw().unwrap() as usize))
    }

    #[inline]
    fn push_current_cont(&mut self) -> Result<Action, ExecError> {
        match self.registers.cont {
            Some(addr) => {
                self.stack
                    .push_front(Word::pack(UnpackedWord::pointer(addr.into_pointer())));
                Ok(Action::Next)
            }
            None => Err(ExecError::EmptyContStack),
        }
    }

    const TRUNCATE_BITS: u32 = 64 - W::INT_SIZE;

    pub fn step(&mut self) -> Result<(), ExecError> {
        let action = match self.fetch().unpack() {
            Unpacked::Halt => Action::Halt,
            Unpacked::PushConstant(cst) => {
                let value = {
                    let Template {
                        constants, index, ..
                    } = &self.program.templates[self.registers.template];
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
            Unpacked::SaveCurrentCont(pc_resume) => self.save_current_cont(pc_resume)?,
            Unpacked::LoadCurrentCont => self.load_current_cont()?,
            Unpacked::PushCurrentCont => self.push_current_cont()?,
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
