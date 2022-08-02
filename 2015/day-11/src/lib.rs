//! Advent of Code 2015: Day 11: Corporate Policy
//!
//! [https://adventofcode.com/2015/day/11](https://adventofcode.com/2015/day/11)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;

puzzle_info! {
	name = "Corporate Policy";
	year = 2015;
	day = 11;
	part_one = |input| logic::part_one (input [0]);
	part_two = |input| logic::part_two (input [0]);
}

pub mod logic {

	use super::*;
	use nums::IntConv;

	const ALL_CHARS: & [char] = & [
		'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
		's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
	];

	const DISALLOWED_CHARS: & [char] = & [ 'i', 'l', 'o' ];

	pub fn part_one (input: & str) -> GenResult <String> {
		let password = next_password (input) ?;
		Ok (password)
	}

	pub fn part_two (input: & str) -> GenResult <String> {
		let password_0 = next_password (input) ?;
		let password_1 = next_password (& password_0) ?;
		Ok (password_1)
	}

	pub fn next_password (input: & str) -> GenResult <String> {
		let disallowed_idxs =
			DISALLOWED_CHARS.iter ().copied ()
				.map (|ch| ALL_CHARS.iter ().copied ()
					.position (|other_ch| ch == other_ch)
					.unwrap ())
				.collect::<Vec <_>> ();
		let mut char_idxs: Vec <usize> =
			input.chars ()
				.map (|ch| ALL_CHARS.iter ().copied ()
					.position (|valid_ch| ch == valid_ch)
					.ok_or_else (|| format! ("Input contains invalid character: {}", ch).into ()))
				.collect::<GenResult <_>> () ?;
		fn idxs_to_chars (char_idxs: & [usize]) -> impl Iterator <Item = char> + '_ + Clone {
			char_idxs.iter ().copied ().map (|idx| ALL_CHARS [idx])
		}
		if let Some (idx) =
			char_idxs.iter ().copied ()
				.position (|idx| disallowed_idxs.contains (& idx)) {
			char_idxs [idx] += 1;
			char_idxs.iter_mut ().skip (idx + 1).for_each (|ch| * ch = 0);
		}
		let new_password = loop {
			for idx in (0 .. char_idxs.len ()).rev () {
				char_idxs [idx] += 1;
				if char_idxs [idx] < ALL_CHARS.len () && (idx < char_idxs.len () - 2
						|| find_pair (idxs_to_chars (& char_idxs [ .. char_idxs.len () - 2]))
							.is_some ())
					{ break }
				if disallowed_idxs.contains (& char_idxs [idx]) { char_idxs [idx] += 1; }
				char_idxs [idx] = 0;
			}
			if is_valid (idxs_to_chars (& char_idxs)) {
				break char_idxs.iter ().copied ()
					.map (|idx| ALL_CHARS [idx])
					.collect::<String> ();
			}
		};
		Ok (new_password)
	}

	pub fn is_valid <Inpt> (input: Inpt) -> bool
			where Inpt: Iterator <Item = char> + Clone {
		if ! input.clone ()
			.tuple_windows::<(_, _, _)> ()
			.any (|(a, b, c)| a.as_u32 () + 1 == b.as_u32 () && b.as_u32 () + 1 == c.as_u32 ())
			{ return false }
		if input.clone ()
			.any (|ch| DISALLOWED_CHARS.contains (& ch))
			{ return false }
		if input.fold ((0_u32, None, None), |(sum, last, prev), next|
			if last == Some (next) && prev != Some (next) {
				(sum + 1, None, Some (next))
			} else {
				(sum, Some (next), prev)
			}
		).0 < 2 { return false }
		true
	}

	pub fn find_pair <Inpt> (input: Inpt) -> Option <(usize, char)>
			where Inpt: Iterator <Item = char> + Clone {
		input
			.tuple_windows::<(_, _)> ()
			.filter (|& (a, b)| a == b)
			.map (|(a, _)| a)
			.enumerate ()
			.next ()
	}

	#[ cfg (test) ]
	mod tests {

		use super::*;

		#[ test ]
		fn is_valid () -> GenResult <()> {
			assert_eq! (false, logic::is_valid ("hijklmmn".chars ()));
			assert_eq! (false, logic::is_valid ("abbceffg".chars ()));
			assert_eq! (false, logic::is_valid ("abbcegjk".chars ()));
			Ok (())
		}

		#[ test ]
		fn next_password () -> GenResult <()> {
			assert_eq! ("abcdffaa", logic::next_password ("abcdefgh") ?);
			assert_eq! ("ghjaabcc", logic::next_password ("ghijklmn") ?);
			Ok (())
		}

	}

}
