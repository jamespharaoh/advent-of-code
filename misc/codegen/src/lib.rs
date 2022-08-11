//! Generate boilerplate code, run from build.rs

use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::iter;
use std::mem;
use std::path::Path;
use std::path::PathBuf;

/// Entry point for code generation
///
/// # Errors
///
/// Returns any errors from the underlying IO operations unchanged
///
/// # Panics
///
/// If the package name does not conform to expectations.
///
#[ allow (clippy::missing_inline_in_public_items) ]
#[ allow (clippy::print_stdout) ]
pub fn invoke () -> Result <(), Box <dyn Error>> {
	println! ("cargo:rerun-if-changed=build.rs");
	println! ("cargo:rerun-if-changed=.");
	let pkg_name = env::var ("CARGO_PKG_NAME") ?;
	if pkg_name == "aoc-common" {
		let home = env::current_dir () ?;
		for year in ["2015", "2021"] {
			env::set_current_dir (format! ("../{}", year)) ?;
			prepare_year (year) ?;
			env::set_current_dir (& home) ?;
		}
	} else {
		let pkg_name_parts: Vec <& str> = pkg_name.split ('-').collect ();
		if pkg_name_parts.len () < 2 { Err ("Invalid package name") ? }
		if pkg_name_parts [0] != "aoc" { Err ("Invalid package name") ? }
		let year = pkg_name_parts [1];
		if pkg_name_parts.len () == 2 {
			prepare_year (year) ?;
		} else if pkg_name_parts.len () == 4 {
			if pkg_name_parts [2] != "day" { Err ("Invalid package name") ? }
			let day = pkg_name_parts [3];
			prepare_day (year, day) ?;
		} else { Err ("Invalid package name") ? }
	}
	Ok (())
}

/// Generate code for a single year
///
fn prepare_year (year: & str) -> Result <(), Box <dyn Error>> {
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
				PathBuf::from (format! ("day-{:02}", day)).exists ().then_some (
					replace_placeholders (
						template,
						& HashMap::from_iter (vec! [
							("${YEAR}", year),
							("${DAY}", & format! ("{:02}", day)),
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
			.chain (static_part (templates::YEAR_LIB [0]))
			.chain (dynamic_part (templates::YEAR_LIB [1]))
			.chain (static_part (templates::YEAR_LIB [2]))
			.chain (dynamic_part (templates::YEAR_LIB [3]))
			.chain (static_part (templates::YEAR_LIB [4])),
	) ?;
	let mut main_path = src_path;
	main_path.push ("main.rs");
	write_file (
		main_path,
		static_part (templates::YEAR_MAIN),
	) ?;
	Ok (())
}

/// Generate code for a single day
///
fn prepare_day (year: & str, day: & str) -> Result <(), Box <dyn Error>> {
	let src_path = PathBuf::from (if Path::new ("src/").exists () { "src/" } else { "./" });
	let mut main_path = src_path;
	main_path.push ("main.rs");
	write_file (
		main_path,
		replace_placeholders (templates::DAY_MAIN, & HashMap::from_iter (vec! [
			("${YEAR}", year),
			("${DAY}", day),
		])),
	) ?;
	Ok (())
}

/// Write the provided lines to a named file
///
fn write_file (
	name: impl AsRef <Path>,
	lines: impl IntoIterator <Item = impl AsRef <str>>,
) -> Result <(), Box <dyn Error>> {
	let mut new_contents = String::new ();
	for line_temp in lines {
		let line = line_temp.as_ref ();
		new_contents.push_str (line);
		new_contents.push ('\n');
	}
	let old_contents = fs::read_to_string (& name).unwrap_or_default ();
	if old_contents != new_contents {
		let mut main_rs_file = File::create (name) ?;
		write! (& mut main_rs_file, "{}", new_contents) ?;
	}
	Ok (())
}

/// Replace placeholders in some strings
///
fn replace_placeholders (lines: & [& str], replacements: & HashMap <& str, & str>) -> Vec <String> {
	lines.iter ().map (|line| {
		let (output, buffer) = line.chars ().fold ((String::new (), String::new ()),
			|(mut output, mut buffer), letter| {
				if (buffer.is_empty () && letter == '$')
						|| (buffer.len () == 1 && letter == '{')
						|| buffer.len () > 1 {
					buffer.push (letter);
					if letter == '}' {
						let replacement = replacements.get (buffer.as_str ()).unwrap_or_else (||
							panic! ("Replacement not found for: {}", buffer),
						);
						output.push_str (replacement);
						buffer = String::new ();
					}
				} else {
					output.push_str (mem::take (& mut buffer).as_str ());
					output.push (letter);
				}
				(output, buffer)
			},
		);
		if ! buffer.is_empty () { panic! () }
		output
	}).collect ()
}

/// Templates for generated file contents
///
mod templates {

	/// Template for lib.rs in a year
	///
	pub const YEAR_LIB: & [& [& str]] = & [
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
			"pub fn puzzle_metadata () -> Vec <Box <dyn puzzle::Puzzle>> {",
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
	pub const YEAR_MAIN: & [& str] = & [
		"use aoc_common::*;",
		"",
		"fn main () -> GenResult <()> {",
		"\tpuzzle::run_year_and_exit (& aoc_${YEAR}::puzzle_metadata (), true)",
		"}",
	];

	/// Template for main.rs in a day
	///
	pub const DAY_MAIN: & [& str] = & [
		"use std::env;",
		"use std::ffi::OsString;",
		"",
		"use aoc_common::*;",
		"use aoc_${YEAR}_day_${DAY}::*;",
		"",
		"fn main () -> GenResult <()> {",
		"	let args: Vec <OsString> = env::args_os ().collect ();",
		"	puzzle_metadata ().invoke (& args)",
		"}",
	];

}
