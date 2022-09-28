//! Logic for solving the puzzles

use super::*;
use input::Input;
use model::Coord;
use model::Dir;
use model::DirVec;
use model::Pos;
use model::Space;

pub fn part_one (input: & Input) -> GenResult <u32> {
	let (_, num) = best_roid (& get_roids (input)).ok_or ("No solution found") ?;
	Ok (num)
}

pub fn part_two (input: & Input) -> GenResult <Coord> {
	let roids = get_roids (input);
	let (base, _) = best_roid (& roids).ok_or ("No solution found") ?;
	let mut roid_dir_vecs: Vec <DirVec> =
		roids.iter ().copied ()
			.filter (|& roid| roid != base)
			.map (|roid| DirVec::new (roid - base))
			.sorted ()
			.collect ();
	let mut roid_dir_vecs_temp = Vec::new ();
	let mut num_roids = 0_u32;
	loop {
		let mut last_dir = None;
		for roid_dir_vec in roid_dir_vecs.drain ( .. ) {
			if Some (roid_dir_vec.dir) == last_dir {
				roid_dir_vecs_temp.push (roid_dir_vec);
				continue;
			}
			last_dir = Some (roid_dir_vec.dir);
			num_roids += 1;
			if num_roids == 200 {
				let roid_pos = base + roid_dir_vec.pos ();
				return Ok (roid_pos.x * 100 + roid_pos.y);
			}
		}
		mem::swap (& mut roid_dir_vecs, & mut roid_dir_vecs_temp);
		if roid_dir_vecs.is_empty () {
			return Err ("No solution found".into ())
		}
	}
}

fn best_roid (roids: & [Pos]) -> Option <(Pos, u32)> {
	roids.iter ()
		.map (|& roid| (roid, num_visible (roids, roid)))
		.max_by_key (|& (_, num)| num)
}

fn get_roids (input: & Input) -> Vec <Pos> {
	input.grid.iter ()
		.filter (|& (_, space)| matches! (space, Space::Asteroid))
		.map (|(pos, _)| pos)
		.collect ()
}

fn num_visible (roids: & [Pos], base: Pos) -> u32 {
	roids.iter ().copied ()
		.filter (|& other| other != base)
		.map (|other| DirVec::new (other - base).dir)
		.collect::<HashSet <Dir>> ()
		.len ()
		.pan_u32 ()
}
