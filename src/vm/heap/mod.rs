#[macro_use]
mod macros;

use std::{iter, marker::PhantomData, ops::{Index, IndexMut, Range}};

use smallvec::SmallVec;

#[doc(hidden)]
#[derive(Debug, Clone, Copy)]
pub enum Field {
    Word,
    WordSlice,
    Raw,
    RawSlice,
}

#[derive(Debug, Fail)]
pub enum HeapError {
    #[fail(display = "tag mismatch")]
    TagMismatch,

    #[fail(display = "expected header for untagged allocation requiring header, found non-header word")]
    BadHeader,

    #[fail(display = "expected a non-raw word, found raw word")]
    BadWord,

    #[fail(display = "expected a value, found a non-value")]
    BadValue,

    #[fail(display = "expected an integer")]
    BadInteger,

    #[fail(display = "access out of bounds")]
    OutOfBounds,

    #[fail(display = "out of memory")]
    OutOfMemory,
}

layout_struct! {
    extern struct Activation {
        env: Word,
        cs: Word,
        tm: Raw,
        pc: Raw,

        stack: [Word],
    }
}

layout_struct! {
    extern struct Environment {
        parent: Word,
        locals: [Word],
    }
}

layout_struct! {
    extern struct Closure {
        environment: Word,
        template: Raw,
    }
}

layout_struct! {
    extern struct Cons {
        head: Word,
        tail: Word,
    }
}

/// Trait for types usable as "machine words" for the VM.
pub trait Word: Default + Copy + Eq + 'static {
    const INT_SIZE: u32;

    type Tag: Copy + Eq + 'static;

    fn pack(Unpacked<Self::Tag>) -> Self;
    fn unpack(self) -> Unpacked<Self::Tag>;

    /// Try to interpret the word as a value.
    fn value(self) -> Option<Value<Self::Tag>> {
        match self.unpack() {
            Unpacked::Value(value) => Some(value),
            Unpacked::Header(_) | Unpacked::Raw(_) => None,
        }
    }

    /// Try to interpret the word as an immediate.
    fn immediate(self) -> Option<Immediate> {
        match self.value() {
            Some(Value::Immediate(imm)) => Some(imm),
            _ => None,
        }
    }

    /// Try to interpret the word as an integer.
    fn integer(self) -> Option<i64> {
        match self.immediate() {
            Some(Immediate::Int(i)) => Some(i),
            _ => None,
        }
    }

    /// Try to interpret the word as a pointer. This is a convenience function implemented in terms
    /// of value.
    fn pointer(self) -> Option<Pointer<Self::Tag>> {
        match self.value() {
            Some(Value::Pointer(ptr)) => Some(ptr),
            _ => None,
        }
    }

    /// Try to interpret the word as a header. This might fail in conditions like debug mode, where
    /// headers are distinguishable from other values.
    fn header(self) -> Option<Header<Self::Tag>> {
        match self.unpack() {
            Unpacked::Value(_) | Unpacked::Raw(_) => None,
            Unpacked::Header(hdr) => Some(hdr),
        }
    }

    /// Try to interpret the word as a raw value. This might fail in conditions like debug mode,
    /// where raw words are safely encoded and can be detected.
    fn raw(self) -> Option<u64>;
}

/// Headers may be found preceding allocations. They provide information on what sort of data
/// is allocated following them, providing type information to the runtime.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Header<T> {
    /// A header indicating that the allocation here has been moved to another heap. This header
    /// also contains the pointer that the allocation was moved to.
    Moved(usize),

    /// An environment roughly corresponds to a lexical scope in source code.
    ///
    /// ``` ignore
    /// [Header::Environment, parent, n, local[0], local[1], .. local[n]].len() == n + 3
    /// ```
    Environment,

    /// An activation record is a "live" continuation; everything needed to completely update the
    /// state of the VM minus the heap. An old stack plus saved registers.
    Activation,

    /// A closure is a function template plus a pointer to a closed-over environment.
    ///
    /// ``` ignore
    /// [Header::Closure, environment, template].len() == 3
    /// ```
    Closure,

    /// A header indicating the type of an allocation.
    Tag(T),
}

