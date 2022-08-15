//! Advent of Code 2016: Day 4: Security Through Obscurity
//!
//! [https://adventofcode.com/2016/day/4](https://adventofcode.com/2016/day/4)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;

puzzle_info! {
	name = "Security Through Obscurity";
	year = 2016;
	day = 4;
	parse = |input| model::Input::parse (input);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}

pub mod logic {

	use super::*;
	use model::Input;
	use model::Room;
	use nums::IntConv;

	pub fn part_one (input: & Input) -> GenResult <u32> {
		let num_valid =
			input.rooms.iter ()
				.filter (|room| room_is_valid (room))
				.map (|room| room.sector)
				.sum ();
		Ok (num_valid)
	}

	pub fn part_two (input: & Input) -> GenResult <u32> {
		let sector =
			input.rooms.iter ()
				.filter (|room| room_is_valid (room))
				.map (|room| Ok::<_, GenError> ((
					room.sector,
					room.name.chars ().map (|ch| match ch {
						'-' => Ok (' '),
						'a' ..= 'z' => Ok ((
							(ch.as_u32 () - 'a'.as_u32 () + room.sector) % 26 + 'a'.as_u32 ()
						).as_char ()),
						_ => Err (format! ("Invalid char: {}", ch).into ()),
					}).collect::<GenResult <String>> () ?)))
				.filter_ok (|& (_, ref room)| room == "northpole object storage")
				.map_ok (|(sector, _)| sector)
				.next ()
				.ok_or ("No solution found") ? ?;
		Ok (sector)
	}

	fn room_is_valid (room: & Room) -> bool {
		let char_groups =
			room.name.chars ()
				.filter (|& ch| ch != '-')
				.sorted ()
				.group_by (|& ch| ch);
		let expected_checksum =
			char_groups.into_iter ()
				.map (|(ch, group)| (ch, group.count ()))
				.sorted_by_key (|& (ch, num)| (cmp::Reverse (num), ch))
				.take (5)
				.map (|(ch, _)| ch)
				.collect::<String> ();
		room.checksum == expected_checksum
	}

}

pub mod model {

	use super::*;

	#[ derive (Clone, Debug, Eq, PartialEq) ]
	pub struct Input {
		pub rooms: Vec <Room>,
	}

	#[ derive (Clone, Debug, Eq, PartialEq) ]
	pub struct Room {
		pub name: String,
		pub sector: u32,
		pub checksum: String,
	}

	impl Input {
		pub fn parse (input: & [& str]) -> GenResult <Self> {
			let rooms = input.iter ()
				.enumerate ()
				.map (|(line_idx, line)|
					Parser::wrap (line, |parser| {
						parser.set_word_pred (|ch| ch.is_ascii_lowercase ());
						let mut num_dashes =
							parser.rest ().chars ()
								.take_while (|& ch| ch.is_ascii_lowercase () || ch == '-')
								.filter (|& ch| ch == '-')
								.count ();
						if num_dashes < 1 { return Err (parser.err ()) }
						let mut name = String::new ();
						loop {
							let next = parser.expect_next () ?;
							if next == '-' {
								num_dashes -= 1;
								if num_dashes == 0 { break }
							}
							name.push (next);
						}
						let sector = parser.int () ?;
						let checksum = parser.expect ("[") ?.word () ?.to_owned ();
						parser.expect ("]") ?.end () ?;
						Ok (Room { name, sector, checksum })
					}).map_parse_err (|_, col_idx| format! (
						"Invalid input: line {}: col {}: {}", line_idx + 1, col_idx + 1, line)))
				.collect::<GenResult <_>> () ?;
			Ok (Self { rooms })
		}
	}

	#[ cfg (test) ]
	mod tests {

		use super::*;

		#[ test ]
		fn input_parse () {
			assert_err! ("Invalid input: line 1: col 1: abc[def]ghi]",
				Input::parse (& [ "abc[def]ghi]" ]));
		}

	}

}

#[ cfg (test) ]
mod examples {

	use super::*;

	const EXAMPLE: & [& str] = & [
		"aaaaa-bbb-z-y-x-123[abxyz]",
		"a-b-c-d-e-f-g-h-987[abcde]",
		"not-a-real-room-404[oarel]",
		"totally-real-room-200[decoy]",
	];

	#[ test ]
	fn part_one () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("1514", puzzle.part_one (EXAMPLE));
	}

	#[ test ]
	fn part_two () {
		let puzzle = puzzle_metadata ();
		assert_err! ("No solution found", puzzle.part_two (EXAMPLE));
	}

}
