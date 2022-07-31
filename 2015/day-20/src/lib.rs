//! Advent of Code 2015: Day 20: Infinite Elves and Infinite Houses
//!
//! [https://adventofcode.com/2015/day/20](https://adventofcode.com/2015/day/20)

use aoc_common::*;

puzzle_info! {
	name = "Infinite Elves and Infinite Houses";
	year = 2015;
	day = 20;
	parse = |input| input [0].parse::<u32> ();
	part_one = |input| logic::part_one (input);
	part_two = |input| logic::part_two (input);
}

/// Logic for solving the puzzles.
///
pub mod logic {

	use super::*;
	use nums::{ Int, IntConv, NumResult };

	pub type Val = u32;

	pub fn part_one (input: Val) -> GenResult <Val> {
		Ok (calc_result (input, 10, Val::MAX) ?)
	}

	pub fn part_two (input: Val) -> GenResult <Val> {
		Ok (calc_result (input, 11, 50) ?)
	}

	pub fn calc_result (input: Val, mul: Val, lim: Val) -> NumResult <Val> {

		let mut divs = Vec::new ();
		let mut extend_sqrt = Val::ONE;
		let mut extend = Val::ONE;

		for house in Val::ONE .. {
			let mut total = Val::ZERO;

			// decrement all the values in `divs`, or if they are at zero then include the
			// corresponding "elf" and its complement in the total and reset to the elf's number
			// minus one

			let mut div = Val::ZERO;
			for next in divs.iter_mut () {
				div = Val::add_2 (div, Val::ONE) ?;
				if * next == Val::ZERO {
					* next = Val::sub_2 (div, 1) ?;
					let comp =
						if div == Val::ONE { house }
						else if div == 2 { house >> 1_i32 }
						else { Val::div_2 (house, div) ? };
					if comp <= lim {
						total = Val::add_2 (total, Val::mul_2 (div, mul) ?) ?;
					}
					if comp != div && div <= lim {
						total = Val::add_2 (total, Val::mul_2 (comp, mul) ?) ?;
					}
				} else {
					* next = Val::sub_2 (* next, Val::ONE) ?;
				}
			}

			// once the house square root reaches a new integer we add it to divs, also we have to
			// include the corresponding elf in our total

			if house == extend {
				if divs.is_empty () {
					divs.push (Val::ZERO);
				} else {
					divs.push (Val::from_usize (divs.len ()) ?);
				}
				total = Val::add_2 (total, Val::mul_2 (extend_sqrt, mul) ?) ?;
				extend_sqrt = Val::add_2 (extend_sqrt, Val::ONE) ?;
				extend = Val::mul_2 (extend_sqrt, extend_sqrt) ?;
			}

			// return when we find a solution

			if total >= input { return Ok (house) }

		}

		unreachable! ();

	}

}

#[ cfg (test) ]
mod tests {

	use super::*;

	#[ test ]
	fn part_one () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("1", puzzle.part_one (& [ "1" ]));
		assert_eq_ok! ("1", puzzle.part_one (& [ "10" ]));
		assert_eq_ok! ("6", puzzle.part_one (& [ "100" ]));
		assert_eq_ok! ("48", puzzle.part_one (& [ "1000" ]));
		assert_eq_ok! ("360", puzzle.part_one (& [ "10000" ]));
		assert_eq_ok! ("3120", puzzle.part_one (& [ "100000" ]));
		assert_eq_ok! ("27720", puzzle.part_one (& [ "1000000" ]));
	}

	#[ test ]
	fn part_two () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("1", puzzle.part_two (& [ "1" ]));
		assert_eq_ok! ("1", puzzle.part_two (& [ "10" ]));
		assert_eq_ok! ("6", puzzle.part_two (& [ "100" ]));
		assert_eq_ok! ("36", puzzle.part_two (& [ "1000" ]));
		assert_eq_ok! ("336", puzzle.part_two (& [ "10000" ]));
		assert_eq_ok! ("2880", puzzle.part_two (& [ "100000" ]));
		assert_eq_ok! ("25200", puzzle.part_two (& [ "1000000" ]));
	}

}
