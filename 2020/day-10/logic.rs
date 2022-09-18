//! Logic for solving the puzzles

use super::*;

use input::Input;

pub fn part_one (input: & Input) -> GenResult <u16> {
	let sequence = get_sequence (input) ?;
	let (num_ones, num_threes) =
		sequence.iter ().copied ()
			.tuple_windows ()
			.map (|(prev, next)| next - prev)
			.fold ((0, 0), |(num_ones, num_threes), diff| (
				num_ones + u16::from (diff == 1),
				num_threes + u16::from (diff == 3),
			));
	Ok (num_ones * num_threes)
}

pub fn part_two (input: & Input) -> GenResult <u64> {
	let sequence = get_sequence (input) ?;
	let mut work = vec! [0; sequence.len ()];
	work [0] = 1;
	for idx in 0 .. work.len () {
		let cur = sequence [idx];
		let num = work [idx];
		for (next_idx, & next) in sequence [idx + 1 .. ].iter ().enumerate () {
			let next_idx = idx + 1 + next_idx;
			if cur + 3 < next { break }
			work [next_idx] = Int::add_2 (work [next_idx], num) ?;
		}
	}
	Ok (* work.last ().unwrap ())
}

fn get_sequence (input: & Input) -> GenResult <Vec <u16>> {
	let mut sequence: Vec <u16> =
		input.adapters.iter ().copied ()
			.chain (iter::once (0))
			.sorted ()
			.collect ();
	sequence.push (* sequence.last ().unwrap () + 3);
	if sequence.iter ().tuple_windows ().any (|(& a, & b)| a == b) {
		return Err ("Duplicated adapter".into ());
	}
	Ok (sequence)
}
