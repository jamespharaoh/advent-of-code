use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::iter;
use std::mem;
use std::path::PathBuf;

fn main () -> Result <(), Box <dyn Error>> {
	println! ("cargo:rerun-if-changed=build.rs");
	let pkg_name = env::var ("CARGO_PKG_NAME") ?;
	let pkg_name_parts = pkg_name.split ('-').collect::<Vec <_>> ();
	if pkg_name_parts [0] != "aoc" { panic! () }
	let year = pkg_name_parts [1];
	if pkg_name_parts.len () == 2 {
		write_file (
			"src/lib.rs",
			iter::empty ()
				.chain (replace_placeholders (
					templates::ALL_LIB_ABOVE_RS,
					& HashMap::from_iter (vec! [
						("${YEAR}", year),
					])))
				.chain ((1 ..= 25)
					.filter_map (|day|
						if PathBuf::from (format! ("day-{:02}", day)).exists () {
							Some (replace_placeholders (
								templates::ALL_LIB_DAY_RS,
								& HashMap::from_iter (vec! [
									("${YEAR}", year),
									("${DAY}", & format! ("{:02}", day)),
								] ),
							))
						} else { None })
					.flatten ())
				.chain (replace_placeholders (
					templates::ALL_LIB_BELOW_RS,
					& HashMap::from_iter (vec! [
						("${YEAR}", year),
					]))),
		) ?;
	} else if pkg_name_parts.len () == 4 {
		if pkg_name_parts [2] != "day" { panic! () }
		let day = pkg_name_parts [3];
		write_file (
			"src/main.rs",
			replace_placeholders (templates::DAY_BIN_RS, & HashMap::from_iter (vec! [
				("${YEAR}", year),
				("${DAY}", day),
			])),
		) ?;
	}
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

	pub const ALL_LIB_ABOVE_RS: & [& str] = & [
		"#![ doc (html_playground_url = \"https://playground.example.com/\") ]",
		"",
		"use aoc_common::*;",
		"",
		"pub fn puzzle_metadata () -> Vec <Box <dyn puzzle::Puzzle>> {",
		"\tvec! [",
	];

	pub const ALL_LIB_DAY_RS: & [& str] = & [
		"\t\taoc_${YEAR}_day_${DAY}::puzzle_metadata (),",
	];

	pub const ALL_LIB_BELOW_RS: & [& str] = & [
		"\t]",
		"}",
	];

	pub const DAY_BIN_RS: & [& str] = & [
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
