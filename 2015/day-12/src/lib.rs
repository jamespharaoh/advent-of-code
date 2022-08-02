//! Advent of Code 2015: Day 12: JSAbacusFramework.io
//!
//! [https://adventofcode.com/2015/day/12](https://adventofcode.com/2015/day/12)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;

puzzle_info! {
	name = "JSAbacusFramework.io";
	year = 2015;
	day = 12;
	parse = |input| model::Json::parse (input [0]);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}

pub mod logic {

	use super::*;
	use model::Json;
	use nums::IntConv;

	pub fn part_one (input: & Json) -> GenResult <i32> {
		let sum = calc_sum (input);
		Ok (sum)
	}

	pub fn part_two (input: & Json) -> GenResult <i32> {
		let sum = calc_sum_no_red (input);
		Ok (sum)
	}

	fn calc_sum (value: & Json) -> i32 {
		match * value {
			Json::Array (ref items) => items.iter ().map (calc_sum).sum (),
			Json::Object (ref items) => items.iter ().map (|& (_, ref item)| calc_sum (item)).sum (),
			Json::Number (ref value) => value.as_i32 (),
			Json::String (_) => 0_i32,
		}
	}

	fn calc_sum_no_red (value: & Json) -> i32 {
		match * value {
			Json::Array (ref items) => items.iter ().map (calc_sum_no_red).sum (),
			Json::Object (ref items) => {
				if ! items.iter ().any (|& (_, ref value)|
					if let Json::String (ref value) = * value {
						value == "red"
					} else { false }
				) {
					items.iter ().map (|& (_, ref item)| calc_sum_no_red (item)).sum ()
				} else { 0_i32 }
			},
			Json::Number (ref value) => value.as_i32 (),
			Json::String (_) => 0_i32,
		}
	}

}

pub mod model {

	use super::*;

	#[ derive (Debug) ]
	pub enum Json {
		Array (Vec <Json>),
		Object (Vec <(String, Json)>),
		Number (i32),
		String (String),
	}

	impl Json {
		pub fn parse (input: & str) -> GenResult <Self> {
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
						Some ('\\') => Err (parser.err ()) ?,
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
	fn part_one () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("6", puzzle.part_one (& ["[1,2,3]"]));
		assert_eq_ok! ("6", puzzle.part_one (& ["{\"a\":2,\"b\":4}"]));
		assert_eq_ok! ("3", puzzle.part_one (& ["[[[3]]]"]));
		assert_eq_ok! ("3", puzzle.part_one (& ["{\"a\":{\"b\":4},\"c\":-1}"]));
		assert_eq_ok! ("0", puzzle.part_one (& ["{\"a\":[-1,1]}"]));
		assert_eq_ok! ("0", puzzle.part_one (& ["[-1,{\"a\":1}]"]));
		assert_eq_ok! ("0", puzzle.part_one (& ["[]"]));
		assert_eq_ok! ("0", puzzle.part_one (& ["{}"]));
	}

	#[ test ]
	fn part_two () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("6", puzzle.part_two (& ["[1,2,3]"]));
		assert_eq_ok! ("4", puzzle.part_two (& ["[1,{\"c\":\"red\",\"b\":2},3]"]));
		assert_eq_ok! ("0", puzzle.part_two (& ["{\"d\":\"red\",\"e\":[1,2,3,4],\"f\":5}"]));
		assert_eq_ok! ("6", puzzle.part_two (& ["[1,\"red\",5]"]));
	}

}
