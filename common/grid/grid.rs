use std::fmt::Debug;
use std::iter;
use std::marker::PhantomData;
use std::slice::Iter as SliceIter;

use aoc_bitvec as bitvec;
use aoc_misc::*;
use aoc_nums as nums;
use aoc_parser::*;
use aoc_pos as pos;
use bitvec::BitVec;
use bitvec::BitVecEncoding;
use bitvec::BitVecIter;
use nums::Int;
use nums::IntConv;
use nums::Overflow;
use pos::PosRowCol;
use pos::PosXY;
use pos::PosYX;

/// Multi dimensional abstraction over a single dimensional collection
///
/// This allows a simple one-dimensional collection to be accessed in a multi-dimensional way. It
/// wraps a backing collection which implements [`GridStorage`], and takes indexes which implement
/// [`GridPos`].

#[ derive (Clone, Debug, Eq, PartialEq) ]
pub struct Grid <Storage, Pos, const DIMS: usize = 2> {
	storage: Storage,
	origin: [isize; DIMS],
	size: [usize; DIMS],
	phantom: PhantomData <Pos>,
}

impl <Item, Pos, const DIMS: usize> Grid <Vec <Item>, Pos, DIMS>
	where
		Item: Clone + Default,
		Pos: GridPos <DIMS> {

	#[ inline ]
	#[ must_use ]
	pub fn new_vec (
		origin: [isize; DIMS],
		size: [usize; DIMS],
	) -> Self {
		Self::wrap (
			iter::repeat (default ())
				.take (size.iter ().copied ().product ())
				.collect::<Vec <_>> (),
			origin,
			size)
	}

	#[ inline ]
	pub fn reset (& mut self) {
		for item in self.storage.iter_mut () {
			* item = default ();
		}
	}

}

impl <Storage, Pos, const DIMS: usize> Grid <Storage, Pos, DIMS>
	where
		Storage: GridStorage + Clone,
		Pos: GridPos <DIMS> {

	#[ inline ]
	pub fn wrap (
		storage: Storage,
		origin: [isize; DIMS],
		size: [usize; DIMS],
	) -> Self {
		assert! (! size.into_iter ().any (|dim| dim == 0),
			"Size must be positive in all dimensions: {:?}", size);
		let expected_len = size.into_iter ().product::<usize> ();
		let actual_len = storage.storage_len ();
		assert! (expected_len == actual_len,
			"Expected {} items but was passed {}", expected_len, actual_len);
		Self { storage, origin, size, phantom: PhantomData }
	}

	#[ inline ]
	pub fn len (& self) -> usize {
		self.size.into_iter ().product ()
	}

	#[ inline ]
	pub fn is_empty (& self) -> bool {
		self.size.into_iter ().any (|dim| dim == 0)
	}

	#[ inline ]
	pub const fn size (& self) -> [usize; DIMS] {
		self.size
	}

	#[ inline ]
	pub const fn raw_origin (& self) -> [isize; DIMS] {
		self.origin
	}

	#[ inline ]
	pub const fn raw_size (& self) -> [usize; DIMS] {
		self.size
	}

	#[ inline ]
	pub fn origin (& self) -> Pos {
		Pos::from_scalar (0, self.origin, self.size).unwrap ()
	}

	#[ inline ]
	pub fn peak (& self) -> Pos {
		Pos::from_scalar (self.len () - 1, self.origin, self.size).unwrap ()
	}

	#[ inline ]
	pub fn get (& self, pos: Pos) -> Option <Storage::Item> {
		Pos::to_scalar (pos, self.origin, self.size)
			.and_then (|index| self.storage.storage_get (index))
	}

	#[ inline ]
	pub fn get_raw (& self, pos: [usize; DIMS]) -> Option <Storage::Item> {
		let mut idx = 0;
		for (val, size) in pos.iter ().copied ().zip (self.size.iter ().copied ()) {
			if size <= val { return None }
			idx = idx * size + val;
		}
		self.storage.storage_get (idx)
	}

	#[ inline ]
	pub fn set (& mut self, pos: Pos, item: Storage::Item) {
		let idx = some_or! (
			Pos::to_scalar (pos, self.origin, self.size),
			panic! ("Unable to map GridPos to scalar: {:?}", pos));
		self.storage.storage_set (idx, item);
	}

	#[ inline ]
	pub fn iter <'sto> (& 'sto self) -> GridIter <<& 'sto Storage as GridStorageIntoIter>::Iter, Pos, DIMS>
			where & 'sto Storage: GridStorageIntoIter {
		GridIter {
			storage: (& self.storage).storage_iter (),
			idx: 0,
			origin: self.origin,
			size: self.size,
			phantom: PhantomData,
		}
	}

	#[ inline ]
	pub fn values <'sto> (& 'sto self) -> <& 'sto Storage as GridStorageIntoIter>::Iter
			where & 'sto Storage: GridStorageIntoIter {
		(& self.storage).storage_iter ()
	}

	#[ inline ]
	pub const fn keys (& self) -> GridKeysIter <Pos, DIMS> {
		GridKeysIter {
			origin: self.origin,
			size: self.size,
			cur: [0; DIMS],
			phantom: PhantomData,
		}
	}

}

