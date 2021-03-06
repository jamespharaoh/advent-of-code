//! Advent of Code 2015: Day 18: Like a GIF For Your Yard
//!
//! [https://adventofcode.com/2015/day/18](https://adventofcode.com/2015/day/18)

use aoc_common::*;

puzzle_info! {
	name = "Like a GIF For Your Yard";
	year = 2015;
	day = 18;
	parse = |input| model::parse_input (input);
	part_one = |input, steps: u32| logic::part_one (input, steps);
	part_two = |input, steps: u32| logic::part_two (input, steps);
	params = [
		steps: u32 = 100;
	];
}

/// Logic for solving the puzzles.
///
pub mod logic {

	use super::*;
	use model::Input;
	use model::Pos;
	use nums::IntConv;

	pub fn part_one (input: Input, loops: u32) -> GenResult <u32> {
		let num_active = calc_result (input, loops, & HashMap::new ());
		Ok (num_active)
	}

	pub fn part_two (input: Input, loops: u32) -> GenResult <u32> {
		let Pos { y: y0, x: x0 } = input.origin ();
		let Pos { y: y1, x: x1 } = input.peak ();
		let overrides = [
			Pos { y: y0, x: x0 }, Pos { y: y0, x: x1 },
			Pos { y: y1, x: x0 }, Pos { y: y1, x: x1 },
		].into_iter ().map (|pos| (pos, true)).collect ();
		let num_active = calc_result (input, loops, & overrides);
		Ok (num_active)
	}

	fn calc_result (lights: Input, loops: u32, overrides: & HashMap <Pos, bool>) -> u32 {
		let mut lights = lights;

		// make sure overrides start in the correct state

		for (& pos, & val) in overrides.iter () {
			lights.set (pos, val);
		}

		// apply rules specified number of times

		for _ in 0 .. loops {
			let new_data = lights.iter ().map (|(pos, val)| {
				if let Some (& val) = overrides.get (& pos) { val } else {
					let num_adjacent =
						pos.adjacent_8 ().iter ().copied ()
							.filter (|& adj_pos| lights.get (adj_pos).unwrap_or (false))
							.count ();
					matches! ((val, num_adjacent), (true, 2) | (_, 3))
				}
			}).collect ();
			lights = Input::wrap (new_data, lights.raw_origin (), lights.raw_size ());
		}

		// count active lights

		let num_active =
			lights.values ()
				.filter (|& val| val)
				.count ().to_u32 ().unwrap ();

		num_active

	}

}

/// Representation of the puzzle input, etc.
///
pub mod model {

	use super::*;
	use grid::Grid;
	use pos::PosYX;

	pub type Coord = u16;
	pub type Pos = PosYX <Coord>;
	pub type Input = Grid <Vec <bool>, Pos>;

	pub fn parse_input (input: & [& str]) -> GenResult <Input> {
		let grid_size = [ input.len (), input [0].chars ().count () ];
		if grid_size [1] == 0 { Err ("Invalid input") ? }
		if let Some ((line_idx, line_len)) =
			input.iter ().enumerate ()
				.map (|(idx, line)| (idx, line.chars ().count ()))
				.find (|& (_, len)| len != grid_size [1]) {
			Err (format! ("Invalid input: line {}: Expected {} chars, not {}",
				line_idx + 1, grid_size [1], line_len)) ?
		}
		let grid_data = input.iter ().enumerate ().flat_map (|(line_idx, line)|
			line.chars ().enumerate ().map (move |(char_idx, ch)| Ok (match ch {
				'#' => true,
				'.' => false,
				_ => Err (format! ("Invalid input: line {}: col {}: {}", line_idx + 1,
					char_idx + 1, ch)) ?,
			}))
		).collect::<GenResult <Vec <_>>> () ?;
		let grid = Grid::wrap (grid_data, [0, 0], grid_size);
		Ok (grid)
	}

}

#[ cfg (test) ]
mod examples {

	use super::*;

	const EXAMPLE: & [& str] = & [
		".#.#.#", "...##.", "#....#", "..#...", "#.#..#", "####.."
	];

	#[ test ]
	fn part_one () {
		let mut puzzle = puzzle_metadata ();
		puzzle.set_param ("steps", 4.to_string ());
		assert_eq_ok! ("4", puzzle.part_one (EXAMPLE));
	}

	#[ test ]
	fn part_two () {
		let mut puzzle = puzzle_metadata ();
		puzzle.set_param ("steps", 5.to_string ());
		assert_eq_ok! ("17", puzzle.part_two (EXAMPLE));
	}

}
