use std::collections::HashMap;
use std::error::Error;
use std::iter;
use std::path::Path;
use std::path::PathBuf;

use crate::replace_placeholders;
use crate::write_file;

/// Generate code for a single year
///
pub fn prepare (year: & str) -> Result <(), Box <dyn Error>> {
	let static_part = |template| {
		replace_placeholders (
			template,
			& HashMap::from_iter (vec! [
				("${YEAR}", year),
			]))
	};
	let dynamic_part = |template| {
		(1_u32 ..= 25)
			.filter_map (move |day|
				PathBuf::from (format! ("day-{day:02}")).exists ().then_some (
					replace_placeholders (
						template,
						& HashMap::from_iter (vec! [
							("${YEAR}", year),
							("${DAY}", & format! ("{day:02}")),
						] ),
					)))
			.flatten ()
	};
	let src_path = PathBuf::from (if Path::new ("src/").exists () { "src/" } else { "./" });
	let mut lib_path = src_path.clone ();
	lib_path.push ("lib.rs");
	write_file (
		lib_path,
		iter::empty ()
			.chain (static_part (templates::LIB [0]))
			.chain (dynamic_part (templates::LIB [1]))
			.chain (static_part (templates::LIB [2]))
			.chain (dynamic_part (templates::LIB [3]))
			.chain (static_part (templates::LIB [4])),
	) ?;
	let mut main_path = src_path;
	main_path.push ("main.rs");
	write_file (
		main_path,
		static_part (templates::MAIN),
	) ?;
	Ok (())
}

/// Templates for generated file contents
///
mod templates {

	/// Template for lib.rs in a year
	///
	pub const LIB: & [& [& str]] = & [
		& [
			"#![ allow (clippy::missing_inline_in_public_items) ]",
			"",
			"use aoc_common::*;",
			"",
		],
		& [
			"pub use aoc_${YEAR}_day_${DAY} as day_${DAY};",
		],
		& [
			"",
			"#[ must_use ]",
			"pub fn puzzle_metadata () -> Vec <Box <dyn Puzzle>> {",
			"\tvec! [",
		],
		& [
			"\t\tday_${DAY}::puzzle_metadata (),",
		],
		& [
			"\t]",
			"}",
		],
	];

	/// Template for main.rs in a year
	///
	pub const MAIN: & [& str] = & [
		"use std::process::ExitCode;",
		"",
		"use aoc_common::*;",
		"",
		"fn main () -> ExitCode {",
		"\tpuzzle::year::main (& aoc_${YEAR}::puzzle_metadata (), true)",
		"}",
	];

}
