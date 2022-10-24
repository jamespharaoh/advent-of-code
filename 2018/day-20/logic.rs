//! Logic for solving the puzzles

use super::*;

use input::Input;
use model::Coord;
use model::Grid;
use model::Pos;
use model::RouteDir;
use model::RouteRegexItem;
use model::RouteRegexString;

const STACK_SIZE: usize = 320;
const GRID_EXPAND: Coord = 24;

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
			todo.push ((adj_pos, chk! (dist + 1_u32) ?));
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
			todo.push ((adj_pos, chk! (dist + 1_u32) ?));
		}
	}
	Ok (num_far_rooms)
}

fn gen_grid (input: & Input) -> GenResult <Grid> {
	use RouteRegexItem::{ Branch, Span };
	type Frame = (RouteRegexString, Pos, u8, u8);
	type Stack = MiniVec <Frame, STACK_SIZE>;
	let mut grid = Grid::new_range (
		Pos::new (- GRID_EXPAND, - GRID_EXPAND),
		Pos::new (GRID_EXPAND + 1, GRID_EXPAND + 1)) ?;
	let mut todo: Vec <(Pos, Stack)> = Vec::new ();
	todo.push ((Pos::ZERO, mini_vec! [ (input.regex.deref ().clone (), Pos::ZERO, 0, 0) ]));
	let mut seen: HashSet <(Pos, MiniVec <(u8, u8), STACK_SIZE>)> = HashSet::new ();
	seen.insert ((Pos::ZERO, mini_vec! [ (0, 0) ]));
	fn make_seen_indexes (stack: & Stack) -> MiniVec <(u8, u8), STACK_SIZE> {
		stack.iter ()
			.map (|& (_, _, branch_idx, string_idx)| (branch_idx, string_idx))
			.collect ()
	}
	while let Some ((mut pos, mut stack)) = todo.pop () {
		let (string, start_pos, branch_idx, string_idx) = stack.pop ().unwrap ();
		if string.len () <= string_idx.to_usize () ? {
			if ! stack.is_empty () {
				todo.push ((pos, stack));
			}
			continue;
		}
		match string [string_idx.to_usize () ?] {
			Branch (ref branches) => {
				for (sub_branch_idx, sub_branch) in branches.iter ().enumerate () {
					let sub_branch_idx = sub_branch_idx.to_u8 () ?;
					let mut stack = stack.clone ();
					stack.push ((string.clone (), start_pos, branch_idx, chk! (string_idx + 1_u8) ?));
					stack.push ((sub_branch.clone (), pos, sub_branch_idx, 0));
					if seen.insert ((pos, make_seen_indexes (& stack))) {
						todo.push ((pos, stack));
					}
				}
			},
			Span (ref span_dirs) => {
				for dir in span_dirs.iter ().copied () {
					* grid.get_mut (pos).unwrap () |= dir;
					pos = (pos + (* dir, 1)) ?;
					if grid.get (pos).is_none () { grid_resize (& mut grid, dir) ?; }
					* grid.get_mut (pos).unwrap () |= dir.rev ();
				}
				stack.push ((string, start_pos, branch_idx, chk! (string_idx + 1_u8) ?));
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
			if 240 < grid.size ().n { return Err ("Max grid size is 240".into ()) }
			* grid = grid.extend_in_place ([(0, GRID_EXPAND), (0, 0)]) ?;
		},
		RouteDir::South => {
			if 240 < grid.size ().n { return Err ("Max grid size is 240".into ()) }
			* grid = grid.extend_in_place ([(GRID_EXPAND, 0), (0, 0)]) ?;
		},
		RouteDir::East => {
			if 240 < grid.size ().e { return Err ("Max grid size is 240".into ()) }
			* grid = grid.extend_in_place ([(0, 0), (0, GRID_EXPAND)]) ?;
		},
		RouteDir::West => {
			if 240 < grid.size ().e { return Err ("Max grid size is 240".into ()) }
			* grid = grid.extend_in_place ([(0, 0), (GRID_EXPAND, 0)]) ?;
		},
	}
	Ok (())
}
