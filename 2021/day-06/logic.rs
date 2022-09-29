use super::*;

use input::Input;

pub fn part_one (input: & Input) -> GenResult <u64> {
	calc_result (input, 80)
}

pub fn part_two (input: & Input) -> GenResult <u64> {
	calc_result (input, 256)
}

pub fn calc_result (input: & Input, days: u32) -> GenResult <u64> {
	let mut fishes: [u64; 9] = [0; 9];
	for & fish in & input.fish {
		fishes [fish.pan_usize ()] += 1;
	}
	for _ in 0 .. days {
		fishes = [
			fishes [1],
			fishes [2],
			fishes [3],
			fishes [4],
			fishes [5],
			fishes [6],
			fishes [7] + fishes [0],
			fishes [8],
			fishes [0],
		];
	}
	Ok (fishes.into_iter ().sum::<u64> ())
}
