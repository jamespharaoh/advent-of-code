use super::*;
use input::Input;

pub fn part_one (input: & Input) -> GenResult <u32> {
	let layers = analyse (input) ?;
	Ok (
		layers.iter ()
			.filter (|layer| layer.offset % layer.period == 0)
			.map (|layer| layer.score)
			.sum ()
	)
}

pub fn part_two (input: & Input) -> GenResult <u32> {

	let layers = analyse (input) ?;

	let mut layer_idx = 0_usize;
	let mut repeat: Vec <u32> = vec! [ 0 ];
	let mut period = 1;

	// iterate over time periods the size of the repeating pattern

	let mut base_time = 0_u32;
	while layer_idx < layers.len () {

		// add another layer to the repeating pattern if possible

		let layer = & layers [layer_idx];
		if layer.offset < base_time && base_time % layer.period == 0 {

			// update the repeating pattern to include this layer

			let mul = layer.period / divisor (period, layer.period);
			repeat = (0 .. )
				.step_by (period.pan_usize ())
				.take (mul.pan_usize ())
				.cartesian_product (repeat)
				.map (|(incr, time)| incr + time)
				.filter (|time| (time + layer.offset) % layer.period != 0)
				.collect ();
			period *= mul;
			layer_idx += 1;

			if repeat.len () > 1000 {
				return Err ("Max 1000 elements in repeating pattern".into ());
			}

		}

		// check the pattern for times which also match inactive layers

		for offset in repeat.iter_vals () {
			let time = u32::add_2 (base_time, offset) ?;
			if layers.iter ().skip (layer_idx)
					.all (|layer| (time + layer.offset) % layer.period != 0) {
				return Ok (time)
			}
		}

		// advance time period for next application of repeating pattern

		base_time = u32::add_2 (base_time, period) ?;

	}

	Err ("No solution found".into ())

}

fn divisor (left: u32, right: u32) -> u32 {
	(2 ..= cmp::min (left, right)).rev ()
		.find (|div| left % div == 0 && right % div == 0)
		.unwrap_or (1)
}

fn analyse (input: & Input) -> GenResult <Vec <LayerInfo>> {
	if input.layers.iter ().any (|layer| layer.range < 2) {
		return Err ("Layer range must be at least two".into ());
	}
	let layers =
		input.layers.iter ().cloned ()
			.map (|layer| LayerInfo {
				offset: layer.depth.pan_u32 (),
				period: (layer.range.pan_u32 () - 1) * 2,
				score: layer.depth.pan_u32 () * layer.range.pan_u32 (),
			})
			.sorted_by_key (|layer| layer.offset)
			.collect::<Vec <LayerInfo>> ();
	Ok (layers)
}

#[ derive (Clone, Copy, Debug) ]
struct LayerInfo {
	offset: u32,
	period: u32,
	score: u32,
}
