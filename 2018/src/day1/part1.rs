use std::error::Error;

pub fn aoc2018_day1_part1 (input: & str) -> Result <String, Box <dyn Error>> {
	let mut total: i64 = 0;
	for line in input.trim ().split ("\n") {
		let delta: i64 = line.parse () ?;
		total += delta;
	}
	Ok (format! ("{}", total))
}
