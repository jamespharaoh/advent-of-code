#![ allow (clippy::as_conversions) ]
#![ allow (clippy::undocumented_unsafe_blocks) ]

use std::borrow::{ Borrow, BorrowMut };
use std::cmp::Ordering;
use std::fmt::{ self, Debug };
use std::hash::{ Hash, Hasher };
use std::iter::FusedIterator;
use std::mem::{ self, MaybeUninit };
use std::ops::{ Deref, DerefMut };
use std::ptr;
use std::slice::{ self, Iter as SliceIter };

pub mod prelude {
	pub use crate::MiniVec;
	pub use crate::TinyVec;
	pub use crate::mini_vec;
	pub use crate::tiny_vec;
}

macro_rules! vec_decl {
	( $name:ident, $len:ident, $into_iter:ident ) => {

		pub struct $name <Item, const LEN: usize> {
			len: $len,
			data: [MaybeUninit <Item>; LEN],
		}

		impl <Item, const LEN: usize> $name <Item, LEN> {

			#[ inline ]
			fn as_mut_ptr (& mut self) -> * mut Item {
				unsafe { mem::transmute (self.data.as_mut_ptr ()) }
			}

			#[ inline ]
			fn as_mut_slice (& mut self) -> & mut [Item] {
				unsafe {
					slice::from_raw_parts_mut (
						self.as_mut_ptr (),
						self.len as usize)
				}
			}

			#[ inline ]
			fn as_ptr (& self) -> * const Item {
				unsafe { mem::transmute (self.data.as_ptr ()) }
			}

			#[ inline ]
			fn as_slice (& self) -> & [Item] {
				unsafe {
					slice::from_raw_parts (
						self.as_ptr (),
						self.len as usize)
				}
			}

			#[ inline ]
			pub fn clear (& mut self) {
				self.len = 0;
				unsafe {
					ptr::drop_in_place (
						ptr::slice_from_raw_parts_mut (
							self.as_mut_ptr (),
							self.len as usize));
				}
			}

			#[ inline ]
			pub fn is_empty (& self) -> bool {
				self.len == 0
			}

			#[ inline ]
			pub fn is_full (& self) -> bool {
				self.len as usize == LEN
			}

			#[ inline ]
			pub fn len (& self) -> usize {
				self.len as usize
			}

			#[ inline ]
			pub fn native_len (& self) -> $len {
				self.len
			}

			#[ inline ]
			#[ must_use ]
			pub fn new () -> Self {
				assert! ($len::try_from (LEN).is_ok ());
				Self {
					len: 0,
					data: unsafe { MaybeUninit::uninit ().assume_init () },
				}
			}

			#[ inline ]
			pub fn push (& mut self, item: Item) {
				assert! ((self.len as usize) < LEN);
				self.data [self.len as usize].write (item);
				self.len += 1;
			}

			#[ inline ]
			pub fn pop (& mut self) -> Option <Item> {
				if self.len == 0 { return None }
				let item = unsafe { self.data [self.len as usize - 1].assume_init_read () };
				self.len -= 1;
				Some (item)
			}

			#[ inline ]
			pub fn retain (
				& mut self,
				mut pred_fn: impl FnMut (& mut Item) -> bool,
			) {
				let orig_len = self.len;
				self.len = 0;
				struct DropGuard <'dat, Item, const LEN: usize> {
					vec: & 'dat mut $name <Item, LEN>,
					num_handled: $len,
					num_deleted: $len,
					orig_len: $len,
				}
				impl <Item, const LEN: usize> Drop for DropGuard <'_, Item, LEN> {
					fn drop (& mut self) {
						if 0 < self.num_deleted {
							unsafe {
								ptr::copy (
									self.vec.as_ptr ().add (self.num_handled as usize),
									self.vec.as_mut_ptr ().add ((self.num_handled - self.num_deleted) as usize),
									(self.orig_len - self.num_handled) as usize);
							}
						}
						self.vec.len = self.orig_len - self.num_deleted;
					}
				}
				let mut guard = DropGuard {
					vec: self,
					num_handled: 0,
					num_deleted: 0,
					orig_len,
				};
				fn process_loop <Item, const LEN: usize, const DELETED: bool> (
					orig_len: $len,
					mut pred_fn: impl FnMut (& mut Item) -> bool,
					guard: & mut DropGuard <Item, LEN>,
				) {
					while guard.num_handled != orig_len {
						let cur = unsafe {
							& mut * guard.vec.as_mut_ptr ().add (
								guard.num_handled as usize)
						};
						if ! pred_fn (cur) {
							guard.num_handled += 1;
							guard.num_deleted += 1;
							unsafe { ptr::drop_in_place (cur) };
							if DELETED { continue } else { break }
						}
						if DELETED {
							unsafe {
								let hole_slot = guard.vec.as_mut_ptr ().add (
									(guard.num_handled - guard.num_deleted) as usize);
								ptr::copy_nonoverlapping (cur, hole_slot, 1);
							}
						}
						guard.num_handled += 1;
					}
				}
				process_loop::<_, LEN, false> (orig_len, & mut pred_fn, & mut guard);
				process_loop::<_, LEN, true> (orig_len, & mut pred_fn, & mut guard);
				drop (guard);		
			}

		}

		impl <Item, const LEN: usize> AsMut <[Item]>
			for $name <Item, LEN> {

			#[ inline ]
			fn as_mut (& mut self) -> & mut [Item] {
				self.as_mut_slice ()
			}

		}

		impl <Item, const LEN: usize> AsMut <$name <Item, LEN>>
			for $name <Item, LEN> {

			#[ inline ]
			fn as_mut (& mut self) -> & mut $name <Item, LEN> {
				self
			}

		}

		impl <Item, const LEN: usize> AsRef <[Item]>
			for $name <Item, LEN> {

			#[ inline ]
			fn as_ref (& self) -> & [Item] {
				self.as_slice ()
			}

		}

		impl <Item, const LEN: usize> AsRef <$name <Item, LEN>>
			for $name <Item, LEN> {

			#[ inline ]
			fn as_ref (& self) -> & $name <Item, LEN> {
				self
			}

		}

		impl <Item, const LEN: usize> Borrow <[Item]>
			for $name <Item, LEN> {

			#[ inline ]
			fn borrow (& self) -> & [Item] {
				self.as_slice ()
			}

		}

		impl <Item, const LEN: usize> BorrowMut <[Item]>
			for $name <Item, LEN> {

			#[ inline ]
			fn borrow_mut (& mut self) -> & mut [Item] {
				self.as_mut_slice ()
			}

		}

		impl <Item, const LEN: usize> Clone
			for $name <Item, LEN>
			where Item: Clone {

			#[ inline ]
			fn clone (& self) -> Self {
				let mut result = Self::new ();
				for item in self {
					result.push (item.clone ());
				}
				result
			}

		}

		impl <Item, const LEN: usize> From <& [Item]>
			for $name <Item, LEN>
			where Item: Clone {

			#[ inline ]
			fn from (src: & [Item]) -> Self {
				src.iter ().cloned ().collect ()
			}

		}

		impl <Item, const LEN: usize> From <& mut [Item]>
			for $name <Item, LEN>
			where Item: Clone {

			#[ inline ]
			fn from (src: & mut [Item]) -> Self {
				src.iter ().cloned ().collect ()
			}

		}

		impl <Item, const LEN: usize> Ord
			for $name <Item, LEN>
			where Item: Ord {

			#[ inline ]
			fn cmp (& self, other: & Self) -> Ordering {
				self.as_slice ().cmp (other.as_slice ())
			}

		}

		impl <Item, const LEN: usize> Debug
			for $name <Item, LEN>
			where Item: Debug {

			#[ inline ]
			fn fmt (& self, fmtr: & mut fmt::Formatter) -> fmt::Result {
				Debug::fmt (self.as_slice (), fmtr)
			}

		}

		impl <Item, const LEN: usize> Default
			for $name <Item, LEN> {

			#[ inline ]
			fn default () -> Self {
				Self::new ()
			}

		}

		impl <Item, const LEN: usize> Deref
			for $name <Item, LEN> {

			type Target = [Item];

			#[ inline ]
			fn deref (& self) -> & [Item] {
				self.as_slice ()
			}

		}

		impl <Item, const LEN: usize> DerefMut
			for $name <Item, LEN> {

			#[ inline ]
			fn deref_mut (& mut self) -> & mut [Item] {
				self.as_mut_slice ()
			}

		}

		impl <Item, const LEN: usize> Drop
			for $name <Item, LEN> {

			#[ inline ]
			fn drop (& mut self) {
				unsafe {
					ptr::drop_in_place (
						ptr::slice_from_raw_parts_mut (
							self.as_mut_ptr (),
							self.len as usize));
				}
			}

		}

		impl <Item, const LEN: usize> Eq
			for $name <Item, LEN>
			where Item: Eq {
		}

		impl <Item, const LEN: usize> Extend <Item>
			for $name <Item, LEN> {

			#[ inline ]
			fn extend <Iter> (& mut self, iter: Iter)
					where Iter: IntoIterator <Item = Item> {
				for item in iter {
					self.push (item);
				}
			}

		}

		impl <'dat, Item, const LEN: usize> Extend <& 'dat Item>
			for $name <Item, LEN>
			where Item: Copy + 'dat {

			#[ inline ]
			fn extend <Iter> (& mut self, iter: Iter)
					where Iter: IntoIterator <Item = & 'dat Item> {
				for & item in iter {
					self.push (item);
				}
			}

		}

		impl <Item, const LEN: usize> FromIterator <Item>
			for $name <Item, LEN> {

			#[ inline ]
			fn from_iter <Iter: IntoIterator <Item = Item>> (iter: Iter) -> Self {
				let mut result = Self::new ();
				for item in iter {
					result.push (item);
				}
				result
			}

		}

		impl <'dat, Item, const LEN: usize> IntoIterator
			for $name <Item, LEN> {

			type IntoIter = $into_iter <Item, LEN>;
			type Item = Item;

			#[ inline ]
			fn into_iter (mut self) -> $into_iter <Item, LEN> {
				let mut iter = $into_iter {
					data: unsafe { MaybeUninit::uninit ().assume_init () },
					start: 0,
					end: self.len,
				};
				unsafe {
					ptr::copy_nonoverlapping (
						self.as_ptr (),
						iter.data [0].as_mut_ptr (),
						self.len as usize);
				}
				self.len = 0;
				iter
			}

		}

		impl <'dat, Item, const LEN: usize> IntoIterator
			for & 'dat $name <Item, LEN> {

			type IntoIter = SliceIter <'dat, Item>;
			type Item = & 'dat Item;

			#[ inline ]
			fn into_iter (self) -> SliceIter <'dat, Item> {
				self.deref ().into_iter ()
			}

		}

		impl <Item, const LEN: usize> Hash
			for $name <Item, LEN>
			where Item: Hash {

			#[ inline ]
			fn hash <Hshr: Hasher> (& self, hasher: & mut Hshr) {
				self.as_slice ().hash (hasher)
			}

		}

		impl <Item, const LEN: usize> PartialEq
			for $name <Item, LEN>
			where Item: PartialEq {

			#[ inline ]
			fn eq (& self, other: & Self) -> bool {
				self.as_slice () == other.as_slice ()
			}

		}

		impl <Item, const LEN: usize> PartialOrd
			for $name <Item, LEN>
			where Item: PartialOrd {

			#[ inline ]
			fn partial_cmp (& self, other: & Self) -> Option <Ordering> {
				self.as_slice ().partial_cmp (other.as_slice ())
			}

		}

		pub struct $into_iter <Item, const LEN: usize> {
			data: [MaybeUninit <Item>; LEN],
			start: $len,
			end: $len,
		}

		impl <Item, const LEN: usize> DoubleEndedIterator
			for $into_iter <Item, LEN> {

			#[ inline ]
			fn next_back (& mut self) -> Option <Item> {
				if self.start == self.end { return None }
				self.end -= 1;
				Some (unsafe {
					self.data [self.end as usize].assume_init_read ()
				})
			}

		}

		impl <Item, const LEN: usize> Drop
			for $into_iter <Item, LEN> {

			#[ inline ]
			fn drop (& mut self) {
				if self.start == self.end { return }
				unsafe {
					ptr::drop_in_place (
						ptr::slice_from_raw_parts_mut (
							self.data [self.start as usize].as_mut_ptr (),
							(self.end - self.start) as usize));
				}
			}

		}

		impl <Item, const LEN: usize> ExactSizeIterator
			for $into_iter <Item, LEN> {

			#[ inline ]
			fn len (& self) -> usize {
				(self.end - self.start) as usize
			}

		}

		impl <Item, const LEN: usize> FusedIterator
			for $into_iter <Item, LEN> {
		}

		impl <Item, const LEN: usize> Iterator
			for $into_iter <Item, LEN> {

			type Item = Item;

			#[ inline ]
			fn next (& mut self) -> Option <Item> {
				if self.start == self.end { return None }
				let item = unsafe {
					self.data [self.start as usize].assume_init_read ()
				};
				self.start += 1;
				Some (item)
			}

			#[ inline ]
			fn size_hint (& self) -> (usize, Option <usize>) {
				let len = (self.end - self.start) as usize;
				(len, Some (len))
			}

		}

	};
}

vec_decl! (TinyVec, u8, TinyIntoIter);
vec_decl! (MiniVec, u16, MiniIntoIter);

#[ macro_export ]
macro_rules! tiny_vec {
	[ $item:expr; $len:literal ] => {
		{
			let item = $item;
			let mut result = TinyVec::new ();
			for _ in 0 .. $len {
				result.push (item);
			}
			result
		}
	};
	[ $($item:expr),* ] => {
		{
			let mut result = TinyVec::new ();
			$( result.push ($item); )*
			result
		}
	};
}

#[ macro_export ]
macro_rules! mini_vec {
	[ $($item:expr),* ] => {
		{
			let mut result = MiniVec::new ();
			$( result.push ($item); )*
			result
		}
	};
}
