#[macro_export]
macro_rules! __layout_ignore {
    ($ignore:tt $actual:tt) => {
        $actual
    };
}

#[macro_export]
macro_rules! __layout_count {
    ($($t:tt)*) => { 0 $(+ __layout_ignore!($t 1))* };
}

#[macro_export]
macro_rules! __layout_offset {
    (@each $heap:ident $addr:ident => $field:ident : Word) => { { $addr += 1; } };
    (@each $heap:ident $addr:ident => $field:ident : Raw) => { { $addr += 1; } };
    (@each $heap:ident $addr:ident => $field:ident : [Word]) => { {
            $addr += $heap.words().get($addr).unwrap().raw().unwrap() as usize + 1;
    } };
    (@each $heap:ident $addr:ident => $field:ident : [Raw]) => { {
            $addr += $heap.words().get($addr).unwrap().raw().unwrap() as usize + 1;
    } };
    ($heap:ident, $addr:ident => $($prev_field:ident: $prev_type:tt)*) => {
        {
            let mut addr = $addr;
            if Self::NEEDS_HEADER { addr += 1; }
            $(__layout_offset!(@each $heap addr => $prev_field : $prev_type);)*
            addr
        }
    };
}

#[macro_export]
macro_rules! __layout_access {
    ([$($suffix:tt)*] $heap:ident, $addr:ident => $t:ident) => {{
        interpolate_idents! {
            $heap
                .[ words $($suffix)* ]()
                .[ get $($suffix)* ]($addr)
                .unwrap()
        }
    }};
    ([$($suffix:tt)*] $heap:ident, $addr:ident => [$t:ident]) => {{
        let len = $heap.words().get($addr).unwrap().raw().unwrap() as usize;
        interpolate_idents! {
            $heap
                .[ words $($suffix)* ]()
                .[ get $($suffix)* ]($addr + 1..$addr + 1 + len)
                .unwrap()
        }
    }};
}

#[macro_export]
macro_rules! __layout_needs_header {
    ($p:tt else $q:tt if Word $($rest:tt)*) => { __layout_needs_header!($p else $q if $($rest)*) };
    ($p:tt else $q:tt if Raw $($rest:tt)*) => { $p };
    ($p:tt else $q:tt if [Word] $($rest:tt)*) => { $p };
    ($p:tt else $q:tt if [Raw] $($rest:tt)*) => { $p };
    ($p:tt else $q:tt if) => { $q };
}

#[macro_export]
macro_rules! __layout_type {
    ($var:ident => Word) => {
        $var
    };
    ($var:ident => Raw) => {
        $var
    };
    ($var:ident => [Word]) => {
        [$var]
    };
    ($var:ident => [Raw]) => {
        [$var]
    };
}

