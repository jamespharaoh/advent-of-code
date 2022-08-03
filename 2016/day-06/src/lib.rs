//! Advent of Code 2016: Day 6: Signals and Noise
//!
//! [https://adventofcode.com/2016/day/6](https://adventofcode.com/2016/day/6)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;

puzzle_info! {
	name = "Signals and Noise";
	year = 2016;
	day = 6;
	parse = |input| model::Input::parse (input);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}

pub mod logic {

	use super::*;
	use model::Input;

	pub fn part_one (input: & Input) -> GenResult <String> {
		Ok (calc_result (
			& input.lines,
			|col_counts| col_counts.iter ()
				.max_by_key (|& (_, num)| num)
				.map (|(& ch, _)| ch)
				.unwrap ()))
	}

	pub fn part_two (input: & Input) -> GenResult <String> {
		Ok (calc_result (
			& input.lines,
			|col_counts|
				col_counts.iter ()
					.min_by_key (|& (_, num)| num)
					.map (|(& ch, _)| ch)
					.unwrap ()))
	}

	fn calc_result (
		lines: & [String],
		map_fn: fn (HashMap <char, usize>) -> char,
	) -> String {
		lines.iter ()
			.fold (
				iter::repeat (HashMap::new ())
					.take (lines [0].chars ().count ())
					.collect::<Vec <_>> (),
				|mut all_counts, line| {
					all_counts.iter_mut ()
						.zip (line.chars ())
						.for_each (|(col_counts, ch)|
							* col_counts.entry (ch).or_insert (0) += 1_usize);
					all_counts
				})
			.into_iter ()
			.map (map_fn)
			.collect ()
	}

}

pub mod model {

	use super::*;

	pub struct Input {
		pub lines: Vec <String>,
	}

	impl Input {
		pub fn parse (input: & [& str]) -> GenResult <Self> {
			let lines: Vec <String> =
				input.iter ()
					.map (|& line| line.to_owned ())
					.collect ();
			if lines [0].is_empty () { Err ("Invalid input") ?; }
			if lines.iter ().any (|line| line.len () != lines [0].len ()) {
				Err ("All lines must be the same length") ?;
			}
			Ok (Self { lines })
		}
	}

}

#[ cfg (test) ]
mod examples {

	use super::*;

	const EXAMPLE: & [& str] = & [
		"eedadn", "drvtee", "eandsr", "raavrd", "atevrs", "tsrnev", "sdttsa", "rasrtv",
		"nssdts", "ntnada", "svetve", "tesnvt", "vntsnd", "vrdear", "dvrsen", "enarar"
	];

	#[ test ]
	fn part_one () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("easter", puzzle.part_one (EXAMPLE));
	}

	#[ test ]
	fn part_two () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("advent", puzzle.part_two (EXAMPLE));
	}

}
