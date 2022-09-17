//! Logic for solving the puzzles

use super::*;

use input::Input;
use model::Pos;
use model::Tile;

pub fn part_one (input: & Input) -> GenResult <u32> {
	Ok (calc_result (input, Pos::new (1, 3)))
}

pub fn part_two (input: & Input) -> GenResult <u64> {
	const TRAJS: & [Pos] = & [
		Pos::new (1, 1),
		Pos::new (1, 3),
		Pos::new (1, 5),
		Pos::new (1, 7),
		Pos::new (2, 1),
	];
	Ok (
		TRAJS.iter ().copied ()
			.map (|traj| calc_result (input, traj).as_u64 ())
			.product ()
	)
}

fn calc_result (input: & Input, traj: Pos) -> u32 {
	itertools::iterate (
			Pos::ZERO,
			|& pos| {
				let mut pos = pos + traj;
				while input.grid.size ().x <= pos.x { pos.x -= input.grid.size ().x; }
				pos
			})
		.take_while (|pos| pos.y < input.grid.size ().y)
		.map (|pos| input.grid.get (pos).unwrap ())
		.filter (|& tile| tile == Tile::Tree)
		.count ()
		.as_u32 ()
}
