use aoc_common::*;

puzzle_info! {
	name = "Not Quite Lisp";
	year = 2015;
	day = 1;
	part_one = |lines| logic::part_one (lines [0]);
	part_two = |lines| logic::part_two (lines [0]);
}

mod logic {

	use super::*;

	pub fn part_one (input: & str) -> GenResult <i64> {
		let input = model::parse_input (input) ?;
		Ok (
			input.iter ().copied ()
				.last ()
				.map (|(_, floor)| floor)
				.unwrap_or (0)
		)
	}

	pub fn part_two (input: & str) -> GenResult <usize> {
		let input = model::parse_input (input) ?;
		Ok (
			input.iter ().copied ()
				.filter_map (|(ch_idx, floor)| (floor < 0).then_some (ch_idx + 1))
				.next ()
				.ok_or_else (|| format! ("Never visited the basement")) ?
		)
	}

}

mod model {

	use super::*;

	pub type Input = Vec <(usize, i64)>;

	pub fn parse_input (input: & str) -> GenResult <Input> {
		Ok (
			input.chars ().enumerate ()
				.map (|(ch_idx, ch)| match ch {
					'(' => Ok ((ch_idx, 1)),
					')' => Ok ((ch_idx, -1)),
					_ => Err (format! ("Invalid character: char {}: {}", ch_idx + 1, ch)),
				})
				.scan (0, |floor, result|
					Some (result.map (|(pos, diff)| { * floor += diff; (pos, * floor) })))
				.collect::<Result <_, _>> () ?
		)
	}
}

#[ cfg (test) ]
mod examples {

	use super::*;

	#[ test ]
	fn part_one () -> GenResult <()> {
		assert_eq! (0, logic::part_one ("(())") ?);
		assert_eq! (0, logic::part_one ("()()") ?);
		assert_eq! (3, logic::part_one ("))(((((") ?);
		assert_eq! (-1, logic::part_one ("())") ?);
		assert_eq! (-1, logic::part_one ("))(") ?);
		assert_eq! (-3, logic::part_one (")))") ?);
		assert_eq! (-3, logic::part_one (")())())") ?);
		assert_eq! ("Invalid character: char 3: X",
			logic::part_one ("()X").unwrap_err ().to_string ());
		Ok (())
	}

	#[ test ]
	fn part_two () -> GenResult <()> {
		assert_eq! (1, logic::part_two (")") ?);
		assert_eq! (5, logic::part_two ("()())") ?);
		assert_err! ("Invalid character: char 3: X", logic::part_two ("()X"));
		assert_err! ("Never visited the basement", logic::part_two ("(()())"));
		Ok (())
	}

}
