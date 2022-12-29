//! Logic for solving the puzzles

use super::*;

use input::Input;
use self::PassportError::{ Length, Missing, Parse, Range };

pub fn part_one (input: & Input) -> GenResult <u32> {
	Ok (
		input.passports.iter ()
			.filter (|& passport| passport_errors (passport).iter ()
				.all (|error| ! matches! (* error, Missing (_))))
			.count ()
			.pan_u32 ()
	)
}

pub fn part_two (input: & Input) -> GenResult <u32> {
	Ok (
		input.passports.iter ()
			.filter (|& passport| passport_errors (passport).iter ()
				.all (|error| ! matches! (* error,
					Missing (_) | Length (_, _) | Parse (_, _) | Range (_, _))))
			.count ()
			.pan_u32 ()
	)
}

fn passport_errors <'inp> (
	passport: & Vec <(InpStr <'inp>, InpStr <'inp>)>,
) -> Vec <PassportError <'inp>> {

	macro_rules! check_fields {

		(
			vars { $passport:ident, $errors:ident }
			$( $field:ident { $($checks:tt)* } )*
		) => {
			$( let mut $field: Option <InpStr <'inp>> = None; )*
			for & (ref name, ref value) in $passport {
				match name.as_ref () {
					$( stringify! ($field) => {
						if $field.is_some () {
							$errors.push (PassportError::Duplicated (stringify! ($field)));
						} else {
							$field = Some (value.clone ());
						}
					}, )*
					_ => $errors.push (PassportError::Unrecognised (name.clone ())),
				}
			}
			$( check_fields! (@field ($field, stringify! ($field), $errors) { $($checks)* }); )*
		};

		( @field ($val:expr, $name:expr, $errors:ident) { reqd; $($rest:tt)* } ) => {
			if let Some (val) = $val {
				check_fields! (@field_rest (val, $name, val, $errors) { $($rest)* });
			} else {
				$errors.push (PassportError::Missing ($name));
			}
		};
		( @field ($val:expr, $name:expr, $errors:ident) { opt; $($rest:tt)* } ) => {
			if let Some (val) = $val {
				check_fields! (@field_rest (val, $name, val, $errors) { $($rest)* });
			}
		};

		( @field_rest ($val:expr, $name:expr, $orig:expr, $errors:ident) { len $len:literal; $($rest:tt)* } ) => {
			if $val.chars ().count () == $len {
				check_fields! (@field_rest ($val, $name, $orig, $errors) { $($rest)* });
			} else {
				$errors.push (PassportError::Length ($name, $orig));
			}
		};
		( @field_rest ($val:expr, $name:expr, $orig:expr, $errors:ident) { parse $parse:path; $($rest:tt)* } ) => {
			if let Ok (val) = $parse (& $val) {
				check_fields! (@field_rest (val, $name, $orig, $errors) { $($rest)* });
			} else {
				$errors.push (PassportError::Parse ($name, $orig));
			}
		};
		( @field_rest ($val:expr, $name:expr, $orig:expr, $errors:ident) { range ($range:pat); $($rest:tt)* } ) => {
			if matches! ($val, $range) {
				check_fields! (@field_rest ($val, $name, $orig, $errors) { $($rest)* });
			} else {
				$errors.push (PassportError::Range ($name, $orig));
			}
		};
		( @field_rest ($val:expr, $name:expr, $orig:expr, $errors:ident) { } ) => {
			#[ allow (let_underscore_drop) ]
			let _ = $val;
		};
	}

	let mut errors = Vec::new ();
	use Height::{ Cm, In };
	check_fields! {
		vars { passport, errors }
		byr { reqd; len 4; parse u32::from_str; range (1920 ..= 2002); }
		iyr { reqd; len 4; parse u32::from_str; range (2010 ..= 2020); }
		eyr { reqd; len 4; parse u32::from_str; range (2020 ..= 2030); }
		hgt { reqd; parse Height::from_str; range (Cm (150 ..= 193) | In (59 ..= 76)); }
		hcl { reqd; parse RgbColour::from_str; }
		ecl { reqd; parse EyeColour::from_str; }
		pid { reqd; len 9; parse u32::from_str; }
		cid { opt; }
	}
	errors

}

#[ derive (Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd) ]
enum PassportError <'inp> {
	Missing (& 'static str),
	Unrecognised (InpStr <'inp>),
	Duplicated (& 'static str),
	Length (& 'static str, InpStr <'inp>),
	Parse (& 'static str, InpStr <'inp>),
	Range (& 'static str, InpStr <'inp>),
}

enum Height { Cm (u32), In (u32) }

impl FromStr for Height {
	type Err = Option <ParseIntError>;
	fn from_str (src: & str) -> Result <Self, Option <ParseIntError>> {
		if let Some (val_src) = src.strip_suffix ("cm") {
			val_src.parse ().map (Height::Cm).map_err (Some)
		} else if let Some (val_src) = src.strip_suffix ("in") {
			val_src.parse ().map (Height::In).map_err (Some)
		} else { Err (None) }
	}
}

struct RgbColour;

impl FromStr for RgbColour {
	type Err = ();
	fn from_str (src: & str) -> Result <Self, ()> {
		src.chars ().enumerate ()
			.try_fold (0_u32, |count, (ch_idx, ch)| {
				let valid = if 0 < ch_idx {
					ch.is_ascii_hexdigit () && ! ch.is_ascii_uppercase ()
				} else { ch == '#' };
				if valid { Ok (count + 1) } else { Err (()) }
			})
			.and_then (|count| if count == 7 { Ok (Self) } else { Err (()) })
	}
}

enum_decl_parser_display! {
	#[ derive (Clone, Copy, Debug) ]
	enum EyeColour {
		Amb = [ "amb" ], Blu = [ "blu" ], Brn = [ "brn" ], Gry = [ "gry" ],
		Grn = [ "grn" ], Hzl = [ "hzl" ], Oth = [ "oth" ],
	}
}

from_parser_to_from_str! (EyeColour);
