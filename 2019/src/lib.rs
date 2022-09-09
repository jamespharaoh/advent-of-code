#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;

pub use aoc_2019_day_01 as day_01;

#[ must_use ]
pub fn puzzle_metadata () -> Vec <Box <dyn puzzle::Puzzle>> {
	vec! [
		day_01::puzzle_metadata (),
	]
}
