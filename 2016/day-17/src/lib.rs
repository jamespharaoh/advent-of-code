//! Advent of Code 2016: Day 17: Two Steps Forward
//!
//! [https://adventofcode.com/2016/day/17](https://adventofcode.com/2016/day/17)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;
use aoc_md5 as md5;
use aoc_pos as pos;

puzzle_info! {
	name = "Two Steps Forward";
	year = 2016;
	day = 17;
	parse = |input| model::Input::parse (input);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}

pub mod logic {

	use super::*;
	use model::Dir;
	use model::Input;
	use model::Pos;

	pub fn part_one (input: & Input) -> GenResult <String> {
		Ok (
			RoutesIter::new (input)
				.next ()
				.ok_or ("No solution found") ?
		)
	}

	pub fn part_two (input: & Input) -> GenResult <usize> {
		Ok (
			RoutesIter::new (input)
				.last ()
				.ok_or ("No solution found") ?
				.chars ()
				.count ()
		)
	}

	struct RoutesIter {
		todo: VecDeque <(Pos, String)>,
		end: Pos,
		size: Pos,
		passcode: String,
	}

	impl RoutesIter {
		fn new (input: & Input) -> Self {
			let mut todo = VecDeque::new ();
			todo.push_back ((input.start, "".to_owned ()));
			let end = input.end;
			let size = input.size;
			let passcode = input.passcode.clone ();
			Self { todo, end, size, passcode }
		}
	}

	impl Iterator for RoutesIter {

		type Item = String;

		fn next (& mut self) -> Option <String> {
			while let Some ((pos, route)) = self.todo.pop_front () {
				if pos == self.end { return Some (route) }
				let hash = md5::md5_hash (format! ("{}{}", self.passcode, route).as_bytes ());
				let hash_hex = hash.as_hex_bytes ();
				for (dir_idx, dir_tag, dir) in [
					(0, "U", Dir::Up), (1, "D", Dir::Down),
					(2, "L", Dir::Left), (3, "R", Dir::Right),
				] {
					let adj_pos = ok_or! (pos.try_add ((dir, 1)), continue);
					if adj_pos.y >= self.size.y || adj_pos.x >= self.size.x { continue }
					if hash_hex [dir_idx] > b'a' {
						self.todo.push_back ((adj_pos, format! ("{}{}", route, dir_tag)));
					}
				}
			}
			None
		}
	}

}

pub mod model {

	use super::*;

	pub type Dir = pos::Dir2d;
	pub type Pos = pos::PosYX <u8>;

	#[ derive (Clone, Debug, Eq, PartialEq) ]
	pub struct Input {
		pub passcode: String,
		pub start: Pos,
		pub end: Pos,
		pub size: Pos,
	}

	impl Input {

		pub fn parse (input: & [& str]) -> GenResult <Self> {
			if input.len () != 1 { return Err ("Input must have exactly one line".into ()) }
			let passcode = input [0].to_owned ();
			let start = Pos { y: 0, x: 0 };
			let end = Pos { y: 3, x: 3 };
			let size = Pos { y: 4, x: 4 };
			Ok (Self { passcode, start, end, size })
		}

	}

}

#[ cfg (test) ]
mod examples {

	use super::*;

	const EXAMPLES: & [& str] = & [
		"ihgpwlah",
		"kglvqrro",
		"ulqzkmiv",
	];

	#[ test ]
	fn part_one () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("DDRRRD", puzzle.part_one (& [ EXAMPLES [0] ]));
		assert_eq_ok! ("DDUDRLRRUDRD", puzzle.part_one (& [ EXAMPLES [1] ]));
		assert_eq_ok! ("DRURDRUDDLLDLUURRDULRLDUUDDDRR", puzzle.part_one (& [ EXAMPLES [2] ]));
	}

	#[ test ]
	fn part_two () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("370", puzzle.part_two (& [ EXAMPLES [0] ]));
		assert_eq_ok! ("492", puzzle.part_two (& [ EXAMPLES [1] ]));
		assert_eq_ok! ("830", puzzle.part_two (& [ EXAMPLES [2] ]));
	}

}
