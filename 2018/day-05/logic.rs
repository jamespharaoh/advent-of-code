//! Logic for solving the puzzles

use super::*;

use input::Input;

pub fn part_one (input: & Input) -> GenResult <u32> {
	Ok (reduce (input.polymer.chars ()).len ().pan_u32 ())
}

pub fn part_two (input: & Input) -> GenResult <u32> {
	let polymer = reduce (input.polymer.chars ());
	Ok (
		('a' ..= 'z')
			.map (|remove_ch| (remove_ch, remove_ch.to_ascii_uppercase ()))
			.map (|(remove_ch_lower, remove_ch_upper)| polymer.iter ().copied ()
				.filter (move |& ch| ch != remove_ch_lower && ch != remove_ch_upper))
			.map (|polymer_chars| reduce (polymer_chars))
			.map (|polymer| polymer.len ().pan_u32 ())
			.min ()
			.unwrap ()
	)
}

pub fn reduce (polymer: impl IntoIterator <Item = char>) -> Vec <char> {
	let mut result: Vec <char> = Vec::new ();
	for next_ch in polymer {
		if let Some (last_ch) = result.last () {
			if last_ch.pan_u32 ().abs_diff (next_ch.pan_u32 ()) == 'a'.pan_u32 () - 'A'.pan_u32 () {
				result.pop ();
				continue;
			}
		}
		result.push (next_ch);
	}
	result
}
