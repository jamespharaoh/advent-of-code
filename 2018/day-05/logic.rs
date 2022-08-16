//! Logic for solving the puzzles

use super::*;
use input::Input;

pub fn part_one (input: & Input) -> GenResult <u32> {
	let mut result = String::new ();
	for ch in input.polymer.chars () {
		if let Some (last_char) = result.chars ().last () {
			if last_char.to_ascii_lowercase () == ch.to_ascii_lowercase ()
					&& last_char.is_ascii_lowercase () != ch.is_ascii_lowercase () {
				result.pop ();
				continue;
			}
		}
		result.push (ch);
	}
	Ok (result.len ().as_u32 ())
}

pub fn part_two (input: & Input) -> GenResult <u32> {
	let mut polymer: Vec <char> = Vec::with_capacity (input.polymer.len ());
	polymer.extend (input.polymer.chars ());
	let mut buffer: Vec <char> = Vec::with_capacity (polymer.len ());
	let shortest = ('a' ..= 'z')
		.map (move |remove_ch| {
			buffer.clear ();
			let mut last_char_opt: Option <char> = None;
			for ch in polymer.iter_vals ()
					.filter (|ch| ch.to_ascii_lowercase () != remove_ch) {
				if let Some (last_char) = last_char_opt {
					if last_char.to_ascii_lowercase () == ch.to_ascii_lowercase ()
							&& last_char.is_ascii_lowercase () != ch.is_ascii_lowercase () {
						buffer.pop ();
						last_char_opt = buffer.last ().copied ();
						continue;
					}
				}
				buffer.push (ch);
				last_char_opt = Some (ch);
			}
			buffer.len ().as_u32 ()
		})
		.min ()
		.unwrap ();
	Ok (shortest)
}
