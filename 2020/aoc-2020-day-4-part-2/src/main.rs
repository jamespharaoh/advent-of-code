use std::collections::HashMap;
use std::fs;

fn main () {
	let input_string = fs::read_to_string ("input").unwrap ();
	let input_lines: Vec <& str> = input_string.trim ().split ('\n').collect ();
	let passports: Vec <Passport> = input_lines.split (|line| * line == "").map (
		|passport_lines| passport_lines.iter ().map (
			|passport_line| passport_line.split (' '),
		).flatten ().map (|field_str| {
			let field_parts: Vec <& str> = field_str.split (':').collect ();
			if field_parts.len () != 2 { panic! () }
			(field_parts [0].to_string (), field_parts [1].to_string ())
		}).collect (),
	).collect ();
	for passport in passports.iter () {
		match ensure_valid (passport) {
			Err (field) => {
				if let Some (value) = passport.get (field) {
					println! ("Invalid value for '{}': {}", field, value);
				} else {
					println! ("Missing field '{}'", field);
				}
			},
			_ => (),
		}
	}
	let num_valid = passports.iter ().filter (|p| is_valid (* p)).count ();
	println! ("Number of valid passports: {}", num_valid);
}

fn is_valid (passport: & HashMap <String, String>) -> bool {
	ensure_valid (passport).is_ok ()
}

fn ensure_valid (passport: & HashMap <String, String>) -> Result <(), & 'static str> {

	let passport_byr: u32 = passport.get ("byr").ok_or ("byr") ?.parse ().unwrap ();
	if passport_byr < 1920 || passport_byr > 2002 { return Err ("byr") }

	let passport_iyr: u32 = passport.get ("iyr").ok_or ("iyr") ?.parse ().unwrap ();
	if passport_iyr < 2010 || passport_iyr > 2020 { return Err ("iyr") }

	let passport_eyr: u32 = passport.get ("eyr").ok_or ("eyr") ?.parse ().unwrap ();
	if passport_eyr < 2020 || passport_eyr > 2030 { return Err ("eyr") }

	let passport_hgt = passport.get ("hgt").ok_or ("hgt") ?;
	let passport_hgt_value: u32 = passport_hgt [0 .. passport_hgt.len () - 2].parse ().unwrap ();
	if passport_hgt.ends_with ("cm") {
		if passport_hgt_value < 150 || passport_hgt_value > 193 { return Err ("hgt") }
	} else if passport_hgt.ends_with ("in") {
		if passport_hgt_value < 59 || passport_hgt_value > 76 { return Err ("hgt") }
	} else {
		return Err ("hgt");
	}

	let passport_hcl = passport.get ("hcl").ok_or ("hcl") ?;
	if passport_hcl.len () != 7 { return Err ("hcl") }
	if passport_hcl.chars ().next ().unwrap () != '#' { return Err ("hcl") }
	if ! passport_hcl.chars ().skip (1).all (
		|ch| ch.is_ascii_digit () || ('a' ..= 'f').contains (& ch),
	) { return Err ("hcl") }

	let passport_ecl = passport.get ("ecl").ok_or ("ecl") ?;
	if ! ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains (& passport_ecl.as_str ()) {
		return Err ("ecl");
	}

	let passport_pid = passport.get ("pid").ok_or ("pid") ?;
	if passport_pid.len () != 9 { return Err ("pid") }
	if ! passport_pid.chars ().all (|ch| ch.is_ascii_digit ()) { return Err ("pid") }
	
	return Ok (())

}

type Passport = HashMap <String, String>;
