//! Advent of Code 2015: Day 17: No Such Thing as Too Much
//!
//! [https://adventofcode.com/2015/day/17](https://adventofcode.com/2015/day/17)

use aoc_common::*;

puzzle_info! {
	name = "No Such Thing as Too Much";
	year = 2015;
	day = 17;
	parse = |input| model::parse_input (input);
	part_one = |input| logic::part_one (input);
	part_two = |input| logic::part_two (input);
}

/// Logic for solving the puzzles.
///
pub mod logic {

	use super::*;
	use model::Input;

	pub fn part_one (input: Input) -> GenResult <u32> {
		let (mut sizes, target) = input;
		if sizes.is_empty () { return Ok (0) }
		sizes.sort_by_key (|& size| cmp::Reverse (size));
		let num_combos = combos (& sizes, target).count ();
		Ok (num_combos as u32)
	}

	pub fn part_two (input: Input) -> GenResult <u32> {
		let (mut sizes, target) = input;
		if sizes.is_empty () { return Ok (0) }
		sizes.sort_by_key (|& size| cmp::Reverse (size));
		let smallest =
			combos (& sizes, target)
				.min_by_key (|combo| combo.len ())
				.map (|combo| combo.len ())
				.unwrap_or (0);

		let num_smallest =
			combos (& sizes, target)
				.filter (|combo| combo.len () == smallest)
				.count ();
		Ok (num_smallest as u32)
	}

	fn combos (sizes: & [u32], target: u32) -> impl Iterator <Item = Vec <u32>> + '_ {
		let mut state = iter::repeat (false).take (sizes.len ()).collect::<Vec <_>> ();
		let mut sum = 0;
		iter::from_fn (move || loop {
			let mut idx = state.len () - 1;
			loop {
				if state [idx] {
					state [idx] = false;
					sum -= sizes [idx];
					if idx > 0 { idx -= 1 } else { return None }
				} else if 150 < sum
						|| sum + sizes.iter ().copied ().skip (idx).sum::<u32> () < target {
					if idx > 0 { idx -= 1 } else { return None }
				} else { break }
			}
			state [idx] = true;
			sum += sizes [idx];
			if sum == target {
				return Some (
					Iterator::zip (sizes.iter ().copied (), state.iter ().copied ())
						.filter (|& (_, state)| state)
						.map (|(size, _)| size)
						.collect::<Vec <_>> ()
				);
			}
		})
	}

}

/// Representation of the puzzle input, etc.
///
pub mod model {

	use super::*;
	use parser::*;

	pub type Input = (Vec <u32>, u32);

	pub fn parse_input (input: & [& str]) -> GenResult <Input> {
		let (input, target) = if let Some (target_str) = input [0].strip_prefix ("TARGET=") {
			(& input [ 1 .. ], target_str.parse () ?)
		} else { (input, 150) };
		let buckets = input.iter ().enumerate ().map (|(line_idx, line)|
			Parser::wrap (line, |parser| Ok (parser.int::<u8> () ? as u32))
				.map_parse_err (|char_idx| format! ("Invalid input: line {}: col {}: {}",
					line_idx + 1, char_idx + 1, line))
		).collect::<GenResult <_>> () ?;
		Ok (( buckets, target ))
	}

}

#[ cfg (test) ]
mod examples {

	use super::*;

	#[ test ]
	fn part_one () -> GenResult <()> {
		let input = model::parse_input (& ["TARGET=25", "20", "15", "10", "5", "5"]) ?;
		assert_eq_ok! (4, logic::part_one (input));
		Ok (())
	}

	#[ test ]
	fn part_two () -> GenResult <()> {
		let input = model::parse_input (& ["TARGET=25", "20", "15", "10", "5", "5"]) ?;
		assert_eq_ok! (3, logic::part_two (input));
		Ok (())
	}

}
