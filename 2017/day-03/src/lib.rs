//! Advent of Code 2017: Day 3: Spiral Memory
//!
//! [https://adventofcode.com/2017/day/3](https://adventofcode.com/2017/day/3)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;
use aoc_pos::GenPosCore as _;

puzzle_info! {
	name = "Spiral Memory";
	year = 2017;
	day = 3;
	parse = |input| model::Input::parse (input);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}

pub mod logic {

	use super::*;
	use model::Input;

	pub type Coord = i16;
	pub type Pos = aoc_pos::PosRowCol <Coord>;
	pub type Dir = aoc_pos::Dir2d;
	pub type Turn = aoc_pos::Turn2d;

	pub fn part_one (input: & Input) -> GenResult <u32> {
		sanity_check (input) ?;
		let (pos, _) = iter_posns ().zip (1 .. ).find (|& (_, val)| val == input.target).unwrap ();
		Ok ((pos.row.unsigned_abs () + pos.col.unsigned_abs ()).as_u32 ())
	}

	pub fn part_two (input: & Input) -> GenResult <u32> {
		sanity_check (input) ?;
		let mut filled: HashMap <Pos, u32> = default ();
		filled.insert (Pos::ZERO, 1);
		for pos in iter_posns ().skip (1) {
			let next = pos.adjacent_8 ().iter ()
				.filter_map (|adj_pos| filled.get (adj_pos))
				.sum ();
			if input.target < next { return Ok (next) }
			filled.insert (pos, next);
		}
		Err ("No solution found".into ())
	}

	fn sanity_check (input: & Input) -> GenResult <()> {
		if input.target < 1 { return Err ("Target must be at least one".into ()) }
		if input.target > 1_000_000 { return Err ("Target must be at most one million".into ()) }
		Ok (())
	}

	fn iter_posns () -> impl Iterator <Item = Pos> {
		let mut dir = Dir::Down;
		let mut pos = Pos::ZERO;
		let mut rem = 0_u32;
		let mut stride = 0_u32;
		iter::from_fn (move || {
			let next_pos = pos;
			if rem == 0 {
				dir = dir + Turn::Left;
				if matches! (dir, Dir::Right | Dir::Left) { stride += 1; }
				rem = stride;
			}
			rem -= 1;
			pos = (pos + (dir, 1)).ok () ?;
			Some (next_pos)
		})
	}

}

pub mod model {

	use super::*;

	pub type Value = u16;

	#[ derive (Clone, Debug, Eq, PartialEq) ]
	pub struct Input {
		pub target: u32,
	}

	impl Input {
		pub fn parse (input: & [& str]) -> GenResult <Self> {
			if input.len () != 1 { return Err ("Must have exactly one line".into ()) }
			let target = input [0].parse () ?;
			Ok (Self { target })
		}
	}

}

#[ cfg (test) ]
mod examples {

	use super::*;

	#[ test ]
	fn part_one () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("0", puzzle.part_one (& [ "1" ]));
		assert_eq_ok! ("3", puzzle.part_one (& [ "12" ]));
		assert_eq_ok! ("2", puzzle.part_one (& [ "23" ]));
		assert_eq_ok! ("31", puzzle.part_one (& [ "1024" ]));
	}

	#[ test ]
	fn part_two () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("2", puzzle.part_two (& [ "1" ]));
		assert_eq_ok! ("23", puzzle.part_two (& [ "12" ]));
		assert_eq_ok! ("25", puzzle.part_two (& [ "23" ]));
		assert_eq_ok! ("1968", puzzle.part_two (& [ "1024" ]));
	}

}
