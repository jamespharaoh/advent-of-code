//! Advent of Code 2015: Day 24: It Hangs in the Balance
//!
//! [https://adventofcode.com/2015/day/24](https://adventofcode.com/2015/day/24)
//!
//! # Input
//!
//! Each line contains a decimal integer, representing the weight of a single package.
//!
//! # Part one
//!
//! Split the packages into three piles of equal weight. Optimise so that one pile has the fewest
//! possible number of packages. Further optimize so that the product of the weights of the
//! packages in the first pile is as small as possible. Produce that product as a result.
//!
//! # Part two
//!
//! The same as part one, but with four piles instead of three.
//!
//! # Algorithm
//!
//! - Iteratively build up a list of items to assign to each pile, always in order from heavier to
//!   lighter. This ensures that we find a solution with the smallest possible number of packages
//!   in the first pile before any other.
//! - Once we find a solution for all three piles, record the size and product of the first pile,
//!   and only consider solutions with a first pile which would signify an improvement.

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;

puzzle_info! {
	name = "It Hangs in the Balance";
	year = 2015;
	day = 24;
	parse = |input| model::Input::parse (input);
	part_one = |input| logic::part_one (input);
	part_two = |input| logic::part_two (input);
}

/// Logic for solving the puzzles.
///
pub mod logic {

	use super::*;
	use model::Input;
	use nums::Int;
	use nums::IntConv;

	pub fn part_one (input: Input) -> GenResult <u64> {
		calc_result::<3> (input)
	}

	pub fn part_two (input: Input) -> GenResult <u64> {
		calc_result::<4> (input)
	}

	fn calc_result <const PILES: usize> (input: Input) -> GenResult <u64> {

		// make sure the weights are in reverse order

		let weights: Vec <u32> =
			input.weights.into_iter ()
				.sorted_by_key (|& weight| cmp::Reverse (weight))
				.collect ();

		// sanity check (mainly for fuzzing)

		if weights.len () > 50 {
			Err ("Refusing to deal with more than 50 items") ?;
		}

		if weights.iter ().dedup ().count () != weights.len () {
			Err ("Refusing to deal with duplicated weights") ?;
		}

		if weights.iter ().copied ().any (|weight| weight > 200) {
			Err ("Refusing to deal with weights over 200") ?;
		}

		// work out the balanced weight of each pile

		let total_weight: u32 =
			weights.iter ().copied ()
				.fold (Ok (0), |sum, item| sum
					.and_then (|sum| u32::add_2 (sum, item))) ?;
		let want_pile_weight = total_weight / PILES.as_u32 ();
		if want_pile_weight * PILES.as_u32 () != total_weight {
			Err (format! ("Total weight is not a multiple of {}", PILES)) ?;
		}

		// once we find one solution we can rule out any first piles which are bigger or have a
		// higher quantum entanglement

		let mut max_len_0 = usize::MAX;
		let mut max_quantum_0 = u64::MAX;

		// stack holds a list of piles and the indexes of their contents

		let mut stack: Vec <Vec <usize>> = vec! [ vec! [] ];

		// todo holds continuations, first value is number of piles to retain, second is number of
		// items in top pile, third is new index to push to top pile, for initial state we have a
		// single pile and branch for the full list of weights as the next item in it

		let mut todo: Vec <(usize, usize, usize)> =
			(0 .. weights.len ())
				.rev ()
				.map (|idx| (1, 0, idx))
				.collect ();

		// iterate through continuations, shortcircuit allows us to quickly get back to the first
		// pile when we find a new solution, because we don't really care about the other piles, so
		// long as we know there's at least one solution for them

		let mut shortcircuit = false;
		while let Some ((trunc_0, trunc_1, idx)) = todo.pop () {
			if shortcircuit && trunc_0 > 1 { continue }
			shortcircuit = false;

			// truncate the number of piles according to the continuation

			stack.truncate (trunc_0);

			// truncate the items in the top pile according to the continuation

			let pile_stack = stack.last_mut ().unwrap ();
			pile_stack.truncate (trunc_1);

			// add on the next item from the continuation

			pile_stack.push (idx);
			let pile_stack_len = pile_stack.len ();

			// work out the top pile's weight, abort if it is too heavy

			let pile_weight: u32 =
				pile_stack.iter ().copied ().map (|idx| weights [idx]).sum ();
			if pile_weight > want_pile_weight { continue }

			// for the first pile only, check the quantum and abort if it's already too high

			if stack.len () == 1 {
				if max_len_0 < stack [0].len () { continue }
				let quantum_0 =
					stack [0].iter ().copied ()
						.map (|idx| weights [idx].as_u64 ())
						.fold (Ok (1), |prod, item| prod
							.and_then (|prod| u64::mul_2 (prod, item))) ?;
				if max_quantum_0 <= quantum_0 { continue }
			}

			// if this pile is now the right weight we start the next pile, or record a solution if
			// this is the last pile

			if pile_weight == want_pile_weight {
				if stack.len () < PILES {
					stack.push (vec! []);
					for idx in (0 .. weights.len ()).rev () {
						if stack.iter ()
							.any (|pile_stack| pile_stack.iter ().copied ()
								.any (|existing_idx| idx == existing_idx))
							{ continue }
						todo.push ((stack.len (), 0, idx));
					}
				} else {
					max_len_0 = stack [0].len ();
					max_quantum_0 =
						stack [0].iter ().copied ()
							.map (|idx| weights [idx].as_u64 ())
							.fold (Ok (1), |prod, item| prod
								.and_then (|prod| u64::mul_2 (prod, item))) ?;
					shortcircuit = true;
				}
				continue;
			}

			// branch out for every possible next weight to add to the top pile

			let min_idx =
				stack.last ().unwrap ().iter ().copied ()
					.map (|idx| idx + 1).max ().unwrap_or (0);
			for idx in (min_idx .. weights.len ()).rev () {
				if stack.iter ()
					.any (|pile_stack| pile_stack.iter ().copied ()
						.any (|existing_idx| idx == existing_idx))
					{ continue }
				todo.push ((stack.len (), pile_stack_len, idx));
			}

		}

		if max_quantum_0 == u64::MAX { Err ("No solution found") ?; }
		Ok (max_quantum_0)

	}

