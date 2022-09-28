//! Logic for solving the puzzles

use super::*;
use input::Input;

pub fn part_one (input: & Input) -> GenResult <u32> {
	sanity_check (input) ?;
	let mut buffer = VecDeque::new ();
	buffer.push_back (0_u32);
	for next in 1 ..= 2017 {
		buffer.rotate_left ((input.advance.pan_usize () + 1) % buffer.len ());
		buffer.push_front (next);
	}
	buffer.rotate_left (1);
	Ok (buffer.pop_front ().unwrap ())
}

pub fn part_two (input: & Input) -> GenResult <u32> {
	sanity_check (input) ?;
	let mut pos = 0_u32;
	let mut size = 1_u32;
	let mut second = 1_u32;
	let mut next = 1_u32;
	while next <= 50_000_000 {

		// advance quickly without wrapping

		if input.advance < size {
			let mut iters = (size - pos - 1) / input.advance;
			if 50_000_000 < iters + next { iters = 50_000_000 - next; }
			pos += iters * (input.advance + 1);
			size += iters;
			next += iters;
		}

		// advance slowly with wrapping

		pos = (pos + input.advance) % size + 1;
		if pos == 1 { second = next; }
		size += 1;
		next += 1;

	}
	Ok (second)
}

fn sanity_check (input: & Input) -> GenResult <()> {
	if ! (2 .. 1_000).contains (& input.advance) {
		return Err ("Advance must be at least two and less than one thousand".into ());
	}
	Ok (())
}
