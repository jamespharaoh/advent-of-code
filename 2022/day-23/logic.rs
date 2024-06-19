use super::*;

use input::Input;
use input::Tile;
use model::Dir;
use model::Pos;

pub fn part_one (input: & Input) -> GenResult <u64> {
	let mut state = State::build (input);
	for _ in 0_i32 .. 10_i32 {
		state.step ();
	}
	Ok (state.answer ())
}

pub fn part_two (input: & Input) -> GenResult <u64> {
	let mut state = State::build (input);
	let mut num_rounds = 0;
	loop {
		num_rounds += 1;
		let num_moves = state.step ();
		if num_moves == 0 { break }
	}
	Ok (num_rounds)
}

struct State {
	dirs: Vec <Dir>,
	elfs: Vec <Pos>,
}

impl State {

	fn build (input: & Input) -> Self {
		let dirs = vec! [ Dir::North, Dir::South, Dir::West, Dir::East ];
		let elfs =
			input.grid.iter ()
				.filter (|& (_, tile)| tile == Tile::Elf)
				.map (|(pos, _)| pos)
				.collect ();
		Self { dirs, elfs }
	}

	fn step (& mut self) -> usize {
		let mut proposed_posns: HashMap <Pos, usize> = HashMap::new ();
		let mut proposed_elfs = Vec::new ();
		let elf_posns: HashSet <Pos> = self.elfs.iter ().copied ().collect ();
		for (idx, & old_pos) in self.elfs.iter ().enumerate () {
			if ! old_pos.adjacent_8 ().into_iter ().any (|pos| elf_posns.contains (& pos)) {
				continue;
			}
			let mut new_pos = None;
			for & dir in & self.dirs {
				let new_posns = new_posns_for_dir (old_pos, dir);
				if ! new_posns.into_iter ().any (|new_pos| elf_posns.contains (& new_pos)) {
					new_pos = Some (new_posns [1]);
					break;
				}
			};
			let Some (new_pos) = new_pos else { continue };
			* proposed_posns.entry (new_pos).or_default () += 1;
			proposed_elfs.push ((idx, new_pos));
		}
		let mut num_moves = 0;
		for (idx, new_pos) in proposed_elfs {
			if proposed_posns [& new_pos] != 1 { continue }
			self.elfs [idx] = new_pos;
			num_moves += 1;
		}
		let dir = self.dirs.remove (0);
		self.dirs.push (dir);
		num_moves
	}

	fn answer (& self) -> u64 {
		let (min_n, max_n) = self.elfs.iter ().map (|pos| pos.n).min_max ().unwrap ();
		let (min_e, max_e) = self.elfs.iter ().map (|pos| pos.e).min_max ().unwrap ();
		let height = (max_n.pan_isize () - min_n.pan_isize () + 1).pan_u64 ();
		let width = (max_e.pan_isize () - min_e.pan_isize () + 1).pan_u64 ();
		height * width - self.elfs.len ().pan_u64 ()
	}

}

fn new_posns_for_dir (pos: Pos, dir: Dir) -> [Pos; 3] {
	match dir {
		Dir::North => [
			pos.north (1).unwrap ().west (1).unwrap (),
			pos.north (1).unwrap (),
			pos.north (1).unwrap ().east (1).unwrap (),
		],
		Dir::South => [
			pos.south (1).unwrap ().west (1).unwrap (),
			pos.south (1).unwrap (),
			pos.south (1).unwrap ().east (1).unwrap (),
		],
		Dir::West => [
			pos.west (1).unwrap ().north (1).unwrap (),
			pos.west (1).unwrap (),
			pos.west (1).unwrap ().south (1).unwrap (),
		],
		Dir::East => [
			pos.east (1).unwrap ().north (1).unwrap (),
			pos.east (1).unwrap (),
			pos.east (1).unwrap ().south (1).unwrap (),
		],
	}
}
