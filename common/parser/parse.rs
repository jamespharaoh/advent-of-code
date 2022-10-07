#[ macro_export ]
macro_rules! parse {
	( $parser:expr, $($rest:tt)* ) => {
		parse! (@recurse $parser, $($rest)*);
	};
	( @recurse $parser:expr $(,)? ) => {
	};
	( @recurse $parser:expr, $it_0:tt $(, $($rest:tt)*)? ) => {
		parse! (@item $parser, $it_0);
		parse! (@recurse $parser $(, $($rest)*)?);
	};
	( @recurse $parser:expr, $it_0:tt $it_1:tt $(, $($rest:tt)*)? ) => {
		parse! (@item $parser, $it_0 $it_1);
		parse! (@recurse $parser $(, $($rest)*)?);
	};
	( @recurse $parser:expr, $it_0:tt $it_1:tt $it_2:tt $(, $($rest:tt)*)? ) => {
		parse! (@item $parser, $it_0 $it_1 $it_2);
		parse! (@recurse $parser $(, $($rest)*)?);
	};
	( @recurse $parser:expr, $it_0:tt $it_1:tt $it_2:tt $it_3:tt $(, $($rest:tt)*)? ) => {
		parse! (@item $parser, $it_0 $it_1 $it_2 $it_3);
		parse! (@recurse $parser $(, $($rest)*)?);
	};
	( @recurse $parser:expr, $it_0:tt $it_1:tt $it_2:tt $it_3:tt $it_4:tt $(, $($rest:tt)*)? ) => {
		parse! (@item $parser, $it_0 $it_1 $it_2 $it_3 $it_4);
		parse! (@recurse $parser $(, $($rest)*)?);
	};
	( @recurse $parser:expr, $it_0:tt $it_1:tt $it_2:tt $it_3:tt $it_4:tt $it_5:tt $(, $($rest:tt)*)? ) => {
		parse! (@item $parser, $it_0 $it_1 $it_2 $it_3 $it_4 $it_5);
		parse! (@recurse $parser $(, $($rest)*)?);
	};
	( @recurse $parser:expr, $it_0:tt $it_1:tt $it_2:tt $it_3:tt $it_4:tt $it_5:tt $it_6:tt $(, $($rest:tt)*)? ) => {
		parse! (@item $parser, $it_0 $it_1 $it_2 $it_3 $it_4 $it_5 $it_6);
		parse! (@recurse $parser $(, $($rest)*)?);
	};
	( @recurse $parser:expr, $it_0:tt $it_1:tt $it_2:tt $it_3:tt $it_4:tt $it_5:tt $it_6:tt $it_7:tt $(, $($rest:tt)*)? ) => {
		parse! (@item $parser, $it_0 $it_1 $it_2 $it_3 $it_4 $it_5 $it_6 $it_7);
		parse! (@recurse $parser $(, $($rest)*)?);
	};
	( @recurse $parser:expr, $it_0:tt $it_1:tt $it_2:tt $it_3:tt $it_4:tt $it_5:tt $it_6:tt $it_7:tt $it_8:tt $(, $($rest:tt)*)? ) => {
		parse! (@item $parser, $it_0 $it_1 $it_2 $it_3 $it_4 $it_5 $it_6 $it_7 $it_8);
		parse! (@recurse $parser $(, $($rest)*)?);
	};
	( @recurse $parser:expr, $it_0:tt $it_1:tt $it_2:tt $it_3:tt $it_4:tt $it_5:tt $it_6:tt $it_7:tt $it_8:tt $it_9:tt $(, $($rest:tt)*)? ) => {
		parse! (@item $parser, $it_0 $it_1 $it_2 $it_3 $it_4 $it_5 $it_6 $it_7 $it_8 $it_9);
		parse! (@recurse $parser $(, $($rest)*)?);
	};
	( @recurse $parser:expr, $it_0:tt $it_1:tt $it_2:tt $it_3:tt $it_4:tt $it_5:tt $it_6:tt $it_7:tt $it_8:tt $it_9:tt $it_10:tt $(, $($rest:tt)*)? ) => {
		parse! (@item $parser, $it_0 $it_1 $it_2 $it_3 $it_4 $it_5 $it_6 $it_7 $it_8 $it_9 $it_10);
		parse! (@recurse $parser $(, $($rest)*)?);
	};
	( @item $parser:expr, $expect_str:literal ) => {
		$parser.expect ($expect_str) ?;
	};
	( @item $parser:expr, $item_name:ident ) => {
		let $item_name = $parser.item () ?;
	};
	( @item $parser:expr, $item_name:ident: $item_type:ty ) => {
		let $item_name: $item_type = $parser.item () ?;
	};
	( @item $parser:expr, $item_name:ident = $item_parse:ident ) => {
		let $item_name = $item_parse ($parser) ?;
	};
	( @item $parser:expr, $name:ident = ($parse:path, $display:path) ) => {
		let $name = $parse ($parser) ?;
	};
	( @item $parser:expr, $item_name:ident { $($nest:tt)* } ) => {
		let $item_name = parse! (@nest $($nest)*) ($parser) ?;
	};
	( @item $parser:expr, ($($item_name:ident),*) = $item_parse:ident ) => {
		let ($($item_name),*) = $item_parse ($parser) ?;
	};
	( @item $parser:expr, ($($name:ident),*) = ($parse:ident, $display:ident) ) => {
		let ($($name),*) = $parse ($parser) ?;
	};
	( @item $parser:expr, $item_name:ident = $item_range:expr ) => {
		let $item_name = $parser.item_range ($item_range) ?;
	};
	( @item $parser:expr, @array $item_name:ident ) => {
		let temp_vec: Vec <_> = $parser
			.repeat (Parser::item)
			.collect ();
		let $item_name = temp_vec.try_into ().map_err (|_err| $parser.err ()) ?;
	};
	( @item $parser:expr, @array_delim $delim:literal $name:ident $($rest:tt)* ) => {
		let temp_vec: Vec <_> = {
			parse! (@item $parser, @delim $delim temp_vec $($rest)*);
			temp_vec
		};
		let $name = temp_vec.try_into ().map_err (|_err| $parser.err ()) ?;
	};
	( @item $parser:expr, @char $name:ident = |$arg:ident| { $($valid:tt)* } ) => {
		let $name: char = match $parser.peek () {
			Some (ch) if (|$arg: char| { $($valid)* }) (ch) => { $parser.next ().unwrap (); ch },
			_ => { return Err ($parser.err ()) },
		};
	};
	( @item $parser:expr, @collect $name:ident ) => {
		let $name = $parser
			.repeat (Parser::item)
			.collect ();
	};
	( @item $parser:expr, @collect $name:ident: $type:ty ) => {
		let $name: $type = $parser
			.repeat (Parser::item)
			.collect ();
	};
	( @item $parser:expr, @collect $name:ident = $parse:path ) => {
		let $name = $parser
			.repeat ($parse)
			.collect ();
	};
	( @item $parser:expr, @collect $name:ident = ($parse:expr, $display:expr) ) => {
		let $name = $parser
			.repeat ($parse)
			.collect ();
	};
	( @item $parser:expr, @collect $name:ident { $($nest:tt)* } ) => {
		let $name = $parser
			.repeat (parse! (@nest $($nest)*))
			.collect ();
	};
	( @item $parser:expr, @collect_some $item_name:ident ) => {
		let mut temp_iter = $parser.repeat (Parser::item);
		let $item_name = match temp_iter.next () {
			Some (first) => iter::once (first).chain (temp_iter).collect (),
			None => return Err ($parser.err ()),
		};
	};
	( @item $parser:expr, @collect_some $item_name:ident: $item_type:ty ) => {
		let $item_name: $item_type = $parser
			.repeat (Parser::item)
			.collect ();
		if $item_name.is_empty () { return Err ($parser.err ()) }
	};
	( @item $parser:expr, @delim $delim:literal $item_name:ident ) => {
		let $item_name = $parser
			.delim_fn ($delim, Parser::item)
			.collect ();
	};
	( @item $parser:expr, (@delim $delim:literal $item_name:ident): $item_type:ty ) => {
		let $item_name: $item_type = $parser
			.delim_fn ($delim, Parser::item)
			.collect ();
	};
	( @item $parser:expr, @delim $delim:literal $name:ident = $parse:path ) => {
		let $name = $parser
			.delim_fn ($delim, $parse)
			.collect ();
	};
	( @item $parser:expr, @delim $delim:literal $name:ident = ($rng_0:literal .. $rng_1:literal) ) => {
		let $name = $parser
			.delim_fn ($delim, |parser| parser.item_range ($rng_0 .. $rng_1))
			.collect ();
	};
	( @item $parser:expr, @delim $delim:literal $name:ident = ($rng_0:literal ..= $rng_1:literal) ) => {
		let $name = $parser
			.delim_fn ($delim, |parser| parser.item_range ($rng_0 ..= $rng_1))
			.collect ();
	};
	( @item $parser:expr, @delim $delim:literal $name:ident = ($parse:path, $display:path) ) => {
		let $name = $parser
			.delim_fn ($delim, $parse)
			.collect ();
	};
	( @item $parser:expr, @delim $delim:literal $item_name:ident { $($nest:tt)* } ) => {
		let $item_name = $parser
			.delim_fn ($delim, parse! (@nest $($nest)*))
			.collect ();
	};
	( @item $parser:expr, @delim_some $delim:literal $item_name:ident ) => {
		let mut temp_iter = $parser.delim_fn ($delim, Parser::item);
		let $item_name = match temp_iter.next () {
			Some (first) => iter::once (first).chain (temp_iter).collect (),
			None => return Err ($parser.err ()),
		};
	};
	( @item $parser:expr, @lines $item_name:ident ) => {
		let $item_name = $parser
			.delim_fn ("\n", Parser::item)
			.collect ();
	};
	( @item $parser:expr, @lines $name:ident: $type:ty { $($nest:tt)* } ) => {
		let $name: $type = $parser
			.delim_fn ("\n", parse! (@nest $($nest)*))
			.collect ();
	};
	( @item $parser:expr, @lines $name:ident = $rng_0:literal ..= $rng_1:literal ) => {
		let $name = $parser
			.delim_fn ("\n", |parser| parser.item_range ($rng_0 ..= $rng_1))
			.collect ();
	};
	( @item $parser:expr, @lines $name:ident = $parse:path ) => {
		let $name = $parser
			.delim_fn ("\n", $parse)
			.collect ();
	};
	( @item $parser:expr, @lines $name:ident = |$arg:ident| { $($body:tt)* } ) => {
		let $name = $parser
			.delim_fn ("\n", |$arg| { $($body)* })
			.collect ();
	};
	( @item $parser:expr, @lines $name:ident = ($parse:path, $display:path) ) => {
		let $name = $parser
			.delim_fn ("\n", $parse)
			.collect ();
	};
	( @item $parser:expr, @lines $item_name:ident { $($nest:tt)* } ) => {
		let $item_name = $parser
			.delim_fn ("\n", parse! (@nest $($nest)*))
			.collect ();
	};
	( @item $parser:expr, @opt $name:ident { $($nest:tt)* } ) => {
		let $name = $parser.any ()
			.of (parse! (@nest $($nest)*))
			.of (|parser| { Ok (default ()) })
			.done () ?;
	};
	( @item $parser:expr, @str $item_name:ident = (|$fn_arg:ident| { $($fn_body:tt)* }, $len:expr) ) => {
		let $item_name = $parser
			.take_rest_while (|$fn_arg| { $($fn_body)* }, $len) ?
			.into ();
	};
	( @item $parser:expr, @str $item_name:ident = ($rng_0:literal ..= $rng_1:literal, $len:expr) ) => {
		let $item_name = $parser
			.take_rest_while (|ch| ($rng_0 ..= $rng_1).contains (& ch), $len) ?
			.into ();
	};
	( @item $parser:expr, @end ) => {
		$parser.end () ?;
	};
	( @item $parser:expr, @confirm ) => {
		$parser.confirm ();
	};
	( @item $parser:expr, @skip ) => {
		$parser.skip_whitespace ( .. ) ?;
	};
	( @item $parser:expr, @skip $display:literal ) => {
		$parser.skip_whitespace ( .. ) ?;
	};

	/*( @nest input_lifetime = $input_life:lifetime; $decl:tt = [ $($parse:tt)* ] ) => {
		|parser: & mut Parser <$life>| {
			parse! (parser, $($parse)*);
			Ok ($decl)
		}
	};*/
	( @nest input_lifetime = $input_life:lifetime; $($var:tt)* ) => {
		|parser: & mut Parser <$input_life>| {
			let parser = parser.any ();
			parse! (@nest_var parser $($var)*);
			parser.done ()
		}
	};
	( @nest $($var:tt)* ) => {
		|parser: & mut Parser| {
			let parser = parser.any ();
			parse! (@nest_var parser $($var)*);
			parser.done ()
		}
	};
	( @nest_var $parser:ident $var:ident = [ $($parse:tt)* ] $(,$($rest:tt)*)? ) => {
		let $parser = $parser.of (|parser| {
			parse! (parser, $($parse)*);
			Ok ($var)
		});
		parse! (@nest_var $parser $($($rest)*)?);
	};
	( @nest_var $parser:ident $enum:ident::$var:ident = [ $($parse:tt)* ] $(,$($rest:tt)*)? ) => {
		let $parser = $parser.of (|parser| {
			parse! (parser, $($parse)*);
			Ok ($enum::$var)
		});
		parse! (@nest_var $parser $($($rest)*)?);
	};
	( @nest_var $parser:ident $var:ident: $type:ty = [ $($parse:tt)* ] $(,$($rest:tt)*)? ) => {
		let $parser = $parser.of (|parser| {
			parse! (parser, $($parse)*);
			Ok ($var)
		});
		parse! (@nest_var $parser $($($rest)*)?);
	};
	( @nest_var $parser:ident ( $($decl:tt)* ) = [ $($parse:tt)* ] $(,$($rest:tt)*)? ) => {
		let $parser = $parser.of (|parser| {
			parse! (parser, $($parse)*);
			Ok (( $($decl)* ))
		});
		parse! (@nest_var $parser $($($rest)*)?);
	};
	( @nest_var $parser:ident $var:ident ( $($decl:tt)* ) = [ $($parse:tt)* ] $(,$($rest:tt)*)? ) => {
		let $parser = $parser.of (|parser| {
			parse! (parser, $($parse)*);
			Ok ($var ($($decl)*))
		});
		parse! (@nest_var $parser $($($rest)*)?);
	};
	( @nest_var $parser:ident $var:ident { $($decl:tt)* } = [ $($parse:tt)* ] $(,$($rest:tt)*)? ) => {
		let $parser = $parser.of (|parser| {
			parse! (parser, $($parse)*);
			Ok ($var { $($decl)* })
		});
		parse! (@nest_var $parser $($($rest)*)?);
	};
	( @nest_var $parser:ident $(,)? ) => { };

}

