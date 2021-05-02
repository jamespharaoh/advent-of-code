use std::collections::HashMap;
use std::fs;

fn main () {
	let input_string = fs::read_to_string ("input").unwrap ();
	let input_lines: Vec <& str> = input_string.trim ().split ('\n').collect ();
	let passports: Vec <HashMap <& str, & str>> = input_lines.split (|line| * line == "").map (
		|passport_lines| passport_lines.iter ().map (
			|passport_line| passport_line.split (' '),
		).flatten ().map (|field_str| {
			let field_parts: Vec <& str> = field_str.split (':').collect ();
			if field_parts.len () != 2 { panic! () }
			(field_parts [0], field_parts [1])
		}).collect (),
	).collect ();
	let required_fields = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
	let num_valid = passports.iter ().filter (
		|passport| required_fields.iter ().all (|field| passport.contains_key (field)),
	).count ();
	println! ("Number of valid passports: {}", num_valid);
}
