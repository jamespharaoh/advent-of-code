//! Logic for solving the puzzles

use super::*;
use input::Input;
use model::Grid;
use model::Pos;
use model::RouteDir;
use model::RouteRegexItem;
use model::RouteRegexString;

const STACK_SIZE: usize = 320;
const GRID_EXPAND: usize = 24;

pub fn part_one (input: & Input) -> GenResult <u32> {
	let grid = gen_grid (input) ?;
	let mut todo: Vec <(Pos, u32)> = Vec::new ();
	todo.push ((Pos::ZERO, 0));
	let mut seen: HashSet <Pos> = HashSet::new ();
	seen.insert (Pos::ZERO);
	let mut furthest = 0;
	while let Some ((pos, dist)) = todo.pop () {
		furthest = cmp::max (dist, furthest);
		for dir in grid.get (pos).unwrap ().doors () {
			let adj_pos = (pos + (dir, 1)).unwrap ();
			if ! seen.insert (adj_pos) { continue }
			todo.push ((adj_pos, dist + 1));
		}
	}
	Ok (furthest)
}

pub fn part_two (input: & Input) -> GenResult <u32> {
	let grid = gen_grid (input) ?;
	let mut todo: Vec <(Pos, u32)> = Vec::new ();
	todo.push ((Pos::ZERO, 0));
	let mut seen: HashSet <Pos> = HashSet::new ();
	seen.insert (Pos::ZERO);
	let mut num_far_rooms = 0;
	while let Some ((pos, dist)) = todo.pop () {
		if input.params.dist_two <= dist { num_far_rooms += 1; }
		for dir in grid.get (pos).unwrap ().doors () {
			let adj_pos = (pos + (dir, 1)).unwrap ();
			if ! seen.insert (adj_pos) { continue }
			todo.push ((adj_pos, dist + 1));
		}
	}
	Ok (num_far_rooms)
}

fn gen_grid (input: & Input) -> GenResult <Grid> {
	use RouteRegexItem::{ Branch, Span };
	type Frame = (u8, RouteRegexString, u8);
	type Stack = ArrayVec <Frame, STACK_SIZE>;
	let mut grid =
		Grid::new_vec (
			[GRID_EXPAND.as_isize (), GRID_EXPAND.as_isize ()],
			[GRID_EXPAND * 2 + 1, GRID_EXPAND * 2 + 1]);
	let mut todo: Vec <(Pos, Stack)> = Vec::new ();
	todo.push ((Pos::ZERO, array_vec! [ (0, input.regex.deref ().clone (), 0) ]));
	let mut seen: HashSet <(Pos, ArrayVec <(u8, u8), STACK_SIZE>)> = HashSet::new ();
	seen.insert ((Pos::ZERO, array_vec! [ (0, 0) ]));
	fn make_seen_indexes (stack: & Stack) -> ArrayVec <(u8, u8), STACK_SIZE> {
		stack.iter ()
			.map (|& (branch_idx, _, string_idx)| (branch_idx, string_idx))
			.collect ()
	}
	while let Some ((mut pos, mut stack)) = todo.pop () {
		let (branch_idx, string, string_idx) = stack.pop ().unwrap ();
		if string.len () <= string_idx.as_usize () {
			if ! stack.is_empty () {
				todo.push ((pos, stack));
			}
			continue;
		}
		match string [string_idx.as_usize ()] {
			Branch (ref branches) => {
				for (sub_branch_idx, sub_branch) in branches.iter ().enumerate () {
					let sub_branch_idx = sub_branch_idx.as_u8 ();
					let mut stack = stack.clone ();
					stack.push ((branch_idx, string.clone (), string_idx + 1));
					stack.push ((sub_branch_idx, sub_branch.clone (), 0));
					if seen.insert ((pos, make_seen_indexes (& stack))) {
						todo.push ((pos, stack));
					}
				}
			},
			Span (ref span_dirs) => {
				for dir in span_dirs.iter ().copied () {
					* grid.get_mut (pos).unwrap () |= dir;
					pos = (pos + (* dir, 1)).unwrap ();
					if grid.get (pos).is_none () { grid_resize (& mut grid, dir) ?; }
					* grid.get_mut (pos).unwrap () |= dir.rev ();
				}
				stack.push ((branch_idx, string, string_idx + 1));
				if seen.insert ((pos, make_seen_indexes (& stack))) {
					todo.push ((pos, stack));
				}
			},
		}
	}
	Ok (grid)
}

fn grid_resize (grid: & mut Grid, dir: RouteDir) -> GenResult <()> {
	match dir {
		RouteDir::North => {
			if 240 < grid.native_size () [0] {
				return Err ("Max grid size is 240".into ());
			}
			* grid = grid.resize (
				[grid.native_origin () [0], grid.native_origin () [1]],
				[grid.native_size () [0] + GRID_EXPAND, grid.native_size () [1]]);
		},
		RouteDir::South => {
			if 240 < grid.native_size () [0] {
				return Err ("Max grid size is 240".into ());
			}
			* grid = grid.resize (
				[grid.native_origin () [0] + GRID_EXPAND.as_isize (), grid.native_origin () [1]],
				[grid.native_size () [0] + GRID_EXPAND, grid.native_size () [1]]);
		},
		RouteDir::East => {
			if 500 < grid.native_size () [1] {
				return Err ("Max grid size is 240".into ());
			}
			* grid = grid.resize (
				[grid.native_origin () [0], grid.native_origin () [1]],
				[grid.native_size () [0], grid.native_size () [1] + GRID_EXPAND]);
		},
		RouteDir::West => {
			if 500 < grid.native_size () [1] {
				return Err ("Max grid size is 240".into ());
			}
			* grid = grid.resize (
				[grid.native_origin () [0], grid.native_origin () [1] + GRID_EXPAND.as_isize ()],
				[grid.native_size () [0], grid.native_size () [1] + GRID_EXPAND]);
		},
	}
	Ok (())
}
