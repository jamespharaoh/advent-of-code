use std::error::Error;

pub fn aoc2018_day5_part1 (input: & str) -> Result <String, Box <dyn Error>> {

	let mut result = String::new ();
	for ch in input.trim ().chars () {
		if let Some (last_char) = result.chars ().last () {
			if last_char.to_ascii_lowercase () == ch.to_ascii_lowercase ()
					&& last_char.is_ascii_lowercase () != ch.is_ascii_lowercase () {
				result.pop ();
				continue;
			}
		}
		result.push (ch);
	}

	println! ("Resulting polymer: {}", result);

	Ok (format! ("{}", result.len ()))

}
