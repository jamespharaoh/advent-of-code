use std::error::Error;
use std::fs;

fn main () -> Result <(), Box <dyn Error>> {
	let input_string = fs::read_to_string ("input") ?;
	let mut result = String::new ();
	for ch in input_string.trim ().chars () {
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
	println! ("Length: {}", result.len ());
	Ok (())
}
