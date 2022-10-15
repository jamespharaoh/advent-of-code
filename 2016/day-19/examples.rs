#![ cfg (test) ]

use super::*;

#[ test ]
fn part_one () {
	let puzzle = puzzle_metadata ();
	for (input, expected) in [
		(2, 1), (3, 3), (4, 1), (5, 3), (6, 5), (7, 7), (8, 1), (9, 3), (10, 5), (11, 7),
		(12, 9), (13, 11), (14, 13), (15, 15), (16, 1), (17, 3), (18, 5), (19, 7), (20, 9),
	].into_iter () {
		assert_eq_ok! (expected.to_string (), puzzle.part_one (& [& input.to_string ()]));
	}
}

#[ test ]
fn part_two () {
	let puzzle = puzzle_metadata ();
	for (input, expected) in [
		(2, 1), (3, 3), (4, 1), (5, 2), (6, 3), (7, 5), (8, 7), (9, 9), (10, 1), (11, 2),
		(12, 3), (13, 4), (14, 5), (15, 6), (16, 7), (17, 8), (18, 9), (19, 11), (20, 13),
	].into_iter () {
		assert_eq_ok! (expected.to_string (), puzzle.part_two (& [& input.to_string ()]));
	}
}
