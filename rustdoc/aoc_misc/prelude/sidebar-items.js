window.SIDEBAR_ITEMS = {"enum":[["AtomicOrdering","Atomic memory orderings"],["BTreeEntry","A view into a single entry in a map, which may either be vacant or occupied."],["Bound","An endpoint of a range of keys."],["Cow","A clone-on-write smart pointer."],["Either","The enum `Either` with variants `Left` and `Right` is a general purpose sum type with two cases."],["EitherOrBoth","Value that either holds a single A or B, or both."],["HashMapEntry","A view into a single entry in a map, which may either be vacant or occupied."],["Infallible","The error type for errors that can never happen."],["Ordering","An `Ordering` is the result of a comparison between two values."]],"fn":[["default",""],["ok_or_err",""]],"macro":[["Debug","Derive macro generating an impl of the trait `Debug`."],["Hash","Derive macro generating an impl of the trait `Hash`."],["array_vec",""],["assert_eq_ok",""],["assert_err",""],["assert_is_err",""],["assert_is_ok",""],["izip","Create an iterator running multiple iterators in lockstep."],["ok_or",""],["ok_or_else",""],["some_or",""],["wrapper_deref",""],["wrapper_deref_mut",""]],"mod":[["array","Utilities for the array primitive type."],["cmp","Utilities for comparing and ordering values."],["fmt","Utilities for formatting and printing `String`s."],["fs","Filesystem manipulation operations."],["hash","Generic hashing support."],["io","Traits, helpers, and type definitions for core I/O functionality."],["iter","Composable external iteration."],["mem","Basic functions for dealing with memory."],["ops","Overloadable operators."],["slice","Utilities for the slice primitive type."],["str","Utilities for the `str` primitive type."],["thread","Native threads."],["time","Temporal quantification."]],"struct":[["Arc","A thread-safe reference-counting pointer. ‘Arc’ stands for ‘Atomically Reference Counted’."],["ArrayVec","A vector with a fixed capacity."],["AtomicUsize","An integer type which can be safely shared between threads."],["BTreeIter","An iterator over the entries of a `BTreeMap`."],["BTreeIterMut","A mutable iterator over the entries of a `BTreeMap`."],["BTreeKeys","An iterator over the keys of a `BTreeMap`."],["BTreeMap","An ordered map based on a B-Tree."],["BTreeSet","An ordered set based on a B-Tree."],["BTreeValues","An iterator over the values of a `BTreeMap`."],["BinaryHeap","A priority queue implemented with a binary heap."],["BuildHasherDefault","Used to create a default [`BuildHasher`] instance for types that implement [`Hasher`] and [`Default`]."],["Cell","A mutable memory location."],["Chars","An iterator over the `char`s of a string slice."],["Condvar","A Condition Variable"],["DefaultHasher","The default [`Hasher`] used by [`RandomState`]."],["HashMap","A `HashMap` using `RandomState` to hash the items. (Requires the `std` feature to be enabled.)"],["HashSet","A `HashSet` using `RandomState` to hash the items. (Requires the `std` feature to be enabled.)"],["JoinHandle","An owned permission to join on a thread (block on its termination)."],["MapToIndex",""],["MultiPeek","See [`multipeek()`] for more information."],["Mutex","A mutual exclusion primitive useful for protecting shared data"],["OsString","A type that can represent owned, mutable platform-native strings, but is cheaply inter-convertible with Rust strings."],["ParseIntError","An error which can be returned when parsing an integer."],["Path","A slice of a path (akin to [`str`])."],["Peekable","An iterator with a `peek()` that returns an optional reference to the next element."],["PhantomData","Zero-sized type used to mark things that “act like” they own a `T`."],["RandomHasher","`RandomState` is the default state for [`HashMap`] types."],["Range","A (half-open) range bounded inclusively below and exclusively above (`start..end`)."],["RangeInclusive","A range bounded inclusively below and above (`start..=end`)."],["Rc","A single-threaded reference-counting pointer. ‘Rc’ stands for ‘Reference Counted’."],["RcWeak","`Weak` is a version of [`Rc`] that holds a non-owning reference to the managed allocation. The allocation is accessed by calling `upgrade` on the `Weak` pointer, which returns an [Option]<[Rc]<T>>."],["RefCell","A mutable memory location with dynamically checked borrow rules"],["SliceIter","Immutable slice iterator"],["SliceIterMut","Mutable slice iterator."],["VecDeque","A double-ended queue implemented with a growable ring buffer."],["VecIntoIter","An iterator that moves out of a vector."]],"trait":[["Add","The addition operator `+`."],["AddAssign","The addition assignment operator `+=`."],["BitAnd","The bitwise AND operator `&`."],["BitAndAssign","The bitwise AND assignment operator `&=`."],["BitOr","The bitwise OR operator `|`."],["BitOrAssign","The bitwise OR assignment operator `|=`."],["Borrow","A trait for borrowing data."],["BorrowMut","A trait for mutably borrowing data."],["BuildHasher","A trait for creating instances of [`Hasher`]."],["Debug","`?` formatting."],["Deref","Used for immutable dereferencing operations, like `*v`."],["DerefMut","Used for mutable dereferencing operations, like in `*v = 1;`."],["Display","Format trait for an empty format, `{}`."],["Div","The division operator `/`."],["Error","`Error` is a trait representing the basic expectations for error values, i.e., values of type `E` in [`Result<T, E>`]."],["FromStr","Parse a value from a string"],["FusedIterator","An iterator that always continues to yield `None` when exhausted."],["Hash","A hashable type."],["Hasher","A trait for hashing an arbitrary stream of bytes."],["Index","Used for indexing operations (`container[index]`) in immutable contexts."],["IndexMut","Used for indexing operations (`container[index]`) in mutable contexts."],["IteratorExt",""],["Itertools","An [`Iterator`] blanket implementation that provides extra adaptors and methods."],["Mul","The multiplication operator `*`."],["Neg","The unary negation operator `-`."],["RangeBounds","`RangeBounds` is implemented by Rust’s built-in range types, produced by range syntax like `..`, `a..`, `..b`, `..=c`, `d..e`, or `f..=g`."],["Rem","The remainder operator `%`."],["ResultEither",""],["Shl","The left shift operator `<<`. Note that because this trait is implemented for all integer types with multiple right-hand-side types, Rust’s type checker has special handling for `_ << _`, setting the result type for integer operations to the type of the left-hand-side operand. This means that though `a << b` and `a.shl(b)` are one and the same from an evaluation standpoint, they are different when it comes to type inference."],["ShlAssign","The left shift assignment operator `<<=`."],["Shr","The right shift operator `>>`. Note that because this trait is implemented for all integer types with multiple right-hand-side types, Rust’s type checker has special handling for `_ >> _`, setting the result type for integer operations to the type of the left-hand-side operand. This means that though `a >> b` and `a.shr(b)` are one and the same from an evaluation standpoint, they are different when it comes to type inference."],["ShrAssign","The right shift assignment operator `>>=`."],["Sub","The subtraction operator `-`."]],"type":[["GenError",""],["GenResult",""]]};