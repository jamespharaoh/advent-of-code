//! Logic for solving the puzzles

use super::*;
use input::Input;
use model::Coord;
use model::Core;
use model::CoreNext;
use model::Game;
use model::GameNext;
use model::Pos;
use model::Tile;

pub fn part_one (input: & Input) -> GenResult <u32> {
	let size = get_size (input) ?;
	let mut game = Game::new (input, false, size) ?;
	game.next () ?;
	Ok (
		game.grid ().values ()
			.filter (|& tile| tile == Tile::Block)
			.count ()
			.as_u32 ()
	)
}

pub fn part_two (input: & Input) -> GenResult <i32> {
	let size = get_size (input) ?;
	let mut game = Game::new (input, true, size) ?;
	for _ in 0 .. input.params.game_max_steps {
		match game.next () ? {
			GameNext::Input => game.input ((game.ball_pos ().x - game.paddle_pos ().x).signum ()),
			GameNext::Halt => return Ok (game.score ()),
		}
	}
	Err ("Max steps exceeded".into ())
}

pub fn get_size (input: & Input) -> GenResult <Pos> {
	let mut core = Core::new (input, false);
	let mut size = Pos::ZERO;
	for _ in 0 .. input.params.size_max_steps {
		match core.next () ? {
			CoreNext::Input => break,
			CoreNext::Output (pos, _) => {
				if pos.x < Coord::ZERO || pos.y < Coord::ZERO {
					return Err ("Coordinates must not be negative".into ());
				}
				if pos.x > 1024_i32 || pos.y > 1024_i32 {
					return Err ("Max size is 1024 in each axis".into ());
				}
				size.x = cmp::max (size.x, pos.x + Coord::ONE);
				size.y = cmp::max (size.y, pos.y + Coord::ONE);
			},
			CoreNext::Halt => break,
		}
	}
	if size.x < Coord::ONE || size.y < Coord::ONE {
		return Err ("Output must not be empty".into ());
	}
	Ok (size)
}
