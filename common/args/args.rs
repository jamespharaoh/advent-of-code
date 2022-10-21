use std::error::Error;
use std::ffi::OsString;
use std::fmt::{ self, Display };
use std::path::PathBuf;

pub trait ArgsParse: Sized {
	fn parse <Iter> (iter: Iter) -> Result <Self, ArgsParseError>
		where Iter: IntoIterator <Item = OsString>;
}

#[ derive (Debug, Clone) ]
pub enum ArgsParseError {
	Unexpected (OsString),
	MissingArg (& 'static str),
	MissingValue (& 'static str),
	Duplicated (& 'static str),
	Invalid (& 'static str, OsString, String),
}

impl Display for ArgsParseError {

	#[ inline ]
	fn fmt (& self, fmtr: & mut fmt::Formatter) -> fmt::Result {
		match * self {
			Self::Unexpected (ref arg) =>
				write! (fmtr, "Unexpected argument {}", arg.to_string_lossy ()),
			Self::MissingArg (arg) =>
				write! (fmtr, "Missing argument {arg}"),
			Self::MissingValue (arg) =>
				write! (fmtr, "Missing value for {arg}"),
			Self::Duplicated (arg) =>
				write! (fmtr, "Duplicated argument {arg}"),
			Self::Invalid (arg, ref val, ref msg) =>
				write! (fmtr, "Invalid value for {arg}: {}: {msg}", val.to_string_lossy ()),
		}
	}
}

impl Error for ArgsParseError {
}

pub trait ArgsParseOuter: Sized {

	type State;

	fn init () -> Self::State;

	fn handle (
		name: & 'static str,
		state: & mut Self::State,
		args: & mut dyn Iterator <Item = OsString>,
	) -> Result <(), ArgsParseError>;

	fn finish (
		name: & 'static str,
		state: Self::State,
	) -> Result <Self, ArgsParseError>;

}

impl ArgsParseOuter for bool {

	type State = Self;

	#[ inline ]
	fn init () -> Self {
		false
	}

	#[ inline ]
	fn handle (
		name: & 'static str,
		state: & mut Self::State,
		_args: & mut dyn Iterator <Item = OsString>,
	) -> Result <(), ArgsParseError> {
		if * state {
			return Err (ArgsParseError::Duplicated (name));
		}
		* state = true;
		Ok (())
	}

	#[ inline ]
	fn finish (
		_name: & 'static str,
		state: Self::State,
	) -> Result <Self, ArgsParseError> {
		Ok (state)
	}

}

impl <Inner> ArgsParseOuter for Option <Inner> where Inner: ArgsParseInner {

	type State = Self;

	#[ inline ]
	fn init () -> Self {
		None
	}

	#[ inline ]
	fn handle (
		name: & 'static str,
		state: & mut Self,
		args: & mut dyn Iterator <Item = OsString>,
	) -> Result <(), ArgsParseError> {
		if state.is_some () { return Err (ArgsParseError::Duplicated (name)); }
		* state = Some (Inner::parse (name, args) ?);
		Ok (())
	}

	#[ inline ]
	fn finish (
		_name: & 'static str,
		state: Self,
	) -> Result <Self, ArgsParseError> {
		Ok (state)
	}

}

impl <Inner> ArgsParseOuter for Vec <Inner> where Inner: ArgsParseInner {

	type State = Self;

	#[ inline ]
	fn init () -> Self {
		Self::new ()
	}

	#[ inline ]
	fn handle (
		name: & 'static str,
		state: & mut Self,
		args: & mut dyn Iterator <Item = OsString>,
	) -> Result <(), ArgsParseError> {
		state.push (Inner::parse (name, args) ?);
		Ok (())
	}

	#[ inline ]
	fn finish (
		_name: & 'static str,
		state: Self,
	) -> Result <Self, ArgsParseError> {
		Ok (state)
	}

}

impl <Inner: ArgsParseInner> ArgsParseOuter for Inner {

	type State = Option <Self>;

	#[ inline ]
	fn init () -> Option <Self> {
		None
	}

	#[ inline ]
	fn handle (
		name: & 'static str,
		state: & mut Option <Self>,
		args: & mut dyn Iterator <Item = OsString>,
	) -> Result <(), ArgsParseError> {
		if state.is_some () { return Err (ArgsParseError::Duplicated (name)); }
		* state = Some (Inner::parse (name, args) ?);
		Ok (())
	}

	#[ inline ]
	fn finish (name: & 'static str, state: Option <Inner>) -> Result <Self, ArgsParseError> {
		state.ok_or (ArgsParseError::MissingArg (name))
	}

}

pub trait ArgsParseInner: Sized {

	fn parse (
		name: & 'static str,
		args: & mut dyn Iterator <Item = OsString>,
	) -> Result <Self, ArgsParseError>;

}

impl ArgsParseInner for PathBuf {

	#[ inline ]
	fn parse (
		name: & 'static str,
		args: & mut dyn Iterator <Item = OsString>,
	) -> Result <Self, ArgsParseError> {
		let arg_os = args.next ().ok_or (ArgsParseError::MissingValue (name)) ?;
		Ok (arg_os.into ())
	}

}

impl ArgsParseInner for String {

	#[ inline ]
	fn parse (
		name: & 'static str,
		args: & mut dyn Iterator <Item = OsString>,
	) -> Result <Self, ArgsParseError> {
		let arg_os = args.next ().ok_or (ArgsParseError::MissingValue (name)) ?;
		let arg = arg_os.into_string ().map_err (|arg_os|
			ArgsParseError::Invalid (name, arg_os, "Invalid UTF-8".to_owned ())) ?;
		Ok (arg)
	}

}

impl ArgsParseInner for u16 {

	#[ inline ]
	fn parse (
		name: & 'static str,
		args: & mut dyn Iterator <Item = OsString>,
	) -> Result <Self, ArgsParseError> {
		let arg_os = args.next ().ok_or (ArgsParseError::MissingValue (name)) ?;
		let arg = arg_os.to_str ().ok_or_else (||
			ArgsParseError::Invalid (name, arg_os.clone (), "Invalid number".to_owned ())) ?;
		arg.parse ().ok ().ok_or_else (||
			ArgsParseError::Invalid (name, arg_os, "Invalid number".to_owned ()))
	}

}

impl ArgsParseInner for u32 {

	#[ inline ]
	fn parse (
		name: & 'static str,
		args: & mut dyn Iterator <Item = OsString>,
	) -> Result <Self, ArgsParseError> {
		let arg_os = args.next ().ok_or (ArgsParseError::MissingValue (name)) ?;
		let arg = arg_os.to_str ().ok_or_else (||
			ArgsParseError::Invalid (name, arg_os.clone (), "Invalid number".to_owned ())) ?;
		arg.parse ().ok ().ok_or_else (||
			ArgsParseError::Invalid (name, arg_os, "Invalid number".to_owned ()))
	}

}

impl ArgsParseInner for u64 {

	#[ inline ]
	fn parse (
		name: & 'static str,
		args: & mut dyn Iterator <Item = OsString>,
	) -> Result <Self, ArgsParseError> {
		let arg_os = args.next ().ok_or (ArgsParseError::MissingValue (name)) ?;
		let arg = arg_os.to_str ().ok_or_else (||
			ArgsParseError::Invalid (name, arg_os.clone (), "Invalid number".to_owned ())) ?;
		arg.parse ().ok ().ok_or_else (||
			ArgsParseError::Invalid (name, arg_os, "Invalid number".to_owned ()))
	}

}

impl ArgsParseInner for usize {

	#[ inline ]
	fn parse (
		name: & 'static str,
		args: & mut dyn Iterator <Item = OsString>,
	) -> Result <Self, ArgsParseError> {
		let arg_os = args.next ().ok_or (ArgsParseError::MissingValue (name)) ?;
		let arg = arg_os.to_str ().ok_or_else (||
			ArgsParseError::Invalid (name, arg_os.clone (), "Invalid number".to_owned ())) ?;
		arg.parse ().ok ().ok_or_else (||
			ArgsParseError::Invalid (name, arg_os, "Invalid number".to_owned ()))
	}

}

#[ inline ]
#[ must_use ]
pub fn arg_matches (search: & str, arg_bytes: & [u8]) -> bool {
	2 < arg_bytes.len () && arg_bytes [0] == b'-' && arg_bytes [1] == b'-'
		&& arg_bytes [2 .. ].iter ().copied ().eq (
			search.bytes ().map (|ch| if ch == b'_' { b'-' } else { ch }))
}

#[ macro_export ]
macro_rules! args_decl {

	(
		$( #[ $($attr:tt)* ] )*
		$vis:vis struct $name:ident { $(
			$mem_vis:vis $mem_name:ident: $mem_type:ty
		),* $(,)? }
	) => {

		$( #[ $($attr)* ] )*
		$vis struct $name { $(
			$mem_vis $mem_name: $mem_type,
		)* }

		impl $crate::ArgsParse for $name {

			#[ allow (clippy::useless_let_if_seq) ]
			fn parse <Iter> (args_iter: Iter) -> Result <Self, $crate::ArgsParseError>
				where Iter: IntoIterator <Item = ::std::ffi::OsString> {

				$( let mut $mem_name = <$mem_type as $crate::ArgsParseOuter>::init (); )*

				let mut args_iter = args_iter.into_iter ();
				let mut literal_args = false;

				while let Some (arg) = args_iter.next () {
					let arg_bytes = ::std::os::unix::ffi::OsStrExt::as_bytes (arg.as_os_str ());
					let mut matched = false;
					if ! literal_args && arg_bytes == b"--" {
						literal_args = true;
						matched = true;
					}
					$(
						if ! literal_args && ! matched
								&& $crate::arg_matches (stringify! ($mem_name), arg_bytes) {
							<$mem_type>::handle (
								stringify! ($mem_name),
								& mut $mem_name,
								& mut args_iter) ?;
							matched = true;
						}
					)*
					if ! matched {
						return Err ($crate::ArgsParseError::Unexpected (arg));
					}
				}

				Ok (Self {
					$(
						$mem_name: <$mem_type as ArgsParseOuter>::finish (
							stringify! ($mem_name),
							$mem_name) ?
					),*
				})

			}

		}

	};

}
