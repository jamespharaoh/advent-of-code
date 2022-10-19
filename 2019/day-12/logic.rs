//! Logic for solving the puzzles

use super::*;

use input::Input;
use model::Axis;
use model::Energy;
use model::Moon;
use model::MoonAxis;
use model::Pos;

pub fn part_one (input: & Input) -> GenResult <Energy> {
	let mut moons: Vec <Moon> =
		input.moons.iter ()
			.map (|& pos| Moon::new (pos))
			.collect ();
	for _ in 0 .. input.params.num_steps_one {
		tick (& mut moons) ?;
	}
	Ok (moons.iter ().map (Moon::total_energy).sum ())
}

pub fn part_two (input: & Input) -> GenResult <u64> {
	let repeat_x = calc_axis_repeat (input, Axis::X) ?;
	let repeat_y = calc_axis_repeat (input, Axis::Y) ?;
	let repeat_z = calc_axis_repeat (input, Axis::Z) ?;
	if repeat_x.0 != 0 || repeat_y.0 != 0 || repeat_z.0 != 0 { panic! () }
	let mut result = 1_u64;
	result = u64::lcm (result, repeat_x.1);
	result = u64::lcm (result, repeat_y.1);
	result = u64::lcm (result, repeat_z.1);
	Ok (result)
}

fn calc_axis_repeat (input: & Input, axis: Axis) -> GenResult <(u64, u64)> {
	let mut moons: Vec <MoonAxis> =
		input.moons.iter ()
			.map (|moon| MoonAxis { pos: moon [axis], vel: 0 })
			.collect ();
	let mut tick: u64 = 0;
	let mut seen: HashMap <Vec <MoonAxis>, u64> = HashMap::new ();
	for _ in 0 .. input.params.num_steps_two {
		if let Some (prev) = seen.insert (moons.clone (), tick) {
			return Ok ((prev, tick));
		}
		for idx_0 in 0 .. moons.len () {
			for idx_1 in 0 .. moons.len () {
				if idx_0 == idx_1 { continue }
				let moon_0 = & moons [idx_0];
				let moon_1 = & moons [idx_1];
				moons [idx_0].vel =
					chk! (moon_0.vel + chk! (moon_1.pos - moon_0.pos) ?.signum ()) ?;
			};
		}
		for moon in moons.iter_mut () {
			moon.pos = chk! (moon.pos + moon.vel) ?;
		}
		tick += 1;
	}
	Err ("Max steps exceeded".into ())
}

fn tick (moons: & mut Vec <Moon>) -> GenResult <()> {
	for idx_0 in 0 .. moons.len () {
		for idx_1 in 0 .. moons.len () {
			if idx_0 == idx_1 { continue }
			let moon_0 = & moons [idx_0];
			let moon_1 = & moons [idx_1];
			moons [idx_0].vel = moon_0.vel + Pos {
				x: (moon_1.pos.x - moon_0.pos.x).signum (),
				y: (moon_1.pos.y - moon_0.pos.y).signum (),
				z: (moon_1.pos.z - moon_0.pos.z).signum (),
			};
		}
	}
	for moon in moons.iter_mut () {
		moon.pos = moon.pos.try_add (moon.vel) ?;
	}
	Ok (())
}
