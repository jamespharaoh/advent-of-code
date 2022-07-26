#![ doc (html_playground_url = "https://playground.example.com/") ]

use aoc_common::*;

pub use aoc_2015_day_01 as day_01;
pub use aoc_2015_day_02 as day_02;
pub use aoc_2015_day_03 as day_03;
pub use aoc_2015_day_04 as day_04;
pub use aoc_2015_day_05 as day_05;
pub use aoc_2015_day_06 as day_06;
pub use aoc_2015_day_07 as day_07;
pub use aoc_2015_day_08 as day_08;
pub use aoc_2015_day_09 as day_09;
pub use aoc_2015_day_10 as day_10;
pub use aoc_2015_day_11 as day_11;
pub use aoc_2015_day_12 as day_12;
pub use aoc_2015_day_13 as day_13;
pub use aoc_2015_day_14 as day_14;
pub use aoc_2015_day_15 as day_15;
pub use aoc_2015_day_16 as day_16;
pub use aoc_2015_day_17 as day_17;
pub use aoc_2015_day_18 as day_18;
pub use aoc_2015_day_19 as day_19;
pub use aoc_2015_day_20 as day_20;
pub use aoc_2015_day_21 as day_21;
pub use aoc_2015_day_22 as day_22;
pub use aoc_2015_day_23 as day_23;
pub use aoc_2015_day_24 as day_24;
pub use aoc_2015_day_25 as day_25;

pub fn puzzle_metadata () -> Vec <Box <dyn puzzle::Puzzle>> {
	vec! [
		day_01::puzzle_metadata (),
		day_02::puzzle_metadata (),
		day_03::puzzle_metadata (),
		day_04::puzzle_metadata (),
		day_05::puzzle_metadata (),
		day_06::puzzle_metadata (),
		day_07::puzzle_metadata (),
		day_08::puzzle_metadata (),
		day_09::puzzle_metadata (),
		day_10::puzzle_metadata (),
		day_11::puzzle_metadata (),
		day_12::puzzle_metadata (),
		day_13::puzzle_metadata (),
		day_14::puzzle_metadata (),
		day_15::puzzle_metadata (),
		day_16::puzzle_metadata (),
		day_17::puzzle_metadata (),
		day_18::puzzle_metadata (),
		day_19::puzzle_metadata (),
		day_20::puzzle_metadata (),
		day_21::puzzle_metadata (),
		day_22::puzzle_metadata (),
		day_23::puzzle_metadata (),
		day_24::puzzle_metadata (),
		day_25::puzzle_metadata (),
	]
}
