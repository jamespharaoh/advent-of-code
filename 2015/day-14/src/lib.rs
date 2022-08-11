//! Advent of Code 2015: Day 14: Reindeer Olympics
//!
//! [https://adventofcode.com/2015/day/14](https://adventofcode.com/2015/day/14)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;

puzzle_info! {
	name = "Reindeer Olympics";
	year = 2015;
	day = 14;
	parse = |input| model::Input::parse (input);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}

pub mod logic {

	use super::*;
	use model::Input;
	use model::Reindeer;
	use nums::IntConv;

	pub fn part_one (input: & Input) -> GenResult <u64> {
		let max_dist =
			input.deers.iter ()
				.map (|deer| calc_distance (deer, input.race_time))
				.max ()
				.unwrap_or (0);
		Ok (max_dist)
	}

	pub fn part_two (input: & Input) -> GenResult <u64> {
		let mut iters = input.deers.iter ().map (iter_distance).collect::<Vec <_>> ();
		let mut dists = iter::repeat (0).take (input.deers.len ()).collect::<Vec <_>> ();
		let mut scores = iter::repeat (0).take (input.deers.len ()).collect::<Vec <_>> ();
		for _ in 0 .. input.race_time {
			dists.clear ();
			dists.extend (iters.iter_mut ().map (|iter| iter.next ().unwrap ()));
			let (idx, _) =
				dists.iter ().copied ()
					.enumerate ()
					.max_by_key (|& (_, dist)| dist)
					.unwrap ();
			scores [idx] += 1;
		}
		let best_score = scores.iter ().copied ().max ().unwrap ();
		Ok (best_score)
	}

	pub fn iter_distance (deer: & Reindeer) -> impl Iterator <Item = u64> + '_ {
		let mut flying = false;
		let mut time = 0;
		let mut dist = 0_u64;
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
			if flying { dist += deer.fly_speed.as_u64 (); }
			Some (dist)
		})
	}

	#[ must_use ]
	pub fn calc_distance (deer: & Reindeer, time: u32) -> u64 {
		iter_distance (deer).nth (time.as_usize ()).unwrap ()
	}

}

pub mod model {

	use super::*;

	pub struct Input {
		pub deers: Vec <Reindeer>,
		pub race_time: u32,
	}

	pub struct Reindeer {
		pub name: String,
		pub fly_speed: u32,
		pub fly_time: u32,
		pub rest_time: u32,
	}

	impl Input {
		pub fn parse (mut input: & [& str]) -> GenResult <Self> {
			let race_time = if let Some (race_time) = input [0].strip_prefix ("RACE_TIME=") {
				input = & input [ 1 .. ];
				race_time.parse () ?
			} else { 2503 };
			if race_time < 1 { Err ("Race time must be at least 1") ?; }
			let deers: Vec <_> =
				input.iter ()
					.enumerate ()
					.map (|(line_idx, line)|
						Parser::wrap (line, |parser| {
							let name = parser.word () ?.to_owned ();
							let fly_speed = parser.expect (" can fly ") ?.int () ?;
							if fly_speed < 1 { Err ("Fly speed must be at least 1") ?; }
							let fly_time = parser.expect (" km/s for ") ?.int () ?;
							if fly_time < 1 { Err ("Fly time must be at least 1") ?; }
							let rest_time = parser.expect (" seconds, but then must rest for ") ?
								.int () ?;
							if rest_time < 1 { Err ("Rest time must be at least 1") ?; }
							parser.expect (" seconds.") ?.end () ?;
							Ok (Reindeer { name, fly_speed, fly_time, rest_time })
						}).map_parse_err (|char_idx|
							format! ("Invalid input: line {}: col {}: {}",
								line_idx + 1, char_idx + 1, line)))
					.collect::<GenResult <_>> () ?;
			if deers.is_empty () { Err ("Must have at least one deer") ?; }
			Ok (Self { deers, race_time })
		}
	}

}

#[ cfg (test) ]
mod examples {

	use super::*;

	const EXAMPLE: & [& str] = & [
		"RACE_TIME=1000",
		"Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.",
		"Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.",
	];

	#[ test ]
	fn part_one () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("1120", puzzle.part_one (EXAMPLE));
	}

	#[ test ]
	fn part_two () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("689", puzzle.part_two (EXAMPLE));
	}

}