#[ macro_export ]
macro_rules! enum_parser {

	( $enum_name:ident, $($rest:tt)* ) => {
		impl <'inp> FromParser <'inp> for $enum_name {
			fn from_parser (parser: & mut Parser <'inp>) -> ParseResult <Self> {
				let mut parser = parser.any ();
				enum_parser! (@variants $enum_name, parser, $($rest)*);
				parser.done ()
			}
		}
	};

	(
		input_lifetime = $inp:lifetime;
		$enum_name:ident <$param:tt>,
		$($rest:tt)*
	) => {
		impl <$inp> FromParser <$inp> for $enum_name <$param> {
			fn from_parser (parser: & mut Parser <'inp>) -> ParseResult <Self> {
				let mut parser = parser.any ();
				enum_parser! (@variants $enum_name, parser, $($rest)*);
				parser.done ()
			}
		}
	};

	( @variants $enum_name:ident, $parser:ident $(,)? ) => {};

	(
		@variants $enum_name:ident, $parser:ident,
		$var_name:ident { $($var_fields:tt)* } = |$var_arg:ident| { $($var_body:tt)* }
		$(,$($rest:tt)*)?
	) => {
		$parser = $parser.of (|$var_arg| {
			$($var_body)*
			Ok (Self::$var_name { $($var_fields)* })
		});
		enum_parser! (@variants $enum_name, $parser, $($($rest)*)?);
	};

	(
		@variants $enum_name:ident, $parser:ident,
		$var_name:ident ( $($var_fields:tt)* ) = |$var_arg:ident| { $($var_body:tt)* }
		$(, $($rest:tt)* )?
	) => {
		$parser = $parser.of (|$var_arg| {
			$($var_body)*
			Ok (Self::$var_name ( $($var_fields)* ))
		});
		enum_parser! (@variants $enum_name, $parser, $(, $($rest)*)?);
	};

	(
		@variants $enum_name:ident, $parser:ident,
		$var_name:ident { $($var_fields:tt)* } = [ $($var_args:tt)* ]
		$(, $($rest:tt)* )?
	) => {
		$parser = $parser.of (|parser| {
			parse! (parser, $($var_args)*);
			Ok (Self::$var_name { $($var_fields)* })
		});
		enum_parser! (@variants $enum_name, $parser, $($($rest)*)?);
	};

	(
		@variants $enum_name:ident, $parser:ident,
		$var_name:ident ( $($var_fields:tt)* ) = [ $($var_args:tt)* ]
		$(, $($rest:tt)* )?
	) => {
		$parser = $parser.of (|parser| {
			parse! (parser, $($var_args)*);
			Ok (Self::$var_name ( $($var_fields)* ))
		});
		enum_parser! (@variants $enum_name, $parser, $($($rest)*)?);
	};

	(
		@variants $enum_name:ident, $parser:ident,
		$var_name:ident = [ $($var_args:tt)* ]
		$(, $($rest:tt)* )?
	) => {
		$parser = $parser.of (|parser| {
			parse! (parser, $($var_args)*);
			Ok (Self::$var_name)
		});
		enum_parser! (@variants $enum_name, $parser, $($($rest)*)?);
	};

	(
		@variants $enum_name:ident, $parser:ident,
		$var_name:ident = |$var_arg:ident| { $($var_body:tt)* }
		$(, $($rest:tt)* )?
	) => {
		$parser = $parser.of (|$var_arg| {
			$($var_body)*
			Ok (Self::$var_name)
		});
		enum_parser! (@variants $enum_name, $parser, $(, $($rest)*)?);
	};

}
