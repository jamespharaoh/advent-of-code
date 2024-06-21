use super::*;

use input::Input;
use model::Dir;
use model::Grid;
use model::Pos;
use model::Tile;

pub fn part_one (input: & Input) -> GenResult <u64> {
	let (_, _, dist) = PipeIter::new (& input.grid) ?.last ().unwrap () ?;
	Ok (dist)
}

pub fn part_two (input: & Input) -> GenResult <u64> {
	let mut clean_grid = Grid::new_range (input.grid.start (), input.grid.end ()).unwrap ();
	for step in PipeIter::new (& input.grid) ? {
		let (pos, tile, _) = step ?;
		clean_grid.set (pos, tile);
	}
	let (_, _, num_inside) =
		clean_grid.iter ()
			.fold ((false, false, 0), |(mut in_top, mut in_bottom, mut count), (_, tile)| {
				match tile {
					Tile::Vertical => { in_top = ! in_top; in_bottom = ! in_bottom; },
					Tile::NorthEast | Tile::NorthWest => in_top = ! in_top,
					Tile::SouthEast | Tile::SouthWest => in_bottom = ! in_bottom,
					Tile::Ground => { if in_top && in_bottom { count += 1; } },
					Tile::Horizontal => (),
					Tile::Start => unreachable! (),
				}
				(in_top, in_bottom, count)
			});
	Ok (num_inside)
}

struct PipeIter <'dat> {
	grid: & 'dat Grid,
	offsets: [GridOffset <Pos, 2>; 4],
	cur_0: GridCursor <Pos, 2>,
	dir_0: Dir,
	cur_1: GridCursor <Pos, 2>,
	dir_1: Dir,
	first: Option <(Pos, Tile)>,
	dist: u64,
	inc_dist: bool,
	complete: bool,
}

impl <'dat> PipeIter <'dat> {

	fn new (grid: & 'dat Grid) -> GenResult <Self> {
		if grid.size ().n < 2 || grid.size ().e < 2 {
			return Err ("Grid size must be at least 2×2".into ());
		}
		let dirs = [ Dir::North, Dir::South, Dir::East, Dir::West ];
		let offsets = dirs.map (|dir| grid.offset (dir).unwrap ());
		let (start_pos, _) =
			grid.iter ()
				.filter (|& (_, tile)| tile.is_start ())
				.exactly_one ()
				.ok_or ("Must have exactly one start tile") ?;
		let start_cur = grid.cursor (start_pos).unwrap ();
		let mut todo = Vec::new ();
		for (dir, offset) in iter::zip (dirs, offsets) {
			let Ok (cur) = chk! (start_cur + offset) else { continue };
			let tile = cur.get (grid);
			if ! tile.is_pipe () { continue }
			if ! tile.dirs ().contains (& dir.around ()) { continue }
			todo.push ((cur, dir));
		}
		if todo.len () != 2 { return Err ("Can't work out pipes around start".into ()) }
		let (cur_0, dir_0) = todo [0];
		let (cur_1, dir_1) = todo [1];
		Ok (Self {
			grid, offsets,
			cur_0, dir_0, cur_1, dir_1,
			first: Some ((start_pos, Tile::for_dirs ([ dir_0, dir_1 ]))),
			dist: 0,
			inc_dist: true,
			complete: false,
		})
	}

}

impl <'dat> Iterator for PipeIter <'dat> {

	type Item = GenResult <(Pos, Tile, u64)>;

	fn next (& mut self) -> Option <GenResult <(Pos, Tile, u64)>> {
		if self.complete { return None }
		if let Some ((pos, tile)) = self.first.take () {
			return Some (Ok ((pos, tile, self.dist)));
		}
		if self.inc_dist { self.dist += 1; }
		self.inc_dist = ! self.inc_dist;
		let last = self.cur_0 == self.cur_1;
		let pos = self.cur_0.pos ();
		let tile = self.cur_0.get (self.grid);
		if ! tile.is_pipe () {
			self.complete = true;
			return Some (Err (format! ("Not a pipe: {pos:?}: {tile:?}").into ()));
		}
		if ! tile.dirs ().contains (& self.dir_0.around ()) {
			self.complete = true;
			return Some (Err (format! ("Tile does not connect: {pos:?}: {tile:?}").into ()));
		}
		self.dir_0 = tile.follow (self.dir_0);
		let offset = match self.dir_0 {
			Dir::North => self.offsets [0],
			Dir::South => self.offsets [1],
			Dir::East => self.offsets [2],
			Dir::West => self.offsets [3],
		};
		self.cur_0 = match chk! (self.cur_0 + offset) {
			Ok (val) => val,
			Err (err) => { self.complete = true; return Some (Err (err.into ())) },
		};
		mem::swap (& mut self.cur_0, & mut self.cur_1);
		mem::swap (& mut self.dir_0, & mut self.dir_1);
		if last { self.complete = true; }
		Some (Ok ((pos, tile, self.dist)))
	}

}
