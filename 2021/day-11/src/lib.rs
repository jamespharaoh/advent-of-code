//! Advent of Code 2021: Day 11: Dumbo Octopus
//!
//! [https://adventofcode.com/2021/day/11](https://adventofcode.com/2021/day/11)

use aoc_common::*;

puzzle_info! {
	name = "Dumbo Octopus";
	year = 2021;
	day = 11;
	part_one = |lines| logic::part_one (lines);
	part_two = |lines| logic::part_two (lines);
}

mod logic {

	use super::*;
	use model::Octopodes;
	use model::Pos;

	pub fn part_one (lines: & [& str]) -> GenResult <u64> {
		let octopodes = model::parse_input (lines) ?;
		Ok (
			step_iter (octopodes)
				.take (100)
				.sum ()
		)
	}

	pub fn part_two (lines: & [& str]) -> GenResult <u64> {
		let octopodes = model::parse_input (lines) ?;
		let num_octopodes = octopodes.len ().pan_u64 ();
		Ok (
			step_iter (octopodes)
				.take_while (|& num_flashes| num_flashes < num_octopodes)
				.count ().pan_u64 () + 1
		)
	}

	fn step_iter (octopodes: Octopodes) -> impl Iterator <Item = u64> {
		iter::repeat (()).scan (octopodes, |octopodes, _| {
			let mut num_flashes: u64 = 0;
			let mut flashed: HashSet <Pos> = HashSet::new ();
			let mut todo: Vec <Pos> = Vec::new ();
			for (& pos, energy) in octopodes.iter_mut () {
				* energy += 1;
				if * energy > 9 {
					* energy = 0;
					flashed.insert (pos);
					todo.push (pos);
					num_flashes += 1;
				}
			}
			while ! todo.is_empty () {
				let todo_temp = todo;
				todo = Vec::new ();
				for pos in todo_temp {
					for adj_pos in pos.adjacent () {
						if let Some (adj_energy) = octopodes.get_mut (& adj_pos) {
							if flashed.contains (& adj_pos) { continue }
							* adj_energy += 1;
							if * adj_energy > 9 {
								* adj_energy = 0;
								flashed.insert (adj_pos);
								todo.push (adj_pos);
								num_flashes += 1;
							}
						}
					}
				}
			}
			Some (num_flashes)
		})
	}

}

mod model {

	use super::*;

	pub type Octopodes = HashMap <Pos, u8>;

	pub fn parse_input (lines: & [& str]) -> GenResult <Octopodes> {
		let mut octopodes: HashMap <Pos, u8> = HashMap::new ();
		for (row, line) in lines.iter ().enumerate () {
			let row = row.pan_i16 ();
			for (col, letter) in line.chars ().enumerate () {
				let col = col.pan_i16 ();
				octopodes.insert (
					Pos { row, col },
					letter.to_digit (10).ok_or ("Invalid input") ?.pan_u8 ());
			}
		}
		Ok (octopodes)
	}

	#[ derive (Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd) ]
	pub struct Pos { pub row: i16, pub col: i16 }

	impl Pos {
		pub fn adjacent (self) -> Vec <Self> {
			vec! [
				Self { row: self.row - 1, col: self.col - 1 },
				Self { row: self.row - 1, col: self.col     },
				Self { row: self.row - 1, col: self.col + 1 },
				Self { row: self.row    , col: self.col - 1 },
				Self { row: self.row    , col: self.col + 1 },
				Self { row: self.row + 1, col: self.col - 1 },
				Self { row: self.row + 1, col: self.col     },
				Self { row: self.row + 1, col: self.col + 1 },
			]
		}
	}

}

#[ cfg (test) ]
mod examples {

	use super::*;

	const EXAMPLE: & [& str] = & [
		"5483143223",
		"2745854711",
		"5264556173",
		"6141336146",
		"6357385478",
		"4167524645",
		"2176841721",
		"6882881134",
		"4846848554",
		"5283751526",
	];

	#[ test ]
	fn part_one () -> GenResult <()> {
		assert_eq! (1656, logic::part_one (EXAMPLE) ?);
		Ok (())
	}

	#[ test ]
	fn part_two () -> GenResult <()> {
		assert_eq! (195, logic::part_two (EXAMPLE) ?);
		Ok (())
	}

}

