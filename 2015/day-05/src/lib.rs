//! Advent of Code 2015: Day 5: Doesn't He Have Intern-Elves For This?
//!
//! [https://adventofcode.com/2015/day/5](https://adventofcode.com/2015/day/5)

use aoc_common::*;

puzzle_info! {
	name = "Doesn't He Have Intern-Elves For This?";
	year = 2015;
	day = 5;
	parse = |input| model::parse_input (input);
	part_one = |input| Ok::<_, Infallible> (logic::part_one (& input));
	part_two = |input| Ok::<_, Infallible> (logic::part_two (& input));
}

mod logic {

	use super::*;
	use model::Input;

	pub fn part_one (input: & Input) -> usize {
		input.iter ()
			.filter (|line| is_nice_one (line))
			.count ()
	}

	pub fn part_two (input: & Input) -> usize {
		input.iter ()
			.filter (|line| is_nice_two (line))
			.count ()
	}

	fn is_nice_one (input: & str) -> bool {
		if input.chars ()
				.filter (|ch| ['a', 'e', 'i', 'o', 'u'].contains (ch))
				.count () < 3
			{ return false }
		if ! input.chars ()
				.tuple_windows::<(_, _)> ()
				.any (|(ch_0, ch_1)| ch_0 == ch_1)
			{ return false }
		if input.chars ()
				.tuple_windows::<(_, _)> ()
				.any (|chars|
					[ ('a', 'b'), ('c', 'd'), ('p', 'q'), ('x', 'y') ]
						.contains (& chars))
			{ return false }
		true
	}

	fn is_nice_two (input: & str) -> bool {
		if ! input.chars ()
				.tuple_windows::<(_, _)> ()
				.enumerate ()
				.any (|(idx, chars_0)|
					input.chars ().skip (idx + 2)
						.tuple_windows::<(_, _)> ()
						.any (|chars_1| chars_0 == chars_1))
			{ return false }
		if ! input.chars ()
				.tuple_windows::<(_, _, _)> ()
				.any (|(ch_0, _, ch_1)| ch_0 == ch_1)
			{ return false }
		true
	}

	#[ cfg (test) ]
	mod tests {

		use super::*;

		#[ test ]
		fn is_nice_one () -> GenResult <()> {
			assert_eq! (true, logic::is_nice_one ("ugknbfddgicrmopn"));
			assert_eq! (true, logic::is_nice_one ("aaa"));
			assert_eq! (false, logic::is_nice_one ("jchzalrnumimnmhp"));
			assert_eq! (false, logic::is_nice_one ("haegwjzuvuyypxyu"));
			assert_eq! (false, logic::is_nice_one ("dvszwmarrgswjxmb"));
			Ok (())
		}

		#[ test ]
		fn is_nice_two () -> GenResult <()> {
			assert_eq! (true, logic::is_nice_two ("qjhvhtzxzqqjkmpb"));
			assert_eq! (true, logic::is_nice_two ("xxyxx"));
			assert_eq! (false, logic::is_nice_two ("uurcxstgmygtbstg"));
			assert_eq! (false, logic::is_nice_two ("ieodomkazucvgmuy"));
			Ok (())
		}

	}

}

mod model {

	use super::*;

	#[ derive (Clone) ]
	pub struct Input (Vec <String>);

	impl Deref for Input {
		type Target = [String];
		fn deref (& self) -> & Self::Target { & self.0 }
	}

	pub fn parse_input (input: & [& str]) -> GenResult <Input> {
		Ok (
			input.iter ().all (|line| line.chars ()
					.all (|ch| char::is_ascii_lowercase (& ch)))
				.then_some (Input (input.iter ()
					.map (|& line| line.to_owned ()).collect ()))
				.ok_or ("Invalid input") ?
		)
	}

}

#[ cfg (test) ]
mod examples {

	use super::*;

	const EXAMPLE_ONE: & [& str] = & [
		"ugknbfddgicrmopn",
		"aaa",
		"jchzalrnumimnmhp",
		"haegwjzuvuyypxyu",
		"dvszwmarrgswjxmb",
	];

	const EXAMPLE_TWO: & [& str] = & [
		"qjhvhtzxzqqjkmpb",
		"xxyxx",
		"uurcxstgmygtbstg",
		"ieodomkazucvgmuy",
	];

	#[ test ]
	fn part_one () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("2", puzzle.part_one (EXAMPLE_ONE));
	}

	#[ test ]
	fn part_two () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("2", puzzle.part_two (EXAMPLE_TWO));
	}

}