/// Immediate values are values which are stored outside of allocations, for example small
/// integers and booleans. These types are small enough that they don't need to be boxed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Immediate {
    Int(i64),
    Bool(bool),
    Nil,
}

/// A pointer value consists of a tag, providing type information about a boxed value, and of
/// course the location of the value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pointer<T>(pub Option<T>, pub usize);

/// "Values" are words which represent runtime values. This is opposed to headers and "raw" words
/// which represent no structured data.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Value<T> {
    Immediate(Immediate),
    Pointer(Pointer<T>),
}

/// `UnpackedWord` also includes a `Raw` variant; this is because `UnpackedWord` is a valid `Word`
/// for running the VM with, and without a `Raw` variant `UnpackedWord` represents only valid words
/// which cannot be interpreted safely as raw data. This also means that when running the VM with
/// `UnpackedWord`s instead of packed 32-bit or 64-bit words, we can detect misinterpretation of
/// values as raw data and vice versa.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Unpacked<T> {
    Value(Value<T>),
    Header(Header<T>),

    /// A "raw" word is "just data", used for defining builtins and primitives in the VM; for
    /// example, in an allocated vector, the first word of the allocation might be a raw length.
    /// This means that it is encoded directly as an integer rather than as an immediate value.
    Raw(u64),
}

impl<T> Default for Unpacked<T> {
    fn default() -> Self {
        Unpacked::Raw(0)
    }
}

