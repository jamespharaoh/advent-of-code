use super::*;

use input::Input;
use model::Pos;

pub fn part_one (input: & Input) -> GenResult <u64> {
	let mut state = State::build (input) ?;
	state.drop ();
	let mut num = 0;
	for idx in 0 .. state.len () {
		let mut state = state.remove (idx);
		if state.drop () == 0 { num += 1; }
	}
	Ok (num)
}

pub fn part_two (input: & Input) -> GenResult <u64> {
	let mut state = State::build (input) ?;
	state.drop ();
	let mut num = 0;
	for idx in 0 .. state.len () {
		let mut state = state.remove (idx);
		num += state.drop ();
	}
	Ok (num)
}

struct State {
	blocks: Vec <Vec <Pos>>,
	parts: HashSet <Pos>,
}

impl State {

	fn build (input: & Input) -> GenResult <Self> {
		for block in & input.blocks {
			if block.end.x < block.start.x || block.end.y < block.start.y || block.end.z < block.start.z {
				return Err (format! ("Invalid block: {block:?}").into ());
			}
			if 500 < block.end.x || 500 < block.end.y || 500 < block.end.z {
				return Err (format! ("Invalid block: {block:?}").into ());
			}
			if block.start.z < 1 {
				return Err (format! ("Invalid block: {block:?}").into ());
			} 
		}
		let mut blocks: Vec <Vec <Pos>> =
			input.blocks.iter ()
				.map (|block| Ok (match (
					block.start.x == block.end.x,
					block.start.y == block.end.y,
					block.start.z == block.end.z,
				) {
					(false, true, true) => (block.start.x ..= block.end.x)
						.map (|x| Pos::new (x, block.start.y, block.start.z))
						.collect::<Vec <Pos>> (),
					(true, false, true) => (block.start.y ..= block.end.y)
						.map (|y| Pos::new (block.start.x, y, block.start.z))
						.collect::<Vec <Pos>> (),
					(true, true, false) => (block.start.z ..= block.end.z)
						.map (|z| Pos::new (block.start.x, block.start.y, z))
						.collect::<Vec <Pos>> (),
					(true, true, true) => vec! [ block.start ],
					_ => return Err (format! ("Invalid block: {block:?}")),
				}))
				.try_collect () ?;
		blocks.sort_by_key (|block| block [0].z);
		let mut parts = HashSet::new ();
		for & pos in blocks.iter ().flatten () {
			if ! parts.insert (pos) {
				return Err (format! ("Overlapping blocks at {pos:?}").into ());
			}
		}
		Ok (Self { blocks, parts })
	}

	fn len (& self) -> usize {
		self.blocks.len ()
	}

	fn remove (& mut self, idx: usize) -> Self {
		let mut blocks = self.blocks.clone ();
		let block = blocks.remove (idx);
		let mut parts = self.parts.clone ();
		for & pos in & block { parts.remove (& pos); }
		Self { blocks, parts }
	}

	fn drop (& mut self) -> u64 {
		let mut num = 0;
		for idx in 0 .. self.blocks.len () {
			for & pos in & self.blocks [idx] { self.parts.remove (& pos); }
			let mut dropped = false;
			while self.blocks [idx].iter ()
					.map (|& pos| pos + Pos::new (0, 0, -1))
					.all (|pos| 0 < pos.z && ! self.parts.contains (& pos)) {
				for pos in & mut self.blocks [idx] { * pos += Pos::new (0, 0, -1); }
				dropped = true;
			}
			for & pos in & self.blocks [idx] { self.parts.insert (pos); }
			if dropped { num += 1; }
		}
		num
	}

}
