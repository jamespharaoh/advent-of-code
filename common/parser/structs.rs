#[ macro_export ]
macro_rules! struct_parser_display {
	( $($rest:tt)* ) => {
		struct_parser! { $($rest)* }
		struct_display! { $($rest)* }
	};
}

#[ macro_export ]
macro_rules! struct_parser {

	( < $($rest:tt)* ) => {
		struct_parser! (@outer 'inp [] < $($rest)*);
	};

	( $first:ident $($rest:tt)* ) => {
		struct_parser! (@outer 'inp [] $first $($rest)*);
	};

	(
		@outer $inp_life_old:tt [$($param_decl:tt)*]
		input_lifetime = $inp_life:tt;
		$($rest:tt)*
	) => {
		struct_parser! (@outer $inp_life [$($param_decl)*] $($rest)*);
	};
		
	(
		@outer $inp_life:tt [$($param_decl_old:tt)*]
		params = { $($param_decl:tt)* }
		$($rest:tt)*
	) => {
		struct_parser! (@outer $inp_life [$($param_decl)*] $($rest)*);
	};
		
	(
		@outer $inp_life:tt [$($param_decl:tt)*]
		$( #[ $($attr:tt)* ] )*
		$name:ident
		$( < $($param:tt),* > )?
		{ $($fields:tt)* }
		= [ $($args:tt)* ]
	) => {
		struct_parser! (
			@main $( #[ $($attr)* ] )* $name $inp_life
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
		struct_parser! (
			@main $name $inp_life
			[ $( $($param_decl)* )? ]
			[ $($param),* ]
			[ ( $($fields)* ) ]
			[ $($args)* ]);
	};

	(
		@main $( #[ $($attr:tt)* ] )* $name:ident $inp_life:tt
		[ $($param_decl:tt)* ]
		[ $($param:tt)* ]
		[ $($fields:tt)* ]
		[ $($args:tt)* ]
	) => {
		$( #[ $($attr)* ] )*
		impl <$inp_life, $($param_decl)*> FromParser <$inp_life> for $name <$($param)*> {
			#[ inline ]
			fn from_parser (parser: & mut Parser <$inp_life>) -> ParseResult <Self> {
				parse! (parser, $($args)*);
				Ok (Self $($fields)*)
			}
		}
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
