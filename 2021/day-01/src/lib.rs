use aoc_common::*;

puzzle! {
	name = "Sonar Sweep";
	year = 2021;
	day = 1;
	part_one = |lines| logic::part_one (lines);
	part_two = |lines| logic::part_two (lines);
}

mod logic {

	use super::*;

	pub fn part_one (lines: & [& str]) -> GenResult <u64> {
		let mut last = None;
		let mut num: u64 = 0;
		for line in lines {
			if line.trim ().is_empty () { continue }
			let cur: u64 = line.parse () ?;
			if let Some (last) = last {
				if last < cur {
					num += 1;
				}
			}
			last = Some (cur);
		}
		Ok (num)
	}

	pub fn part_two (lines: & [& str]) -> GenResult <u64> {
		let mut last_0 = None;
		let mut last_1 = None;
		let mut last_2 = None;
		let mut num: u64 = 0;
		for line in lines {
			if line.trim ().is_empty () { continue }
			let cur: u64 = line.parse () ?;
			if let Some (last_2) = last_2 {
				if last_2 < cur {
					num += 1;
				}
			}
			last_2 = last_1;
			last_1 = last_0;
			last_0 = Some (cur);
		}
		Ok (num)
	}

}

#[ cfg (test) ]
mod examples {

	use super::*;

	const EXAMPLE: & [& str] = & [
		"199", "200", "208", "210", "200", "207", "240", "269", "260", "263"
	];

	#[ test ]
	fn part_one () -> GenResult <()> {
		assert_eq! (7, logic::part_one (EXAMPLE) ?);
		Ok (())
	}

	#[ test ]
	fn part_two () -> GenResult <()> {
		assert_eq! (5, logic::part_two (EXAMPLE) ?);
		Ok (())
	}

}
