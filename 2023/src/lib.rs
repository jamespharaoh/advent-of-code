#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;

pub use aoc_2023_day_01 as day_01;

#[ must_use ]
pub fn puzzle_metadata () -> Vec <Box <dyn Puzzle>> {
	vec! [
		day_01::puzzle_metadata (),
	]
}
