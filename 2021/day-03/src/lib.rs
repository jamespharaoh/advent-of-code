use aoc_common::*;

puzzle_info! {
	name = "Binary Diagnostic";
	year = 2021;
	day = 3;
	part_one = |lines| logic::part_one (lines);
	part_two = |lines| logic::part_two (lines);
}

mod logic {

	use super::*;
	use model::Mode;

	pub fn part_one (lines: & [& str]) -> GenResult <i64> {
		let mut sums: Vec <i64> = Vec::new ();
		for line in lines {
			if line.trim ().is_empty () { continue }
			let sums_temp = sums;
			let prev_sums = Iterator::chain (sums_temp.iter ().cloned (), iter::repeat (0));
			sums = Vec::new ();
			for (ch, sum) in iter::zip (line.chars (), prev_sums) {
				sums.push (sum + match ch {
					'0' => -1,
					'1' => 1,
					_ => Err (format! ("Invalid bit: {}", ch)) ?,
				});
			}
		}
		for sum in sums.iter ().cloned () {
			if sum == 0 {
				Err (format! ("Equal number of bits")) ?;
			}
		}
		let gamma_str: String = sums.iter ().map (|sum|
			match sum.signum () {
				-1 => '0',
				1 => '1',
				_ => unreachable! (),
			}
		).collect ();
		let gamma = i64::from_str_radix (& gamma_str, 2) ?;
		let epsilon_str: String = sums.iter ().map (|sum|
			match sum.signum () {
				-1 => '1',
				1 => '0',
				_ => unreachable! (),
			}
		).collect ();
		let epsilon = i64::from_str_radix (& epsilon_str, 2) ?;
		Ok (gamma * epsilon)
	}

	pub fn part_two (lines: & [& str]) -> GenResult <i64> {
		let oxygen = calc_rating (lines.to_vec (), Mode::Oxygen) ?;
		let co2 = calc_rating (lines.to_vec (), Mode::CO2) ?;
		Ok (oxygen * co2)
	}

	fn calc_rating (mut lines: Vec <& str>, mode: Mode) -> GenResult <i64> {
		let mut offset: usize = 0;
		while lines.len () > 1 {
			let mut sum: i64 = 0;
			for line in lines.iter () {
				sum += if line.chars ().skip (offset).next ().unwrap () == '1' { 1 } else { -1 };
			}
			let sel = if (sum < 0) == (mode == Mode::Oxygen) { '0' } else { '1' };
			lines = lines.into_iter ().filter (
				|line| line.chars ().skip (offset).next ().unwrap () == sel
			).collect ();
			offset += 1;
		}
		Ok (i64::from_str_radix (lines [0], 2) ?)
	}

}

mod model {

	#[ derive (PartialEq) ]
	pub enum Mode { Oxygen, CO2 }

}

#[ cfg (test) ]
mod examples {

	use super::*;

	const EXAMPLE: & [& str] = & [
		"00100", "11110", "10110", "10111", "10101", "01111",
		"00111", "11100", "10000", "11001", "00010", "01010",
	];

	#[ test ]
	fn part_one () -> GenResult <()> {
		assert_eq! (198, logic::part_one (EXAMPLE) ?);
		Ok (())
	}

	#[ test ]
	fn part_two () -> GenResult <()> {
		assert_eq! (230, logic::part_two (EXAMPLE) ?);
		Ok (())
	}

}
