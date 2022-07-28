//! Advent of Code 2015: Day 8: Matchsticks
//!
//! [https://adventofcode.com/2015/day/8](https://adventofcode.com/2015/day/8)

use aoc_common::*;

puzzle_info! {
	name = "Matchsticks";
	year = 2015;
	day = 8;
	parse = |input| model::parse_input (input);
	part_one = |input| logic::part_one (input);
	part_two = |input| logic::part_two (input);
}

pub mod logic {

	use super::*;
	use model::Input;

	pub fn part_one (input: Input) -> GenResult <usize> {
		Ok (
			input.iter ()
				.map (|(code, value)|
					code.chars ().count () - value.chars ().count ())
				.sum ()
		)
	}

	pub fn part_two (input: Input) -> GenResult <usize> {
		Ok (
			input.iter ()
				.map (|(code, _)| (model::encode (code), code))
				.map (|(code, value)|
					code.chars ().count () - value.chars ().count ())
				.sum ()
		)
	}

}

pub mod model {

	use super::*;
	use parser::*;

	pub type Input = Vec <(String, String)>;

	pub fn encode (input: & str) -> String {
		iter::once ('"')
			.chain (input.chars ().flat_map::<ArrayVec <char, 2>, _> (|ch|
				match ch {
					'\\' => array_vec! [ '\\', '\\' ],
					'"' => array_vec! [ '\\', '"' ],
					ch => array_vec! [ ch ],
				}))
			.chain (iter::once ('"'))
			.collect ()
	}

	pub fn decode (input: & str) -> GenResult <String> {
		Parser::wrap (input, decode_real)
			.map_parse_err (|col_idx| format! ("Invalid input: col {}: {}", col_idx + 1, input))
	}

	fn decode_real (parser: & mut Parser) -> ParseResult <String> {
		parser.expect ("\"") ?;
		let mut result = String::new ();
		loop {
			match parser.expect_next () ? {
				'"' => break,
				'\\' => match parser.expect_next () ? {
					'x' => {
						let digit_high = parser.expect_next () ?.to_digit (16)
							.ok_or_else (|| parser.err ()) ?;
						let digit_low = parser.expect_next () ?.to_digit (16)
							.ok_or_else (|| parser.err ()) ?;
						let ch_code = digit_high << 4 | digit_low;
						let ch = char::from_u32 (ch_code).ok_or_else (|| parser.err ()) ?;
						result.push (ch);
					},
					'\\' => result.push ('\\'),
					'"' => result.push ('"'),
					_ => Err (parser.err ()) ?,
				},
				ch => result.push (ch),
			}
		}
		Ok (result)
	}

	pub fn parse_input (input: & [& str]) -> GenResult <Input> {
		input.iter ().enumerate ()
			.map (|(line_idx, line)|
				Parser::wrap (line, |parser| Ok ((line.to_string (), decode_real (parser) ?)))
					.map_parse_err (|col_idx| format! ("Invalid input: line {}: col {}: {}",
						line_idx + 1, col_idx + 1, line)))
			.collect ()
	}

}

#[ cfg (test) ]
mod examples {

	use super::*;

	const EXAMPLE: & [& str] = & [
		"\"\"",
		"\"abc\"",
		"\"aaa\\\"aaa\"",
		"\"\\x27\"",
	];

	#[ test ]
	fn part_one () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("12", puzzle.part_one (EXAMPLE));
	}

	#[ test ]
	fn part_two () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("19", puzzle.part_two (EXAMPLE));
	}

}
