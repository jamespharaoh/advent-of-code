use super::*;

use input::Input;
use model::Dir;
use model::Grid;
use model::Pos;
use model::Tile;

pub fn part_one (input: & Input) -> GenResult <u32> {
	calc_result (& input.grid, Pos::ZERO, Dir::Right)
}

pub fn part_two (input: & Input) -> GenResult <u32> {
	let grid = & input.grid;
	iter::empty ()
		.chain ((grid.start ().y .. grid.end ().y).map (|y| (Pos::new (y, 0), Dir::Right)))
		.chain ((grid.start ().y .. grid.end ().y).map (|y| (Pos::new (y, grid.end ().x - 1), Dir::Left)))
		.chain ((grid.start ().x .. grid.end ().x).map (|x| (Pos::new (0, x), Dir::Down)))
		.chain ((grid.start ().x .. grid.end ().x).map (|x| (Pos::new (grid.end ().y - 1, x), Dir::Up)))
		.map (|(pos, dir)| calc_result (& input.grid, pos, dir))
		.try_fold (0, |max, result| GenOk (cmp::max (max, result ?)))
}

fn calc_result (grid: & Grid <Tile>, start_pos: Pos, start_dir: Dir) -> GenResult <u32> {
	let offsets = [
		grid.offset (Dir::Up) ?,
		grid.offset (Dir::Down) ?,
		grid.offset (Dir::Left) ?,
		grid.offset (Dir::Right) ?,
	];
	let index_for_dir = |dir| match dir {
		Dir::Up => 0,
		Dir::Down => 1,
		Dir::Left => 2,
		Dir::Right => 3,
	};
	let offset_for_dir = |dir| offsets [index_for_dir (dir)];
	let mut seen = Grid::<[bool; 4]>::new_range (grid.start (), grid.end ()) ?;
	let mut todo = vec! [ (grid.cursor (start_pos).unwrap (), start_dir) ];
	let mut num = 0;
	while let Some ((cur, dir)) = todo.pop () {
		let dir_index = index_for_dir (dir);
		let pos = cur.pos ();
		let tile = cur.get (grid);
		let seen_vals = seen.get_mut (pos).unwrap ();
		if seen_vals [dir_index] { continue }
		if seen_vals.iter ().all (|& val| ! val) { num += 1; }
		seen_vals [dir_index] = true;
		let dirs: TinyVec <Dir, 2> = match (tile, dir) {
			(Tile::Empty, dir) => tiny_vec! [ dir ],
			(Tile::MirrorBack, Dir::Up) => tiny_vec! [ Dir::Left ],
			(Tile::MirrorBack, Dir::Down) => tiny_vec! [ Dir::Right ],
			(Tile::MirrorBack, Dir::Left) => tiny_vec! [ Dir::Up ],
			(Tile::MirrorBack, Dir::Right) => tiny_vec! [ Dir::Down ],
			(Tile::MirrorForward, Dir::Up) => tiny_vec! [ Dir::Right ],
			(Tile::MirrorForward, Dir::Down) => tiny_vec! [ Dir::Left ],
			(Tile::MirrorForward, Dir::Left) => tiny_vec! [ Dir::Down ],
			(Tile::MirrorForward, Dir::Right) => tiny_vec! [ Dir::Up ],
			(Tile::SplitterVertical, Dir::Up | Dir::Down) => tiny_vec! [ dir ],
			(Tile::SplitterVertical, Dir::Left | Dir::Right) => tiny_vec! [ Dir::Up, Dir::Down ],
			(Tile::SplitterHorizontal, Dir::Up | Dir::Down) => tiny_vec! [ Dir::Left, Dir::Right ],
			(Tile::SplitterHorizontal, Dir::Left | Dir::Right) => tiny_vec! [ dir ],
		};
		for dir in dirs {
			if let Ok (cur) = chk! (cur + offset_for_dir (dir)) {
				todo.push ((cur, dir));
			}
		}
	}
	Ok (num)
}
