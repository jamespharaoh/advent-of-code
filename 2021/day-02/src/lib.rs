//! Advent of Code 2021: Day 2: Dive!
//!
//! [https://adventofcode.com/2021/day/2](https://adventofcode.com/2021/day/2)

use aoc_common::*;

puzzle_info! {
	name = "Dive!";
	year = 2021;
	day = 2;
	part_one = |lines| logic::part_one (lines);
	part_two = |lines| logic::part_two (lines);
}

mod logic {

	use super::*;

	pub fn part_one (lines: & [& str]) -> GenResult <i64> {
		let mut distance: i64 = 0;
		let mut depth: i64 = 0;
		for line in lines.iter () {
			if line.trim ().is_empty () { continue }
			let line_parts: Vec <_> = line.split (" ").collect ();
			if line_parts.len () != 2 { panic! () }
			let line_verb = line_parts [0];
			let line_arg: i64 = line_parts [1].parse () ?;
			match line_verb {
				"forward" => distance += line_arg,
				"down" => depth += line_arg,
				"up" => depth -= line_arg,
				_ => panic! (),
			}
		}
		Ok (distance * depth)
	}

	pub fn part_two (lines: & [& str]) -> GenResult <i64> {
		let mut distance: i64 = 0;
		let mut depth: i64 = 0;
		let mut aim: i64 = 0;
		for line in lines.iter () {
			if line.trim ().is_empty () { continue }
			let line_parts: Vec <_> = line.split (" ").collect ();
			if line_parts.len () != 2 { panic! () }
			let line_verb = line_parts [0];
			let line_arg: i64 = line_parts [1].parse () ?;
			match line_verb {
				"forward" => {
					distance += line_arg;
					depth += aim * line_arg;
				},
				"down" => aim += line_arg,
				"up" => aim -= line_arg,
				_ => panic! (),
			}
		}
		Ok (distance * depth)
	}

}

#[ cfg (test) ]
mod examples {

	use super::*;

	const EXAMPLE: & [& str] = & [
		"forward 5",
		"down 5",
		"forward 8",
		"up 3",
		"down 8",
		"forward 2",
	];

	#[ test ]
	fn part_one () -> GenResult <()> {
		assert_eq! (150, logic::part_one (EXAMPLE) ?);
		Ok (())
	}

	#[ test ]
	fn part_two () -> GenResult <()> {
		assert_eq! (900, logic::part_two (EXAMPLE) ?);
		Ok (())
	}

}
