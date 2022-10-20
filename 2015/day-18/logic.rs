//! Logic for solving the puzzles.

use super::*;

use input::Input;
use model::Grid;
use model::Light;
use model::Pos;

pub fn part_one (input: & Input) -> GenResult <u32> {
	calc_result (input, |_| ())
}

pub fn part_two (input: & Input) -> GenResult <u32> {
	calc_result (input, |lights| {
		lights.set (Pos::new (lights.first_key ().y, lights.first_key ().x), Light::On);
		lights.set (Pos::new (lights.first_key ().y, lights.last_key ().x), Light::On);
		lights.set (Pos::new (lights.last_key ().y, lights.first_key ().x), Light::On);
		lights.set (Pos::new (lights.last_key ().y, lights.last_key ().x), Light::On);
	})
}

fn calc_result (
	input: & Input,
	override_fn: fn (& mut Grid),
) -> GenResult <u32> {

	if input.grid.size ().x < 2 || input.grid.size ().y < 2 {
		return Err ("Minimum grid size is 2×2".into ());
	}

	let mut lights = input.grid.clone ();

	let offsets: Vec <_> =
		Pos::ZERO.adjacent_8 ().iter ()
			.map (|& pos| lights.offset (pos).unwrap ())
			.collect ();

	override_fn (& mut lights);

	// apply rules specified number of times

	for _ in 0 .. input.params.num_steps {
		lights = lights.map (|cur| {
			let num_adjacent =
				offsets.iter ()
					.filter_map (|& off| chk! (cur + off).ok ())
					.map (|cur| cur.get (& lights))
					.filter (|& light| matches! (light, Light::On))
					.count ();
			let val = cur.get (& lights);
			let on = matches! ((val, num_adjacent), (Light::On, 2) | (_, 3));
			if on { Light::On } else { Light::Off }
		});
		override_fn (& mut lights);
	}

	// count active lights

	Ok (
		lights.values ()
			.filter (|& val| matches! (val, Light::On))
			.count ()
			.pan_u32 ()
	)

}
