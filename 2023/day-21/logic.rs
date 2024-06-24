use super::*;

use input::Input;
use model::Dir;
use model::Grid;
use model::Pos;
use model::Tile;

pub fn part_one (input: & Input) -> GenResult <u64> {
	check_input (input) ?;
	let grid = & input.grid;
	let start_pos =
		grid.iter ()
			.filter (|& (_, tile)| tile == Tile::Start)
			.map (|(pos, _)| pos)
			.exactly_one ()
			.ok_or ("Must have exactly one start position") ?;
	calc_grid (grid, start_pos, input.params.num_steps_one)
}

pub fn part_two (input: & Input) -> GenResult <u64> {
	check_input (input) ?;
	if input.params.test != 0 { return part_two_test (input) }
	let grid = & input.grid;
	if grid.size () != Pos::new (131_i32, 131_i32) {
		return Err ("Grid size must be 131×131".into ());
	}
	if input.params.num_steps_two < 65 || (input.params.num_steps_two - 65) % 262 != 0 {
		return Err ("Num steps must be 65 + 262 × n, where n is an integer".into ());
	}
	let num_rings = (input.params.num_steps_two.pan_u64 () - 65) / 131;
	let start_pos =
		grid.iter ()
			.filter (|& (_, tile)| tile == Tile::Start)
			.map (|(pos, _)| pos)
			.exactly_one ()
			.ok_or ("Must have exactly one start position") ?;
	if start_pos != Pos::new (65_i32, 65_i32) {
		return Err ("Start position must be in centre".into ());
	}
	if (grid.start ().n .. grid.end ().n)
				.any (|n| grid [Pos::new (n, start_pos.e)] == Tile::Rock)
			|| (grid.start ().e .. grid.end ().e)
				.any (|e| grid [Pos::new (start_pos.n, e)] == Tile::Rock) {
		return Err ("Start row and col must not contain rocks".into ());
	}
	let edge_inner_ne = calc_grid (grid, Pos::new (0_i32, 0_i32), 195) ?;
	let edge_inner_nw = calc_grid (grid, Pos::new (0_i32, 130_i32), 195) ?;
	let edge_inner_se = calc_grid (grid, Pos::new (130_i32, 0_i32), 195) ?;
	let edge_inner_sw = calc_grid (grid, Pos::new (130_i32, 130_i32), 195) ?;
	let edge_inner = edge_inner_ne + edge_inner_nw + edge_inner_se + edge_inner_sw;
	let edge_outer_ne = calc_grid (grid, Pos::new (0_i32, 0_i32), 64) ?;
	let edge_outer_nw = calc_grid (grid, Pos::new (0_i32, 130_i32), 64) ?;
	let edge_outer_se = calc_grid (grid, Pos::new (130_i32, 0_i32), 64) ?;
	let edge_outer_sw = calc_grid (grid, Pos::new (130_i32, 130_i32), 64) ?;
	let edge_outer = edge_outer_ne + edge_outer_nw + edge_outer_se + edge_outer_sw;
	let corner_n = calc_grid (grid, Pos::new (0_i32, 65_i32), 130) ?;
	let corner_s = calc_grid (grid, Pos::new (130_i32, 65_i32), 130) ?;
	let corner_e = calc_grid (grid, Pos::new (65_i32, 0_i32), 130) ?;
	let corner_w = calc_grid (grid, Pos::new (65_i32, 130_i32), 130) ?;
	let corner = corner_n + corner_s + corner_e + corner_w;
	let middle_one = calc_grid (grid, Pos::new (65_i32, 65_i32), 199) ?;
	let middle_two = calc_grid (grid, Pos::new (65_i32, 65_i32), 200) ?;
	let mut sum = middle_one;
	for ring_idx in 0 .. num_rings {
		if ring_idx + 1 < num_rings {
			if ring_idx & 1 == 0 {
				sum += middle_two * (ring_idx + 1) * 4;
			} else {
				sum += middle_one * (ring_idx + 1) * 4;
			}
		} else {
			sum += corner;
			sum += (ring_idx + 1) * edge_outer;
			sum += ring_idx * edge_inner;
		}
	}
	Ok (sum)
}

fn check_input (input: & Input) -> GenResult <()> {
	let grid = & input.grid;
	if grid.size ().n < 2_i32 || grid.size ().e < 2_i32 {
		return Err ("Grid must be at least 2×2".into ());
	}
	Ok (())
}

fn calc_grid (grid: & Grid <Tile>, start_pos: Pos, steps: u32) -> GenResult <u64> {
	let offsets = [ Dir::North, Dir::South, Dir::East, Dir::West ]
		.map (|dir| grid.offset (dir).unwrap ());
	let mut dests: Grid <bool> = Grid::new_range (grid.start (), grid.end ()) ?;
	dests.set (start_pos, true);
	for _ in 0 .. steps {
		let mut new_dests = Grid::new_range (grid.start (), grid.end ()) ?;
		for cur in grid.cursors () {
			if cur.get (grid) == Tile::Rock { continue }
			if offsets.into_iter ()
					.filter_map (|off| chk! (cur + off).ok ())
					.any (|next_cur| next_cur.get (& dests)) {
				new_dests.set (cur.pos (), true);
			}
		}
		dests = new_dests;
	}
	Ok (dests.values ().filter (|& val| val).count ().pan_u64 ())
}

fn part_two_test (input: & Input) -> GenResult <u64> {
	let grid = & input.grid;
	let start_pos =
		grid.iter ()
			.filter (|& (_, tile)| tile == Tile::Start)
			.map (|(pos, _)| pos)
			.exactly_one ()
			.ok_or ("Must have exactly one start position") ?;
	let steps = [
		(Pos::from (Dir::North), Pos::from (Dir::North)),
		(Pos::from (Dir::South), Pos::from (Dir::South)),
		(Pos::from (Dir::East), Pos::from (Dir::East)),
		(Pos::from (Dir::West), Pos::from (Dir::West)),
		(Pos::from (Dir::North), Pos::from (Dir::South) * (grid.size ().n - 1_i32)),
		(Pos::from (Dir::South), Pos::from (Dir::North) * (grid.size ().n - 1_i32)),
		(Pos::from (Dir::East), Pos::from (Dir::West) * (grid.size ().e - 1_i32)),
		(Pos::from (Dir::West), Pos::from (Dir::East) * (grid.size ().e - 1_i32)),
	].map (|(step, offset)| (step, grid.offset (offset).unwrap ()));
	let mut fronts = vec! [ (start_pos, grid.cursor (start_pos).unwrap ()) ];
	let mut fronts_temp = Vec::new ();
	let mut posns_0 = HashSet::from ([ start_pos ]);
	let mut posns_1 = HashSet::new ();
	let (mut nums_0, mut nums_1) = (1, 0);
	for _ in 0 .. input.params.num_steps_two {
		let posns_2 = posns_1;
		posns_1 = posns_0;
		posns_0 = HashSet::new ();
		(nums_1, nums_0) = (nums_0, nums_1);
		fronts_temp.clear ();
		for & (pos, cur) in & fronts {
			for (step, offset) in steps {
				let Ok (cur) = chk! (cur + offset) else { continue };
				if cur.get (grid) == Tile::Rock { continue };
				let pos = pos + step;
				if posns_1.contains (& pos) { continue }
				if posns_2.contains (& pos) { continue }
				if ! posns_0.insert (pos) { continue }
				fronts_temp.push ((pos, cur));
				nums_0 += 1;
			}
		}
		mem::swap (& mut fronts, & mut fronts_temp);
	}
	Ok (nums_0)
}
