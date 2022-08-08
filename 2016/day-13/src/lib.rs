//! Advent of Code 2016: Day 13: A Maze of Twisty Little Cubicles
//!
//! [https://adventofcode.com/2016/day/13](https://adventofcode.com/2016/day/13)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;

puzzle_info! {
	name = "A Maze of Twisty Little Cubicles";
	year = 2016;
	day = 13;
	parse = |input| model::Input::parse (input);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}

pub mod logic {

	use super::*;
	use model::Input;
	use model::Pos;

	pub fn part_one (input: & Input) -> GenResult <u32> {
		let mut todo = VecDeque::new ();
		todo.push_back ((input.start, 0_u32));
		let mut seen = HashSet::new ();
		seen.insert (input.start);
		while let Some ((pos, dist)) = todo.pop_front () {
			if dist > input.max_dist { break }
			if pos == input.end { return Ok (dist) }
			for adj_pos in pos.adjacent_4 () {
				if ! seen.insert (adj_pos) { continue }
				let Pos { x, y } = adj_pos;
				if (x*x + 3*x + 2*x*y + y + y*y + input.seed).count_ones () & 1 == 1 { continue }
				todo.push_back ((adj_pos, dist + 1));
			}
		}
		Err ("No solution found".into ())
	}

	pub fn part_two (input: & Input) -> GenResult <u32> {
		let mut todo = VecDeque::new ();
		todo.push_back ((input.start, 0_u32));
		let mut seen = HashSet::new ();
		seen.insert (input.start);
		let mut posns = 0;
		while let Some ((pos, dist)) = todo.pop_front () {
			if dist > input.count_dist { break }
			posns += 1;
			for adj_pos in pos.adjacent_4 () {
				if ! seen.insert (adj_pos) { continue }
				let Pos { x, y } = adj_pos;
				if (x*x + 3*x + 2*x*y + y + y*y + input.seed).count_ones () & 1 == 1 { continue }
				todo.push_back ((adj_pos, dist + 1));
			}
		}
		Ok (posns)
	}

}

pub mod model {

	use super::*;
	use parser::*;

	pub type Coord = u32;
	pub type Pos = pos::PosXY <Coord>;

	#[ derive (Clone, Debug, Default, Eq, PartialEq) ]
	pub struct Input {
		pub seed: u32,
		pub start: Pos,
		pub end: Pos,
		pub max_dist: u32,
		pub count_dist: u32,
	}

	impl Input {
		pub fn parse (mut input: & [& str]) -> GenResult <Self> {
			let start_x = parser::input_param (& mut input, "START_X=", 1_u32) ?;
			let start_y = parser::input_param (& mut input, "START_Y=", 1_u32) ?;
			let start = Pos { x: start_x, y: start_y };
			let end_x = parser::input_param (& mut input, "END_X=", 31_u32) ?;
			let end_y = parser::input_param (& mut input, "END_Y=", 39_u32) ?;
			let end = Pos { x: end_x, y: end_y };
			let max_dist = parser::input_param (& mut input, "MAX_DIST=", 100_u32) ?;
			let count_dist = parser::input_param (& mut input, "COUNT_DIST=", 50_u32) ?;
			if input.len () != 1 { return Err ("Input must be one line only".into ()) }
			let seed =
				Parser::wrap (input [0],
					|parser| {
						let seed = parser.int () ?;
						parser.end () ?;
						Ok (seed)
					}).map_parse_err (|col_idx|
						format! ("Invalid input: {}: {}", col_idx + 1, input [0])) ?;
			Ok (Self { seed, start, end, max_dist, count_dist })
		}
	}

}

#[ cfg (test) ]
mod examples {

	use super::*;

	const EXAMPLE: & [& str] = & [ "END_X=7", "END_Y=4", "MAX_DIST=20", "COUNT_DIST=10", "10" ];

	#[ test ]
	fn part_one () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("11", puzzle.part_one (EXAMPLE));
	}

	#[ test ]
	fn part_two () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("18", puzzle.part_two (EXAMPLE));
	}

}
