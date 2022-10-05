#![ allow (clippy::missing_inline_in_public_items) ]

use super::*;

use input::Input;
use model::Coord;
use model::Pos;

pub fn part_one (input: & Input) -> GenResult <Coord> {
	let results = find_solutions (input) ?;
	Ok (
		results.into_iter ()
			.map (|(_, height)| height)
			.max ()
			.ok_or ("No solution found") ?
	)
}

pub fn part_two (input: & Input) -> GenResult <u32> {
	let results = find_solutions (input) ?;
	Ok (results.len ().pan_u32 ())
}

fn find_solutions (input: & Input) -> GenResult <Vec <(Pos, Coord)>> {
	let mut results: Vec <(Pos, Coord)> = Vec::new ();
	for x_velocity in 1 ..= input.target_x_end {
		for y_velocity in input.target_y_start ..= -2 * input.target_y_end {
			let velocity = Pos { x: x_velocity, y: y_velocity };
			if let Some (max_height) = simulate (input, velocity) ? {
				results.push ((velocity, max_height));
			}
		}
	}
	Ok (results)
}

fn simulate (input: & Input, velocity: Pos) -> GenResult <Option <Coord>> {
	let mut position = Pos { x: 0, y: 0 };
	let mut velocity = velocity;
	let mut max_height = 0;
	while ! ((velocity.x == 0 && position.x < input.target_x_start)
			|| (input.target_x_end < position.x)
			|| (velocity.y <= 0 && position.y < input.target_y_start)) {
		if (input.target_x_start ..= input.target_x_end).contains (& position.x)
				&& (input.target_y_start ..= input.target_y_end).contains (& position.y) {
			return Ok (Some (max_height));
		}
		chk! (position.x += velocity.x) ?;
		if velocity.x > 0 { chk! (velocity.x -= 1) ?; }
		chk! (position.y += velocity.y) ?;
		chk! (velocity.y -= 1) ?;
		if position.y > max_height { max_height = position.y; }
	}
	Ok (None)
}
