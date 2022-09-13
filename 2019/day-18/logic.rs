//! Logic for solving the puzzles

use super::*;

use input::Input;
use model::Grid;
use model::Pos;
use model::Tile::{ self, DeadEnd, Door, Entrance, Key, Open, Wall };

pub fn part_one (input: & Input) -> GenResult <u32> {
	let mut grid = input.grid.clone ();
	simplify_grid (& mut grid);
	calc_result (input, & grid)
}

pub fn part_two (input: & Input) -> GenResult <u32> {
	let mut grid = input.grid.clone ();
	multiply_entrance (& mut grid) ?;
	simplify_grid (& mut grid);
	calc_result (input, & grid)
}

fn multiply_entrance (grid: & mut Grid) -> GenResult <()> {
	let (start_pos, _) = grid.iter ()
		.find (|& (_, tile)| matches! (tile, Entrance))
		.ok_or ("No entrance found") ?;
	for (offset, tile) in [
		(Pos { y: -1, x: -1 }, Entrance),
		(Pos { y: -1, x: 0 }, Wall),
		(Pos { y: -1, x: 1 }, Entrance),
		(Pos { y: 0, x: -1 }, Wall),
		(Pos { y: 0, x: 0 }, Wall),
		(Pos { y: 0, x: 1 }, Wall),
		(Pos { y: 1, x: -1 }, Entrance),
		(Pos { y: 1, x: 0 }, Wall),
		(Pos { y: 1, x: 1 }, Entrance),
	] {
		let pos = start_pos.try_add (offset).ok ().ok_or ("Entrance on edge of map") ?;
		grid.try_set (pos, tile).ok_or ("Entrance on edge of map") ?;
	}
	Ok (())
}

fn calc_result (input: & Input, grid: & Grid) -> GenResult <u32> {
	if grid.iter ().filter (|& (_, tile)| matches! (tile, Entrance)).count () > 4 {
		return Err ("More than four entrances".into ());
	}
	let paths = calc_paths (input, grid) ?;
	let need_keys = grid.values ()
		.filter_map (|tile| match tile {
			Key (id) => Some (id),
			DeadEnd | Door (_) | Entrance | Open | Wall => None,
		})
		.fold (0_u32, |sum, val| sum | (1 << val));
	let mut search = PrioritySearch::with_hash_map (
		|(posns, need_keys): (ArrayVec <Pos, 4>, u32), dist: u32, mut adder: PrioritySearchAdder <_, _, _>| {
			for (pos_idx, & pos) in posns.iter ().enumerate () {
				for path in & paths [& pos] {
					if need_keys & (1 << path.key_id) == 0 { continue }
					if need_keys & path.need_keys != 0 { continue }
					let mut posns = posns.clone ();
					posns [pos_idx] = * path.route.last ().unwrap ();
					let need_keys = need_keys & ! (1 << path.key_id);
					let dist = dist + path.route.len ().as_u32 ();
					adder.add ((posns, need_keys), dist);
				}
			}
			(posns, need_keys, dist)
		});
	let start_posns: ArrayVec <Pos, 4> =
		grid.iter ()
			.filter (|& (_, tile)| matches! (tile, Tile::Entrance))
			.map (|(pos, _)| pos)
			.collect ();
	search.push ((start_posns, need_keys), 0_u32);
	for (num_iters, (_, need_keys, dist)) in search.enumerate () {
		if num_iters.as_u32 () == input.params.max_iters {
			return Err ("Giving up after max iters".into ());
		}
		if need_keys != 0 { continue }
		return Ok (dist);
	}
	Err ("No solution found".into ())
}

#[ derive (Clone, Debug) ]
struct Path {
	key_id: u8,
	need_keys: u32,
	route: Vec <Pos>,
}

type Paths = HashMap <Pos, Vec <Path>>;

fn calc_paths (input: & Input, grid: & Grid) -> GenResult <Paths> {

	let mut all_paths = Paths::new ();

	// iterate over path starting points (entrances and keys)

	let mut num_starts = 0;
	for (start_pos, start_tile) in grid.iter ()
			.filter (|& (_, tile)| matches! (tile, Entrance | Key (_))) {
		let mut paths = Vec::new ();
		num_starts += 1;
		if input.params.max_path_starts < num_starts {
			return Err ("Refusing to handle too many path start points".into ());
		}

		// breadth-first search paths from starting point

		let mut todo: VecDeque <(u32, Vec <Pos>)> = VecDeque::new ();
		todo.push_back ((0, vec! [ start_pos ]));
		let mut seen: HashSet <Pos> = HashSet::new ();
		seen.insert (start_pos);
		while let Some ((need_keys, route)) = todo.pop_front () {
			let pos = * route.last ().unwrap ();

			// iterate next steps

			for adj_pos in pos.adjacent_4 () {
				if ! seen.insert (adj_pos) { continue }
				let adj_tile = some_or! (grid.get (adj_pos), continue);
				if matches! (adj_tile, Wall) { continue }
				if matches! (start_tile, Entrance) && matches! (adj_tile, Entrance) {
					return Err ("Multiple connected entrances".into ());
				}

				// continue iteration via this position

				let next_need_keys =
					if let Door (key_id) | Key (key_id) = adj_tile {
						need_keys | 1 << key_id
					} else { need_keys };
				let mut next_route = route.clone ();
				next_route.push (adj_pos);
				todo.push_back ((next_need_keys, next_route));

				// record path to keys

				if let Key (key_id) = adj_tile {
					paths.push (Path {
						key_id,
						need_keys,
						route: route.iter ().skip (1).copied ()
							.chain (iter::once (adj_pos))
							.collect (),
					});
				}

			}

		}

		all_paths.insert (start_pos, paths);

	}

	Ok (all_paths)

}

fn simplify_grid (grid: & mut Grid) {
	const fn can_pass (tile: Option <Tile>) -> bool {
		matches! (tile, Some (Open | Key (_) | Door (_) | Entrance))
	}
	fn is_dead_end (grid: & Grid, pos: Pos) -> bool {
		matches! (grid.get (pos), Some (Open))
			&& pos.adjacent_4 ().into_iter ()
				.filter (|& pos| can_pass (grid.get (pos)))
				.count () <= 1
	}
	let mut todo: Vec <Pos> =
		grid.keys ()
			.filter (|& pos| is_dead_end (grid, pos))
			.collect ();
	while let Some (pos) = todo.pop () {
		grid.set (pos, DeadEnd);
		todo.extend (pos.adjacent_4 ().iter ().copied ()
			.filter (|& pos| is_dead_end (grid, pos)));
	}
}
