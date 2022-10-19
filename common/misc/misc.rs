#![ allow (clippy::inline_always) ]

mod collections;
mod iter;
pub mod prelude;

mod result {

	pub use crate::prelude::*;

	pub use crate::ok_or;
	pub use crate::ok_or_else;
	pub use crate::assert_is_err;
	pub use crate::assert_is_ok;
	pub use crate::assert_eq_ok;
	pub use crate::assert_err;

	pub type GenResult <Ok> = Result <Ok, GenError>;

	pub trait ResultEither <Val> {
		fn either (self) -> Val;
	}

	impl <Val> ResultEither <Val> for Result <Val, Val> {
		#[ inline (always) ]
		#[ must_use ]
		fn either (self) -> Val {
			match self { Ok (val) => val, Err (val) => val }
		}
	}

	#[ allow (clippy::missing_const_for_fn) ]
	#[ inline (always) ]
	#[ must_use ]
	pub fn ok_or_err <Val> (result: Result <Val, Val>) -> Val {
		match result {
			Ok (val) => val,
			Err (val) => val,
		}
	}

	pub trait ResultMapRef <Val, Error: Copy> {
		fn map_ref <Out> (& self, map_fn: impl FnMut (& Val) -> Out) -> Result <Out, Error>;
	}

	impl <Val, Error: Copy> ResultMapRef <Val, Error> for Result <Val, Error> {
		#[ inline (always) ]
		fn map_ref <Out> (& self, map_fn: impl FnMut (& Val) -> Out) -> Result <Out, Error> {
			self.as_ref ().map_err (|& err| err).map (map_fn)
		}
	}

	#[ macro_export ]
	macro_rules! ok_or {
		( $val:expr, $if_err:expr $(,)? ) => {
			match ($val) { Ok (val) => val, Err (_) => $if_err }
		};
	}

	#[ macro_export ]
	macro_rules! ok_or_else {
		( $val:expr, |$arg:ident| $if_err:expr $(,)? ) => {
			match ($val) { Ok (val) => val, Err ($arg) => $if_err }
		};
	}

	#[ macro_export ]
	macro_rules! assert_is_ok {
		( $actual:expr ) => {
			assert! ($actual.is_ok ());
		};
	}

	#[ macro_export ]
	macro_rules! assert_eq_ok {
		( $expect:expr , $actual:expr ) => {
			let actual = $actual;
			assert! (actual.is_ok (), "Expected Ok but got {:?}", actual);
			assert_eq! ($expect, actual.unwrap ());
		};
		( $expect: expr , $actual:expr , $($rest:tt)* ) => {
			let actual = $actual;
			assert! (actual.is_ok (), $($rest)*);
			assert_eq! ($expect, actual.unwrap ());
		};
	}

	#[ macro_export ]
	macro_rules! assert_err {
		( $expect:expr , $actual:expr ) => {
			assert_eq! ($expect, $actual.unwrap_err ().to_string ());
		};
	}

	#[ macro_export ]
	macro_rules! assert_is_err {
		( $actual:expr ) => {
			let actual = $actual;
			assert! (actual.is_err (), "Expected Err (_) but got {actual:?}");
		};
	}

}

mod error {

	use crate::prelude::*;

	pub type GenError = Box <dyn Error>;

}

mod default {

	#[ inline (always) ]
	#[ must_use ]
	pub fn default <T: Default> () -> T {
		Default::default ()
	}

}

mod option {

	pub use crate::some_or;

	#[ macro_export ]
	macro_rules! some_or {
		( $val:expr, $if_err:expr $(,)? ) => {
			match ($val) { Some (val) => val, None => $if_err }
		};
	}

}

mod array_vec {

	pub use crate::array_vec;

	#[ macro_export ]
	macro_rules! array_vec {
		() => { ArrayVec::new () };
		( $($val:expr),* $(,)? ) => {
			{
				let mut result = ArrayVec::new ();
				$( result.push ($val); )*
				result
			}
		};
		( $val:expr; $num:expr ) => {
			[$val; $num].into_iter ().collect ()
		};
	}

}

mod deref {

	pub use crate::wrapper_deref;
	pub use crate::wrapper_deref_mut;

	#[ macro_export ]
	macro_rules! wrapper_deref {
		(
			$(#[$struct_meta:meta])*
			$struct_vis:vis struct $struct_name:ident $(<$($struct_param:tt),*>)? {
				$field_vis:vis $field_name:ident: $field_type:ty,
			}
		) => {

			$(#[$struct_meta])*
			$struct_vis struct $struct_name $(<$($struct_param),*>)? {
				$field_vis $field_name: $field_type,
			}

			impl $(<$($struct_param),*>)? ::std::ops::Deref for $struct_name $(<$($struct_param),*>)? {
				type Target = $field_type;
				#[ inline ]
				fn deref (& self) -> & $field_type {
					& self.$field_name
				}
			}

		};
	}

	#[ macro_export ]
	macro_rules! wrapper_deref_mut {
		(
			$(#[$struct_meta:meta])*
			$struct_vis:vis struct $struct_name:ident $(<$($struct_param:tt),*>)? {
				$field_vis:vis $field_name:ident: $field_type:ty,
			}
		) => {

			$(#[$struct_meta])*
			$struct_vis struct $struct_name $(<$($struct_param),*>)? {
				$field_vis $field_name: $field_type,
			}

			impl $(<$($struct_param),*>)? ::std::ops::Deref for $struct_name $(<$($struct_param),*>)? {
				type Target = $field_type;
				#[ inline ]
				fn deref (& self) -> & $field_type {
					& self.$field_name
				}
			}

			impl $(<$($struct_param),*>)? ::std::ops::DerefMut for $struct_name $(<$($struct_param),*>)? {
				#[ inline ]
				fn deref_mut (& mut self) -> & mut $field_type {
					& mut self.$field_name
				}
			}

		};
	}

}
