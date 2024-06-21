use super::*;

use input::Input;
use model::Dir;
use model::Grid;
use model::Pos;
use model::Tile;

pub fn part_one (input: & Input) -> GenResult <u32> {
	check_input (input) ?;
	let mut state = State::new (& input.grid);
	state.roll (Dir::Up);
	Ok (state.load ())
}

pub fn part_two (input: & Input) -> GenResult <u32> {
	check_input (input) ?;
	let mut state = State::new (& input.grid);
	let mut history = HashMap::new ();
	let mut iter_idx = 0_u32;
	let mut num_iters = 0_u32;
	while iter_idx < 1_000_000_000 {
		if let Some (prev_idx) = history.insert (state.clone (), iter_idx) {
			let loop_size = iter_idx - prev_idx;
			if iter_idx + loop_size <= 1_000_000_000 {
				iter_idx += (1_000_000_000 - iter_idx) / loop_size * loop_size;
			}
		}
		state.roll (Dir::Up);
		state.roll (Dir::Left);
		state.roll (Dir::Down);
		state.roll (Dir::Right);
		iter_idx += 1;
		num_iters += 1;
		if num_iters == 200 { return Err ("Exceeded max iters".into ()); }
	}
	Ok (state.load ())
}

fn check_input (input: & Input) -> GenResult <()> {
	if input.grid.size ().row < 2 || input.grid.size ().col < 2 {
		return Err ("Grid size must be at least 2×2".into ());
	}
	if 100 < input.grid.size ().row || 100 < input.grid.size ().col {
		return Err ("Grid size must be at most 100×100".into ());
	}
	Ok (())
}

#[ derive (Clone, Eq, Hash, Ord, PartialEq, PartialOrd) ]
struct State {
	grid: Grid,
	posns: Vec <Pos>,
}

impl State {
	fn new (grid: & Grid) -> Self {
		let grid = grid.clone ();
		let posns: Vec <Pos> =
			grid.iter ()
				.filter (|& (_, tile)| tile == Tile::RoundRock)
				.map (|(pos, _)| pos)
				.collect ();
		Self { grid, posns }
	}
	fn roll (& mut self, dir: Dir) {
		match dir {
			Dir::Up => self.posns.sort_by_key (|& pos| pos.row),
			Dir::Down => self.posns.sort_by_key (|& pos| cmp::Reverse (pos.row)),
			Dir::Left => self.posns.sort_by_key (|& pos| pos.col),
			Dir::Right => self.posns.sort_by_key (|& pos| cmp::Reverse (pos.col)),
		}
		let offset = self.grid.offset (dir).unwrap ();
		for idx in 0 .. self.posns.len () {
			let pos = self.posns [idx];
			self.grid.set (pos, Tile::Empty);
			let mut cur = self.grid.cursor (pos).unwrap ();
			while let Ok (next_cur) = chk! (cur + offset) {
				if next_cur.get (& self.grid) != Tile::Empty { break }
				cur = next_cur;
			}
			let pos = cur.pos ();
			self.grid.set (pos, Tile::RoundRock);
			self.posns [idx] = pos;
		}
	}
	fn load (& self) -> u32 {
		let num_rows = self.grid.size ().row;
		self.posns.iter ()
			.map (|pos| (num_rows - pos.row).pan_u32 ())
			.sum ()
	}
}
