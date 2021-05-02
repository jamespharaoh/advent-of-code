use intcode::Machine;
use intcode::RunResult;
use std::collections::HashMap;
use std::io;
use std::io::Write as _;
use std::fs;

mod intcode;

fn main () {
	let programme_source = fs::read_to_string ("input").unwrap ();
	let programme = intcode::from_str (& programme_source);
	let test_pos = |pos: Pos| {
		let mut machine = Machine::new (programme.clone ());
		machine.input (pos.x as i64);
		machine.input (pos.y as i64);
			match machine.run () {
				RunResult::Output (1) => true,
				RunResult::Output (0) => false,
				unexpected => panic! ("Unexpected result: {:?}", unexpected),
			}
	};

	let mut base_radius: u32 = 0;
	let mut candidates: Vec <Pos> = Vec::new ();
	let mut cache: HashMap <Pos, bool> = HashMap::new ();
	let mut cache_get = |pos| {
		* cache.entry (pos).or_insert_with (|| test_pos (pos))
	};

	while candidates.is_empty () {
		base_radius += 100;
		print! ("\rSearching up to {}...", base_radius);
		io::stdout ().flush ().unwrap ();

		for coarse_x in (0 .. base_radius).step_by (100) {
		for coarse_y in (0 .. base_radius).step_by (100) {
			let coarse_pos: Pos = (coarse_x, coarse_y).into ();
			if ! cache_get (coarse_pos.offset (99, 99)) { continue }

			for fine_x in (coarse_x .. coarse_x + 100).step_by (10) {
			for fine_y in (coarse_y .. coarse_y + 100).step_by (10) {
				let fine_pos: Pos = (fine_x, fine_y).into ();
				if ! cache_get (fine_pos.offset (9, 9)) { continue }

				for origin_x in fine_x .. fine_x + 10 {
				for origin_y in fine_y .. fine_y + 10 {
					let origin_pos: Pos = (origin_x, origin_y).into ();

					if ! cache_get (origin_pos.offset (9, 9).round (10)) { continue }
					if ! cache_get (origin_pos.offset (9, 99).round (10)) { continue }
					if ! cache_get (origin_pos.offset (99, 9).round (10)) { continue }
					if ! cache_get (origin_pos.offset (99, 99).round (10)) { continue }

					if ! cache_get (origin_pos.offset (0, 0)) { continue }
					if ! cache_get (origin_pos.offset (0, 99)) { continue }
					if ! cache_get (origin_pos.offset (99, 0)) { continue }
					if ! cache_get (origin_pos.offset (99, 99)) { continue }

					let mut num_affected: u64 = 0;
				'MEMBER:
					for member_x in origin_x .. origin_x + 100 {
					for member_y in origin_y .. origin_y + 100 {
						if cache_get (origin_pos.offset (member_x, member_y)) {
							num_affected += 1;
						} else {
							break 'MEMBER;
						}
					} }
					if num_affected == 10000 { candidates.push (origin_pos) }

				} }

			} }

		} }

	}
	print! (" done\n");

	candidates.sort_by_key (Pos::dist);

	let closest = candidates [0];
	println! ("Closest point: {},{}", closest.x, closest.y);

}

#[ derive (Clone, Copy, Debug, Eq, Hash, PartialEq) ]
struct Pos { x: u32, y: u32 }

impl Pos {
	fn offset (self, x_off: u32, y_off: u32) -> Pos {
		Pos { x: self.x + x_off, y: self.y + y_off }
	}
	fn dist (& self) -> u64 {
		self.x as u64 * self.x as u64 + self.y as u64 + self.y as u64
	}
	fn round (& self, val: u32) -> Pos {
		Pos { x: self.x - self.x % val, y: self.y - self.y % val }
	}
}

impl From <(u32, u32)> for Pos {
	fn from ((x, y): (u32, u32)) -> Pos {
		Pos { x, y }
	}
}
