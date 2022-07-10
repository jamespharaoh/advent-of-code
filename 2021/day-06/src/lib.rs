use aoc_common::*;

puzzle! {
	name = "Lanternfish";
	year = 2021;
	day = 6;
	part_one = |lines| logic::calc_result_part_one (lines [0]);
	part_two = |lines| logic::calc_result_part_two (lines [0]);
}

mod logic {

	use super::*;

	pub fn calc_result_part_one (input: & str) -> GenResult <u64> {
		calc_result (input, 80)
	}

	pub fn calc_result_part_two (input: & str) -> GenResult <u64> {
		calc_result (input,256)
	}

	pub fn calc_result (input: & str, days: u64) -> GenResult <u64> {
		let mut fishes: [u64; 9] = [0; 9];
		for fish_str in input.split (",") {
			let fish: usize = fish_str.parse () ?;
			fishes [fish] += 1;
		}
		for _ in 0 .. days {
			fishes = [
				fishes [1],
				fishes [2],
				fishes [3],
				fishes [4],
				fishes [5],
				fishes [6],
				fishes [7] + fishes [0],
				fishes [8],
				fishes [0],
			];
		}
		Ok (fishes.into_iter ().sum::<u64> () as u64)
	}

}

#[ cfg (test) ]
mod examples {

	use super::*;

	const EXAMPLE: & str = "3,4,3,1,2";

	#[ test ]
	fn part_one () -> GenResult <()> {
		assert_eq! (5934, logic::calc_result_part_one (EXAMPLE) ?);
		Ok (())
	}

	#[ test ]
	fn part_two () -> GenResult <()> {
		assert_eq! (26984457539, logic::calc_result_part_two (EXAMPLE) ?);
		Ok (())
	}

}
