use super::*;

use input::Input;

pub fn part_one (input: & Input) -> GenResult <u32> {
	calc_result::<4> (input)
}

pub fn part_two (input: & Input) -> GenResult <u32> {
	calc_result::<14> (input)
}

fn calc_result <const TARGET: usize> (input: & Input) -> GenResult <u32> {
	let mut counts = [0_u32; 26];
	let mut chars = VecDeque::new ();
	for (pos, ch) in
		input.data.chars ()
			.map (|ch| ch.pan_usize () - 'a'.pan_usize ())
			.enumerate () {
		chars.push_back (ch);
		counts [ch] += 1;
		if chars.len () < TARGET { continue }
		if TARGET < chars.len () {
			let ch = chars.pop_front ().unwrap ();
			counts [ch] -= 1;
		}
		if counts.iter ().all (|& num| num < 2) {
			return Ok (pos.pan_u32 () + 1);
		}
	}
	Err ("No solution found".into ())
}