	#[ cfg (test) ]
	mod tests {

		use super::*;

		#[ test ]
		fn calc_result () {
			assert_eq_ok! (5, logic::calc_result::<3> (Input { weights: (1 ..= 5).collect () }));
			assert_err! ("No solution found",
				logic::calc_result::<3> (Input { weights: (1 ..= 3).collect () }));
			assert_err! ("Refusing to deal with more than 50 items",
				logic::calc_result::<3> (Input { weights: (0 ..= 50).collect () }));
			assert_err! ("Refusing to deal with duplicated weights",
				logic::calc_result::<3> (Input { weights: vec! [ 1, 1, 1 ] }));
			assert_err! ("Total weight is not a multiple of 3",
				logic::calc_result::<3> (Input { weights: vec! [ 1, 2, 3, 4 ] }));
		}

	}

}

/// Representation of the puzzle input, etc.
///
pub mod model {

	use super::*;

	#[ derive (Clone, Debug) ]
	pub struct Input {
		pub weights: Vec <u32>,
	}

	impl Input {
		pub fn parse (input: & [& str]) -> GenResult <Self> {
			let weights = input.iter ().enumerate ()
				.map (|(line_idx, line)| Ok (
					line.parse ().map_err (|_err|
						format! ("Invalid input: line {}: {}", line_idx + 1, line)) ?))
				.collect::<GenResult <Vec <_>>> () ?;
			Ok (Self { weights })
		}
	}

	#[ cfg (test) ]
	mod tests {

		use super::*;

		#[ test ]
		fn input_parse () {
			assert_err! ("Invalid input: line 2: 456X", Input::parse (& [ "123", "456X" ]));
		}

	}

}

#[ cfg (test) ]
mod examples {

	use super::*;

	const EXAMPLE: & [& str] = & [ "1", "2", "3", "4", "5", "7", "8", "9", "10", "11" ];

	#[ test ]
	fn part_one () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("99", puzzle.part_one (EXAMPLE));
	}

	#[ test ]
	fn part_two () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("44", puzzle.part_two (EXAMPLE));
	}

}
