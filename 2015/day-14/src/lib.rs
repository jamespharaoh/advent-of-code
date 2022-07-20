//! Advent of Code 2015: Day 14: Reindeer Olympics
//!
//! [https://adventofcode.com/2015/day/14](https://adventofcode.com/2015/day/14)

use aoc_common::*;

puzzle_info! {
	name = "Reindeer Olympics";
	year = 2015;
	day = 14;
	part_one = |input| logic::part_one (input, 2503);
	part_two = |input| logic::part_two (input, 2503);
}

mod logic {

	use super::*;
	use model::Reindeer;

	pub fn part_one (input: & [& str], time: u32) -> GenResult <u32> {
		let deers = model::parse_input (input) ?;
		let max_dist =
			deers.iter ()
				.map (|deer| calc_distance (deer, time))
				.max ()
				.unwrap_or (0);
		Ok (max_dist)
	}

	pub fn part_two (input: & [& str], time: u32) -> GenResult <u32> {
		let deers = model::parse_input (input) ?;
		let mut iters = deers.iter ().map (iter_distance).collect::<Vec <_>> ();
		let mut dists = iter::repeat (0).take (input.len ()).collect::<Vec <_>> ();
		let mut scores = iter::repeat (0).take (input.len ()).collect::<Vec <_>> ();
		for _ in 0 .. time {
			dists.clear ();
			dists.extend (iters.iter_mut ().map (|iter| iter.next ().unwrap ()));
			let (idx, _) = dists.iter ().copied ().enumerate ()
				.max_by_key (|& (_, dist)| dist)
				.unwrap ();
			scores [idx] += 1;
		}
		let best_score = scores.iter ().copied ().max ().unwrap ();
		Ok (best_score)
	}

	pub fn iter_distance (deer: & Reindeer) -> impl Iterator <Item = u32> + '_ {
		let mut flying = false;
		let mut time = 0;
		let mut dist = 0;
		iter::from_fn (move || {
			if time == 0 {
				if flying {
					time = deer.rest_time;
					flying = false;
				} else {
					time = deer.fly_time;
					flying = true;
				}
			}
			time -= 1;
			if flying { dist += deer.fly_speed; }
			Some (dist)
		})
	}

	pub fn calc_distance (deer: & Reindeer, time: u32) -> u32 {
		iter_distance (deer).nth (time as usize).unwrap ()
	}

}

mod model {

	use super::*;

	pub type Input = Vec <Reindeer>;

	pub struct Reindeer {
		pub name: String,
		pub fly_speed: u32,
		pub fly_time: u32,
		pub rest_time: u32,
	}

	pub fn parse_input (input: & [& str]) -> GenResult <Input> {
		use parser::*;
		input.iter ().enumerate ().map (|(line_idx, line)|
			Parser::wrap (line, |parser| {
				let name = parser.word () ?.to_string ();
				let fly_speed = parser.expect (" can fly ") ?.int () ?;
				let fly_time = parser.expect (" km/s for ") ?.int () ?;
				let rest_time = parser.expect (" seconds, but then must rest for ") ?.int () ?;
				parser.expect (" seconds.") ?.end () ?;
				Ok (Reindeer { name, fly_speed, fly_time, rest_time })
			}).map_parse_err (|char_idx|
				format! ("Invalid input: line {}: col {}: {}", line_idx + 1, char_idx + 1, line)
			)
		).collect ()
	}

}

#[ cfg (test) ]
mod examples {

	use super::*;

	const EXAMPLE: & [& str] = & [
		"Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.",
		"Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.",
	];

	#[ test ]
	fn part_one () -> GenResult <()> {
		assert_eq! (1120, logic::part_one (EXAMPLE, 1000) ?);
		Ok (())
	}

	#[ test ]
	fn part_two () -> GenResult <()> {
		assert_eq! (689, logic::part_two (EXAMPLE, 1000) ?);
		Ok (())
	}

}
