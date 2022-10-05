#![ cfg (test) ]

use super::*;

const EXAMPLE: & [& str] = & [ "target area: x=20..30, y=-10..-5" ];

#[ test ]
fn part_one () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("45", puzzle.part_one (EXAMPLE));
}

#[ test ]
fn test_example () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("112", puzzle.part_two (EXAMPLE));
}
