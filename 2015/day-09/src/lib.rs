//! Advent of Code 2015: Day 9: All in a Single Night
//!
//! [https://adventofcode.com/2015/day/9](https://adventofcode.com/2015/day/9)

use aoc_common::*;

puzzle_info! {
	name = "All in a Single Night";
	year = 2015;
	day = 9;
	part_one = |input| logic::part_one (input);
	part_two = |input| logic::part_two (input);
}

mod logic {

	use super::*;
	use model::Input;
	use model::Place;

	pub fn part_one (input: & [& str]) -> GenResult <u32> {
		let input = model::parse_input (input) ?;
		Ok (distances (& input).min ().unwrap ())
	}

	pub fn part_two (input: & [& str]) -> GenResult <u32> {
		let input = model::parse_input (input) ?;
		Ok (distances (& input).max ().unwrap ())
	}

	pub fn distances (input: & Input) -> impl Iterator <Item = u32> {
		let (num_places, distances) = gen_dist_table (input);
		let mut route = Vec::new ();
		let mut used = iter::repeat (false).take (num_places).collect::<Vec <_>> ();
		let mut finished = false;
		iter::from_fn (move || {
			if finished { return None }
			// find the next free place
			#[ inline ]
			fn next (used: & [bool], start: usize) -> Option <usize> {
				used.iter ().copied ().enumerate ()
					.skip (start)
					.filter (|& (_, used)| ! used)
					.map (|(idx, _)| idx)
					.next ()
			}
			// find a place index we can increment (except first time)
			if ! route.is_empty () {
				loop {
					if let Some (val) = route.pop () {
						used [val] = false;
						if let Some (val) = next (& used, val + 1) {
							route.push (val);
							used [val] = true;
							break;
						}
					} else { finished = true; return None }
				}
			}
			// fill in the rest with the minimum possible
			while route.len () < num_places {
				let val = next (& used, 0).unwrap ();
				route.push (val);
				used [val] = true;
			}
			// return
			Some (
				route.iter ()
					.tuple_windows::<(_, _)> ()
					.map (|(place_0, place_1)| distances [place_0 * num_places + place_1])
					.sum::<u32> ()
			)
		})
	}

	fn gen_dist_table (input: & Input) -> (usize, Vec <u32>) {
		let place_indexes: HashMap <Place, usize> =
			input.iter ()
				.flat_map (|(place_0, place_1, _)| [ place_0.clone (), place_1.clone () ])
				.sorted ()
				.dedup ()
				.enumerate ()
				.map (|(idx, place)| (place, idx))
				.collect ();
		let num_places = place_indexes.len ();
		let mut distances = iter::repeat (0).take (num_places * num_places).collect::<Vec <_>> ();
		for & (ref place_0, ref place_1, dist) in input {
			let place_0 = place_indexes [place_0];
			let place_1 = place_indexes [place_1];
			distances [place_0 * num_places + place_1] = dist;
			distances [place_1 * num_places + place_0] = dist;
		}
		(num_places, distances)
	}

}

mod model {

	use super::*;

	pub type Place = Rc <str>;
	pub type Input = Vec <(Place, Place, u32)>;

	pub fn parse_input (input: & [& str]) -> GenResult <Input> {
		use parser::*;
		input.iter ().enumerate ().map (|(line_idx, line)|
			Parser::wrap (line, |parser| {
				parser.set_ignore_whitespace (true);
				let place_0 = parser.word () ?;
				let place_1 = parser.expect_word ("to") ?.word () ?;
				let distance = parser.expect_word ("=") ?.int () ?;
				parser.end () ?;
				Ok ((place_0.into (), place_1.into (), distance))
			}).map_parse_err (|col_idx|
				format! ("Invalid input: line {}: col {}: {}", line_idx + 1, col_idx + 1, line)
			)
		).collect ()
	}

}

#[ cfg (test) ]
mod examples {

	use super::*;

	const EXAMPLE: & [& str] = & [
		"London to Dublin = 464",
		"London to Belfast = 518",
		"Dublin to Belfast = 141",
	];

	#[ test ]
	fn part_one () -> GenResult <()> {
		assert_eq! (605, logic::part_one (EXAMPLE) ?);
		Ok (())
	}

	#[ test ]
	fn part_two () -> GenResult <()> {
		assert_eq! (982, logic::part_two (EXAMPLE) ?);
		Ok (())
	}

}

