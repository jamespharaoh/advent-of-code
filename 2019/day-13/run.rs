use super::*;

use std::path::PathBuf;

use input::Input;
use model::Coord;
use model::Game;
use model::GameNext;
use model::Tile;

#[ derive (clap::Parser) ]
pub struct RunArgs {

	#[ clap (from_global, value_parser = PathBuf) ]
	input: PathBuf,

}

#[ allow (clippy::print_stdout) ]
pub fn run (args: RunArgs) -> GenResult <()> {
	let input_string = fs::read_to_string (args.input) ?;
	let input_lines: Vec <& str> = input_string.trim_end ().split ('\n').collect ();
	let input = Input::parse_from_lines (& input_lines) ?;
	let size = logic::get_size (& input) ?;
	let mut game = Game::new (& input, true, size) ?;
	for _ in Coord::ZERO .. size.y + Coord::ONE { println! (); }
	loop {
		let prev_grid = game.grid ().clone ();
		if matches! (game.next () ?, GameNext::Halt) { break }
		print! ("\x1b[{num}A", num = size.y + Coord::ONE);
		println! ("{score}\x1b[K", score = game.score ());
		let mut skipped = 0_u32;
		for ((pos, tile), prev_tile) in game.grid ().iter ().zip (prev_grid.values ()) {
			if pos.y > Coord::ZERO && pos.x == Coord::ZERO { println! (); skipped = 0; }
			if tile == prev_tile { skipped += 2; continue }
			if skipped > 0 {
				print! ("\x1b[{right}C{tile}", right = skipped, tile = display_tile (tile));
			} else {
				print! ("{tile}", tile = display_tile (tile));
			}
			skipped = 0;
		}
		println! ();
		thread::sleep (time::Duration::from_micros (10_000));
		game.input ((game.ball_pos ().x - game.paddle_pos ().x).signum ());
	}
	Ok (())
}

const fn display_tile (tile: Tile) -> & 'static str {
	match tile {
		Tile::Empty => "  ",
		Tile::Wall => "██",
		Tile::Block => "📦",
		Tile::Paddle => "══",
		Tile::Ball => "⚽",
	}
}
