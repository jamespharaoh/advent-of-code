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
macro_rules! display {

	( $formatter:ident $(,)? ) => {};

	// field

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
		display! (@nest $($nest)*) ($field, $formatter) ?;
		display! ($formatter, $($($rest)*)?);
	};
	( $formatter:ident, $field:ident = $range:expr $(,$($rest:tt)*)? ) => {
		::std::fmt::Display::fmt (& $field, $formatter) ?;
		display! ($formatter, $($($rest)*)?);
	};

	// (tuple, ...)

	( $formatter:ident, ($($field:ident),*) = $display:path $(,$($rest:tt)*)? ) => {
		$display (& ($($field),*), $formatter) ?;
		display! ($formatter, $($($rest)*)?);
	};
	( $formatter:ident, ($($field:ident),*) = ($parse:path, $display:path) $(,$($rest:tt)*)? ) => {
		$display (& ($($field),*), $formatter) ?;
		display! ($formatter, $($($rest)*)?);
	};
	( $formatter:ident, ($($field:ident),*) { $($nest:tt)* } $(,$($rest:tt)*)? ) => {
		display! (@nest $($nest)*) (& ($($field),*), $formatter) ?;
		display! ($formatter, $($($rest)*)?);
	};
	( $formatter:ident, $expect:literal $(,$($rest:tt)*)? ) => {
		::std::fmt::Display::fmt ($expect, $formatter) ?;
		display! ($formatter, $($($rest)*)?);
	};

	// @array

	( $formatter:ident, @array $field:ident $(,$($rest:tt)*)? ) => {
		::std::fmt::Display::fmt (& $field.display_delim (""), $formatter) ?;
		display! ($formatter, $($($rest)*)?);
	};

	// @array_delim

	( $formatter:ident, @array_delim $delim:literal $field:ident $(,$($rest:tt)*)? ) => {
		::std::fmt::Display::fmt (& $field.display_delim ($delim), $formatter) ?;
		display! ($formatter, $($($rest)*)?);
	};
	( $formatter:ident, @array_delim $delim:literal $field:ident { $($nest:tt)* } $(,$($rest:tt)*)? ) => {
		::std::fmt::Display::fmt (
			& $field.display_delim_with ($delim, display! (@nest $($nest)*)),
			$formatter) ?;
		display! ($formatter, $($($rest)*)?);
	};

	// @char

	( $formatter:ident, @char $field:ident = |$arg:ident| { $($parse:tt)* } $(,$($rest:tt)*)? ) => {
		$formatter.write_char (* $field) ?;
		display! ($formatter, $($($rest)*)?);
	};

	// @collect

	( $formatter:ident, @collect $field:ident $(,$($rest:tt)*)? ) => {
		::std::fmt::Display::fmt (& $field.display_delim (""), $formatter) ?;
		display! ($formatter, $($($rest)*)?);
	};
	( $formatter:ident, @collect $field:ident = $parse:path $(,$($rest:tt)*)? ) => {
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

	// @collect_max

	( $formatter:ident, @collect_max $max:literal $field:ident $(,$($rest:tt)*)? ) => {
		::std::fmt::Display::fmt (& $field.display_delim (""), $formatter) ?;
		display! ($formatter, $($($rest)*)?);
	};

	// @collect_some

	( $formatter:ident, @collect_some $field:ident $(,$($rest:tt)*)? ) => {
		::std::fmt::Display::fmt (& $field.display_delim (""), $formatter) ?;
		display! ($formatter, $($($rest)*)?);
	};

	// @collect_some_max

	( $formatter:ident, @collect_some_max $max:literal $field:ident $(,$($rest:tt)*)? ) => {
		::std::fmt::Display::fmt (& $field.display_delim (""), $formatter) ?;
		display! ($formatter, $($($rest)*)?);
	};

	// @confirm

	( $formatter:ident, @confirm $(,$($rest:tt)*)? ) => {
		display! ($formatter, $($($rest)*)?);
	};

	// @delim

	( $formatter:ident, @delim $delim:literal $field:ident $(,$($rest:tt)*)? ) => {
		::std::fmt::Display::fmt (& $field.display_delim ($delim), $formatter) ?;
		display! ($formatter, $($($rest)*)?);
	};
	( $formatter:ident, @delim $delim:literal $field:ident { $($nest:tt)* } $(,$($rest:tt)*)?) => {
		let display_fn = display! (@nest $($nest)*);
		::std::fmt::Display::fmt (& $field.display_delim_with ($delim, display_fn), $formatter) ?;
		display! ($formatter, $($($rest)*)?);
	};
	( $formatter:ident, @delim $delim:literal $field:ident = ($rng_0:literal .. ) $(,$($rest:tt)*)? ) => {
		::std::fmt::Display::fmt (& $field.display_delim ($delim), $formatter) ?;
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

	// @delim_some

	( $formatter:ident, @delim_some $delim:literal $field:ident $(,$($rest:tt)*)? ) => {
		::std::fmt::Display::fmt (& $field.display_delim ($delim), $formatter) ?;
		display! ($formatter, $($($rest)*)?);
	};

	// @display

	( $formatter:ident, @display { $($body:tt)* } $(,$($rest:tt)*)? ) => {
		$($body)*;
		display! ($formatter, $($($rest)*)?);
	};

	// @lines

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

	// @opt

	( $formatter:ident, @opt $field:ident { $($nest:tt)* } $(,$($rest:tt)*)? ) => {
		if $field != default () {
			let display_fn = display! (@nest $($nest)*);
			display_fn (& $field, $formatter) ?;
		}
	};

	// @parse

	( $formatter:ident, @parse $(|$arg:ident|)? { $($body:tt)* } $(,$($rest:tt)*)? ) => {
		display! ($formatter, $($($rest)*)?);
	};
	( $formatter:ident, @parse $field:ident { $($body:tt)* } $(,$($rest:tt)*)? ) => {
		let _ = $field;
		display! ($formatter, $($($rest)*)?);
	};

	// @skip

	( $formatter:ident, @skip $display:literal $(,$($rest:tt)*)? ) => {
		$formatter.write_str ($display) ?;
		display! ($formatter, $($($rest)*)?);
	};

	// @str

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
	( @nest type = $type:ty; $($nest:tt)* ) => {
		|val: & $type, formatter: & mut ::std::fmt::Formatter| {
			display! (@nest_var formatter val $($nest)*);
			::std::result::Result::Ok (())
		}
	};
	( @nest display_type = $type:ty; $($nest:tt)* ) => {
		|val: & $type, formatter: & mut ::std::fmt::Formatter| {
			display! (@nest_var formatter val $($nest)*);
			::std::result::Result::Ok (())
		}
	};
	( @nest input_lifetime = $input_life:lifetime; type = $type:ty; $($nest:tt)* ) => {
		|val: & $type, formatter: & mut ::std::fmt::Formatter| {
			display! (@nest_var formatter val $($nest)*);
			::std::result::Result::Ok (())
		}
	};
	( @nest $($nest:tt)* ) => {
		|val, formatter: & mut ::std::fmt::Formatter| {
			display! (@nest_var formatter val $($nest)*);
			::std::result::Result::Ok (())
		}
	};

	( @nest_var $formatter:ident $val:ident $lit:literal = [ $($display:tt)* ] $(,$($rest:tt)*)? ) => {
		if let & lit = $val {
			display! ($formatter, $($display)*);
		}
		display! (@nest_var $formatter $val $($($rest)*)?);
	};
	( @nest_var $formatter:ident $val:ident $name:ident $( if ($cond:expr) )? = [ $($display:tt)* ] $(,$($rest:tt)*)? ) => {
		if let ($name, true) = ($val, display! (@opt_cond $($cond)?)) {
			display! ($formatter, $($display)*);
		} else {
			display! (@nest_var $formatter $val $($($rest)*)?);
		}
	};
	( @nest_var $formatter:ident $val:ident $name_0:ident::$name_1:ident = [ $($display:tt)* ] $(,$($rest:tt)*)? ) => {
		if let $name_0::$name_1 = $val {
			display! ($formatter, $($display)*);
		} else {
			display! (@nest_var $formatter $val $($($rest)*)?);
		}
	};
	( @nest_var $formatter:ident $val:ident $name:ident: $type:ty = [ $($display:tt)* ] $(,$($rest:tt)*)? ) => {
		if let $name = $val {
			display! ($formatter, $($display)*);
		} else {
			display! (@nest_var $formatter $val $($($rest)*)?);
		}
	};
	( @nest_var $formatter:ident $val:ident ( $($item:tt),* ) $( if ($cond:expr) )? = [ $($display:tt)* ] $(,$($rest:tt)*)? ) => {
		if let (($($item),*), true) = ($val, display! (@opt_cond $($cond)?)) {
			display! ($formatter, $($display)*);
		} else {
			display! (@nest_var $formatter $val $($($rest)*)?);
		}
	};
	( @nest_var $formatter:ident $val:ident $var:ident ( $($decl:tt)* ) = [ $($display:tt)* ] $(,$($rest:tt)*)? ) => {
		if let $var ($($decl)*) = $val {
			display! ($formatter, $($display)*);
		} else {
			display! (@nest_var $formatter $val $($($rest)*)?);
		}
	};
	( @nest_var $formatter:ident $val:ident $var:ident { $($decl:tt)* } = [ $($display:tt)* ] $(,$($rest:tt)*)? ) => {
		if let & $var { $($decl)* } = $val {
			display! ($formatter, $($display)*);
		} else {
			display! (@nest_var $formatter $val $($($rest)*)?);
		}
	};
	( @nest_var $formatter:ident $val:ident $(,)? ) => { };

	( @opt_cond $cond:expr ) => {
		$cond
	};
	( @opt_cond ) => {
		true
	};

}
