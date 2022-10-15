#![ cfg (test) ]

use super::*;

const EXAMPLE: & [& str] = & [ "END_X=7", "END_Y=4", "MAX_DIST=20", "COUNT_DIST=10", "10" ];

#[ test ]
fn part_one () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("11", puzzle.part_one (EXAMPLE));
}

#[ test ]
fn part_two () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("18", puzzle.part_two (EXAMPLE));
}
