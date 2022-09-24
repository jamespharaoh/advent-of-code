//! Logic for solving the puzzles

use super::*;
use input::Input;
use model::Core;
use model::CoreStep;
use model::Dir;
use model::Grid;
use model::Pos;
use model::Tile;

pub fn part_one (input: & Input) -> GenResult <u32> {
	let mut seen = HashSet::new ();
	seen.insert (Pos::ZERO);
	let mut todo = Vec::new ();
	todo.push ((Core::new (input), Pos::ZERO, 0));
	while let Some ((core, pos, dist)) = todo.pop () {
		for dir in [ Dir::North, Dir::South, Dir::West, Dir::East ] {
			let adj_pos = (pos + (dir, 1)) ?;
			if ! seen.insert (adj_pos) { continue }
			let mut core = core.clone ();
			match core.step (dir) ? {
				CoreStep::Blocked => continue,
				CoreStep::Moved => todo.push ((core, adj_pos, dist + 1)),
				CoreStep::Found => return Ok (dist + 1),
			}
		}
	}
	Err ("No solution found".into ())
}

pub fn part_two (input: & Input) -> GenResult <u32> {
	let grid = calc_grid (input) ?;
	calc_mins (grid)
}

fn calc_mins (mut grid: Grid) -> GenResult <u32> {
	let mut todo: Vec <Pos> =
		grid.iter ()
			.filter (|& (_, tile)| tile == Tile::OxygenSystem)
			.map (|(pos, _)| pos)
			.collect ();
	let mut todo_temp = Vec::new ();
	let mut num_mins = 0;
	loop {
		for pos in todo.drain ( .. ) {
			for adj_pos in pos.adjacent_4 () {
				if 128 < adj_pos.n.abs () + adj_pos.e.abs () {
					return Err ("Giving up due to distance from starting point".into ());
				}
				if grid.get (adj_pos).unwrap () != Tile::Empty { continue }
				grid.set (adj_pos, Tile::Oxygen);
				todo_temp.push (adj_pos);
			}
		}
		mem::swap (& mut todo, & mut todo_temp);
		if todo.is_empty () { break }
		num_mins += 1;
	}
	Ok (num_mins)
}

fn calc_grid (input: & Input) -> GenResult <Grid> {
	let mut grid = Grid::new ([16, 16], [33, 33]);
	grid.set (Pos::ZERO, Tile::Empty);
	let mut todo = Vec::new ();
	todo.push ((Core::new (input), Pos::ZERO));
	while let Some ((core, pos)) = todo.pop () {
		for dir in [ Dir::North, Dir::South, Dir::West, Dir::East ] {
			let adj_pos = (pos + (dir, 1)) ?;
			if 128 < adj_pos.n.abs () + adj_pos.e.abs () {
				return Err ("Giving up due to distance from starting point".into ());
			}
			let adj_tile = grid.get (adj_pos).ok_or (()).or_else (|()| {
				grid = grid.resize (
					[ grid.native_origin () [0] + 16, grid.native_origin () [1] + 16 ],
					[ grid.native_size () [0] + 32, grid.native_size () [1] + 32 ]) ?;
				Ok::<_, Overflow> (Tile::Unknown)
			}) ?;
			if adj_tile != Tile::Unknown { continue }
			let mut core = core.clone ();
			let adj_tile = match core.step (dir) ? {
				CoreStep::Blocked => Tile::Wall,
				CoreStep::Moved => Tile::Empty,
				CoreStep::Found => Tile::OxygenSystem,
			};
			grid.set (adj_pos, adj_tile);
			if adj_tile != Tile::Wall {
				todo.push ((core, adj_pos));
			}
		}
	}
	Ok (grid)
}