impl <Storage, Pos, const DIMS: usize> Grid <Storage, Pos, DIMS>
	where
		Storage: GridStorageMut + Clone,
		Pos: GridPos <DIMS> {

	#[ inline ]
	pub fn get_ref (& self, pos: Pos) -> Option <& Storage::Item> {
		Pos::to_scalar (pos, self.origin, self.size)
			.and_then (|index| self.storage.storage_ref (index))
	}

	#[ inline ]
	pub fn get_mut (& mut self, pos: Pos) -> Option <& mut Storage::Item> {
		Pos::to_scalar (pos, self.origin, self.size)
			.and_then (|index| self.storage.storage_mut (index))
	}

}

impl <Storage, Pos> Display for Grid <Storage, Pos, 2>
	where
		Storage: GridStorage + Clone,
		Storage::Item: Display,
		Pos: GridPos <2> {

	#[ inline ]
	fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
		for row in 0 .. self.size [0] {
			if row != 0 { write! (formatter, "\n") ?; }
			for col in 0 .. self.size [1] {
				let item = self.get_raw ([row, col]).unwrap ();
				Display::fmt (& item, formatter) ?;
			}
		}
		Ok (())
	}

}

impl <'inp, Item, Pos> FromParser <'inp> for Grid <Vec <Item>, Pos, 2>
	where
		Item: Clone + Default + FromParser <'inp>,
		Pos: GridPos <2> {

	#[ inline ]
	fn from_parser (parser: & mut Parser <'inp>) -> ParseResult <Self> {
		let lines: Vec <Vec <Item>> = parser.delim_fn ("\n", |parser| {
			parser.any ().of (|parser| {
				let items: Vec <Item> = parser.repeat (Parser::item).collect ();
				if items.is_empty () { return Err (parser.err ()) }
				Ok (items)
			}).done ()
		}).try_collect () ?;
		let height = lines.len ();
		let width = lines.iter ().map (Vec::len).max ().unwrap_or (0);
		Ok (Self::wrap (
			lines.iter ()
				.flat_map (|line| line.iter ().cloned ()
					.chain (iter::repeat (Item::default ()))
					.take (width))
				.collect (),
			[ 0, 0 ],
			[ height, width ],
		))
	}

}

/// Iterator returned by [`Grid::iter`].
///
/// This iterator wraps the iterator from an implementation of [`GridStorage`] and maps from `Item`
/// to `([GridPos], Item)`.
///
#[ derive (Clone) ]
pub struct GridIter <Storage, Pos, const DIMS: usize> {
	storage: Storage,
	idx: usize,
	origin: [isize; DIMS],
	size: [usize; DIMS],
	phantom: PhantomData <Pos>,
}

impl <Storage, Pos, const DIMS: usize> Iterator for GridIter <Storage, Pos, DIMS>
	where
		Storage: Iterator,
		Pos: GridPos <DIMS> {

	type Item = (Pos, Storage::Item);

	#[ inline ]
	fn next (& mut self) -> Option <(Pos, Storage::Item)> {
		if let Some (item) = self.storage.next () {
			let idx = self.idx;
			self.idx += 1;
			Some ((Pos::from_scalar (idx, self.origin, self.size).unwrap (), item))
		} else { None }
	}

	#[ inline ]
	fn nth (& mut self, num: usize) -> Option <(Pos, Storage::Item)> {
		if let Some (item) = self.storage.nth (num) {
			let idx = self.idx;
			self.idx += num + 1;
			Some ((Pos::from_scalar (idx, self.origin, self.size).unwrap (), item))
		} else { None }
	}

}

pub struct GridKeysIter <Pos: GridPos <DIMS>, const DIMS: usize> {
	origin: [isize; DIMS],
	size: [usize; DIMS],
	cur: [usize; DIMS],
	phantom: PhantomData <Pos>,
}

impl <Pos: GridPos <DIMS>, const DIMS: usize> Iterator for GridKeysIter <Pos, DIMS> {

	type Item = Pos;

	#[ inline ]
	fn next (& mut self) -> Option <Pos> {
		let pos = Pos::from_array (self.cur, self.origin);
		for idx in (0 .. DIMS).rev () {
			self.cur [idx] += 1;
			if self.cur [idx] < self.size [idx] { break }
			if idx == 0 { return None }
			self.cur [idx] = 0;
		}
		pos
	}

}

