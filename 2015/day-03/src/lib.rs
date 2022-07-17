//! Advent of Code 2015: Day 3: Perfectly Spherical Houses in a Vacuum
//!
//! [https://adventofcode.com/2015/day/3](https://adventofcode.com/2015/day/3)

use aoc_common::*;

puzzle_info! {
	name = "Perfectly Spherical Houses in a Vacuum";
	year = 2015;
	day = 3;
	part_one = |input| logic::part_one (input [0]);
	part_two = |input| logic::part_two (input [0]);
}

mod logic {

	use super::*;
	use model::Pos;

	pub fn part_one (input: & str) -> GenResult <u32> {
		let input = model::parse_input (input) ?;
		let (seen, _) = input.iter_copied ().fold (
			(HashMap::<_, u32>::from_iter ([ (Pos::ZERO, 1) ]), Pos::ZERO),
			|(mut seen, pos), dir| {
				let pos = pos + dir.to_pos ();
				* seen.entry (pos).or_insert (0) += 1;
				(seen, pos)
			},
		);
		Ok (seen.len ().try_into () ?)
	}

	pub fn part_two (input: & str) -> GenResult <u32> {
		let input = model::parse_input (input) ?;
		let (seen, _, _) = input.iter_copied ().fold (
			(HashMap::<_, u32>::from_iter ([ (Pos::ZERO, 1) ]), Pos::ZERO, Pos::ZERO),
			|(mut seen, pos_0, pos_1), dir| {
				let pos_0 = pos_0 + dir.to_pos ();
				* seen.entry (pos_0).or_insert (0) += 1;
				(seen, pos_1, pos_0)
			},
		);
		Ok (seen.len ().try_into () ?)
	}

}

mod model {

	use super::*;

	pub type Input = Vec <Dir>;
	pub type Pos = pos::PosGeo <i16>;

	#[ derive (Clone, Copy, Debug, Eq, Hash, PartialEq) ]
	pub enum Dir {
		North, South, East, West,
	}

	impl Dir {
		pub fn to_pos (self) -> Pos {
			match self {
				Dir::North => Pos::ZERO.north (1),
				Dir::South => Pos::ZERO.south (1),
				Dir::East => Pos::ZERO.east (1),
				Dir::West => Pos::ZERO.west (1),
			}
		}
	}

	pub fn parse_input (input: & str) -> GenResult <Input> {
		input.chars ().enumerate ().map (|(ch_idx, ch)| Ok (match ch {
			'^' => Dir::North,
			'v' => Dir::South,
			'>' => Dir::East,
			'<' => Dir::West,
			_ => Err (format! ("Invalid input: char {}: {}", ch_idx + 1, ch)) ?,
		})).collect::<GenResult <_>> ()
	}

}

#[ cfg (test) ]
mod examples {

	use super::*;

	#[ test ]
	fn part_one () -> GenResult <()> {
		assert_eq! (2, logic::part_one (">") ?);
		assert_eq! (4, logic::part_one ("^>v<") ?);
		assert_eq! (2, logic::part_one ("^v^v^v^v^v") ?);
		Ok (())
	}

	#[ test ]
	fn part_two () -> GenResult <()> {
		assert_eq! (3, logic::part_two ("^v") ?);
		assert_eq! (3, logic::part_two ("^>v<") ?);
		assert_eq! (11, logic::part_two ("^v^v^v^v^v") ?);
		Ok (())
	}

}
