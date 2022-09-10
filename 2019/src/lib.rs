#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;

pub use aoc_2019_day_01 as day_01;
pub use aoc_2019_day_02 as day_02;
pub use aoc_2019_day_03 as day_03;
pub use aoc_2019_day_04 as day_04;
pub use aoc_2019_day_05 as day_05;
pub use aoc_2019_day_06 as day_06;
pub use aoc_2019_day_07 as day_07;
pub use aoc_2019_day_08 as day_08;
pub use aoc_2019_day_09 as day_09;
pub use aoc_2019_day_10 as day_10;
pub use aoc_2019_day_11 as day_11;

#[ must_use ]
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
	]
}
