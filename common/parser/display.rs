use super::*;

pub struct DisplayDelim <Delim, Inner> {
	delim: Delim,
	inner: Inner,
}

impl <Delim, Inner> Display for DisplayDelim <Delim, Inner>
	where
		Delim: Clone + Display,
		Inner: Clone + IntoIterator,
		Inner::Item: Display {

	#[ inline ]
	fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
		let mut first = true;
		for item in self.inner.clone () {
			if ! first { Display::fmt (& self.delim, formatter) ?; }
			Display::fmt (& item, formatter) ?;
			first = false;
		}
		Ok (())
	}

}

pub struct DisplayDelimWith <Delim, Inner, Item, DisplayFn> {
	delim: Delim,
	inner: Inner,
	display_fn: DisplayFn,
	phantom: PhantomData <Item>,
}

impl <Delim, Inner, Item, DisplayFn> Display for DisplayDelimWith <Delim, Inner, Item, DisplayFn>
	where
		Delim: Clone + Display,
		DisplayFn: Fn (Item, & mut fmt::Formatter) -> fmt::Result,
		Inner: Clone + IntoIterator <Item = Item> {

	#[ inline ]
	fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
		let mut first = true;
		for item in self.inner.clone () {
			if ! first { Display::fmt (& self.delim, formatter) ?; }
			(self.display_fn) (item, formatter) ?;
			first = false;
		}
		Ok (())
	}

}

pub trait IntoIteratorDisplayDelim: IntoIterator {

	#[ inline ]
	fn display_delim <Delim> (
		self,
		delim: Delim,
	) -> DisplayDelim <Delim, Self>
			where Self: Sized {
		DisplayDelim { delim, inner: self }
	}

	#[ inline ]
	fn display_delim_with <Delim, Item, DisplayFn> (
		self,
		delim: Delim,
		display_fn: DisplayFn,
	) -> DisplayDelimWith <Delim, Self, Item, DisplayFn>
			where Self: Sized {
		DisplayDelimWith { delim, inner: self, display_fn, phantom: PhantomData }
	}

}

impl <SomeIter> IntoIteratorDisplayDelim for SomeIter where SomeIter: IntoIterator {
}

#[ macro_export ]
macro_rules! struct_parser_display {
	( $($rest:tt)* ) => {
		struct_parser! ($($rest)*);
		struct_display! ($($rest)*);
	};
}