#[macro_export]
macro_rules! __layout_field_ref {
    ($name:ident { $($prev_field:ident: $prev_type:tt)* => $field:ident : $type:tt }) => {
        impl<'a, W: Word> $name<'a, W> {
            pub fn $field(&'a self) -> &'a __layout_type!(W => $type) {
                let heap = &*self.heap;
                let addr = self.offset;
                __layout_offset!(heap, addr => $($prev_field : $prev_type)*);
                __layout_access!([] heap, addr => $type)
            }
        }
    };
}

#[macro_export]
macro_rules! __layout_field_mut {
    ($name:ident { $($prev_field:ident: $prev_type:tt)* => $field:ident : $type:tt }) => {
        impl<'a, W: Word> $name<'a, W> {
            // [$type] here because interpolate_idents! fucks things up otherwise
            interpolate_idents! {
                pub fn [ $field _mut ] (&mut self) -> &mut __layout_type!(W => [$type]) {
                    let heap = &mut *self.heap;
                    let addr = self.offset;
                    __layout_offset!(heap, addr => $($prev_field : $prev_type)*);
                    __layout_access!([[_mut]] heap, addr => [$type])
                }
            }
        }
    };
}

#[macro_export]
macro_rules! __layout_count_indices {
    (@filter Word) => { 1 };
    (@filter [Word]) => { 1 };
    (@filter Raw) => { 0 };
    (@filter [Raw]) => { 0 };
    ($($type:tt)*) => { 0 $(+ __layout_count_indices!(@filter $type))* };
}

#[macro_export]
macro_rules! __layout_alloc_args {
    (@filter ($($acc:tt,)*) [$t:ident] $($rest:tt)*) => {
        __layout_alloc_args!(@filter ($($acc,)* usize,) $($rest)*)
    };
    (@filter $acc:tt $t:ident $($rest:tt)*) => {
        __layout_alloc_args!(@filter $acc $($rest)*)
    };
    (@filter $acc:tt) => { $acc };
    ($($type:tt)*) => { __layout_alloc_args!(@filter () $($type)*) };
}

#[macro_export]
macro_rules! __layout_alloc_args_pattern {
    (@filter ($($acc:tt,)*) $field:ident : [$t:ident] $($rest:tt)*) => {
        __layout_alloc_args_pattern!(@filter ($($acc,)* $field,) $($rest)*)
    };
    (@filter $acc:tt $field:ident : $t:ident $($rest:tt)*) => {
        __layout_alloc_args_pattern!(@filter $acc $($rest)*)
    };
    (@filter $acc:tt) => { $acc };
    ($($rest:tt)*) => {
        __layout_alloc_args_pattern!(@filter () $($rest)*)
    };
}

#[macro_export]
macro_rules! __layout_alloc_size {
    (@filter $field:ident : [$t:ident]) => { 1 + $field };
    (@filter $field:ident : $t:ident) => { 1 };
    ($($field:ident : $type:tt)*) => {
        Self::NEEDS_HEADER as usize $(+ __layout_alloc_size!(@filter $field : $type))*
    };
}

#[macro_export]
macro_rules! __layout_alloc {
    (@init $slice:ident, $offset:ident => $field:ident : [$t:ident] $($rest:tt)*) => {
        $slice[$offset] = Word::pack(Unpacked::Raw($field as u64));
        let after = $offset + 1 + $field;
        __layout_alloc!(@init $slice, after => $($rest)*);
    };
    (@init $slice:ident, $offset:ident => $field:ident : $t:ident $($rest:tt)*) => {
        let after = $offset + 1;
        __layout_alloc!(@init $slice, after => $($rest)*);
    };
    (@init $slice:ident, $offset:ident => ) => {};
    ($args:ident, $heap:ident => $($field:ident : $type:tt)*) => {
        {
            let __layout_alloc_args_pattern!($($field : $type)*) = $args;
            let size = __layout_alloc_size!($($field : $type)*);
            let root = $heap.bump(size)?;

            {
                let slice = $heap.words_mut().get_mut(root..root + size).unwrap();
                let offset = Self::NEEDS_HEADER as usize;
                __layout_alloc!(@init slice, offset => $($field : $type)*);
            }

            root
        }
    };
}

#[macro_export]
macro_rules! __layout_indices {
    (@map $heap:ident, $addr:ident => $field:ident Word) => {
        let $field = WordIndex::Word($addr);
        $addr += 1;
    };
    (@map $heap:ident, $addr:ident => $field:ident [Word]) => {
        let len = $heap.words().get($addr).unwrap().raw().unwrap() as usize;
        $addr += 1;
        let $field = WordIndex::WordSlice($addr..$addr + len);
        $addr += len;
    };
    (@map $heap:ident, $addr:ident => $field:ident Raw) => {
        $addr += 1;
    };
    (@map $heap:ident, $addr:ident => $field:ident [Raw]) => {
        let len = $heap.words().get($addr).unwrap().raw().unwrap() as usize;
        $addr += 1 + len;
    };
    (@filter [$($acc:tt),*] $field:ident Word $($rest:tt)*) => {
        __layout_indices!(@filter [$($acc,)* $field] $($rest)*)
    };
    (@filter [$($acc:tt),*] $field:ident [Word] $($rest:tt)*) => {
        __layout_indices!(@filter [$($acc,)* $field] $($rest)*)
    };
    (@filter $acc:tt $field:ident Raw $($rest:tt)*) => {
        __layout_indices!(@filter $acc $($rest)*)
    };
    (@filter $acc:tt $field:ident [Raw] $($rest:tt)*) => {
        __layout_indices!(@filter $acc $($rest)*)
    };
    (@filter $acc:tt) => { $acc };
    ($heap:ident, $offset:ident => $($field:ident : $type:tt)*) => {
        {
            let mut addr = $offset;
            if Self::NEEDS_HEADER { addr += 1; }
            $(__layout_indices!(@map $heap, addr => $field $type);)*
            __layout_indices!(@filter [] $($field $type)*)
        }
    };
}

#[macro_export]
macro_rules! __layout_folded_fields_ref {
    ($name:ident $($fold:tt)*) => {
        $(
            __layout_field_ref!($name $fold);
        )*
    };
}

#[macro_export]
macro_rules! __layout_folded_fields_mut {
    ($name:ident $($fold:tt)*) => {
        $(
            __layout_field_ref!($name $fold);
            __layout_field_mut!($name $fold);
        )*
    };
}

#[macro_export]
macro_rules! __layout_fold {
    (@rec $k:tt [$($prev_field:ident : $prev_type:tt)*] [$($agg:tt)*] $field:ident : $type:tt $($rest:tt)*) => {
        __layout_fold!(@rec
            $k
            [$($prev_field: $prev_type)* $field: $type]
            [$($agg)* { $($prev_field : $prev_type)* => $field : $type }]
            $($rest)*
        );
    };
    (@rec ($k:ident!($($kargs:tt)*)) [$($field:ident : $type:tt)*] [$($agg:tt)*]) => {
        $k!($($kargs)* $($agg)*);
    };
    ($k:ident!($($kargs:tt)*) $($field:ident : $type:tt)*) => { __layout_fold!(@rec ($k!($($kargs)*)) [] [] $($field : $type)*); };
}

#[macro_export]
macro_rules! __layout_field_state {
    (Word) => {
        Field::Word
    };
    ([Word]) => {
        Field::WordSlice
    };
    (Raw) => {
        Field::Raw
    };
    ([Raw]) => {
        Field::RawSlice
    };
}

#[macro_export]
macro_rules! layout_struct {
    (@inner struct $name:tt { $($field:ident : $type:tt),* $(,)* }) => {
        interpolate_idents! {
            #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
            pub struct [ Of $name ];

            impl [ Of $name ] {
                pub const NEEDS_HEADER: bool = {
                    // Extra `[]`s due to `interpolate_idents!` shenanigans.
                    __layout_needs_header!(true else false if $([$type])*)
                };
            }

            impl<'a, W: Word> Layout<'a, W> for [ Of $name ] {
                type View = $name<'a, W>;
                type ViewMut = [ $name Mut ]<'a, W>;

                type Words = [ $name Words ]<'a, W>;
                type WordsMut = [ $name WordsMut ]<'a, W>;
                type WordIndices = [ $name WordIndices ];

                #[inline]
                fn view(&self, offset: usize, heap: &'a Heap<W>) -> Self::View {
                    $name { offset, heap }
                }

                #[inline]
                fn view_mut(&self, offset: usize, heap: &'a mut Heap<W>) -> Self::ViewMut {
                    [ $name Mut ] { offset, heap }
                }

                #[inline]
                fn words(&self, idx: usize, heap: &'a Heap<W>) -> Self::Words {
                    let start = idx;
                    // `[]`s due to `interpolate_idents!` shenanigans.
                    let end = __layout_offset!(heap, start => $($field : [$type])*);
                    let slice = heap.words().get(start + Self::NEEDS_HEADER as usize..end).unwrap();

                    [ $name Words ] { slice, state: 0 }
                }

                #[inline]
                fn words_mut(&self, idx: usize, heap: &'a mut Heap<W>) -> Self::WordsMut {
                    let start = idx;
                    // `[]`s due to `interpolate_idents!` shenanigans.
                    let end = __layout_offset!(heap, start => $($field : [$type])*);
                    let slice = heap.words_mut().get_mut(start + Self::NEEDS_HEADER as usize..end).unwrap();

                    [ $name WordsMut ] { slice, state: 0 }
                }

                #[inline]
                fn word_indices(&self, idx: usize, heap: &Heap<W>) -> Self::WordIndices {
                    // Extra `[]`s on `$type` due to `interpolate_idents!` shenanigans.
                    [ $name WordIndices ] { i: 0, indices: __layout_indices!(heap, idx => $($field : [$type])*) }
                }
            }

            #[derive(Debug)]
            pub struct [ $name Words ]<'a, W: Word> {
                slice: &'a [[W]],
                state: usize,
            }

            impl<'a, W: Word> Iterator for [ $name Words ]<'a, W> {
                type Item = &'a [[W]];

                #[inline]
                fn next(&mut self) -> Option<Self::Item> {
                    const FIELDS: &'static [[Field]] = &[$(__layout_field_state!([$type])),*];

                    FIELDS.get(self.state).and_then(|field| {
                        self.state += 1;

                        match field {
                            Field::Word => {
                                let (w, slice) = self.slice.split_at(1);
                                self.slice = slice;
                                Some(w)
                            }
                            Field::Raw => {
                                self.slice = &self.slice[1..];
                                self.next()
                            }
                            Field::WordSlice => {
                                let (fst, rest) = self.slice.split_first().unwrap();
                                let (words, slice) = rest.split_at(fst.raw().unwrap() as usize);
                                self.slice = slice;
                                Some(words)
                            }
                            Field::RawSlice => {
                                let (fst, rest) = self.slice.split_first().unwrap();
                                self.slice = &self.slice[fst.raw().unwrap() as usize..];
                                self.next()
                            }
                        }
                    })
                }
            }

            #[derive(Debug)]
            pub struct [ $name WordsMut ]<'a, W: Word> {
                slice: &'a mut [[W]],
                state: usize,
            }

            impl<'a, W: Word> Iterator for [ $name WordsMut ]<'a, W> {
                type Item = &'a mut [[W]];

                #[inline]
                fn next(&mut self) -> Option<Self::Item> {
                    // Extra `[]` for interpolate_idents! shenanigans.
                    const FIELDS: &'static [[Field]] = &[$(__layout_field_state!([$type])),*];

                    FIELDS.get(self.state).cloned().and_then(|field| {
                        self.state += 1;

                        match field {
                            Field::Word => {
                                let src = ::std::mem::replace(&mut self.slice, &mut []);
                                let (w, slice) = src.split_at_mut(1);
                                self.slice = slice;
                                Some(w)
                            }
                            Field::Raw => {
                                let src = ::std::mem::replace(&mut self.slice, &mut []);
                                self.slice = &mut src[1..];
                                self.next()
                            }
                            Field::WordSlice => {
                                let src = ::std::mem::replace(&mut self.slice, &mut []);
                                let (fst, rest) = src.split_first_mut().unwrap();
                                let (words, slice) = rest.split_at_mut(fst.raw().unwrap() as usize);
                                self.slice = slice;
                                Some(words)
                            }
                            Field::RawSlice => {
                                let src = ::std::mem::replace(&mut self.slice, &mut []);
                                let (fst, rest) = src.split_first_mut().unwrap();
                                self.slice = &mut rest[fst.raw().unwrap() as usize..];
                                self.next()
                            }
                        }
                    })
                }
            }

            #[derive(Debug, Clone)]
            pub struct [ $name WordIndices ] {
                i: usize,
                indices: [WordIndex; __layout_count_indices!($($type)*)],
            }

            impl Iterator for [ $name WordIndices ] {
                type Item = WordIndex;

                fn next(&mut self) -> Option<Self::Item> {
                    if self.i < self.indices.len() {
                        let item = self.indices[self.i].clone();
                        self.i += 1;
                        Some(item)
                    } else {
                        None
                    }
                }
            }

            #[derive(Debug)]
            pub struct [ $name Mut ]<'a, W: Word> {
                offset: usize,
                heap: &'a mut Heap<W>,
            }

            // [$type] here because $type may be surrounded in `[]` which could trick
            // interpolate_idents!.
            __layout_fold!(__layout_folded_fields_mut!([ $name Mut ]) $($field : [$type])*);
            
            impl<'a, W: Word> [ $name Mut ]<'a, W> {
                const NEEDS_HEADER: bool = [ Of $name ]::NEEDS_HEADER;

                pub fn as_slice(&self) -> &[[W]] {
                    let heap = &*self.heap;
                    let start = self.offset;
                    // `[]`s due to `interpolate_idents!` shenanigans.
                    let end = __layout_offset!(heap, start => $($field : [$type])*);
                    heap.words().get(start..end).unwrap()
                }

                pub fn as_mut_slice(&mut self) -> &mut [[W]] {
                    let heap = &mut *self.heap;
                    let start = self.offset;
                    // `[]`s due to `interpolate_idents!` shenanigans.
                    let end = __layout_offset!(heap, start => $($field : [$type])*);
                    heap.words_mut().get_mut(start..end).unwrap()
                }
            }

            impl<'a, W: Word> AsRef<[[W]]> for [ $name Mut ]<'a, W> {
                fn as_ref(&self) -> &[[W]] {
                    self.as_slice()
                }
            }

            impl<'a, W: Word> AsMut<[[W]]> for [ $name Mut ]<'a, W> {
                fn as_mut(&mut self) -> &mut [[W]] {
                    self.as_mut_slice()
                }
            }
        }

        #[derive(Debug)]
        pub struct $name<'a, W: Word> {
            offset: usize,
            heap: &'a Heap<W>,
        }

        __layout_fold!(__layout_folded_fields_ref!($name) $($field : $type)*);

        impl<'a, W: Word> $name<'a, W> {
            const NEEDS_HEADER: bool = interpolate_idents! { [ Of $name ]::NEEDS_HEADER };

            pub fn as_slice(&self) -> &[W] {
                let heap = &*self.heap;
                let start = self.offset;
                let end = __layout_offset!(heap, start => $($field : $type)*);
                heap.words().get(start..end).unwrap()
            }
        }

        impl<'a, W: Word> AsRef<[W]> for $name<'a, W> {
            fn as_ref(&self) -> &[W] {
                self.as_slice()
            }
        }
    };
    (extern struct $name:tt { $($field:ident : $type:tt),* $(,)* }) => {
        interpolate_idents! {
            impl<W: Word> Type<W> for [ Of $name ] {
                // Surround $type here because of interpolate_idents! shenanigans.
                type Alloc = __layout_alloc_args!($([$type])*);

                #[inline]
                fn alloc(alloc: Self::Alloc, heap: &mut Heap<W>) -> Result<(usize, Self), HeapError> {
                    // Surround $type here because of interpolate_idents! shenanigans.
                    let offset = __layout_alloc!(alloc, heap => $($field : [$type])*);
                    Ok((offset, [ Of $name ]))
                }

                #[inline]
                fn check<'a>(pointer: Pointer<W::Tag>, heap: &'a Heap<W>) -> Result<Self, HeapError> {
                    let header = heap.words()
                        .get(pointer.1)
                        .cloned()
                        .ok_or(HeapError::OutOfBounds)?
                        .header()
                        .ok_or(HeapError::BadHeader)?;

                    match header {
                        Header::$name => Ok([ Of $name ]),
                        _ => Err(HeapError::TagMismatch),
                    }
                }

                #[inline]
                fn tag(&self) -> Option<W::Tag> {
                    None
                }
            }
        }

        layout_struct!(@inner struct $name { $($field : $type),* });
    };
    (struct $name:tt { $($field:ident : $type:tt),* $(,)* }) => {
        interpolate_idents! {
            impl<W: Word<Tag = __Tag>> Type<W> for [ Of $name ] {
                // Surround $type here because of interpolate_idents! shenanigans.
                type Alloc = __layout_alloc_args!($([$type])*);

                #[inline]
                fn alloc(alloc: Self::Alloc, heap: &mut Heap<W>) -> Result<(usize, Self), HeapError> {
                    // Surround $type here because of interpolate_idents! shenanigans.
                    let offset = __layout_alloc!(alloc, heap => $($field : [$type])*);
                    Ok((offset, [ Of $name ]))
                }

                #[inline]
                fn check(pointer: Pointer<W::Tag>, heap: &Heap<W>) -> Result<Self, HeapError> {
                    match pointer.0 {
                        Some(__Tag::$name) => Ok([ Of $name ]),
                        None if Self::NEEDS_HEADER => {
                            let header = heap.words()
                                .get(pointer.1)
                                .cloned()
                                .ok_or(HeapError::OutOfBounds)?
                                .header()
                                .ok_or(HeapError::BadHeader)?;

                            match header {
                                Header::Tag(__Tag::$name) => Ok([ Of $name ]),
                                _ => Err(HeapError::TagMismatch),
                            }
                        }
                        _ => Err(HeapError::TagMismatch),
                    }
                }

                #[inline]
                fn tag(&self) -> Option<W::Tag> {
                    if Self::NEEDS_HEADER {
                        Some(__Tag::$name)
                    } else {
                        None
                    }
                }
            }
        }       

        layout_struct!(@inner struct $name { $($field : $type),* });
    };
}

