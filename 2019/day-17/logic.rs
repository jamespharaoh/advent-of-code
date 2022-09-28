//! Logic for solving the puzzles

use super::*;

use input::Input;
use model::Cpu;
use model::Dir;
use model::Grid;
use model::Pos;
use model::Step;
use model::Tile;
use model::Turn;
use model::Val;

pub fn part_one (input: & Input) -> GenResult <u32> {
	let grid = calc_grid (input) ?;
	Ok (
		grid.iter ()
			.filter (|& (_, tile)| ! matches! (tile, Tile::Empty))
			.filter (|& (pos, _)| pos.adjacent_4 ().into_iter ()
				.map (|adj_pos| grid.get (adj_pos).unwrap_or (Tile::Empty))
				.filter (|& tile| ! matches! (tile, Tile::Empty))
				.count () == 4)
			.map (|(pos, _)| pos.x.pan_u32 () * pos.y.pan_u32 ())
			.sum ()
	)
}

pub fn part_two (input: & Input) -> GenResult <Val> {
	let mut grid = calc_grid (input) ?;
	let (bot_pos, bot_dir) = find_bot (& mut grid) ?;
	let route = find_route (& grid, bot_pos, bot_dir) ?;
	let (main, funcs) = find_funcs (& route) ?;
	let result = exec_route (input, & main, & funcs) ?;
	Ok (result)
}

pub type Func = ArrayVec <Step, 10>;
pub type Funcs = ArrayVec <Func, 3>;
pub type Main = ArrayVec <char, 10>;

fn calc_grid (input: & Input) -> GenResult <Grid> {
	let mut cpu = Cpu::new (input.data.clone ());
	cpu.set_mem_limit (8_192);
	cpu.set_max_ops (100_000);
	let mut output = String::new ();
	while let Some (val) = cpu.run ().output () ? {
		output.push (val.to_char () ?);
	}
	Grid::parse_from_str (output.trim_end ())
}

fn find_bot (grid: & mut Grid) -> GenResult <(Pos, Dir)> {
	let (bot_pos, bot_dir) = grid.iter ()
		.filter_map (|(pos, tile)| Some ((pos, match tile {
			Tile::RobotUp => Dir::Up,
			Tile::RobotDown => Dir::Down,
			Tile::RobotLeft => Dir::Left,
			Tile::RobotRight => Dir::Right,
			Tile::Empty | Tile::Scaffold => return None,
		})))
		.exactly_one ().ok ().ok_or ("Must have exactly one robot tile") ?;
	grid.set (bot_pos, Tile::Scaffold);
	Ok ((bot_pos, bot_dir))
}

fn find_route (grid: & Grid, mut pos: Pos, mut dir: Dir) -> GenResult <Vec <Step>> {
	let mut result = Vec::new ();
	let mut seen = HashSet::new ();
	loop {
		if ! seen.insert (pos) { return Err ("No solution found".into ()) }
		let has_left = grid.get (pos.try_add ((dir + Turn::Left, 1)) ?) == Some (Tile::Scaffold);
		let has_right = grid.get (pos.try_add ((dir + Turn::Right, 1)) ?) == Some (Tile::Scaffold);
		if ! has_left && ! has_right { break }
		if has_left && has_right { return Err ("No solution found".into ()) }
		if has_left { dir = dir + Turn::Left; }
		if has_right { dir = dir + Turn::Right; }
		let mut dist = 0;
		loop {
			let next_pos = pos.try_add ((dir, 1)) ?;
			if grid.get (next_pos) != Some (Tile::Scaffold) { break }
			pos = next_pos;
			dist += 1;
		}
		result.push (if has_left { Step::Left (dist) } else { Step::Right (dist) });
	}
	Ok (result)
}

fn find_funcs (route: & [Step]) -> GenResult <(Main, Funcs)> {

	let mut todo = Vec::new ();
	todo.push ((0, Main::new (), Funcs::new ()));
	let mut num_iters = 1000_u32;

	while let Some ((prefix_len, main, funcs)) = todo.pop () {

		if num_iters == 0 { return Err ("Giving up after too many iterations".into ()) }
		num_iters -= 1;

		// find matching functions for the remaining route

		for (func_id, func) in ('A' .. ).zip (& funcs) {
			if ! itertools::equal (
					route.iter ().copied ().skip (prefix_len).take (func.len ()),
					func.iter ().copied ()) {
				continue;
			}
			let mut main = main.clone ();
			main.push (func_id);
			if prefix_len + func.len () == route.len () { return Ok ((main, funcs)) }
			todo.push ((prefix_len + func.len (), main, funcs.clone ()));
		}

		// define new functions from prefixes of the remaining route

		if funcs.is_full () { continue }
		for func_len in 1 .. (route.len () - prefix_len) {
			let func: Func =
				route [prefix_len .. prefix_len + func_len].iter ().copied ().collect ();
			if 20 < (& func).display_delim (",").to_string ().len () { break }
			if funcs.contains (& func) { continue }
			let func_id = ('A'.pan_u32 () + funcs.len ().pan_u32 ()).pan_char ();
			let mut funcs = funcs.clone ();
			funcs.push (func);
			let mut main = main.clone ();
			main.push (func_id);
			todo.push ((prefix_len + func_len, main, funcs));
		}

	}

	Err ("No solution found".into ())

}

fn exec_route (input: & Input, main: & Main, funcs: & Funcs) -> GenResult <Val> {
	let mut cpu = Cpu::new (input.data.clone ());
	cpu.set_mem_limit (8_192);
	cpu.set_max_ops (200_000);
	cpu.mem_set (Val::ZERO, Val::TWO).unwrap ();
	cpu.input_str (& format! ("{}\n", main.display_delim (",")));
	for func in funcs {
		cpu.input_str (& format! ("{}\n", func.display_delim (",")));
	}
	cpu.input_str ("n\n");
	let mut last_output = None;
	while let Some (output) = cpu.run ().output () ? { last_output = Some (output); }
	Ok (last_output.ok_or ("No solution found") ?)
}
