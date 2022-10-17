#![ cfg (test) ]

use super::*;

const EXAMPLE: & [& str] = & [ "0: 3", "1: 2", "4: 4", "6: 4" ];

#[ test ]
fn part_one () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("24", puzzle.part_one (EXAMPLE));
}

#[ test ]
fn part_two () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("10", puzzle.part_two (EXAMPLE));
}
