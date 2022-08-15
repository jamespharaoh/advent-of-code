//! Advent of Code 2015: Day 17: No Such Thing as Too Much
//!
//! [https://adventofcode.com/2015/day/17](https://adventofcode.com/2015/day/17)
//!
//! # Input
//!
//! Each line is a decimal integer, representing the size of an available container.
//!
//! # Part one
//!
//! Work out the number of combinations of containers so that the size adds up to exactly 150. The
//! containers count separately, even if they are the same size.
//!
//! # Part two
//!
//! Work out the minimum number of containers to hold exactly 150, then work out the total number
//! of combinations of that many containers which can hold exactly 150.
//!
//! # Algorithm
//!
//! The private function `combos` provides an iterator over a list of combinations of container
//! sizes to match a given target. TODO how does this work?
//!
//! For part one, we just use [`count`](Iterator::count). For part two, we invoke `combos` twice,
//! the first time using [`min`](Iterator::min) to work out the minimum number of containers, then
//! using [`filter`](Iterator::filter) and [`count`](Iterator::count) to determine the number of
//! combinations with that specific number of containers.

#![ allow (clippy::missing_inline_in_public_items) ]

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
	use nums::IntConv;

	pub fn part_one (input: Input) -> GenResult <u32> {
		let (mut sizes, target) = input;
		sanity_checks (& sizes, target) ?;

		sizes.sort_by_key (|& size| cmp::Reverse (size));
		let num_combos = combos (& sizes, target).count ();
		Ok (num_combos.as_u32 ())
	}

	pub fn part_two (input: Input) -> GenResult <u32> {
		let (mut sizes, target) = input;
		sanity_checks (& sizes, target) ?;

		// sort first, this makes the algorithm more efficient

		sizes.sort_by_key (|& size| cmp::Reverse (size));

		// work out smallest number of containers

		let smallest =
			combos (& sizes, target)
				.min_by_key (Vec::len)
				.map_or (0, |combo| combo.len ());

		// count combinations with this number

		let num_smallest =
			combos (& sizes, target)
				.filter (|combo| combo.len () == smallest)
				.count ();

		// return

		Ok (num_smallest.as_u32 ())

	}

	fn sanity_checks (sizes: & [u32], target: u32) -> GenResult <()> {
		if sizes.len () > 20 { Err ("Refusing to deal with more than 20 containers") ?; }
		if sizes.iter ().copied ().sum::<u32> () < target { Err ("No solution found") ?; }
		Ok (())
	}

	/// Iterate over combinations of provided container given a specific total.
	///
	/// Returns an [`Iterator`] over [`Vec`]s of the sizes of each container. Note that these can
	/// appear to be duplicates, if there is more than one container with the same size.
	///
	fn combos (sizes: & [u32], target: u32) -> impl Iterator <Item = Vec <u32>> + '_ {
		let mut state = iter::repeat (false).take (sizes.len ()).collect::<Vec <_>> ();
		let mut sum = 0;
		iter::from_fn (move || loop {
			if state.is_empty () { return None }
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
	use nums::IntConv;

	pub type Input = (Vec <u32>, u32);

	pub fn parse_input (input: & [& str]) -> GenResult <Input> {
		let (input, target) = if let Some (target_str) = input [0].strip_prefix ("TARGET=") {
			(& input [ 1 .. ], target_str.parse () ?)
		} else { (input, 150) };
		let buckets = input.iter ().enumerate ().map (|(line_idx, line)|
			Parser::wrap (line, |parser| {
				let val = parser.int::<u8> () ?.as_u32 ();
				parser.end () ?;
				Ok (val)
			}).map_parse_err (|_, char_idx|
				format! ("Invalid input: line {}: col {}: {}", line_idx + 1, char_idx + 1, line))
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
