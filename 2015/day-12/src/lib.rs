//! Advent of Code 2015: Day 12: JSAbacusFramework.io
//!
//! [https://adventofcode.com/2015/day/12](https://adventofcode.com/2015/day/12)

use aoc_common::*;

puzzle_info! {
	name = "JSAbacusFramework.io";
	year = 2015;
	day = 12;
	part_one = |input| logic::part_one (input [0]);
	part_two = |input| logic::part_two (input [0]);
}

mod logic {

	use super::*;
	use model::*;

	pub fn part_one (input: & str) -> GenResult <i32> {
		let input = Json::parse (input) ?;
		let sum = calc_sum (& input);
		Ok (sum)
	}

	pub fn part_two (input: & str) -> GenResult <i32> {
		let input = Json::parse (input) ?;
		let sum = calc_sum_no_red (& input);
		Ok (sum)
	}

	pub fn calc_sum (value: & Json) -> i32 {
		match value {
			Json::Array (items) => items.iter ().map (calc_sum).sum (),
			Json::Object (items) => items.iter ().map (|(_, item)| calc_sum (item)).sum (),
			& Json::Number (value) => value as i32,
			Json::String (_) => 0,
		}
	}

	pub fn calc_sum_no_red (value: & Json) -> i32 {
		match value {
			Json::Array (items) => items.iter ().map (calc_sum_no_red).sum (),
			Json::Object (items) => {
				if ! items.iter ().any (|(_, value)|
					if let Json::String (value) = value {
						value == "red"
					} else { false }
				) {
					items.iter ().map (|(_, item)| calc_sum_no_red (item)).sum ()
				} else { 0 }
			},
			& Json::Number (value) => value as i32,
			Json::String (_) => 0,
		}
	}

}

mod model {

	use super::*;

	#[ derive (Debug) ]
	pub enum Json {
		Array (Vec <Json>),
		Object (Vec <(String, Json)>),
		Number (i32),
		String (String),
	}

	impl Json {
		pub fn parse (input: & str) -> GenResult <Json> {
			use parser::*;
			fn parse_item (parser: & mut Parser) -> ParseResult <Json> {
				parser.any ()
					.of (|parser| {
						let mut items = Vec::new ();
						parser.skip_whitespace ().expect ("[") ?;
						if parser.skip_whitespace ().peek () == Some (']') {
							parser.next ();
							return Ok (Json::Array (items));
						}
						loop {
							items.push (parse_item (parser) ?);
							match parser.skip_whitespace ().next () {
								Some (',') => continue,
								Some (']') => break,
								_ => Err (parser.err ()) ?,
							}
						}
						Ok (Json::Array (items))
					})
					.of (|parser| {
						let mut items = Vec::new ();
						parser.skip_whitespace ().expect ("{") ?;
						if parser.skip_whitespace ().peek () == Some ('}') {
							parser.next ();
							return Ok (Json::Object (items));
						}
						loop {
							let name = parse_string (parser) ?;
							parser.skip_whitespace ().expect (":") ?;
							let value = parse_item (parser) ?;
							items.push ((name, value));
							match parser.skip_whitespace ().next () {
								Some (',') => continue,
								Some ('}') => break,
								_ => Err (parser.err ()) ?,
							}
						}
						Ok (Json::Object (items))
					})
					.of (|parser| {
						let value = parser.skip_whitespace ().int () ?;
						Ok (Json::Number (value))
					})
					.of (|parser| {
						let value = parse_string (parser) ?;
						Ok (Json::String (value))
					})
					.done ()
			}
			fn parse_string (parser: & mut Parser) -> ParseResult <String> {
				parser.skip_whitespace ().expect ("\"") ?;
				let mut value = String::new ();
				loop {
					match parser.next () {
						Some ('\\') => todo! (),
						Some ('"') => break,
						Some (ch) => value.push (ch),
						None => Err (parser.err ()) ?,
					}
				}
				Ok (value)
			}
			Parser::wrap (input, |parser| {
				let item = parse_item (parser) ?;
				parser.skip_whitespace ().end () ?;
				Ok (item)
			}).map_parse_err (|char_idx| format! ("Invalid input: col {}", char_idx + 1))
		}
	}

}

#[ cfg (test) ]
mod examples {

	use super::*;

	#[ test ]
	fn part_one () -> GenResult <()> {
		assert_eq! (6, logic::part_one ("[1,2,3]") ?);
		assert_eq! (6, logic::part_one ("{\"a\":2,\"b\":4}") ?);
		assert_eq! (3, logic::part_one ("[[[3]]]") ?);
		assert_eq! (3, logic::part_one ("{\"a\":{\"b\":4},\"c\":-1}") ?);
		assert_eq! (0, logic::part_one ("{\"a\":[-1,1]}") ?);
		assert_eq! (0, logic::part_one ("[-1,{\"a\":1}]") ?);
		assert_eq! (0, logic::part_one ("[]") ?);
		assert_eq! (0, logic::part_one ("{}") ?);
		Ok (())
	}

	#[ test ]
	fn part_two () -> GenResult <()> {
		assert_eq! (6, logic::part_two ("[1,2,3]") ?);
		assert_eq! (4, logic::part_two ("[1,{\"c\":\"red\",\"b\":2},3]") ?);
		assert_eq! (0, logic::part_two ("{\"d\":\"red\",\"e\":[1,2,3,4],\"f\":5}") ?);
		assert_eq! (6, logic::part_two ("[1,\"red\",5]") ?);
		Ok (())
	}

}
