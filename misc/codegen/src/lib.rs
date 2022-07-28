use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::iter;
use std::mem;
use std::path::PathBuf;

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
		if pkg_name_parts.len () < 2 { panic! () }
		if pkg_name_parts [0] != "aoc" { panic! () }
		let year = pkg_name_parts [1];
		if pkg_name_parts.len () == 2 {
			prepare_year (year) ?
		} else if pkg_name_parts.len () == 4 {
			if pkg_name_parts [2] != "day" { panic! () }
			let day = pkg_name_parts [3];
			prepare_day (year, day) ?
		} else { panic! () }
	}
	Ok (())
}

fn prepare_year (year: & str) -> Result <(), Box <dyn Error>> {
	let static_part = |template| {
		replace_placeholders (
			template,
			& HashMap::from_iter (vec! [
				("${YEAR}", year),
			]))
	};
	let dynamic_part = |template| {
		(1 ..= 25)
			.filter_map (move |day|
				if PathBuf::from (format! ("day-{:02}", day)).exists () {
					Some (replace_placeholders (
						template,
						& HashMap::from_iter (vec! [
							("${YEAR}", year),
							("${DAY}", & format! ("{:02}", day)),
						] ),
					))
				} else { None })
			.flatten ()
	};
	write_file (
		"src/lib.rs",
		iter::empty ()
			.chain (static_part (templates::YEAR_LIB [0]))
			.chain (dynamic_part (templates::YEAR_LIB [1]))
			.chain (static_part (templates::YEAR_LIB [2]))
			.chain (dynamic_part (templates::YEAR_LIB [3]))
			.chain (static_part (templates::YEAR_LIB [4])),
	) ?;
	write_file (
		"src/main.rs",
		static_part (templates::YEAR_MAIN),
	) ?;
	Ok (())
}

fn prepare_day (year: & str, day: & str) -> Result <(), Box <dyn Error>> {
	write_file (
		"src/main.rs",
		replace_placeholders (templates::DAY_MAIN, & HashMap::from_iter (vec! [
			("${YEAR}", year),
			("${DAY}", day),
		])),
	) ?;
	Ok (())
}

fn write_file <Item: AsRef <str>, LinesIter: IntoIterator <Item = Item>> (
	name: & str,
	lines: LinesIter,
) -> Result <(), Box <dyn Error>> {
	let mut new_contents = String::new ();
	for line in lines.into_iter () {
		let line = line.as_ref ();
		new_contents.push_str (line);
		new_contents.push ('\n');
	}
	let old_contents = fs::read_to_string (name).unwrap_or_default ();
	if old_contents != new_contents {
		let mut main_rs_file = File::create (name) ?;
		write! (& mut main_rs_file, "{}", new_contents) ?;
	}
	Ok (())
}

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

mod templates {

	pub const YEAR_LIB: & [& [& str]] = & [
		& [
			"use aoc_common::*;",
			"",
		],
		& [
			"pub use aoc_${YEAR}_day_${DAY} as day_${DAY};",
		],
		& [
			"",
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

	pub const YEAR_MAIN: & [& str] = & [
		"use aoc_common::*;",
		"",
		"fn main () -> GenResult <()> {",
		"\tpuzzle::run_year_and_exit (& aoc_${YEAR}::puzzle_metadata (), true)",
		"}",
	];

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