#[ macro_export ]
macro_rules! display {

	( $formatter:ident $(,)? ) => {};
	( $formatter:ident, $field:ident $(,$($rest:tt)*)? ) => {
		Display::fmt (& $field, $formatter) ?;
		display! ($formatter, $($($rest)*)?);
	};
	( $formatter:ident, $field:ident = $parse_fn:path $(,$($rest:tt)*)? ) => {
		Display::fmt (& $field, $formatter) ?;
		display! ($formatter, $($($rest)*)?);
	};
	( $formatter:ident, ($($field:ident),*) = $display:path $(,$($rest:tt)*)? ) => {
		$display (& ($($field),*), $formatter) ?;
		display! ($formatter, $($($rest)*)?);
	};
	( $formatter:ident, ($($field:ident),*) = ($parse:path, $display:path) $(,$($rest:tt)*)? ) => {
		$display (& ($($field),*), $formatter) ?;
		display! ($formatter, $($($rest)*)?);
	};
	( $formatter:ident, $field:ident = $rng_0:literal .. $(,$($rest:tt)*)? ) => {
		Display::fmt (& $field, $formatter) ?;
		display! ($formatter, $($($rest)*)?);
	};
	( $formatter:ident, $field:ident = $rng_0:literal ..= $rng_1:literal $(,$($rest:tt)*)? ) => {
		Display::fmt (& $field, $formatter) ?;
		display! ($formatter, $($($rest)*)?);
	};
	( $formatter:ident, $expect:literal $(,$($rest:tt)*)? ) => {
		Display::fmt ($expect, $formatter) ?;
		display! ($formatter, $($($rest)*)?);
	};
	( $formatter:ident, @array $field:ident $(,$($rest:tt)*)? ) => {
		Display::fmt (& $field.display_delim (""), $formatter) ?;
		display! ($formatter, $($($rest)*)?);
	};
	( $formatter:ident, @collect $field:ident $(,$($rest:tt)*)? ) => {
		Display::fmt (& $field.display_delim (""), $formatter) ?;
		display! ($formatter, $($($rest)*)?);
	};
	( $formatter:ident, @collect_some $field:ident $(,$($rest:tt)*)? ) => {
		Display::fmt (& $field.display_delim (""), $formatter) ?;
		display! ($formatter, $($($rest)*)?);
	};
	( $formatter:ident, @confirm $(,$($rest:tt)*)? ) => {
		display! ($formatter, $($($rest)*)?);
	};
	( $formatter:ident, @delim $delim:literal $field:ident $(,$($rest:tt)*)? ) => {
		Display::fmt (& $field.display_delim ($delim), $formatter) ?;
		display! ($formatter, $($($rest)*)?);
	};
	( $formatter:ident, @delim $delim:literal $field:ident { $($nest:tt)* } $(,$($rest:tt)*)?) => {
		let display_fn = display! (@nest $($nest)*);
		Display::fmt (& $field.display_delim_with ($delim, display_fn), $formatter) ?;
		display! ($formatter, $($($rest)*)?);
	};
	( $formatter:ident, @delim_some $delim:literal $field:ident $(,$($rest:tt)*)? ) => {
		Display::fmt (& $field.display_delim ($delim), $formatter) ?;
		display! ($formatter, $($($rest)*)?);
	};
	( $formatter:ident, @lines $field:ident $(,$($rest:tt)*)? ) => {
		Display::fmt (& $field.display_delim ("\n"), $formatter) ?;
		display! ($formatter, $($($rest)*)?);
	};
	( $formatter:ident, @lines $field:ident { $($nest:tt)* } $(,$($rest:tt)*)? ) => {
		let display_fn = display! (@nest $($nest)*);
		Display::fmt (& $field.display_delim_with ("\n", display_fn), $formatter) ?;
		display! ($formatter, $($($rest)*)?);
	};
	( $formatter:ident, @lines $field:ident = $display:ident $(,$($rest:tt)*)? ) => {
		Display::fmt (& $field.display_delim_with ("\n", $display), $formatter) ?;
		display! ($formatter, $($($rest)*)?);
	};
	( $formatter:ident, @str $field:ident = ($ch_0:literal ..= $ch_1:literal, $len:expr) $(,$($rest:tt)*)? ) => {
		Display::fmt (& $field, $formatter) ?;
		display! ($formatter, $($($rest)*)?);
	};
	( $formatter:ident, @str $field:ident = (|$ch_arg:ident| { $($ch_body:tt)* }, $len:expr) $(,$($rest:tt)*)? ) => {
		Display::fmt (& $field, $formatter) ?;
		display! ($formatter, $($($rest)*)?);
	};

	( @nest $($nest:tt)* ) => {
		|val, formatter: & mut fmt::Formatter| {
			display! (@nest_var formatter val $($nest)*);
			Ok (())
		}
	};

	( @nest_var $formatter:ident $val:ident $name:ident = [ $($display:tt)* ] $(,$($rest:tt)*)? ) => {
		if let & $name = $val {
			display! ($formatter, $($display)*);
		}
		display! (@nest_var $formatter $val $($($rest)*)?);
	};
	( @nest_var $formatter:ident $val:ident $name:ident: $type:ty = [ $($display:tt)* ] $(,$($rest:tt)*)? ) => {
		if let & $name = $val {
			display! ($formatter, $($display)*);
		}
		display! (@nest_var $formatter $val $($($rest)*)?);
	};
	( @nest_var $formatter:ident $val:ident $var:ident ( $($decl:tt)* ) = [ $($display:tt)* ] $(,$($rest:tt)*)? ) => {
		if let & $var ($($decl)*) = $val {
			display! ($formatter, $($display)*);
		}
		display! (@nest_var $formatter $val $($($rest)*)?);
	};
	( @nest_var $formatter:ident $val:ident $var:ident { $($decl:tt)* } = [ $($display:tt)* ] $(,$($rest:tt)*)? ) => {
		if let & $var { $($decl)* } = $val {
			display! ($formatter, $($display)*);
		}
		display! (@nest_var $formatter $val $($($rest)*)?);
	};
	( @nest_var $formatter:ident $val:ident $(,)? ) => { };

}

