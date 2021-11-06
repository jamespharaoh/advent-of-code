use std::collections::HashMap;
use std::error::Error;

pub fn aoc2018_day5_part2 (input: & str) -> Result <(), Box <dyn Error>> {
	let mut results: HashMap <char, String> = HashMap::new ();
	for remove_ch in 'a' ..= 'z' {
		let polymer: String = input.trim ().chars ().filter (
			|ch| ch.to_ascii_lowercase () != remove_ch,
		).collect ();
		let mut result = String::new ();
		for ch in polymer.chars () {
			if let Some (last_char) = result.chars ().last () {
				if last_char.to_ascii_lowercase () == ch.to_ascii_lowercase ()
						&& last_char.is_ascii_lowercase () != ch.is_ascii_lowercase () {
					result.pop ();
					continue;
				}
			}
			result.push (ch);
		}
		results.insert (remove_ch, result);
	}
	let (ch, result) = results.iter ().min_by_key (
		|(_, result)| result.len (),
	).unwrap ();
	println! ("Most problematic unit type: {}", ch);
	println! ("Resulting polymer: {}", result);
	println! ("Length: {}", result.len ());
	Ok (())
}
