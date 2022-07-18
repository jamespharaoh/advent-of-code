//! Advent of Code 2015: Day 8: Matchsticks
//!
//! [https://adventofcode.com/2015/day/8](https://adventofcode.com/2015/day/8)

use aoc_common::*;

puzzle_info! {
	name = "Matchsticks";
	year = 2015;
	day = 8;
	part_one = |input| logic::part_one (input);
	part_two = |input| logic::part_two (input);
}

mod logic {

	use super::*;

	pub fn part_one (input: & [& str]) -> GenResult <usize> {
		let strings = model::parse_input (input) ?;
		Ok (
			strings.iter ()
				.map (|(code, value)|
					code.chars ().count () - value.chars ().count ())
				.sum ()
		)
	}

	pub fn part_two (input: & [& str]) -> GenResult <usize> {
		let strings = model::parse_input (input) ?;
		Ok (
			strings.iter ()
				.map (|(code, value)| (encode (code), code))
				.map (|(code, value)|
					code.chars ().count () - value.chars ().count ())
				.sum ()
		)
	}

	pub fn encode (input: & str) -> String {
		iter::once ('"')
			.chain (input.chars ().flat_map (|ch|
				match ch {
					'\\' => array_vec! [ '\\', '\\' ],
					'"' => array_vec! [ '\\', '"' ],
					ch => array_vec! [ ch ],
				} as ArrayVec <char, 2>))
			.chain (iter::once ('"'))
			.collect ()
	}

}

mod model {

	use super::*;

	pub type Input = Vec <(String, String)>;

	pub fn parse_input (input: & [& str]) -> GenResult <Input> {
		use parser::*;
		input.iter ().enumerate ().map (|(line_idx, line)|
			Parser::wrap (line, |parser| {
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
				Ok ((line.to_string (), result))
			}).map_parse_err (|col_idx|
				format! ("Invalid input: line {}: col {}: {}", line_idx + 1, col_idx + 1, line)
			)
		).collect ()
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
	fn part_one () -> GenResult <()> {
		assert_eq! (12, logic::part_one (EXAMPLE) ?);
		Ok (())
	}

	#[ test ]
	fn part_two () -> GenResult <()> {
		assert_eq! (19, logic::part_two (EXAMPLE) ?);
		Ok (())
	}

}

