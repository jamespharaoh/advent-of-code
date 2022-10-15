pub trait ArgsParse {
	fn parse <Iter> (iter: Iter) -> Result <Self, ArgsParseError>
		where Iter: IntoIterator <Item = OsString>;
}

pub enum ArgsParseError {
	Unexpected (OsString),
	MissingValue (OsString),
	Invalid (OsString, OsString, String),
}

impl Display for ArgsParseError {
	fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
		match * self {
			Self::Unexpected (ref arg) =>
				write! ("Unexpected argument {}", arg.display ()),
			Self::MissingValue (ref arg) =>
				write! ("Missing value for {}", arg.display ()),
			Self::Invalid (ref arg, ref val, ref msg) =>
				write! ("Invalid value for {}: {}: {}", arg.display (), val.display (), msg),
		}
	}
}

impl Error for ArgsParseError {
}

/*
basic requirements:

- handle basic commands "(none)", "part-1" and "part-2"
- args for those commands "--input" and "--repeat"
- extension commands for puzzles
- args for those extension commands
*/

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

		impl ArgsParse for $name {

			const ARGS_DEF: & [(& str, & str)] = & [ $(
				(stringify! ($mem_name), stringify! ($mem_type)),
			)* ];

			$( if stringify! ($mem_type == "bool") {
				$mem_name = Some (false);
			} )*

			for & (arg_name, arg_type) in ARGS_DEF {
				// TODO
			}

			fn parse <Iter> (iter: Iter) -> Result <Self, ArgsParseError>
					where Iter: IntoIterator <Item = OsString> {
				$( let mut $mem_name: Option <$mem_type> = None; )*
				todo! ();
				Ok (Self { $(
					$mem_name: $mem_name.ok_or (ArgsParseError::MissingValue (stringify! ($mem_name))) ?,
				)* })
			}

		}

	};
}
