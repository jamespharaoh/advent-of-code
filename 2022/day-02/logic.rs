use super::*;

use input::Input;
use input::InputChoice::{ A, B, C };
use input::OutputChoice::{ X, Y, Z };

pub fn part_one (input: & Input) -> GenResult <u32> {
	Ok (
		input.strategy.iter ()
			.map (|& (inp, out)| {
				let shape = match out {
					X => 1,
					Y => 2,
					Z => 3,
				};
				let round = match (inp, out) {
					(A, Z) | (B, X) | (C, Y) => 0,
					(A, X) | (B, Y) | (C, Z) => 3,
					(A, Y) | (B, Z) | (C, X) => 6,
				};
				shape + round
			})
			.sum ()
	)
}

pub fn part_two (input: & Input) -> GenResult <u32> {
	Ok (
		input.strategy.iter ()
			.map (|& (inp, out)| {
				let shape = match (inp, out) {
					(A, Y) | (B, X) | (C, Z) => 1,
					(A, Z) | (B, Y) | (C, X) => 2,
					(A, X) | (B, Z) | (C, Y) => 3,
				};
				let round = match out {
					X => 0,
					Y => 3,
					Z => 6,
				};
				shape + round
			})
			.sum ()
	)
}
