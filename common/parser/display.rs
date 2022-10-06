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
		::std::fmt::Display::fmt (& $field, $formatter) ?;
		display! ($formatter, $($($rest)*)?);
	};
	( $formatter:ident, $field:ident = $parse_fn:path $(,$($rest:tt)*)? ) => {
		::std::fmt::Display::fmt (& $field, $formatter) ?;
		display! ($formatter, $($($rest)*)?);
	};
	( $formatter:ident, $field:ident = ($parse:path, $display:path) $(,$($rest:tt)*)? ) => {
		$display (& $field, $formatter) ?;
		display! ($formatter, $($($rest)*)?);
	};
	( $formatter:ident, $field:ident { $($nest:tt)* } $(,$($rest:tt)*)? ) => {
		display! (@nest $($nest)*) (& $field, $formatter) ?;
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
	( $formatter:ident, $field:ident = $range:expr $(,$($rest:tt)*)? ) => {
		::std::fmt::Display::fmt (& $field, $formatter) ?;
		display! ($formatter, $($($rest)*)?);
	};
	( $formatter:ident, $expect:literal $(,$($rest:tt)*)? ) => {
		::std::fmt::Display::fmt ($expect, $formatter) ?;
		display! ($formatter, $($($rest)*)?);
	};
	( $formatter:ident, @array $field:ident $(,$($rest:tt)*)? ) => {
		::std::fmt::Display::fmt (& $field.display_delim (""), $formatter) ?;
		display! ($formatter, $($($rest)*)?);
	};
	( $formatter:ident, @array_delim $delim:literal $field:ident $(,$($rest:tt)*)? ) => {
		::std::fmt::Display::fmt (& $field.display_delim ($delim), $formatter) ?;
		display! ($formatter, $($($rest)*)?);
	};
	( $formatter:ident, @char $field:ident = |$arg:ident| { $($parse:tt)* } $(,$($rest:tt)*)? ) => {
		$formatter.write_char (* $field) ?;
		display! ($formatter, $($($rest)*)?);
	};
	( $formatter:ident, @collect $field:ident $(,$($rest:tt)*)? ) => {
		::std::fmt::Display::fmt (& $field.display_delim (""), $formatter) ?;
		display! ($formatter, $($($rest)*)?);
	};
	( $formatter:ident, @collect $field:ident = ($parse:path, $display:path) $(,$($rest:tt)*)? ) => {
		::std::fmt::Display::fmt (& $field.display_delim_with ("", $display), $formatter) ?;
		display! ($formatter, $($($rest)*)?);
	};
	( $formatter:ident, @collect $field:ident { $($nest:tt)* } $(,$($rest:tt)*)? ) => {
		let display_fn = display! (@nest $($nest)*);
		::std::fmt::Display::fmt (& $field.display_delim_with ("", display_fn), $formatter) ?;
		display! ($formatter, $($($rest)*)?);
	};
	( $formatter:ident, @collect_some $field:ident $(,$($rest:tt)*)? ) => {
		::std::fmt::Display::fmt (& $field.display_delim (""), $formatter) ?;
		display! ($formatter, $($($rest)*)?);
	};
	( $formatter:ident, @confirm $(,$($rest:tt)*)? ) => {
		display! ($formatter, $($($rest)*)?);
	};
	( $formatter:ident, @delim $delim:literal $field:ident $(,$($rest:tt)*)? ) => {
		::std::fmt::Display::fmt (& $field.display_delim ($delim), $formatter) ?;
		display! ($formatter, $($($rest)*)?);
	};
	( $formatter:ident, @delim $delim:literal $field:ident { $($nest:tt)* } $(,$($rest:tt)*)?) => {
		let display_fn = display! (@nest $($nest)*);
		::std::fmt::Display::fmt (& $field.display_delim_with ($delim, display_fn), $formatter) ?;
		display! ($formatter, $($($rest)*)?);
	};
	( $formatter:ident, @delim $delim:literal $field:ident = ($rng_0:literal .. $rng_1:literal) $(,$($rest:tt)*)? ) => {
		::std::fmt::Display::fmt (& $field.display_delim ($delim), $formatter) ?;
		display! ($formatter, $($($rest)*)?);
	};
	( $formatter:ident, @delim $delim:literal $field:ident = ($rng_0:literal ..= $rng_1:literal) $(,$($rest:tt)*)? ) => {
		::std::fmt::Display::fmt (& $field.display_delim ($delim), $formatter) ?;
		display! ($formatter, $($($rest)*)?);
	};
	( $formatter:ident, @delim $delim:literal $field:ident = $parse:ident $(,$($rest:tt)*)? ) => {
		::std::fmt::Display::fmt (& $field.display_delim ($delim), $formatter) ?;
		display! ($formatter, $($($rest)*)?);
	};
	( $formatter:ident, @delim $delim:literal $field:ident = ($parse:path, $display:path) $(,$($rest:tt)*)? ) => {
		::std::fmt::Display::fmt (& $field.display_delim_with ($delim, $display), $formatter) ?;
		display! ($formatter, $($($rest)*)?);
	};
	( $formatter:ident, @delim_some $delim:literal $field:ident $(,$($rest:tt)*)? ) => {
		::std::fmt::Display::fmt (& $field.display_delim ($delim), $formatter) ?;
		display! ($formatter, $($($rest)*)?);
	};
	( $formatter:ident, @lines $field:ident $(,$($rest:tt)*)? ) => {
		::std::fmt::Display::fmt (& $field.display_delim ("\n"), $formatter) ?;
		display! ($formatter, $($($rest)*)?);
	};
	( $formatter:ident, @lines $field:ident { $($nest:tt)* } $(,$($rest:tt)*)? ) => {
		let display_fn = display! (@nest $($nest)*);
		::std::fmt::Display::fmt (& $field.display_delim_with ("\n", display_fn), $formatter) ?;
		display! ($formatter, $($($rest)*)?);
	};
	( $formatter:ident, @lines $field:ident = $display:ident $(,$($rest:tt)*)? ) => {
		::std::fmt::Display::fmt (& $field.display_delim_with ("\n", $display), $formatter) ?;
		display! ($formatter, $($($rest)*)?);
	};
	( $formatter:ident, @lines $field:ident = ($parse:ident, $display:ident) $(,$($rest:tt)*)? ) => {
		::std::fmt::Display::fmt (& $field.display_delim_with ("\n", $display), $formatter) ?;
		display! ($formatter, $($($rest)*)?);
	};
	( $formatter:ident, @opt $field:ident { $($nest:tt)* } $(,$($rest:tt)*)? ) => {
		if $field != default () {
			let display_fn = display! (@nest $($nest)*);
			display_fn (& $field, $formatter) ?;
		}
	};
	( $formatter:ident, @skip $display:literal $(,$($rest:tt)*)? ) => {
		$formatter.write_str ($display) ?;
		display! ($formatter, $($($rest)*)?);
	};
	( $formatter:ident, @str $field:ident = ($ch_0:literal ..= $ch_1:literal, $len:expr) $(,$($rest:tt)*)? ) => {
		::std::fmt::Display::fmt (& $field, $formatter) ?;
		display! ($formatter, $($($rest)*)?);
	};
	( $formatter:ident, @str $field:ident = (|$ch_arg:ident| { $($ch_body:tt)* }, $len:expr) $(,$($rest:tt)*)? ) => {
		::std::fmt::Display::fmt (& $field, $formatter) ?;
		display! ($formatter, $($($rest)*)?);
	};

	( @nest $name:ident = [ $($display:tt)* ] ) => {
		|$name, formatter: & mut ::std::fmt::Formatter| {
			display! (formatter, $($display)*);
			::std::result::Result::Ok (())
		}
	};
	( @nest $($nest:tt)* ) => {
		|val, formatter: & mut ::std::fmt::Formatter| {
			display! (@nest_var formatter val $($nest)*);
			::std::result::Result::Ok (())
		}
	};

	( @nest_var $formatter:ident $val:ident $name:ident = [ $($display:tt)* ] $(,$($rest:tt)*)? ) => {
		if let & $name = $val {
			display! ($formatter, $($display)*);
		}
		display! (@nest_var $formatter $val $($($rest)*)?);
	};
	( @nest_var $formatter:ident $val:ident $name_0:ident::$name_1:ident = [ $($display:tt)* ] $(,$($rest:tt)*)? ) => {
		if let & $name_0::$name_1 = $val {
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
	( @nest_var $formatter:ident $val:ident ( $($decl:tt)* ) = [ $($display:tt)* ] $(,$($rest:tt)*)? ) => {
		if let & ( $($decl)* ) = $val {
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
		impl ::std::fmt::Display for $enum_name {
			fn fmt (& self, formatter: & mut ::std::fmt::Formatter) -> ::std::fmt::Result {
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
			return (|| -> ::std::fmt::Result { $($var_body)*; Ok (()) }) ();
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

#[ macro_export ]
macro_rules! struct_display {

	( < $($rest:tt)* ) => {
		struct_display! (@outer 'inp [] < $($rest)*);
	};

	( $first:ident $($rest:tt)* ) => {
		struct_display! (@outer 'inp [] $first $($rest)*);
	};

	(
		@outer $inp_life_old:tt [$($param_decl:tt)*]
		input_lifetime = $inp_life:tt;
		$($rest:tt)*
	) => {
		struct_display! (@outer $inp_life [$($param_decl)*] $($rest)*);
	};

	(
		@outer $inp_life:tt [$($param_decl_old:tt)*]
		params = { $($param_decl:tt)* }
		$($rest:tt)*
	) => {
		struct_display! (@outer $inp_life [$($param_decl)*] $($rest)*);
	};

	(
		@outer $inp_life:tt [$($param_decl:tt)*]
		$name:ident
		$( < $($param:tt),* > )?
		{ $($fields:tt)* }
		= [ $($args:tt)* ]
	) => {
		struct_display! (
			@main $name $inp_life
			[ $($param_decl)* ]
			[ $( $($param),* )? ]
			[ { $($fields)* } ]
			[ $($args)* ]);
	};

	(
		@outer $inp_life:tt [$($param_decl:tt)*]
		$name:ident
		$( < $($param:tt),* > )?
		( $($fields:tt)* )
		= [ $($args:tt)* ]
	) => {
		struct_display! (
			@main $name $inp_life
			[ $($param_decl)* ]
			[ $( $($param),* )? ]
			[ ( $($fields)* ) ]
			[ $($args)* ]);
	};

	(
		@main $name:ident $inp_life:tt
		[ $($param_decl:tt)* ]
		[ $($param:tt),* ]
		[ $($fields:tt)* ]
		[ $($args:tt)* ]
	) => {
		impl <$inp_life, $($param_decl)*> ::std::fmt::Display
				for $name <$($param),*> {
			fn fmt (& self, formatter: & mut ::std::fmt::Formatter) -> ::std::fmt::Result {
				let Self $($fields)* = self;
				display! (formatter, $($args)*);
				::std::result::Result::Ok (())
			}
		}
	};

}
