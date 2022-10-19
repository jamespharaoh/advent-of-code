//! Logic for solving the puzzles

use super::*;

use input::Input;
use model::Coord;
use model::Pos;

pub fn part_one (input: & Input) -> GenResult <u32> {
	let mut remain: Vec <Pos> = input.coords.clone ();
	let mut remain_temp = Vec::new ();
	let mut constellations: Vec <Vec <Pos>> = Vec::new ();
	while let Some (first_pos) = remain.pop () {
		let mut todo = Vec::new ();
		todo.push (first_pos);
		let mut constellation = Vec::new ();
		while let Some (pos_0) = todo.pop () {
			constellation.push (pos_0);
			for pos_1 in remain.drain ( .. ) {
				if get_dist (pos_0, pos_1) ? <= 3 {
					todo.push (pos_1);
				} else {
					remain_temp.push (pos_1);
				}
			}
			mem::swap (& mut remain, & mut remain_temp);
		}
		constellations.push (constellation);
	}
	Ok (constellations.len ().pan_u32 ())
}

fn get_dist (pos_0: Pos, pos_1: Pos) -> NumResult <Coord> {
	chk! (chk! (pos_0.x - pos_1.x) ?.abs (),
		+ chk! (pos_0.y - pos_1.y) ?.abs (),
		+ chk! (pos_0.z - pos_1.z) ?.abs (),
		+ chk! (pos_0.t - pos_1.t) ?.abs ())
}
