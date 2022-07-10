use std::env;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::collections::HashMap;
use std::mem;

fn main () -> Result <(), Box <dyn Error>> {
	println! ("cargo:rerun-if-changed=build.rs");
	let pkg_name = env::var ("CARGO_PKG_NAME") ?;
	if ! pkg_name.starts_with ("aoc-2021-day-") { panic! () }
	let day = & pkg_name [13 .. ];
	write_file (
		& format! ("src/main.rs"),
		replace_placeholders (templates::BIN_RS, & HashMap::from_iter (vec! [
			("${YEAR}", "2021"),
			("${DAY}", day),
		])),
	) ?;
	Ok (())
}


fn write_file <'a, Item: AsRef <str>, LinesIter: IntoIterator <Item = Item>> (
	name: & str,
	lines: LinesIter,
) -> Result <(), Box <dyn Error>> {
	let mut new_contents = String::new ();
	for line in lines.into_iter () {
		let line = line.as_ref ();
		new_contents.push_str (& line);
		new_contents.push ('\n');
	}
	let old_contents = fs::read_to_string (name).unwrap_or (String::new ());
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
				if (buffer.len () == 0 && letter == '$')
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
					output.push_str (mem::replace (& mut buffer, String::new ()).as_str ());
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

	pub const BIN_RS: & 'static [& 'static str] = & [
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
