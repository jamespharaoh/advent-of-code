//! Logic for solving the puzzles

use super::*;

use input::Input;
use model::Cpu;
use model::Pos;
use model::Val;

pub fn part_one (input: & Input) -> GenResult <u32> {
	let mut lookup = Lookup::new (input.data.clone ());
	let mut num_affected: u32 = 0;
	let mut x_min = 0;
	let mut y_min = 0;
	for size in 0 .. 50 {
		let mut new_x_min = x_min;
		let mut new_y_min = y_min;
		while new_y_min < size * 2 + 1 - new_x_min {
			let pos = if new_y_min <= size {
				Pos::new (new_y_min, size)
			} else {
				Pos::new (size, size * 2 - new_y_min)
			};
			if ! lookup.get (pos) ? { new_y_min += 1; } else { break }
		}
		while new_x_min < size * 2 + 1 - new_y_min {
			let pos = if new_x_min <= size {
				Pos::new (size, new_x_min)
			} else {
				Pos::new (size * 2 - new_x_min, size)
			};
			if ! lookup.get (pos) ? { new_x_min += 1; } else { break }
		}
		let step_affected = size * 2 + 1 - new_y_min - new_x_min;
		if 0 < step_affected {
			y_min = new_y_min;
			x_min = new_x_min;
			num_affected += step_affected.as_u32 ();
		}
	}
	Ok (num_affected)
}

pub fn part_two (input: & Input) -> GenResult <u32> {
	let mut lookup = Lookup::new (input.data.clone ());
	let mut candidates: Vec <Pos> = Vec::new ();
	for base_radius in (100 .. 2_500).step_by (100) {

		#[ derive (Clone, Copy) ]
		enum Level { First, Second, Third }
		let mut todo: Vec <(Level, Pos)> = Vec::new ();

		for coarse_pos in
			iter::once (Pos::new (base_radius - 100, base_radius - 100))
				.chain ((0 .. base_radius - 100).step_by (100).map (|y| Pos::new (y, base_radius - 100)))
				.chain ((0 .. base_radius - 100).step_by (100).map (|x| Pos::new (base_radius - 100, x))) {

			if 5_000 < lookup.len () {
				return Err ("Giving up after too many executions".into ());
			}

			if ! lookup.get (coarse_pos + Pos::new (99, 99)) ? { continue }
			todo.push ((Level::First, coarse_pos));

		}

		while let Some ((level, todo_pos)) = todo.pop () {

			if 5_000 < lookup.len () {
				return Err ("Giving up after too many executions".into ());
			}

			let (size, step, incr, next_level) = match level {
				Level::First => (100, 25, 24, Some (Level::Second)),
				Level::Second => (25, 5, 4, Some (Level::Third)),
				Level::Third => (5, 1, 0, None),
			};

			for search_pos in
				(todo_pos.y .. todo_pos.y + size).step_by (step).flat_map (|y|
					(todo_pos.x .. todo_pos.x + size).step_by (step).map (move |x|
						Pos::new (y, x))) {

				if ! lookup.get (search_pos + Pos::new (99, incr)) ? { continue }
				if ! lookup.get (search_pos + Pos::new (incr, 99)) ? { continue }
				if ! lookup.get (search_pos + Pos::new (incr, incr)) ? { continue }
				if ! lookup.get (search_pos + Pos::new (99, 99)) ? { continue }

				if let Some (next_level) = next_level {
					todo.push ((next_level, search_pos));
				} else {
					candidates.push (search_pos);
				}

			}

		}

		if ! candidates.is_empty () { break }

	}

	let closest = candidates.iter ().copied ()
		.min_by_key (|pos| {
			let y = pos.y.unsigned_abs ().as_i32 ();
			let x = pos.x.unsigned_abs ().as_i32 ();
			y * y + x * x
		})
		.ok_or ("No solution found") ?;

	Ok (closest.x.as_u32 () * 10_000 + closest.y.as_u32 ())

}

struct Lookup {
	prog: Vec <Val>,
	cache: HashMap <Pos, bool>,
}

impl Lookup {
	fn new (prog: Vec <Val>) -> Self {
		let cache = HashMap::new ();
		Self { prog, cache }
	}
	fn len (& self) -> usize {
		self.cache.len ()
	}
	fn get (& mut self, pos: Pos) -> GenResult <bool> {
		if let Some (& val) = self.cache.get (& pos) { return Ok (val) }
		let mut cpu = Cpu::new (self.prog.clone ());
		cpu.set_max_ops (500);
		cpu.input (Val::from (pos.x));
		cpu.input (Val::from (pos.y));
		let val = match cpu.run ().output () ? {
			Some (1) => true,
			Some (0) => false,
			Some (other) => return Err (format! ("Invalid output: {other}").into ()),
			None => return Err ("Machine halted".into ()),
		};
		self.cache.insert (pos, val);
		Ok (val)
	}
}