/// Trait for backing stores for a [`Grid`]
///
/// This provides a simple abstraction over a fixed size array of items. It is implemented for
/// [`Vec`] and [`BitVec`].

pub trait GridStorage {

	type Item;

	fn storage_get (& self, idx: usize) -> Option <Self::Item>;
	fn storage_set (& mut self, idx: usize, item: Self::Item);
	fn storage_len (& self) -> usize;

}

impl <Item> GridStorage for Vec <Item> where Item: Clone {
	type Item = Item;

	#[ inline ]
	fn storage_get (& self, idx: usize) -> Option <Item> {
		self.get (idx).cloned ()
	}

	#[ inline ]
	fn storage_set (& mut self, idx: usize, item: Self::Item) {
		self [idx] = item;
	}

	#[ inline ]
	fn storage_len (& self) -> usize {
		self.len ()
	}

}

impl <Item, Encoding> GridStorage for BitVec <Item, Encoding>
	where
		Encoding: BitVecEncoding <Item>,
		Item: Clone {

	type Item = Item;

	#[ inline ]
	fn storage_get (& self, idx: usize) -> Option <Item> {
		self.get (idx)
	}

	#[ inline ]
	fn storage_set (& mut self, idx: usize, item: Self::Item) {
		self.set (idx, item);
	}

	#[ inline ]
	fn storage_len (& self) -> usize {
		self.len ()
	}

}

/// Additional trait for backing stores which which can provide references to items
///
pub trait GridStorageMut: GridStorage {
	fn storage_ref (& self, idx: usize) -> Option <& Self::Item>;
	fn storage_mut (& mut self, idx: usize) -> Option <& mut Self::Item>;
}

impl <Item> GridStorageMut for Vec <Item> where Item: Clone {

	#[ inline ]
	fn storage_ref (& self, idx: usize) -> Option <& Item> {
		self.get (idx)
	}

	#[ inline ]
	fn storage_mut (& mut self, idx: usize) -> Option <& mut Item> {
		self.get_mut (idx)
	}

}

/// Extra trait for [`GridStorage`] to support iteration.
///
/// This is a separate trait to make the lifetimes work. It should be implemented on a reference to
/// the storage, rather than directly. This allows us to capture the lifetime without polluting the
/// main trait.
///
pub trait GridStorageIntoIter {

	type Item;
	type Iter: Iterator <Item = Self::Item>;

	fn storage_iter (& self) -> Self::Iter;

}

impl <'sto, Item> GridStorageIntoIter for & 'sto Vec <Item> where Item: Clone {

	type Item = Item;
	type Iter = GridStorageClone <SliceIter <'sto, Item>>;

	#[ inline ]
	fn storage_iter (& self) -> Self::Iter {
		GridStorageClone::new (self.iter ())
	}

}

impl <'sto, Item, Encoding> GridStorageIntoIter for & 'sto BitVec <Item, Encoding>
	where
		Encoding: BitVecEncoding <Item>,
		Item: Clone {

	type Item = Item;
	type Iter = BitVecIter <'sto, Item, Encoding>;

	#[ inline ]
	fn storage_iter (& self) -> Self::Iter {
		self.iter ()
	}

}

/// Trait for values to use as indices for a [`Grid`].
///
/// For example, a two dimensional grid might be indexed with a struct containing an `x` and a `y`
/// coordinate.
///
/// This trait provides methods to translate whatever coordinate system is in use to and from a
/// single `usize` value.
///
pub trait GridPos <const DIMS: usize>: Copy + Debug + Sized {

	fn to_array (self, origin: [isize; DIMS]) -> Option <[usize; DIMS]>;

	#[ inline ]
	fn to_scalar (self, origin: [isize; DIMS], size: [usize; DIMS]) -> Option <usize> {
		self.to_array (origin) ?
			.iter ().copied ()
			.zip (size.iter ().copied ())
			.fold (Ok (0), |scalar, (val, size)| {
				if size <= val { return Err (Overflow) }
				Int::add_2 (Int::mul_2 (scalar ?, size) ?, val)
			})
			.ok ()
	}

	fn from_array (array: [usize; DIMS], origin: [isize; DIMS]) -> Option <Self>;

	#[ inline ]
	#[ must_use ]
	fn from_scalar (
		mut scalar: usize,
		origin: [isize; DIMS],
		size: [usize; DIMS],
	) -> Option <Self> {
		let mut array = [0; DIMS];
		for idx in (0 .. DIMS).rev () {
			array [idx] = scalar % size [idx];
			scalar /= size [idx];
		}
		if scalar != 0 { return None }
		Self::from_array (array, origin)
	}

}

impl <Val: Int, const DIMS: usize> GridPos <DIMS> for [Val; DIMS] {

