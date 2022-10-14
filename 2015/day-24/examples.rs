#![ cfg (test) ]

use super::*;

const EXAMPLE: & [& str] = & [ "1", "2", "3", "4", "5", "7", "8", "9", "10", "11" ];

#[ test ]
fn part_one () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("99", puzzle.part_one (EXAMPLE));
}

#[ test ]
fn part_two () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("44", puzzle.part_two (EXAMPLE));
}
