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
