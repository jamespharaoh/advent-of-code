//! Logic for solving the puzzles.

use super::*;

use model::Attr;
use input::Input;

pub fn part_one (input: & Input) -> GenResult <u16> {
	calc_result (input, |attr, num| match attr {
		Attr::Children => num == 3,
		Attr::Cats => num == 7,
		Attr::Samoyeds => num == 2,
		Attr::Pomeranians => num == 3,
		Attr::Akitas => num == 0,
		Attr::Vizslas => num == 0,
		Attr::Goldfish => num == 5,
		Attr::Trees => num == 3,
		Attr::Cars => num == 2,
		Attr::Perfumes => num == 1,
	})
}

pub fn part_two (input: & Input) -> GenResult <u16> {
	calc_result (input, |attr, num| match attr {
		Attr::Children => num == 3,
		Attr::Cats => num > 7,
		Attr::Samoyeds => num == 2,
		Attr::Pomeranians => num < 3,
		Attr::Akitas => num == 0,
		Attr::Vizslas => num == 0,
		Attr::Goldfish => num < 5,
		Attr::Trees => num > 3,
		Attr::Cars => num == 2,
		Attr::Perfumes => num == 1,
	})
}

pub fn calc_result (input: & Input, ticker_fn: fn (Attr, u8) -> bool) -> GenResult <u16> {
	let the_sue =
		input.sues.iter ()
			.filter (|sue| sue.attrs.iter ().copied ()
				.all (|(attr, num)| ticker_fn (attr, num)))
			.exactly_one ()
			.ok_or ("Expected exactly one match") ?;
	Ok (the_sue.number)
}
