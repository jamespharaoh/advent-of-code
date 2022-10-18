//! Logic for solving the puzzles

use super::*;

use input::Input;

pub fn part_one (input: & Input) -> GenResult <u32> {
	let mut num_twos = 0_u32;
	let mut num_threes = 0_u32;
	let mut counts: HashMap <char, u16> = HashMap::new ();
	for box_id in input.box_ids.iter () {
		counts.clear ();
		for ch in box_id.chars () {
			* counts.entry (ch).or_insert (0) += 1;
		}
		if counts.values ().any (|& count| count == 2) { num_twos += 1; }
		if counts.values ().any (|& count| count == 3) { num_threes += 1; }
	}
	Ok (num_twos * num_threes)
}

pub fn part_two (input: & Input) -> GenResult <String> {
	for index_0 in 0 .. input.box_ids.len () - 1 {
		let line_0 = & input.box_ids [index_0];
		let mut buffer = String::new ();
		for index_1 in index_0 + 1 .. input.box_ids.len () {
			let line_1 = & input.box_ids [index_1];
			buffer.clear ();
			buffer.extend (
				Iterator::zip (line_0.chars (), line_1.chars ())
					.filter (|& (ch_0, ch_1)| ch_0 == ch_1)
					.map (|(ch, _)| ch));
			if buffer.len () + 1 != line_0.len () { continue }
			return Ok (buffer);
		}
	}
	Err ("No solution found".into ())
}
