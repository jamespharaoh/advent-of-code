//! Logic for solving the puzzles

use super::*;
use input::Input;

pub fn part_one (input: & Input) -> GenResult <u32> {
	Ok (
		input.module_masses.iter ().copied ()
			.map (|mass| if 6 < mass { mass / 3 - 2 } else { 0 })
			.try_sum::<u32> () ?
	)
}

pub fn part_two (input: & Input) -> GenResult <u32> {
	Ok (
		input.module_masses.iter ().copied ()
			.map (|mut mass| {
				let mut sum = 0;
				loop {
					mass /= 3;
					if mass <= 2 { break }
					mass -= 2;
					sum += mass;
				}
				sum
			})
			.try_fold (0, |sum, item| chk! (sum + item)) ?
	)
}
