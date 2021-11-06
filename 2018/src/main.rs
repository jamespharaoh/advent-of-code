use regex::Regex;
use std::env;
use std::error::Error;
use std::fs;
use std::process;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod shared;

use day1::*;
use day2::*;
use day3::*;
use day4::*;
use day5::*;
use day6::*;
use day7::*;

pub fn main () -> Result <(), Box <dyn Error>> {
	let args: Vec <String> = env::args ().collect ();
	let (day, part): (u64, u64);
	let command_name_re = Regex::new (r"^aoc2018-day(\d+)-part(\d+)$").unwrap ();
	if let Some (captures) = command_name_re.captures (& args [0]) {
		day = captures.get (1).unwrap ().as_str ().parse () ?;
		part = captures.get (2).unwrap ().as_str ().parse () ?;
	} else {
		if args.len () != 3 {
			println! ("Syntax: {} DAY PART", args [0]);
			process::exit (1);
		}
		day = args [1].parse () ?;
		part = args [2].parse () ?;
	}
	let input = fs::read_to_string (& format! ("input/day{}", day)) ?;
	match (day, part) {
		(1, 1) => aoc2018_day1_part1 (& input),
		(1, 2) => aoc2018_day1_part2 (& input),
		(2, 1) => aoc2018_day2_part1 (& input),
		(2, 2) => aoc2018_day2_part2 (& input),
		(3, 1) => aoc2018_day3_part1 (& input),
		(3, 2) => aoc2018_day3_part2 (& input),
		(4, 1) => aoc2018_day4_part1 (& input),
		(4, 2) => aoc2018_day4_part2 (& input),
		(5, 1) => aoc2018_day5_part1 (& input),
		(5, 2) => aoc2018_day5_part2 (& input),
		(6, 1) => aoc2018_day6_part1 (& input),
		(6, 2) => aoc2018_day6_part2 (& input),
		(7, 1) => aoc2018_day7_part1 (& input),
		(7, 2) => aoc2018_day7_part2 (& input),
		_ => panic! (),
	}
}
