use super::*;

use input::Input;
use model::Cube;
use model::Step;

const BOUND_50: Cube = Cube { x0: -50, y0: -50, z0: -50, x1: 51, y1: 51, z1: 51 };

pub fn part_one (input: & Input) -> GenResult <i64> {
	let steps: Vec <Step> = input.steps.iter ().copied ().map (Step::from).collect ();
	let steps = bound_steps (& steps, BOUND_50);
	calc_result (& steps)
}

pub fn part_two (input: & Input) -> GenResult <i64> {
	let steps: Vec <Step> = input.steps.iter ().copied ().map (Step::from).collect ();
	calc_result_with_splits (& steps, 10)
}

fn calc_result_with_splits (steps: & [Step], num_splits: i32) -> GenResult <i64> {
	let bound = steps.iter ().copied ().fold (Cube::ZERO, |bound, step| Cube {
		x0: cmp::min (bound.x0, step.cube.x0),
		x1: cmp::max (bound.x1, step.cube.x1),
		y0: cmp::min (bound.y0, step.cube.y0),
		y1: cmp::max (bound.y1, step.cube.y1),
		z0: cmp::min (bound.z0, step.cube.z0),
		z1: cmp::max (bound.z1, step.cube.z1),
	});
	let x_bounds: Vec <_> = (0_i32 .. num_splits)
		.map (|idx| Ok (Cube {
			x0: chk! (bound.x0 + (bound.x1 - bound.x0) * idx / num_splits) ?,
			x1: chk! (bound.x0 + (bound.x1 - bound.x0) * (idx + 1_i32) / num_splits) ?,
			.. bound
		}))
		.try_collect::<_, _, Overflow> () ?;
	let y_bounds: Vec <_> = (0_i32 .. num_splits)
		.map (|idx| Ok (Cube {
			y0: chk! (bound.y0 + (bound.y1 - bound.y0) * idx / num_splits) ?,
			y1: chk! (bound.y0 + (bound.y1 - bound.y0) * (idx + 1_i32) / num_splits) ?,
			.. bound
		}))
		.try_collect::<_, _, Overflow> () ?;
	let z_bounds: Vec <_> = (0_i32 .. num_splits)
		.map (|idx| Ok (Cube {
			z0: chk! (bound.z0 + (bound.z1 - bound.z0) * idx / num_splits) ?,
			z1: chk! (bound.z0 + (bound.z1 - bound.z0) * (idx + 1_i32) / num_splits) ?,
			.. bound
		}))
		.try_collect::<_, _, Overflow> () ?;
	let mut total = 0;
	for x_bound in x_bounds.iter ().copied () {
		let steps = bound_steps (steps, x_bound);
		for y_bound in y_bounds.iter ().copied () {
			let steps = bound_steps (& steps, y_bound);
			for z_bound in z_bounds.iter ().copied () {
				let steps = bound_steps (& steps, z_bound);
				total += calc_result (& steps) ?;
			}
		}
	}
	Ok (total)
}

fn calc_result (steps: & [Step]) -> GenResult <i64> {
	let mut core: Vec <(Cube, bool)> = Vec::new ();
	let mut buffer: Vec <(Cube, bool)> = Vec::new ();
	for step in steps.iter () {
		if step.state {
			buffer.push ((step.cube, true));
		}
		for (core_cube, core_state) in core.iter_vals () {
			if let Some (intersect) = core_cube.intersect (step.cube) {
				buffer.push ((intersect, ! core_state));
			}
		}
		core.append (& mut buffer);
		if 100_000 < core.len () {
			return Err ("Giving up because of too many cubes to track".into ());
		}
	}
	Ok (
		core.iter_vals ()
			.map (|(cube, state)| cube.volume () * if state { 1 } else { -1 })
			.sum::<i64> ()
	)
}

fn bound_steps (steps: & [Step], bound: Cube) -> Vec <Step> {
	steps.iter ().copied ().filter_map (
		|mut step| step.cube.intersect (bound).map (|cube| { step.cube = cube; step }),
	).collect ()
}
