//! Logic for solving the puzzles

use super::*;
use input::Input;
use model::Dir;
use model::Grid;
use model::Pos;
use model::Tile;
use model::Turn;

pub fn part_one (input: & Input) -> GenResult <String> {
	let mut state = State::new (input);
	for _ in 0_u32 .. 200 {
		state.tick () ?;
		if let Some (& (pos, _, _, _, _)) =
			state.carts.iter ()
				.find (|&& (_, _, _, _, crashed)| crashed) {
			return Ok (format! ("{},{}", pos.x, pos.y));
		}
	}
	Err ("Giving up after 200 iters".into ())
}

pub fn part_two (input: & Input) -> GenResult <String> {
	let mut state = State::new (input);
	for _ in 0_u32 .. 20_000 {
		state.tick () ?;
		let mut carts_iter =
			state.carts.iter ()
				.filter (|&& (_, _, _, _, crashed)| ! crashed);
		if let Some (& (pos, _, _, _, _)) = carts_iter.next () {
			if carts_iter.next ().is_none () {
				return Ok (format! ("{},{}", pos.x, pos.y));
			}
		} else { return Err ("No solution found".into ()) }
	}
	Err ("Giving up after 20k iters".into ())
}

struct State {
	grid: Grid,
	carts: Vec <(Pos, Dir, Turn, Tile, bool)>,
}

impl State {
	fn new (input: & Input) -> Self {
		let grid = input.grid.clone ();
		let carts = grid.iter ()
			.filter_map (|(pos, item)|
				Some (match item {
					Tile::CartUp => (pos, Dir::Up, Turn::Left, Tile::Vert, false),
					Tile::CartDown => (pos, Dir::Down, Turn::Left, Tile::Vert, false),
					Tile::CartLeft => (pos, Dir::Left, Turn::Left, Tile::Horiz, false),
					Tile::CartRight => (pos, Dir::Right, Turn::Left, Tile::Horiz, false),
					Tile::Empty | Tile::Vert | Tile::Horiz | Tile::Crossing | Tile::CornerFwd |
						Tile::CornerBck => return None,
				})
			)
			.collect ();
		Self { grid, carts }
	}
	fn tick (& mut self) -> GenResult <()> {
		self.carts.sort_by_key (|& (pos, _, _, _, _)| pos);
		for idx in 0 .. self.carts.len () {
			let (prev_pos, prev_dir, prev_turn, prev_tile, prev_crashed) = self.carts [idx];
			if prev_crashed { continue }
			let next_pos = prev_pos.try_add ((prev_dir, 1)) ?;
			let next_tile = self.grid.get (next_pos).ok_or ("Cart left grid") ?;
			let (next_dir, next_turn, next_crashed) = match (prev_dir, prev_turn, next_tile) {
				(Dir::Up, turn, Tile::Vert) => (Dir::Up, turn, false),
				(Dir::Down, turn, Tile::Vert) => (Dir::Down, turn, false),
				(Dir::Left, turn, Tile::Horiz) => (Dir::Left, turn, false),
				(Dir::Right, turn, Tile::Horiz) => (Dir::Right, turn, false),
				(Dir::Up, turn, Tile::CornerBck) => (Dir::Left, turn, false),
				(Dir::Up, turn, Tile::CornerFwd) => (Dir::Right, turn, false),
				(Dir::Down, turn, Tile::CornerBck) => (Dir::Right, turn, false),
				(Dir::Down, turn, Tile::CornerFwd) => (Dir::Left, turn, false),
				(Dir::Left, turn, Tile::CornerBck) => (Dir::Up, turn, false),
				(Dir::Left, turn, Tile::CornerFwd) => (Dir::Down, turn, false),
				(Dir::Right, turn, Tile::CornerBck) => (Dir::Down, turn, false),
				(Dir::Right, turn, Tile::CornerFwd) => (Dir::Up, turn, false),
				(dir, Turn::Left, Tile::Crossing) => (dir + Turn::Left, Turn::None, false),
				(dir, Turn::None, Tile::Crossing) => (dir + Turn::None, Turn::Right, false),
				(dir, Turn::Right, Tile::Crossing) => (dir + Turn::Right, Turn::Left, false),
				(dir, turn, tile) if tile.is_cart () => (dir, turn, true),
				_ => return Err (format! (
					"Unable to tick: {prev_dir:?} {prev_turn:?} {next_tile:?}").into ()),
			};
			self.carts [idx] = (next_pos, next_dir, next_turn, next_tile, next_crashed);
			self.grid.set (prev_pos, prev_tile);
			if next_crashed {
				let & mut (_, _, _, other_tile, ref mut other_crashed) =
					self.carts.iter_mut ()
						.find (|&& mut (other_pos, _, _, _, other_crashed)|
							other_pos == next_pos && ! other_crashed)
						.unwrap ();
				* other_crashed = true;
				self.grid.set (next_pos, other_tile);
			} else {
				self.grid.set (next_pos, match next_dir {
					Dir::Up => Tile::CartUp,
					Dir::Down => Tile::CartDown,
					Dir::Left => Tile::CartLeft,
					Dir::Right => Tile::CartRight,
				});
			}
		}
		Ok (())
	}
}
