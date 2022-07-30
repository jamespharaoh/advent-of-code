//! Advent of Code 2015: Day 9: All in a Single Night
//!
//! [https://adventofcode.com/2015/day/9](https://adventofcode.com/2015/day/9)
//!
//! # Input
//!
//! Each line represents the distance between two locations in the form "$0 to $1 = $2". The first
//! two parameters are alphanumeric strings representing the locations, tand the third is a
//! integer representing the distance.

use aoc_common::*;

puzzle_info! {
	name = "All in a Single Night";
	year = 2015;
	day = 9;
	parse = |input| model::parse_input (input);
	part_one = |input| logic::part_one (input);
	part_two = |input| logic::part_two (input);
}

pub mod logic {

	use super::*;
	use model::Input;
	use model::Place;

	pub fn part_one (input: Input) -> GenResult <u32> {
		sanity_check (& input) ?;
		Ok (distances (& input).min ().unwrap ())
	}

	pub fn part_two (input: Input) -> GenResult <u32> {
		sanity_check (& input) ?;
		Ok (distances (& input).max ().unwrap ())
	}

	fn sanity_check (input: & Input) -> GenResult <()> {
		if input.len () > 60 { Err ("Refusing to handle more than 60 distances") ?; }
		let num_places =
			input.iter ()
				.flat_map (|(from, to, _)| [from, to])
				.sorted ()
				.dedup ()
				.count ();
		if num_places > 10 {
			Err ("Refusing to handle more than 10 places") ?;
		}
		if input.len () != num_places * (num_places - 1) / 2 {
			Err ("Wrong number of distances for given number of places") ?;
		}
		Ok (())
	}

	fn distances (input: & Input) -> impl Iterator <Item = u32> {
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
				.flat_map (|(place_0, place_1, _)| [ Rc::clone (place_0), Rc::clone (place_1) ])
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

pub mod model {

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
				if distance < 1 { Err ("Distance must be at least one") ?; }
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
	fn part_one () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("605", puzzle.part_one (EXAMPLE));
	}

	#[ test ]
	fn part_two () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("982", puzzle.part_two (EXAMPLE));
	}

}

