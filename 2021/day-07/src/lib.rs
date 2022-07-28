//! Advent of Code 2021: Day 7: The Treachery of Whales
//!
//! [https://adventofcode.com/2021/day/7](https://adventofcode.com/2021/day/7)

use aoc_common::*;

puzzle_info! {
	name = "The Treachery of Whales";
	year = 2021;
	day = 7;
	part_one = |lines| logic::part_one (lines [0]);
	part_two = |lines| logic::part_two (lines [0]);
}

mod logic {

	use super::*;
	use nums::IntConv;

	pub fn part_one (input: & str) -> GenResult <u64> {
		let start_posns = model::parse_input (input) ?;
		calc_result (& start_posns, (0, 0), |(fuel, crabs): & mut (u64, u64), pos| {
			* fuel += * crabs;
			* crabs += start_posns.iter ().cloned ().filter (
				|start_pos| * start_pos == pos,
			).count ().as_u64 ();
			Some (* fuel)
		})
	}

	pub fn part_two (input: & str) -> GenResult <u64> {
		let start_posns = model::parse_input (input) ?;
		calc_result (& start_posns, (0, 0, 0), |(fuel, incr, crabs): & mut (u64, u64, u64), pos| {
			* incr += * crabs;
			* fuel += * incr;
			* crabs += start_posns.iter ().cloned ().filter (
				|start_pos| * start_pos == pos,
			).count ().as_u64 ();
			Some (* fuel)
		})
	}

	pub fn calc_result <
		State: Copy,
		ScanFn: Fn (& mut State, u64) -> Option <u64> + Clone,
	> (
		start_posns: & [u64],
		initial_state: State,
		scan_fn: ScanFn,
	) -> GenResult <u64> {
		let max = start_posns.iter ().cloned ().max ().unwrap ();
		let lower: Vec <u64> = (0 ..= max).scan (initial_state, scan_fn.clone ()).collect ();
		let higher: Vec <u64> = (0 ..= max).rev ().scan (initial_state, scan_fn).collect ();
		let moves: Vec <u64> = Iterator::zip (
			lower.into_iter (),
			higher.into_iter ().rev (),
		).map (
			|(lower, higher)| lower + higher,
		).collect ();
		Ok (moves.into_iter ().min ().unwrap ())
	}

}

mod model {

	use super::*;

	pub fn parse_input (input: & str) -> GenResult <Vec <u64>> {
		Ok (input.split (',').map (|pos_str| pos_str.parse ()).collect::<Result <_, _>> ()
			.map_err (|_| format! ("Invalid input")) ?)
	}

}

#[ cfg (test) ]
mod examples {

	use super::*;

	const EXAMPLE: & str = "16,1,2,0,4,2,7,1,2,14";

	#[ test ]
	fn part_one () -> GenResult <()> {
		assert_eq! (37, logic::part_one (EXAMPLE) ?);
		Ok (())
	}

	#[ test ]
	fn part_two () -> GenResult <()> {
		assert_eq! (168, logic::part_two (EXAMPLE) ?);
		Ok (())
	}

}
