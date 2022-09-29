#![ cfg (test) ]

use super::*;

const EXAMPLE: & [& str] = & [ "16,1,2,0,4,2,7,1,2,14" ];

#[ test ]
fn part_one () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("37", puzzle.part_one (EXAMPLE));
}

#[ test ]
fn part_two () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("168", puzzle.part_two (EXAMPLE));
}
