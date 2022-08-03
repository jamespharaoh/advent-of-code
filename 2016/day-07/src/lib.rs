//! Advent of Code 2016: Day 7: Internet Protocol Version 7
//!
//! [https://adventofcode.com/2016/day/7](https://adventofcode.com/2016/day/7)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;

puzzle_info! {
	name = "Internet Protocol Version 7";
	year = 2016;
	day = 7;
	parse = |input| model::Input::parse (input);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}

pub mod logic {

	use super::*;
	use model::Input;

	pub fn part_one (input: & Input) -> GenResult <usize> {
		Ok (input.lines.iter ()
			.filter (|line|
				has_abba_unbracketed (line)
					&& ! has_abba_bracketed (line))
			.count ())
	}

	pub fn part_two (input: & Input) -> GenResult <usize> {
		Ok (input.lines.iter ()
			.filter (|line|
				iter_abas_unbracketed (line)
					.any (|(a, b)| has_bab_bracketed (line, a, b)))
			.count ())
	}

	fn has_abba_unbracketed (addr: & str) -> bool {
		addr.chars ()
			.scan (true, |state, ch| {
				Some (match ch {
					'[' => { * state = false; Some ('[') },
					']' => { * state = true; Some (']') },
					_ => (* state).then_some (ch),
				})
			})
			.flatten ()
			.tuple_windows::<(_, _, _, _)> ()
			.any (|(a, b, c, d)| a == d && b == c && a != b)
	}

	fn has_abba_bracketed (addr: & str) -> bool {
		addr.chars ()
			.scan (false, |state, ch| {
				Some (match ch {
					'[' => { * state = true; Some ('[') },
					']' => { * state = false; Some (']') },
					_ => (* state).then_some (ch),
				})
			})
			.flatten ()
			.tuple_windows::<(_, _, _, _)> ()
			.any (|(a, b, c, d)| a == d && b == c && a != b)
	}

	fn iter_abas_unbracketed (addr: & str) -> impl Iterator <Item = (char, char)> + '_ {
		addr.chars ()
			.scan (true, |state, ch| {
				Some (match ch {
					'[' => { * state = false; Some ('[') },
					']' => { * state = true; Some (']') },
					_ => (* state).then_some (ch),
				})
			})
			.flatten ()
			.tuple_windows::<(_, _, _)> ()
			.filter (|& (a, b, c)| a == c && a != b)
			.map (|(a, b, _)| (a, b))
	}

	fn has_bab_bracketed (addr: & str, a: char, b: char) -> bool {
		addr.chars ()
			.scan (false, |state, ch| {
				Some (match ch {
					'[' => { * state = true; Some ('[') },
					']' => { * state = false; Some (']') },
					_ => (* state).then_some (ch),
				})
			})
			.flatten ()
			.tuple_windows::<(_, _, _)> ()
			.any (|(a1, b1, c1)| a == b1 && b == a1 && b == c1)
	}

}

pub mod model {

	use super::*;

	pub struct Input <'dat> {
		pub lines: & 'dat [& 'dat str],
	}

	impl <'dat> Input <'dat> {
		pub fn parse (lines: & 'dat [& 'dat str]) -> GenResult <Self> {
			if lines.iter ().any (|line|
				line.chars ()
					.fold (Some (false), |state, ch|
						match (state, ch) {
							(Some (state), ('a' ..= 'z')) => Some (state),
							(Some (false), '[') => Some (true),
							(Some (true), ']') => Some (false),
							_ => None,
						})
					.is_none ()) {
				Err ("Invalid input") ?;
			}
			Ok (Self { lines })
		}
	}

}

#[ cfg (test) ]
mod examples {

	use super::*;

	const EXAMPLE: & [& str] = & [
		"abba[mnop]qrst",
		"abcd[bddb]xyyx",
		"aaaa[qwer]tyui",
		"ioxxoj[asdfgh]zxcvbn",
		"aba[bab]xyz",
		"xyx[xyx]xyx",
		"aaa[kek]eke",
		"zazbz[bzb]cdb",
	];

	#[ test ]
	fn part_one () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("2", puzzle.part_one (EXAMPLE));
	}

	#[ test ]
	fn part_two () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("3", puzzle.part_two (EXAMPLE));
	}

}
