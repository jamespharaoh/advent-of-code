//! Advent of Code 2021: Day 9: Smoke Basin
//!
//! [https://adventofcode.com/2021/day/9](https://adventofcode.com/2021/day/9)

use aoc_common::*;

puzzle_info! {
	name = "Smoke Basin";
	year = 2021;
	day = 9;
	part_one = |lines| logic::part_one (lines);
	part_two = |lines| logic::part_two (lines);
}

mod logic {

	use super::*;
	use model::Pos;

	pub fn part_one (lines: & [& str]) -> GenResult <u64> {
		let floor = model::parse_input (lines) ?;
		let mut sum: u64 = 0;
		'OUTER: for (& pos, & height) in floor.iter () {
			for next_pos in [
				Pos { row: pos.row, col: pos.col - 1 },
				Pos { row: pos.row, col: pos.col + 1 },
				Pos { row: pos.row - 1, col: pos.col },
				Pos { row: pos.row + 1, col: pos.col },
			] {
				if let Some (& next_height) = floor.get (& next_pos) {
					if next_height <= height { continue 'OUTER }
				}
			}
			sum += height as u64 + 1;
		}
		Ok (sum)
	}

	pub fn part_two (lines: & [& str]) -> GenResult <u64> {
		let floor = model::parse_input (lines) ?;
		let mut basin_sizes: Vec <u64> = Vec::new ();
		'OUTER: for (& pos, & height) in floor.iter () {
			for adj_pos in pos.adjacent () {
				if let Some (& adj_height) = floor.get (& adj_pos) {
					if adj_height <= height { continue 'OUTER }
				}
			}
			let mut visited: HashSet <Pos> = vec! [ pos ].into_iter ().collect ();
			let mut todo: Vec <Pos> = vec! [ pos ];
			let mut basin_size: u64 = 1;
			while ! todo.is_empty () {
				let mut next_todo = Vec::new ();
				for pos in todo.into_iter () {
					let height = floor [& pos];
					for adj_pos in pos.adjacent () {
						if visited.contains (& adj_pos) { continue }
						if let Some (& adj_height) = floor.get (& adj_pos) {
							if adj_height == 9 { continue }
							if adj_height <= height { continue }
							next_todo.push (adj_pos);
							visited.insert (adj_pos);
							basin_size += 1;
						}
					}
				}
				todo = next_todo;
			}
			basin_sizes.push (basin_size);
		}
		basin_sizes.sort ();
		Ok (basin_sizes.into_iter ().rev ().take (3).fold (1, |product, value| product * value))
	}

}

mod model {

	use super::*;

	pub fn parse_input (lines: & [& str]) -> GenResult <HashMap <Pos, u8>> {
		let mut floor: HashMap <Pos, u8> = HashMap::new ();
		for (row, line) in lines.iter ().enumerate () {
			let row = row as i32;
			for (col, letter) in line.chars ().enumerate () {
				let col = col as i32;
				floor.insert (Pos { row, col }, letter.to_digit (10).unwrap () as u8);
			}
		}
		Ok (floor)
	}

	#[ derive (Clone, Copy, Debug, Eq, Hash, PartialEq) ]
	pub struct Pos { pub row: i32, pub col: i32 }

	impl Pos {
		pub fn adjacent (& self) -> Vec <Pos> {
			vec! [
				Pos { row: self.row, col: self.col - 1 },
				Pos { row: self.row, col: self.col + 1 },
				Pos { row: self.row - 1, col: self.col },
				Pos { row: self.row + 1, col: self.col },
			]
		}
	}

}

#[ cfg (test) ]
mod examples {

	use super::*;

	const EXAMPLE: & [& str] = & [
		"2199943210",
		"3987894921",
		"9856789892",
		"8767896789",
		"9899965678",
	];

	#[ test ]
	fn part_one () -> GenResult <()> {
		assert_eq! (15, logic::part_one (EXAMPLE) ?);
		Ok (())
	}

	#[ test ]
	fn part_two () -> GenResult <()> {
		assert_eq! (1134, logic::part_two (EXAMPLE) ?);
		Ok (())
	}

}
