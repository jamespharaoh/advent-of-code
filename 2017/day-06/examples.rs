#![ cfg (test) ]

use super::*;

const EXAMPLE: & [& str] = & [ "0\t2\t7\t0" ];

#[ test ]
fn part_one () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("5", puzzle.part_one (EXAMPLE));
}

#[ test ]
fn part_two () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("4", puzzle.part_two (EXAMPLE));
}
