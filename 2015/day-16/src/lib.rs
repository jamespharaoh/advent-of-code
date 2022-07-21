//! Advent of Code 2015: Day 16: Aunt Sue
//!
//! [https://adventofcode.com/2015/day/16](https://adventofcode.com/2015/day/16)

use aoc_common::*;

puzzle_info! {
	name = "Aunt Sue";
	year = 2015;
	day = 16;
	part_one = |input| logic::part_one (input);
	part_two = |input| logic::part_two (input);
}

/// Logic for solving the puzzles.
///
pub mod logic {

	use super::*;
	use model::Attr;

	pub fn part_one (input: & [& str]) -> GenResult <u16> {
		let all_sues = model::parse_input (input) ?;
		fn ticker (attr: Attr) -> u8 {
			match attr {
				Attr::Children => 3,
				Attr::Cats => 7,
				Attr::Samoyeds => 2,
				Attr::Pomeranians => 3,
				Attr::Akitas => 0,
				Attr::Vizslas => 0,
				Attr::Goldfish => 5,
				Attr::Trees => 3,
				Attr::Cars => 2,
				Attr::Perfumes => 1,
			}
		}
		let the_sue = all_sues.iter ()
			.filter (|sue| sue.attrs.iter ().copied ().all (|(attr, num)|
				num == ticker (attr)))
			.exactly_one ().map_err (|iter| format! ("Expected exactly one match but found {}",
				iter.count ())) ?;
		Ok (the_sue.number)
	}

	pub fn part_two (input: & [& str]) -> GenResult <u16> {
		let all_sues = model::parse_input (input) ?;
		fn ticker (attr: Attr, num: u8) -> bool {
			match attr {
				Attr::Children => num == 3,
				Attr::Cats => num > 7,
				Attr::Samoyeds => num == 2,
				Attr::Pomeranians => num < 3,
				Attr::Akitas => num == 0,
				Attr::Vizslas => num == 0,
				Attr::Goldfish => num < 5,
				Attr::Trees => num > 3,
				Attr::Cars => num == 2,
				Attr::Perfumes => num == 1,
			}
		}
		let the_sue = all_sues.iter ()
			.filter (|sue| sue.attrs.iter ().copied ()
				.all (|(attr, num)| ticker (attr, num)))
			.exactly_one ().map_err (|iter| format! ("Expected exactly one match but found {}",
				iter.count ())) ?;
		Ok (the_sue.number)
	}

}

/// Representation of the puzzle input, etc.
///
pub mod model {

	use super::*;

	pub type Input = Vec <AuntSue>;

	#[ derive (Clone, Debug, Eq, PartialEq) ]
	pub struct AuntSue {
		pub number: u16,
		pub attrs: ArrayVec <(Attr, u8), 10>,
	}

	#[ derive (Clone, Copy, Debug, Eq, PartialEq) ]
	pub enum Attr {
		Children,
		Cats,
		Samoyeds,
		Pomeranians,
		Akitas,
		Vizslas,
		Goldfish,
		Trees,
		Cars,
		Perfumes,
	}

	pub fn parse_input (input: & [& str]) -> GenResult <Input> {
		use parser::*;
		input.iter ().enumerate ().map (|(line_idx, line)|
			Parser::wrap (line, |parser| {
				parser.set_ignore_whitespace (true)
					.set_word_pred (char::is_alphanumeric);
				let number = parser.expect ("Sue") ?.int () ?;
				parser.expect (":") ?;
				let mut attrs = ArrayVec::new ();
				while parser.peek ().is_some () {
					let attr = match parser.word () ? {
						"children" => Attr::Children,
						"cats" => Attr::Cats,
						"samoyeds" => Attr::Samoyeds,
						"pomeranians" => Attr::Pomeranians,
						"akitas" => Attr::Akitas,
						"vizslas" => Attr::Vizslas,
						"goldfish" => Attr::Goldfish,
						"trees" => Attr::Trees,
						"cars" => Attr::Cars,
						"perfumes" => Attr::Perfumes,
						_ => Err (parser.err ()) ?,
					};
					let num = parser.expect (":") ?.int () ?;
					attrs.push ((attr, num));
					match parser.next () {
						Some (',') => continue,
						None => break,
						Some (_) => Err (parser.err ()) ?,
					}
				}
				parser.end () ?;
				Ok (AuntSue { number, attrs })
			}).map_parse_err (|char_idx|
				format! ("Invalid input: line {}: col {}: {}", line_idx + 1, char_idx + 1, line)
			)
		).collect ()
	}

}