impl<T> Word for Unpacked<T>
where
    T: Copy + Eq + 'static,
{
    const INT_SIZE: u32 = 64;
    type Tag = T;

    fn pack(this: Self) -> Self {
        this
    }

    fn unpack(self) -> Self {
        self
    }

    fn raw(self) -> Option<u64> {
        match self {
            Unpacked::Raw(raw) => Some(raw),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WordIndex {
    Word(usize),
    WordSlice(Range<usize>),
}

pub trait Layout<'a, W: Word>: Sized {
    type View: AsRef<[W]>;
    type ViewMut: AsRef<[W]> + AsMut<[W]>;

    type Words: IntoIterator<Item = &'a [W]>;
    type WordsMut: IntoIterator<Item = &'a mut [W]>;
    type WordIndices: IntoIterator<Item = WordIndex>;

    fn view(&self, idx: usize, heap: &'a Heap<W>) -> Self::View;
    fn view_mut(&self, idx: usize, heap: &'a mut Heap<W>) -> Self::ViewMut;

    fn words(&self, idx: usize, heap: &'a Heap<W>) -> Self::Words;
    fn words_mut(&self, idx: usize, heap: &'a mut Heap<W>) -> Self::WordsMut;
    fn word_indices(&self, idx: usize, heap: &Heap<W>) -> Self::WordIndices;
}

pub trait Type<W: Word>: for<'a> Layout<'a, W> {
    type Alloc;

    fn alloc(Self::Alloc, &mut Heap<W>) -> Result<(usize, Self), HeapError>;
    fn check(Pointer<W::Tag>, &Heap<W>) -> Result<Self, HeapError>;
    fn tag(&self) -> Option<W::Tag>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Address<W: Word, Of: Type<W>>(usize, Of, PhantomData<W>);

impl<W: Word, Of: Type<W>> Address<W, Of> {
    pub fn into_pointer(self) -> Pointer<W::Tag> {
        Pointer(self.1.tag(), self.0)
    }
}

#[derive(Debug, Clone)]
pub struct Heap<W: Word> {
    words: Vec<W>,
    top: usize,
}

impl<W: Word> Heap<W> {
    pub fn with_size(size: usize) -> Self {
        let mut words = Vec::new();
        words.resize(size, W::default());

        Self { words, top: 0 }
    }

    pub fn from_words(words: Vec<W>, top: usize) -> Self {
        Self { words, top }
    }

    pub fn bump(&mut self, len: usize) -> Result<usize, HeapError> {
        if self.top + len <= self.words.len() {
            let offset = self.top;
            self.top += len;
            Ok(offset)
        } else {
            Err(HeapError::OutOfMemory)
        }
    }

    pub fn alloc_raw(&mut self, words: &[W]) -> Result<usize, HeapError> {
        let start = self.bump(words.len())?;
        self.words[start..self.top].copy_from_slice(words);
        Ok(start)
    }

    pub fn alloc_with<Of: Type<W>>(
        &mut self,
        args: Of::Alloc,
    ) -> Result<Address<W, Of>, HeapError> {
        let (offset, of) = Of::alloc(args, self)?;
        Ok(Address(offset, of, PhantomData))
    }

    pub fn alloc0<Of: Type<W, Alloc = ()>>(&mut self) -> Result<Address<W, Of>, HeapError> {
        self.alloc_with(())
    }

    pub fn alloc1<Of: Type<W, Alloc = (usize,)>>(
        &mut self,
        size: usize,
    ) -> Result<Address<W, Of>, HeapError> {
        self.alloc_with((size,))
    }

    pub fn address<Of: Type<W>>(&self, addr: Pointer<W::Tag>) -> Result<Address<W, Of>, HeapError> {
        let of = Of::check(addr, self)?;
        Ok(Address(addr.1, of, PhantomData))
    }

    pub fn get<'a, Of: Type<W>>(&'a self, addr: Address<W, Of>) -> <Of as Layout<'a, W>>::View {
        addr.1.view(addr.0, self)
    }

    pub fn get_mut<'a, Of: Type<W>>(
        &'a mut self,
        addr: Address<W, Of>,
    ) -> <Of as Layout<'a, W>>::ViewMut {
        addr.1.view_mut(addr.0, self)
    }

    pub fn words(&self) -> &[W] {
        &self.words
    }

    pub fn words_mut(&mut self) -> &mut [W] {
        &mut self.words
    }
}

/// A non-mutating deep copy. This is a less efficient version of `DeepCopyMut` intended for when
/// the from-space must be preserved and cannot be mutated. It keeps a mapping from previously
/// visited addresses to moved addresses in the form of an association list.
///
/// For a more efficient version when the from-space can be mutated and then thrown away or
/// recycled afterwards, see `DeepCopyMut`.
#[derive(Debug)]
pub struct DeepCopy<'a, W: Word> {
    from: &'a Heap<W>,
    to: &'a mut Heap<W>,

    scan: usize,
    visited: SmallVec<[(usize, usize); 16]>,
}

impl<'a, W: Word> DeepCopy<'a, W>
where
    W::Tag: Type<W>,
{
    pub fn new(from: &'a Heap<W>, to: &'a mut Heap<W>) -> Self {
        let scan = to.top; // Scan new allocations only.
        Self {
            from,
            to,
            scan,
            visited: SmallVec::new(),
        }
    }

    fn move_object<T: Type<W>>(&mut self, addr: usize, ty: T) -> Result<usize, HeapError> {
        let new_addr = {
            let view = ty.view(addr, self.from);
            let slice = view.as_ref();
            self.to.alloc_raw(slice)?
        };
        self.visited.push((addr, new_addr));

        Ok(new_addr)
    }

    fn scan_pointer(&mut self, ptr: Pointer<W::Tag>) -> Result<Pointer<W::Tag>, HeapError> {
        if let Some(&addr) = self.visited.iter().find(|(from, to)| from == &ptr.1) {
            return Ok(Pointer(ptr.0, addr.1));
        }

        let maybe_header = self.from
            .words()
            .get(ptr.1)
            .ok_or(HeapError::OutOfBounds)?
            .header();

        match maybe_header {
            Some(Header::Environment) => self.move_object(ptr.1, OfEnvironment)
                .map(|addr| Pointer(None, addr)),
            Some(Header::Activation) => self.move_object(ptr.1, OfActivation)
                .map(|addr| Pointer(None, addr)),
            Some(Header::Closure) => self.move_object(ptr.1, OfClosure)
                .map(|addr| Pointer(None, addr)),
            Some(Header::Tag(tag)) => {
                assert!(ptr.0.is_none());
                self.move_object(ptr.1, tag).map(|addr| Pointer(None, addr))
            }
            None => {
                assert!(ptr.0.is_some());
                self.move_object(ptr.1, ptr.0.unwrap())
                    .map(|addr| Pointer(ptr.0, addr))
            }

            Some(Header::Moved(_)) => unreachable!("deep copy cannot modify from space"),
        }
    }

    fn scan_indices<I: IntoIterator<Item = usize>>(&mut self, indices: I) -> Result<(), HeapError> {
        for idx in indices {
            if let Unpacked::Value(Value::Pointer(ptr)) = self.to.words[idx].unpack() {
                self.to.words[idx] =
                    W::pack(Unpacked::Value(Value::Pointer(self.scan_pointer(ptr)?)));
            }
        }

        Ok(())
    }

    fn scan_header<T: Type<W>>(&mut self, hdr: T) -> Result<usize, HeapError> {
        let len = hdr.view(self.scan, self.to).as_ref().len();
        let indices = hdr.word_indices(self.scan, &self.to);

        for word_index in indices {
            match word_index {
                WordIndex::Word(i) => self.scan_indices(iter::once(i))?,
                WordIndex::WordSlice(r) => self.scan_indices(r)?,
            }
        }

        Ok(len)
    }

    fn scan_object(&mut self) -> Result<usize, HeapError> {
        match self.to.words[self.scan].unpack() {
            Unpacked::Value(value) => {
                if let Value::Pointer(ptr) = value {
                    self.to.words[self.scan] =
                        W::pack(Unpacked::Value(Value::Pointer(self.scan_pointer(ptr)?)));
                }

                Ok(1)
            }
            Unpacked::Header(hdr) => match hdr {
                Header::Moved(_) => unreachable!("`Moved` in to-space during collection"),
                Header::Environment => self.scan_header(OfEnvironment),
                Header::Activation => self.scan_header(OfActivation),
                Header::Closure => self.scan_header(OfClosure),
                Header::Tag(tag) => self.scan_header(tag),
            },
            Unpacked::Raw(_) => Err(HeapError::BadWord),
        }
    }

    pub fn root(&mut self, word: W) -> Result<W, HeapError> {
        match word.value() {
            Some(Value::Pointer(ptr)) => {
                let new_ptr = self.scan_pointer(ptr)?;
                while self.scan < self.to.top {
                    self.scan += self.scan_object()?;
                }

                Ok(W::pack(Unpacked::Value(Value::Pointer(new_ptr))))
            }
            Some(Value::Immediate(_)) => Ok(word),
            None => Err(HeapError::BadValue),
        }
    }
}

/// A mutating deep copy, essentially a small copying garbage collection pass.
///
/// Unlike `DeepCopy`, this scanning copy algorithm will use the from-space itself as a visited
/// set, overwriting objects at addresses which have already been scanned with `Moved` headers.
/// This is more efficient than `DeepCopy` and also a good deal more clever. Isn't copying garbage
/// collection cool?
#[derive(Debug)]
pub struct DeepCopyMut<'a, W: Word> {
    from: &'a mut Heap<W>,
    to: &'a mut Heap<W>,

    scan: usize,
}

impl<'a, W: Word> DeepCopyMut<'a, W>
where
    W::Tag: Type<W>,
{
    pub fn new(from: &'a mut Heap<W>, to: &'a mut Heap<W>) -> Self {
        Self { from, to, scan: 0 }
    }

    fn move_object<T: Type<W>>(&mut self, addr: usize, ty: T) -> Result<usize, HeapError> {
        let new_addr = {
            let mut view = ty.view(addr, self.from);
            let slice = view.as_ref();
            self.to.alloc_raw(slice)?
        };
        self.from.words[addr] = W::pack(Unpacked::Header(Header::Moved(new_addr)));

        Ok(new_addr)
    }

    fn scan_pointer(&mut self, ptr: Pointer<W::Tag>) -> Result<Pointer<W::Tag>, HeapError> {
        let maybe_header = self.from
            .words()
            .get(ptr.1)
            .ok_or(HeapError::OutOfBounds)?
            .header();

        match maybe_header {
            Some(Header::Moved(offset)) => Ok(Pointer(ptr.0, offset)),
            Some(Header::Environment) => self.move_object(ptr.1, OfEnvironment)
                .map(|addr| Pointer(None, addr)),
            Some(Header::Activation) => self.move_object(ptr.1, OfActivation)
                .map(|addr| Pointer(None, addr)),
            Some(Header::Closure) => self.move_object(ptr.1, OfClosure)
                .map(|addr| Pointer(None, addr)),
            Some(Header::Tag(tag)) => {
                assert!(ptr.0.is_none());
                self.move_object(ptr.1, tag).map(|addr| Pointer(None, addr))
            }
            None => {
                assert!(ptr.0.is_some());
                self.move_object(ptr.1, ptr.0.unwrap())
                    .map(|addr| Pointer(ptr.0, addr))
            }
        }
    }

    fn scan_indices<I: IntoIterator<Item = usize>>(&mut self, indices: I) -> Result<(), HeapError> {
        for idx in indices {
            if let Unpacked::Value(Value::Pointer(ptr)) = self.to.words[idx].unpack() {
                self.to.words[idx] =
                    W::pack(Unpacked::Value(Value::Pointer(self.scan_pointer(ptr)?)));
            }
        }

        Ok(())
    }

    fn scan_header<T: Type<W>>(&mut self, hdr: T) -> Result<usize, HeapError> {
        let len = hdr.view(self.scan, &mut self.to).as_ref().len();
        let indices = hdr.word_indices(self.scan, &self.to);

        for word_index in indices {
            match word_index {
                WordIndex::Word(i) => self.scan_indices(iter::once(i))?,
                WordIndex::WordSlice(r) => self.scan_indices(r)?,
            }
        }

        Ok(len)
    }

    fn scan_object(&mut self) -> Result<usize, HeapError> {
        println!("scanning at: {:?}", self.scan);
        match self.to.words[self.scan].unpack() {
            Unpacked::Value(value) => {
                println!("value found");
                if let Value::Pointer(ptr) = value {
                    println!("it's a pointer.");
                    self.to.words[self.scan] =
                        W::pack(Unpacked::Value(Value::Pointer(self.scan_pointer(ptr)?)));
                }

                Ok(1)
            }
            Unpacked::Header(hdr) => match hdr {
                Header::Moved(_) => unreachable!("`Moved` in to-space during collection"),
                Header::Environment => self.scan_header(OfEnvironment),
                Header::Activation => self.scan_header(OfActivation),
                Header::Closure => self.scan_header(OfClosure),
                Header::Tag(tag) => self.scan_header(tag),
            },
            Unpacked::Raw(_) => Err(HeapError::BadWord),
        }
    }

    pub fn root(&mut self, word: W) -> Result<W, HeapError> {
        match word.value() {
            Some(Value::Pointer(ptr)) => {
                let new_ptr = self.scan_pointer(ptr)?;
                while self.scan < self.to.top {
                    self.scan += self.scan_object()?;
                }

                Ok(W::pack(Unpacked::Value(Value::Pointer(new_ptr))))
            }
            Some(Value::Immediate(_)) => Ok(word),
            None => Err(HeapError::BadValue),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn environment_needs_header() {
        assert!(OfEnvironment::NEEDS_HEADER);
    }

    #[test]
    fn closure_needs_header() {
        assert!(OfClosure::NEEDS_HEADER);
    }

    #[test]
    fn cons_needs_header() {
        assert!(!OfCons::NEEDS_HEADER);
    }

    layout! {
        type Tag = Tag;
        type Alloc = Alloc;

        type View = View;
        type ViewMut = ViewMut;

        type Words = Words;
        type WordsMut = WordsMut;

        type WordIndices = WordIndices;

        where {
            struct Cons {
                head: Word,
                tail: Word,
            }
        }
    }

    const TEST_WORDS: &'static [Unpacked<Tag>] = &[
        // Pointer(None, 0)
        Unpacked::Header(Header::Environment),
        Unpacked::Value(Value::Pointer(Pointer(None, 0))),
        Unpacked::Raw(3),
        Unpacked::Value(Value::Immediate(Immediate::Int(0))),
        Unpacked::Value(Value::Immediate(Immediate::Int(1))),
        Unpacked::Value(Value::Immediate(Immediate::Int(2))),
        // Pointer(Some(Tag::Cons), 6)
        Unpacked::Value(Value::Immediate(Immediate::Bool(false))),
        Unpacked::Value(Value::Immediate(Immediate::Bool(true))),
        // Pointer(None, 8)
        Unpacked::Header(Header::Closure),
        Unpacked::Value(Value::Pointer(Pointer(None, 0))),
        Unpacked::Raw(0),
        // Pointer(Some(Tag::Cons), 11)
        Unpacked::Value(Value::Pointer(Pointer(None, 8))),
        Unpacked::Value(Value::Immediate(Immediate::Nil)),
    ];

    const TEST_WORDS_FROM: &'static [Unpacked<Tag>] = &[
        // Pointer(None, 0)
        Unpacked::Header(Header::Moved(5)),
        Unpacked::Value(Value::Pointer(Pointer(None, 0))),
        Unpacked::Raw(3),
        Unpacked::Value(Value::Immediate(Immediate::Int(0))),
        Unpacked::Value(Value::Immediate(Immediate::Int(1))),
        Unpacked::Value(Value::Immediate(Immediate::Int(2))),
        // Pointer(Some(Tag::Cons), 6)
        Unpacked::Value(Value::Immediate(Immediate::Bool(false))),
        Unpacked::Value(Value::Immediate(Immediate::Bool(true))),
        // Pointer(None, 8)
        Unpacked::Header(Header::Moved(2)),
        Unpacked::Value(Value::Pointer(Pointer(None, 0))),
        Unpacked::Raw(0),
        // Pointer(Some(Tag::Cons), 11)
        Unpacked::Header(Header::Moved(0)),
        Unpacked::Value(Value::Immediate(Immediate::Nil)),
    ];

    const TEST_WORDS_TO: &'static [Unpacked<Tag>] = &[
        // Pointer(Some(Tag::Cons), 0)
        Unpacked::Value(Value::Pointer(Pointer(None, 2))),
        Unpacked::Value(Value::Immediate(Immediate::Nil)),
        // Pointer(None, 2)
        Unpacked::Header(Header::Closure),
        Unpacked::Value(Value::Pointer(Pointer(None, 5))),
        Unpacked::Raw(0),
        // Pointer(None, 5)
        Unpacked::Header(Header::Environment),
        Unpacked::Value(Value::Pointer(Pointer(None, 5))),
        Unpacked::Raw(3),
        Unpacked::Value(Value::Immediate(Immediate::Int(0))),
        Unpacked::Value(Value::Immediate(Immediate::Int(1))),
        Unpacked::Value(Value::Immediate(Immediate::Int(2))),
    ];

    #[test]
    fn deep_copy_mut_test_words() {
        let mut from = Heap::from_words(TEST_WORDS.into(), TEST_WORDS.len());
        let mut to = Heap::with_size(11);

        let result = {
            let mut copy = DeepCopyMut::new(&mut from, &mut to);
            copy.root(Unpacked::Value(Value::Pointer(Pointer(
                Some(Tag::Cons),
                11,
            ))))
        };

        match result {
            Ok(word) => assert_eq!(
                word,
                Unpacked::Value(Value::Pointer(Pointer(Some(Tag::Cons), 0)))
            ),
            Err(err) => {
                panic!("aaaaa {}\nFROM: {:?}\nTO: {:?}\n", err, from, to);
            }
        }

        assert_eq!(from.words(), TEST_WORDS_FROM);
        assert_eq!(to.words(), TEST_WORDS_TO);
    }

    #[test]
    fn environment_view_as_ref() {
        let heap = Heap::from_words(TEST_WORDS.into(), TEST_WORDS.len());
        let addr = heap.address::<OfEnvironment>(Pointer(None, 0)).unwrap();
        let view = heap.get(addr);

        assert_eq!(view.as_ref(), &TEST_WORDS[0..6]);
    }

    #[test]
    fn environment_view_mut_as_ref() {
        let mut heap = Heap::from_words(TEST_WORDS.into(), TEST_WORDS.len());
        let addr = heap.address::<OfEnvironment>(Pointer(None, 0)).unwrap();
        let view_mut = heap.get_mut(addr);

        assert_eq!(view_mut.as_ref(), &TEST_WORDS[0..6]);
    }

    #[test]
    fn environment_view_mut_as_mut() {
        let mut heap = Heap::from_words(TEST_WORDS.into(), TEST_WORDS.len());
        let addr = heap.address::<OfEnvironment>(Pointer(None, 0)).unwrap();
        let mut view_mut = heap.get_mut(addr);

        assert_eq!(view_mut.as_mut(), &TEST_WORDS[0..6]);
    }

    #[test]
    fn environment_words() {
        let heap = Heap::from_words(TEST_WORDS.into(), TEST_WORDS.len());
        let mut words = OfEnvironment.words(0, &heap);

        assert_eq!(words.next(), Some(&TEST_WORDS[1..2]));
        assert_eq!(words.next(), Some(&TEST_WORDS[3..6]));
        assert_eq!(words.next(), None);
    }

    #[test]
    fn environment_words_mut() {
        let mut heap = Heap::from_words(TEST_WORDS.into(), TEST_WORDS.len());
        let mut words = OfEnvironment.words_mut(0, &mut heap);

        assert_eq!(words.next().map(|t| &*t), Some(&TEST_WORDS[1..2]));
        assert_eq!(words.next().map(|t| &*t), Some(&TEST_WORDS[3..6]));
        assert_eq!(words.next(), None);
    }

    #[test]
    fn environment_word_indices() {
        let heap = Heap::from_words(TEST_WORDS.into(), TEST_WORDS.len());
        let mut indices = OfEnvironment.word_indices(0, &heap);

        assert_eq!(indices.next(), Some(WordIndex::Word(1)));
        assert_eq!(indices.next(), Some(WordIndex::WordSlice(3..6)));
        assert_eq!(indices.next(), None);
    }

    #[test]
    fn closure_view_as_ref() {
        let heap = Heap::from_words(TEST_WORDS.into(), TEST_WORDS.len());
        let addr = heap.address::<OfClosure>(Pointer(None, 8)).unwrap();
        let view = heap.get(addr);

        assert_eq!(view.as_ref(), &TEST_WORDS[8..11]);
    }

    #[test]
    fn closure_view_mut_as_ref() {
        let mut heap = Heap::from_words(TEST_WORDS.into(), TEST_WORDS.len());
        let addr = heap.address::<OfClosure>(Pointer(None, 8)).unwrap();
        let view = heap.get_mut(addr);

        assert_eq!(view.as_ref(), &TEST_WORDS[8..11]);
    }

    #[test]
    fn closure_view_mut_as_mut() {
        let mut heap = Heap::from_words(TEST_WORDS.into(), TEST_WORDS.len());
        let addr = heap.address::<OfClosure>(Pointer(None, 8)).unwrap();
        let mut view = heap.get_mut(addr);

        assert_eq!(view.as_mut(), &TEST_WORDS[8..11]);
    }

    #[test]
    fn closure_words() {
        let heap = Heap::from_words(TEST_WORDS.into(), TEST_WORDS.len());
        let mut words = OfClosure.words(8, &heap);

        assert_eq!(words.next(), Some(&TEST_WORDS[9..10]));
        assert_eq!(words.next(), None);
    }

    #[test]
    fn closure_words_mut() {
        let mut heap = Heap::from_words(TEST_WORDS.into(), TEST_WORDS.len());
        let mut words = OfClosure.words_mut(8, &mut heap);

        assert_eq!(words.next().map(|t| &*t), Some(&TEST_WORDS[9..10]));
        assert_eq!(words.next(), None);
    }

    #[test]
    fn closure_word_indices() {
        let heap = Heap::from_words(TEST_WORDS.into(), TEST_WORDS.len());
        let mut indices = OfClosure.word_indices(8, &heap);

        assert_eq!(indices.next(), Some(WordIndex::Word(9)));
        assert_eq!(indices.next(), None);
    }

    #[test]
    fn cons_view_as_ref() {
        let heap = Heap::from_words(TEST_WORDS.into(), TEST_WORDS.len());
        let addr = heap.address::<OfCons>(Pointer(Some(Tag::Cons), 11))
            .unwrap();
        let view = heap.get(addr);

        assert_eq!(view.as_ref(), &TEST_WORDS[11..13]);
    }

    #[test]
    fn cons_view_mut_as_ref() {
        let mut heap = Heap::from_words(TEST_WORDS.into(), TEST_WORDS.len());
        let addr = heap.address::<OfCons>(Pointer(Some(Tag::Cons), 11))
            .unwrap();
        let view = heap.get_mut(addr);

        assert_eq!(view.as_ref(), &TEST_WORDS[11..13]);
    }

    #[test]
    fn cons_view_mut_as_mut() {
        let mut heap = Heap::from_words(TEST_WORDS.into(), TEST_WORDS.len());
        let addr = heap.address::<OfCons>(Pointer(Some(Tag::Cons), 11))
            .unwrap();
        let mut view = heap.get_mut(addr);

        assert_eq!(view.as_mut(), &TEST_WORDS[11..13]);
    }

    #[test]
    fn cons_words() {
        let heap = Heap::from_words(TEST_WORDS.into(), TEST_WORDS.len());
        let mut words = OfCons.words(11, &heap);

        assert_eq!(words.next(), Some(&TEST_WORDS[11..12]));
        assert_eq!(words.next(), Some(&TEST_WORDS[12..13]));
        assert_eq!(words.next(), None);
    }

    #[test]
    fn cons_words_mut() {
        let mut heap = Heap::from_words(TEST_WORDS.into(), TEST_WORDS.len());
        let mut words = OfCons.words_mut(11, &mut heap);

        assert_eq!(words.next().map(|t| &*t), Some(&TEST_WORDS[11..12]));
        assert_eq!(words.next().map(|t| &*t), Some(&TEST_WORDS[12..13]));
        assert_eq!(words.next(), None);
    }

    #[test]
    fn cons_word_indices() {
        let heap = Heap::from_words(TEST_WORDS.into(), TEST_WORDS.len());
        let mut indices = OfCons.word_indices(11, &heap);

        assert_eq!(indices.next(), Some(WordIndex::Word(11)));
        assert_eq!(indices.next(), Some(WordIndex::Word(12)));
        assert_eq!(indices.next(), None);
    }
}
