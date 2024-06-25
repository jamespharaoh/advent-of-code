use super::*;

use input::Input;
use model::Dir;
use model::Pos;
use model::Tile;

pub fn part_one (input: & Input) -> GenResult <u64> {
	calc_result (input, true)
}

pub fn part_two (input: & Input) -> GenResult <u64> {
	calc_result (input, false)
}

fn calc_result (input: & Input, slippery: bool) -> GenResult <u64> {
	let grid = & input.grid;
	if grid.size ().y < 3 || grid.size ().x < 3 {
		return Err ("Grid size must be at least 3×3".into ());
	}
	if 200 < grid.size ().y || 200 < grid.size ().x {
		return Err ("Grid size must be at most 200×200".into ());
	}
	let offsets = [
		(Dir::Up, grid.offset (Dir::Up).unwrap ()),
		(Dir::Down, grid.offset (Dir::Down).unwrap ()),
		(Dir::Left, grid.offset (Dir::Left).unwrap ()),
		(Dir::Right, grid.offset (Dir::Right).unwrap ()),
	];
	let (start_pos, start_tile) =
		grid.iter ()
			.filter (|& (pos, _)| pos.y == 0)
			.filter (|& (_, tile)| tile != Tile::Forest)
			.exactly_one ()
			.ok_or ("Must be exactly only path on the top row") ?;
	if start_tile != Tile::Path { return Err ("Start tile must be path".into ()); }
	let (end_pos, end_tile) =
		grid.iter ()
			.filter (|& (pos, _)| pos.y == grid.end ().y - 1)
			.filter (|& (_, tile)| tile != Tile::Forest)
			.exactly_one ()
			.ok_or ("Must be exactly one path on the bottom row") ?;
	if end_tile != Tile::Path { return Err ("End tile must be path".into ()); }
	if grid.iter ()
			.filter (|& (pos, _)| pos.x == 0 || pos.x == grid.end ().x - 1)
			.any (|(_, tile)| tile != Tile::Forest) {
		return Err ("All edge tiles, except start and end, must be forest".into ());
	}
	let mut nodes = vec! [ start_pos, end_pos ];
	for cur in grid.cursors () {
		if cur.get (grid) == Tile::Forest { continue }
		let num_dirs =
			offsets.into_iter ()
				.filter_map (|(_, offset)| chk! (cur + offset).ok ())
				.filter (|cur| cur.get (grid) != Tile::Forest)
				.count ();
		if num_dirs < 3 { continue }
		if cur.get (grid) != Tile::Path { return Err ("Slope can't be intersection".into ()); }
		nodes.push (cur.pos ());
		if 40 < nodes.len () { return Err ("Too many intersections".into ()); }
	}
	let nodes_index: HashMap <Pos, usize> =
		nodes.iter ().enumerate ()
			.map (|(idx, & pos)| (pos, idx))
			.collect ();
	let mut paths: HashMap <Pos, Vec <(Pos, u64)>> = HashMap::new ();
	for & start_pos in & nodes {
		let mut todo = vec! [ (grid.cursor (start_pos).unwrap (), None, 0) ];
		while let Some ((cur, dir, len)) = todo.pop () {
			let pos = cur.pos ();
			if 0 < len && pos == start_pos { continue }
			if 0 < len && nodes.contains (& pos) {
				paths.entry (start_pos).or_default ().push ((cur.pos (), len));
				continue;
			}
			let tile = cur.get (grid);
			if slippery && tile.dir ().is_some () && dir.is_some () && dir != tile.dir () { continue }
			for (next_dir, offset) in offsets {
				if Some (next_dir.around ()) == dir { continue }
				let Ok (next_cur) = chk! (cur + offset) else { continue };
				let next_tile = next_cur.get (grid);
				if next_tile == Tile::Forest { continue };
				if slippery && next_tile.dir ().is_some_and (|tile_dir| tile_dir != next_dir) {
					continue;
				}
				todo.push ((next_cur, Some (next_dir), len + 1));
				if 100 < todo.len () { return Err ("Max complexity exceeded".into ()); }
			}
		}
	}
	let mut todo = vec! [ (start_pos, 0, 1_u64) ];
	let mut lens = Vec::new ();
	let mut num_iters = 0;
	while let Some ((pos, len, seen)) = todo.pop () {
		num_iters += 1;
		if input.params.max_iters < num_iters { return Err ("Max iterations exceeded".into ()); }
		if pos == end_pos {
			lens.push (len);
			continue;
		}
		let Some (dests) = paths.get (& pos) else { continue };
		for & (next_pos, next_len) in dests {
			let next_idx = nodes_index [& next_pos];
			if seen & (1 << next_idx) != 0 { continue }
			let next_seen = seen | (1 << next_idx);
			todo.push ((next_pos, len + next_len, next_seen));
			if 100 < todo.len () { return Err ("Max complexity exceeded".into ()); }
		}
	}
	if lens.is_empty () { return Err ("No solution found".into ()); }
	Ok (lens.into_iter ().max ().unwrap ())
}
