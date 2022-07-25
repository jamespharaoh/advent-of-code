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
		for house in (input as f64 * 2.0 / 10.0).sqrt () as u32 .. {
			let mut total = 0;
			for div_0 in 1 .. {
				if div_0 * div_0 > house { break }
				let div_1 = house / div_0;
				if div_0 * div_1 != house { continue }
				total += div_0 * 10;
				if div_0 != div_1 { total += div_1 * 10; }
			}
			if total >= input { return Ok (house) }
		}
		unreachable! ();
	}

	pub fn part_two (input: u32) -> GenResult <u32> {
		for house in (input as f64 * 2.0 / 11.0).sqrt () as u32 .. {
			let mut total = 0;
			for div_0 in 1 .. {
				if div_0 * div_0 > house { break }
				let div_1 = house / div_0;
				if div_0 * div_1 != house { continue }
				if div_1 <= 50 { total += div_0 * 11; }
				if div_0 <= 50 && div_0 != div_1 { total += div_1 * 11; }
			}
			if total >= input { return Ok (house) }
		}
		unreachable! ();
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

}
