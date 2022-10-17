#[ macro_export ]
macro_rules! enum_parser_display {
	( $($rest:tt)* ) => {
		enum_parser! ($($rest)*);
		enum_display! ($($rest)*);
	};
}

#[ macro_export ]
macro_rules! enum_display {

	( $enum_name:ident $( <$($param:tt),*> )?, $($rest:tt)* ) => {
		impl $( <$($param),*> )? ::std::fmt::Display for $enum_name $(<$($param),*>)? {
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
macro_rules! parse_display_enum {

	( $(
		$( #[ $($enum_attrs:tt)* ] )*
		$enum_vis:vis enum $enum_name:ident {
			$(
				$( #[ $($var_attrs:tt)* ] )*
				$var_name:ident = $var_str:literal
			),*
			$(,)?
		}
	)* ) => { $(

		$( #[ $($enum_attrs)* ] )*
		$enum_vis enum $enum_name {
			$(
				$( #[ $($var_attrs)* ] )*
				$var_name,
			)*
		}

		impl $enum_name {

			pub const NUM_VARIANTS: usize = [ $(stringify! ($var_name)),* ].len ();
			pub const VARIANTS: [$enum_name; Self::NUM_VARIANTS] = [ $(Self::$var_name),* ];

			#[ inline ]
			pub const fn as_str (self) -> & 'static str {
				match self {
					$( Self::$var_name => $var_str, )*
				}
			}

			#[ inline ]
			pub const fn idx (self) -> usize {
				parse_display_enum! (@variant_to_idx self [0] [] $($var_name)*)
			}

		}

		impl ::std::fmt::Display for $enum_name {
			fn fmt (
				& self,
				formatter: & mut ::std::fmt::Formatter,
			) -> ::std::fmt::Result {
				write! (formatter, "{}", match * self {
					$( Self::$var_name => $var_str, )*
				}) ?;
				Ok (())
			}
		}

		impl <'inp> ::aoc_common::parser::FromParser <'inp> for $enum_name {
			fn from_parser (
				parser: & mut ::aoc_common::parser::Parser <'inp>,
			) -> ::aoc_common::parser::ParseResult <Self> {
				parser.any ()
					$( .of (|parser| {
						parser.expect ($var_str) ?;
						Ok (Self::$var_name)
					}) ) *
					.done ()
			}
		}

		impl ::std::str::FromStr for $enum_name {
			type Err = ();
			fn from_str (src: & str) -> Result <Self, ()> {
				match src {
					$( $var_str => Ok (Self::$var_name), )*
					_ => Err (()),
				}
			}
		}

	)* };

	(@variant_to_idx $self:ident [$next_idx:expr] [$($data:tt)*] $var_name:ident $($rest:tt)*) => {
		parse_display_enum! (@variant_to_idx $self [$next_idx + 1] [$($data)* ($var_name, $next_idx)] $($rest)*)
	};
	(@variant_to_idx $self:ident [$next_idx:expr] [$(($name:ident, $idx:expr))*]) => {
		match $self { $(Self::$name => $idx),* }
	};

}

#[ macro_export ]
macro_rules! enum_decl_parser_display {

	( $(
		$( #[ $($attrs:tt)* ] )*
		$vis:vis enum $name:ident {
			$(
				$( #[ $($var_attr:tt)* ] )*
				$var_name:ident
				$(( $($tuple_name:ident: $tuple_type:ty),* ))?
				$({ $($struct_name:ident: $struct_type:ty),* })?
					= [ $($var_parse:tt)* ]
			),*
			$(,)?
		}
	)* ) => { $(

		$( #[ $($attrs)* ] )*
		$vis enum $name {
			$(
				$( #[ $($var_attr)* ] )*
				$var_name
				$(( $($tuple_type),* ))?
				$({ $($struct_name: $struct_type),* })?
			),*
		}

		enum_display! {
			$name,
			$(
				$var_name
				$(( $($tuple_name),* ))?
				$({ $($struct_name),* })?
					= [ $($var_parse)* ]
			),*
		}

		enum_parser! {
			$name,
			$(
				$var_name
				$(( $($tuple_name),* ))?
				$({ $($struct_name),* })?
					= [ $($var_parse)* ]
			),*
		}

	)* };

	(
		input_lifetime = $inp:tt;
		$( #[ $($attrs:tt)* ] )*
		$vis:vis enum $name:ident <$param:tt> {
			$(
				$( #[ $($var_attr:tt)* ] )*
				$var_name:ident
					$(($($tuple_name:ident: $tuple_type:ty),*))?
						= [ $($var_parse:tt)* ]
			),*
			$(,)?
		}
	) => {

		$( #[ $($attrs)* ] )*
		$vis enum $name <$param> {
			$(
				$( #[ $($var_attr)* ] )*
				$var_name $(( $($tuple_type),* ))?
			),*
		}

		enum_display! {
			$name <$param>,
			$( $var_name $(($($tuple_name),*))? = [ $($var_parse)* ] ),*
		}

		enum_parser! {
			input_lifetime = $inp;
			$name <$param>,
			$( $var_name $(($($tuple_name),*))? = [ $($var_parse)* ] ),*
		}

	};

}
