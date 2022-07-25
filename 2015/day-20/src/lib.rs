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

	pub fn part_one (input: u32) -> GenResult <u32> {
		Ok (calc_result (input, 10, u32::MAX))
	}

	pub fn part_two (input: u32) -> GenResult <u32> {
		Ok (calc_result (input, 11, 50))
	}

	pub fn calc_result (input: u32, mul: u32, lim: u32) -> u32 {
		let mut divs = Vec::new ();
		let mut extend_sqrt = 1;
		let mut extend = 1;
		for house in 1 .. {
			let mut total = 0_u32;
			for (div, val) in divs.iter_mut ().enumerate () {
				let div = div as u32 + 1;
				if * val == 0 {
					* val = div - 1;
					let comp = house / div;
					if comp <= lim { total += div * mul; }
					if comp != div && div <= lim { total += comp * mul; }
				} else {
					* val -= 1;
				}
			}
			if house == extend {
				if divs.is_empty () {
					divs.push (0);
				} else {
					divs.push (divs.len () as u32);
				}
				total += extend_sqrt * mul;
				extend_sqrt += 1;
				extend = extend_sqrt * extend_sqrt;
			}
			if total >= input { return house }
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
	}

}
