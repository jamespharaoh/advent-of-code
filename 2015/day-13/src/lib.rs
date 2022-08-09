//! Advent of Code 2015: Day 13: Knights of the Dinner Table
//!
//! [https://adventofcode.com/2015/day/13](https://adventofcode.com/2015/day/13)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;

puzzle_info! {
	name = "Knights of the Dinner Table";
	year = 2015;
	day = 13;
	parse = |input| model::parse_input (input);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}

pub mod logic {

	use super::*;
	use model::Input;

	pub fn part_one (input: & Input) -> GenResult <i32> {
		let best = calc_best (input) ?;
		Ok (best)
	}

	pub fn part_two (input: & Input) -> GenResult <i32> {
		let my_scores =
			input.iter ()
				.map (|& (ref name, _, _)| name)
				.sorted ()
				.dedup ()
				.flat_map (|name| [
					(Rc::from ("Myself"), Rc::clone (name), 0_i32),
					(Rc::clone (name), Rc::from ("Myself"), 0_i32),
				])
				.collect::<Vec <_>> ();
		let input: Vec <_> =
			input.iter ().cloned ()
				.chain (my_scores)
				.collect ();
		let best = calc_best (& input) ?;
		Ok (best)
	}

	pub fn calc_best (input: & Input) -> GenResult <i32> {
		let names: Vec <_> =
			input.iter ()
				.map (|& (ref name, _, _)| name)
				.sorted ()
				.dedup ()
				.map (Rc::clone)
				.collect ();
		let scores =
			input.iter ().cloned ()
				.chain (names.iter ()
					.map (|name| (Rc::clone (name), Rc::clone (name), 0_i32)))
				.sorted ()
				.map (|(_, _, score)| score)
				.collect::<Vec <_>> ();
		if scores.len () != names.len () * names.len () {
			Err ("Missing scores for some combinations of names") ?;
		}
		Ok ((0 .. names.len ())
			.permutations (names.len ())
			.map (|plan| plan.iter ()
				.circular_tuple_windows::<(_, _)> ()
				.fold (Ok (0_i32), |sum, indexes| sum.and_then (|sum| {
					let score_0 = scores [indexes.0 * names.len () + indexes.1];
					let score_1 = scores [indexes.1 * names.len () + indexes.0];
					i32::add_3 (sum, score_0, score_1)
				})))
			.fold (Ok (i32::MIN), |max, item| max
				.and_then (|max| item
					.map (|item| cmp::max (max, item)))) ?)
	}

}

pub mod model {

	use super::*;

	pub type Item = (Rc <str>, Rc <str>, i32);
	pub type Input = Vec <Item>;

	pub fn parse_input (input: & [& str]) -> GenResult <Input> {
		use parser::*;
		input.iter ().enumerate ().map (|(line_idx, line)|
			Parser::wrap (line, |parser| {
				parser
					.set_word_pred (char::is_alphabetic)
					.set_ignore_whitespace (true);
				let name_0 = parser.word () ?;
				let verb = parser.expect_word ("would") ?.word () ?;
				let amount: i32 = parser.int () ?;
				let amount = match verb {
					"gain" => amount,
					"lose" => - amount,
					_ => Err (parser.err ()) ?,
				};
				let name_1 = parser.expect ("happiness units by sitting next to ") ?.word () ?;
				parser.expect (".") ?.end () ?;
				Ok ((name_0.into (), name_1.into (), amount))
			}).map_parse_err (|char_idx|
				format! ("Invalid input: line {}: col {}: {}", line_idx + 1, char_idx + 1, line)
			)
		).collect ()
	}

}

#[ cfg (test) ]
mod examples {

	use super::*;

	const EXAMPLE: & [& str] = & [
		"Alice would gain 54 happiness units by sitting next to Bob.",
		"Alice would lose 79 happiness units by sitting next to Carol.",
		"Alice would lose 2 happiness units by sitting next to David.",
		"Bob would gain 83 happiness units by sitting next to Alice.",
		"Bob would lose 7 happiness units by sitting next to Carol.",
		"Bob would lose 63 happiness units by sitting next to David.",
		"Carol would lose 62 happiness units by sitting next to Alice.",
		"Carol would gain 60 happiness units by sitting next to Bob.",
		"Carol would gain 55 happiness units by sitting next to David.",
		"David would gain 46 happiness units by sitting next to Alice.",
		"David would lose 7 happiness units by sitting next to Bob.",
		"David would gain 41 happiness units by sitting next to Carol.",
	];

	#[ test ]
	fn part_one () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("330", puzzle.part_one (EXAMPLE));
	}

	#[ test ]
	fn part_two () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("286", puzzle.part_two (EXAMPLE));
	}

}
