use super::*;

use input::Input;
use model::Dir;
use model::Pos;

pub fn part_one (input: & Input) -> GenResult <String> {
	Ok (
		routes_iter (input)
			.next ()
			.ok_or ("No solution found") ?
	)
}

pub fn part_two (input: & Input) -> GenResult <u32> {
	Ok (
		routes_iter (input)
			.last ()
			.ok_or ("No solution found") ?
			.chars ()
			.count ()
			.pan_u32 ()
	)
}

fn routes_iter (input: & Input) -> RoutesIter {
	RoutesIter::new (
		Pos::ZERO,
		Pos::new (3, 3),
		Pos::new (4, 4),
		input.passcode.to_string ())
}

struct RoutesIter {
	todo: VecDeque <(Pos, String)>,
	end: Pos,
	size: Pos,
	passcode: String,
}

impl RoutesIter {
	fn new (start: Pos, end: Pos, size: Pos, passcode: String) -> Self {
		let mut todo = VecDeque::new ();
		todo.push_back ((start, "".to_owned ()));
		Self { todo, end, size, passcode }
	}
}

impl Iterator for RoutesIter {

	type Item = String;

	fn next (& mut self) -> Option <String> {
		while let Some ((pos, route)) = self.todo.pop_front () {
			if pos == self.end { return Some (route) }
			let hash = md5::md5_hash (format! ("{}{}", self.passcode, route).as_bytes ());
			let hash_hex = hash.as_hex_bytes ();
			for (dir_idx, dir_tag, dir) in [
				(0, "U", Dir::Up), (1, "D", Dir::Down),
				(2, "L", Dir::Left), (3, "R", Dir::Right),
			] {
				let adj_pos = ok_or! (pos.try_add ((dir, 1)), continue);
				if adj_pos.y >= self.size.y || adj_pos.x >= self.size.x { continue }
				if hash_hex [dir_idx] > b'a' {
					self.todo.push_back ((adj_pos, format! ("{}{}", route, dir_tag)));
				}
			}
		}
		None
	}
}
