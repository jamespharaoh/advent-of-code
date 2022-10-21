use super::*;

use input::Input;
use model::Reindeer;

pub fn part_one (input: & Input) -> GenResult <u64> {
	let max_dist =
		input.deers.iter ()
			.map (|deer| iter_distance (deer)
				.nth (input.params.race_time.pan_usize () - 1)
				.unwrap ())
			.max ()
			.unwrap_or (0);
	Ok (max_dist)
}

pub fn part_two (input: & Input) -> GenResult <u64> {
	let mut iters: Vec <_> =
		input.deers.iter ()
			.map (iter_distance)
			.collect ();
	let mut scores = vec! [0; input.deers.len ()];
	let mut dists = Vec::new ();
	for _ in 0 .. input.params.race_time {
		dists.clear ();
		dists.extend (iters.iter_mut ().map (|iter| iter.next ().unwrap ()));
		let max_dist = dists.iter ().copied ().max ().unwrap ();
		for (_, score) in dists.iter ().zip (scores.iter_mut ())
				.filter (|& (& dist, _)| dist == max_dist) {
			* score += 1;
		}
	}
	Ok (scores.iter ().copied ().max ().unwrap ())
}

fn iter_distance <'inp> (deer: & 'inp Reindeer <'inp>) -> impl Iterator <Item = u64> + 'inp {
	let mut flying = false;
	let mut time = 0;
	let mut dist = 0_u64;
	iter::from_fn (move || {
		if time == 0 {
			if flying { time = deer.rest_time; flying = false; }
			else { time = deer.fly_time; flying = true; }
		}
		time -= 1;
		if flying { dist += deer.fly_speed.pan_u64 (); }
		Some (dist)
	})
}