#[macro_export]
macro_rules! __layout_def {
    (type Tag = $tag:ident; where { $(struct $name:ident $innards:tt)* }) => {
        use self::$tag as __Tag;

        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
        pub enum $tag {
            $($name,)*
        }
    };
    (type Alloc = $alloc:ident; where { $(struct $name:ident $innards:tt)* }) => {
        use self::$alloc as __Alloc;

        interpolate_idents! {
            #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
            pub enum $alloc<W: Word<Tag = __Tag>> {
                $($name(<[ Of $name ] as Type<W>>::Alloc),)*
            }
        }
    };
    (type View = $view:ident; where { $(struct $name:ident $innards:tt)* }) => {
        use self::$view as __View;

        #[derive(Debug)]
        pub enum $view<'a, W: Word> {
            $($name($name<'a, W>),)*
        }
    };
    (type ViewMut = $view_mut:ident; where { $(struct $name:ident $innards:tt)* }) => {
        use self::$view_mut as __ViewMut;

        interpolate_idents! {
            #[derive(Debug)]
            pub enum $view_mut<'a, W: Word> {
                $($name([ $name Mut ]<'a, W>),)*
            }
        }
    };
    (type Words = $words:ident; where { $(struct $name:ident $innards:tt)* }) => {
        use self::$words as __Words;

        interpolate_idents! {
            #[derive(Debug)]
            pub enum $words<'a, W: Word> {
                $($name([ $name Words ]<'a, W>),)*
            }
        }
    };
    (type WordsMut = $words_mut:ident; where { $(struct $name:ident $innards:tt)* }) => {
        use self::$words_mut as __WordsMut;

        interpolate_idents! {
            #[derive(Debug)]
            pub enum $words_mut<'a, W: Word> {
                $($name([ $name WordsMut ]<'a, W>),)*
            }
        }
    };
    (type WordIndices = $words_mut:ident; where { $(struct $name:ident $innards:tt)* }) => {
        use self::$words_mut as __WordIndices;

        interpolate_idents! {
            #[derive(Debug)]
            pub enum $words_mut {
                $($name([ $name WordIndices ]),)*
            }
        }
    };
}

#[macro_export]
macro_rules! __layout_splat_defs {
    ($(type $foo:ident = $bar:ident;)* where $stuff:tt) => {
        $(__layout_def!(type $foo = $bar; where $stuff);)*
    };
}

#[macro_export]
macro_rules! layout {
    ($(type $foo:ident = $bar:ident;)* where { $(struct $name:ident { $($field:ident : $type:tt),* $(,)* })+ }) => {
        __layout_splat_defs!($(type $foo = $bar;)* where { $(struct $name { $($field : $type)* })* });
        $(layout_struct!(struct $name { $($field : $type,)* });)*

        interpolate_idents! {
            impl<'a, W: Word<Tag = __Tag>> Layout<'a, W> for __Tag {
                type View = __View<'a, W>;
                type ViewMut = __ViewMut<'a, W>;

                type Words = __Words<'a, W>;
                type WordsMut = __WordsMut<'a, W>;
                type WordIndices = __WordIndices;

                #[inline]
                fn view(&self, idx: usize, heap: &'a Heap<W>) -> Self::View {
                    match self {
                        $(__Tag::$name => __View::$name([ Of $name ].view(idx, heap)),)*
                    }
                }

                #[inline]
                fn view_mut(&self, idx: usize, heap: &'a mut Heap<W>) -> Self::ViewMut {
                    match self {
                        $(__Tag::$name => __ViewMut::$name([ Of $name ].view_mut(idx, heap)),)*
                    }
                }

                #[inline]
                fn words(&self, idx: usize, heap: &'a Heap<W>) -> Self::Words {
                    unimplemented!()
                }
                
                #[inline]
                fn words_mut(&self, idx: usize, heap: &'a mut Heap<W>) -> Self::WordsMut {
                    unimplemented!()
                }

                #[inline]
                fn word_indices(&self, idx: usize, heap: &Heap<W>) -> Self::WordIndices {
                    unimplemented!()
                }
            }

            impl<W: Word<Tag = __Tag>> Type<W> for __Tag {
                type Alloc = __Alloc<W>;

                #[inline]
                fn alloc(alloc: Self::Alloc, heap: &mut Heap<W>) -> Result<(usize, Self), HeapError> {
                    match alloc {
                        $(__Alloc::$name(args) => {
                            let (offset, tag) = [ Of $name ]::alloc(args, heap)?;
                            Ok((offset, __Tag::$name))
                        })*
                    }
                }

                #[inline]
                fn check(pointer: Pointer<W::Tag>, heap: &Heap<W>) -> Result<Self, HeapError> {
                    match pointer.0 {
                        Some(tag) => Ok(tag),
                        None => {
                            let header = pointer.1
                                .checked_sub(1)
                                .and_then(|addr| heap.words().get(addr).cloned())
                                .ok_or(HeapError::OutOfBounds)?
                                .header()
                                .ok_or(HeapError::BadHeader)?;

                            match header {
                                Header::Tag(tag) => Ok(tag),
                                _ => Err(HeapError::TagMismatch),
                            }
                        }
                    }
                }

                #[inline]
                fn tag(&self) -> Option<W::Tag> {
                    match self {
                        $(__Tag::$name => <[ Of $name ] as Type<W>>.tag(),)*
                    }
                }
            }
        }

        impl<'a, W: Word<Tag = __Tag>> AsRef<[W]> for __View<'a, W> {
            fn as_ref(&self) -> &[W] {
                match self {
                    $(__View::$name(view) => view.as_ref(),)*
                }
            }
        }

        impl<'a, W: Word<Tag = __Tag>> AsRef<[W]> for __ViewMut<'a, W> {
            fn as_ref(&self) -> &[W] {
                match self {
                    $(__ViewMut::$name(view) => view.as_ref(),)*
                }
            }
        }

        impl<'a, W: Word<Tag = __Tag>> AsMut<[W]> for __ViewMut<'a, W> {
            fn as_mut(&mut self) -> &mut [W] {
                match self {
                    $(__ViewMut::$name(view) => view.as_mut(),)*
                }
            }
        }

        impl<'a, W: Word> Iterator for __Words<'a, W> {
            type Item = &'a [W];

            fn next(&mut self) -> Option<Self::Item> {
                match self {
                    $(__Words::$name(iter) => iter.next(),)*
                }
            }
        }

        impl<'a, W: Word> Iterator for __WordsMut<'a, W> {
            type Item = &'a mut [W];

            fn next(&mut self) -> Option<Self::Item> {
                match self {
                    $(__WordsMut::$name(iter) => iter.next(),)*
                }
            }
        }

        impl Iterator for __WordIndices {
            type Item = WordIndex;

            fn next(&mut self) -> Option<Self::Item> {
                match self {
                    $(__WordIndices::$name(iter) => iter.next(),)*
                }
            }
        }
    };
}