	#[ inline ]
	fn to_array (self, origin: [isize; DIMS]) -> Option <[usize; DIMS]> {
		let mut result = [0; DIMS];
		for idx in 0 .. DIMS {
			let val_isize = self [idx].to_isize ().ok () ?;
			let adj_isize = isize::add_2 (val_isize, origin [idx]).ok () ?;
			result [idx] = adj_isize.to_usize ().ok () ?;
		}
		Some (result)
	}

	#[ inline ]
	fn from_array (array: [usize; DIMS], origin: [isize; DIMS]) -> Option <Self> {
		let mut result = [Val::ZERO; DIMS];
		for idx in 0 .. DIMS {
			let val_isize = array [idx].to_isize ().ok () ?;
			let adj_isize = isize::sub_2 (val_isize, origin [idx]).ok () ?;
			result [idx] = Val::from_isize (adj_isize).ok () ?;
		}
		Some (result)
	}

}

impl <Val: Int> GridPos <2> for PosXY <Val> {

	#[ inline ]
	fn to_array (self, origin: [isize; 2]) -> Option <[usize; 2]> {
		GridPos::to_array ([ self.x, self.y ], origin)
	}

	#[ inline ]
	fn from_array (array: [usize; 2], origin: [isize; 2]) -> Option <Self> {
		let array = <[Val; 2]>::from_array (array, origin) ?;
		Some (Self { x: array [0], y: array [1] })
	}

}

impl <Val: Int> GridPos <2> for PosYX <Val> {

	#[ inline ]
	fn to_array (self, origin: [isize; 2]) -> Option <[usize; 2]> {
		GridPos::to_array ([ self.y, self.x ], origin)
	}

	#[ inline ]
	fn from_array (array: [usize; 2], origin: [isize; 2]) -> Option <Self> {
		let array = <[Val; 2]>::from_array (array, origin) ?;
		Some (Self { y: array [0], x: array [1] })
	}

}

impl <Val: Int> GridPos <2> for PosRowCol <Val> {

	#[ inline ]
	fn to_array (self, origin: [isize; 2]) -> Option <[usize; 2]> {
		GridPos::to_array ([ self.row, self.col ], origin)
	}

	#[ inline ]
	fn from_array (array: [usize; 2], origin: [isize; 2]) -> Option <Self> {
		let array = <[Val; 2]>::from_array (array, origin) ?;
		Some (Self { row: array [0], col: array [1] })
	}

}

/// Wrapping iterator which clones items.
///
/// We don't use [`Cloned`](iter::Cloned) from the standard library because it doesn't handle
/// [`Iterator::skip`] the way we would like. Although it says in the documentation that there is
/// no guarantee each element will be processed, it seems like it does so. Instead, we want to
/// completely bypass any elements which aren't required.
///
#[ derive (Clone) ]
pub struct GridStorageClone <Storage> {
	storage: Storage,
}

impl <Storage> GridStorageClone <Storage> {
	const fn new (storage: Storage) -> Self {
		Self { storage }
	}
}

impl <'sto, Storage, Item> Iterator for GridStorageClone <Storage>
	where
		Storage: Iterator <Item = & 'sto Item>,
		Item: Clone + 'sto {

	type Item = Item;

	#[ inline ]
	fn next (& mut self) -> Option <Item> {
		self.storage.next ().cloned ()
	}

	#[ inline ]
	fn nth (& mut self, num: usize) -> Option <Item> {
		self.storage.nth (num).cloned ()
	}

}

pub struct GridPrint <'grd, Storage, Pos, MapFn, Out>
	where
		Storage: GridStorage + Clone,
		Pos: GridPos <2>,
		MapFn: Fn (Storage::Item) -> Out,
		Out: Display {
	grid: & 'grd Grid <Storage, Pos, 2>,
	map_fn: MapFn,
}

impl <'grd, Storage, Pos, MapFn, Out> Display for GridPrint <'grd, Storage, Pos, MapFn, Out>
	where
		Storage: GridStorage + Clone,
		Pos: GridPos <2>,
		MapFn: Fn (Storage::Item) -> Out,
		Out: Display {

	#[ inline ]
	fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
		for row in 0 .. self.grid.size [0] {
			for col in 0 .. self.grid.size [1] {
				let item = self.grid.get_raw ([row, col]).unwrap ();
				write! (formatter, "{}", (self.map_fn) (item)) ?;
			}
			write! (formatter, "\n") ?;
		}
		Ok (())
	}

}

impl <Storage, Pos> Grid <Storage, Pos, 2>
	where Storage: GridStorage + Clone, Pos: GridPos <2> {

	#[ inline ]
	pub const fn print <MapFn, Out> (
		& self,
		map_fn: MapFn,
	) -> GridPrint <Storage, Pos, MapFn, Out>
			where MapFn: Fn (Storage::Item) -> Out, Out: Display {
		GridPrint { grid: self, map_fn }
	}

}
