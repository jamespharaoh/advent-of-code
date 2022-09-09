#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;

pub use aoc_2019_day_01 as day_01;
pub use aoc_2019_day_02 as day_02;
pub use aoc_2019_day_03 as day_03;
pub use aoc_2019_day_04 as day_04;
pub use aoc_2019_day_05 as day_05;
pub use aoc_2019_day_06 as day_06;

#[ must_use ]
pub fn puzzle_metadata () -> Vec <Box <dyn puzzle::Puzzle>> {
	vec! [
		day_01::puzzle_metadata (),
		day_02::puzzle_metadata (),
		day_03::puzzle_metadata (),
		day_04::puzzle_metadata (),
		day_05::puzzle_metadata (),
		day_06::puzzle_metadata (),
	]
}