//! Advent of Code 2016: Day 20: Firewall Rules
//!
//! [https://adventofcode.com/2016/day/20](https://adventofcode.com/2016/day/20)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;

puzzle_info! {
	name = "Firewall Rules";
	year = 2016;
	day = 20;
	parse = |input| model::Input::parse (input);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}

pub mod logic {

	use super::*;
	use model::Input;

	pub fn part_one (input: & Input) -> GenResult <u32> {
		let rules = input.rules.iter ().cloned ().sorted ().collect::<Vec <_>> ();
		let mut first = 0;
		for rule in rules {
			if first < rule.start { continue }
			if rule.end == u32::MAX { return Err ("No solution found".into ()) }
			first = cmp::max (first, rule.end + 1);
		}
		Ok (first)
	}

	pub fn part_two (input: & Input) -> GenResult <u32> {
		let rules = input.rules.iter ().cloned ().sorted ().collect::<Vec <_>> ();
		let mut last = 0;
		let mut valid = 0;
		for rule in rules {
			if last < rule.start {
				valid += rule.start - last - 1;
			}
			last = cmp::max (last, rule.end);
		}
		Ok (valid + (u32::MAX - last))
	}

}

pub mod model {

	use super::*;

	#[ derive (Clone, Debug, Eq, PartialEq) ]
	pub struct Input {
		pub rules: Vec <Rule>,
	}

	#[ derive (Clone, Debug, Eq, Ord, PartialEq, PartialOrd) ]
	pub struct Rule {
		pub start: u32,
		pub end: u32,
	}

	impl Input {

		pub fn parse (input: & [& str]) -> GenResult <Self> {
			let rules: Vec <_> = input.iter ().enumerate ()
				.map (|(line_idx, line)| {
					#[ allow (clippy::redundant_closure_for_method_calls) ]
					Parser::wrap (line, |parser| parser.item ())
						.map_parse_err (|col_idx| 
							format! ("Invalid input: line {}: col {}: {}",
								line_idx + 1, col_idx + 1, input [0]))
				})
				.collect::<GenResult <_>> () ?;
			if rules.is_empty () { return Err ("Must have at least one rule".into ()) }
			Ok (Self { rules })
		}

	}

	impl <'inp> FromParser <'inp> for Rule {
		fn from_parser (parser: & mut Parser <'inp>) -> ParseResult <Self> {
			let start = parser.int () ?;
			let end = parser.expect ("-") ?.int () ?;
			parser.end () ?;
			if end < start { return Err ("Rule start must be less or equal to end".into ()) }
			Ok (Self { start, end })
		}
	}

}

#[ cfg (test) ]
mod examples {

	use super::*;

	#[ test ]
	fn part_one () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("3", puzzle.part_one (& [ "5-8", "0-2", "4-7"]));
		assert_eq_ok! ("4294967295", puzzle.part_one (& [ "0-4294967294" ]));
		assert_err! ("No solution found", puzzle.part_one (& [ "0-4294967295" ]));
	}

	#[ test ]
	fn part_two () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("4294967288", puzzle.part_two (& [ "5-8", "0-2", "4-7"]));
		assert_eq_ok! ("1", puzzle.part_two (& [ "0-4294967294" ]));
		assert_eq_ok! ("0", puzzle.part_two (& [ "0-4294967295" ]));
	}

}