#[ macro_export ]
macro_rules! enum_display {

	( $enum_name:ident, $($rest:tt)* ) => {
		impl Display for $enum_name {
			fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
				enum_display! (@variants $enum_name, self, formatter, $($rest)*);
				panic! ("Unhandled variant {}::{:?}", stringify! ($enum_name), self);
			}
		}
	};

	( @variants $enum_name:ident, $self:ident, $formatter:ident $(,)? ) => {};

	(
		@variants $enum_name:ident, $self:ident, $formatter:ident,
		$var_name:ident { $($var_fields:tt)* } = |$var_arg:ident| { $($var_body:tt)* }
		$(, $($rest:tt)* )?
	) => {
		if let $enum_name::$var_name { $($var_fields)* } = $self {
			let $var_arg = $formatter;
			return (|| -> fmt::Result { $($var_body)*; Ok (()) }) ();
		};
		enum_display! (@variants $enum_name, $self, $formatter, $($($rest)*)?);
	};

	(
		@variants $enum_name:ident, $self:ident, $formatter:ident,
		$var_name:ident ( $($var_fields:tt)* ) = |$var_arg:ident| { $($var_body:tt)* }
		$(, $($rest:tt)* )?
	) => {
		if let $enum_name::$var_name ( $($var_fields)* ) = $self {
			let $var_arg = & mut $formatter;
			$($var_body)*
			return Ok (());
		};
		enum_display! (@variants $enum_name, $self, $formatter, $($($rest)*)?);
	};

	(
		@variants $enum_name:ident, $self:ident, $formatter:ident,
		$var_name:ident = |$var_arg:ident| { $($var_body:tt)* }
		$(, $($rest:tt)* )?
	) => {
		if let $enum_name::$var_name = $self {
			let $var_arg = & mut $formatter;
			$($var_body)*
			return Ok (());
		};
		enum_display! (@variants $enum_name, $self, $formatter, $($($rest)*)?);
	};

	(
		@variants $enum_name:ident, $self:ident, $formatter:ident,
		$var_name:ident { $($var_fields:tt)* } = [ $($var_arg:tt)* ]
		$(, $($rest:tt)* )?
	) => {
		if let $enum_name::$var_name { $($var_fields)* } = $self {
			display! ($formatter, $($var_arg)*);
			return Ok (());
		};
		enum_display! (@variants $enum_name, $self, $formatter, $($($rest)*)?);
	};

	(
		@variants $enum_name:ident, $self:ident, $formatter:ident,
		$var_name:ident ( $($var_fields:tt)* ) = [ $($var_arg:tt)* ]
		$(, $($rest:tt)* )?
	) => {
		if let $enum_name::$var_name ( $($var_fields)* ) = $self {
			display! ($formatter, $($var_arg)*);
			return Ok (());
		};
		enum_display! (@variants $enum_name, $self, $formatter, $($($rest)*)?);
	};

	(
		@variants $enum_name:ident, $self:ident, $formatter:ident,
		$var_name:ident = [ $($var_arg:tt)* ]
		$(, $($rest:tt)* )?
	) => {
		if let $enum_name::$var_name = $self {
			display! ($formatter, $($var_arg)*);
			return Ok (());
		};
		enum_display! (@variants $enum_name, $self, $formatter, $($($rest)*)?);
	};

}
