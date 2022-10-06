#![ cfg (test) ]

use super::*;

const EXAMPLE: & [& str] = & [ "2x3x4", "1x1x10" ];

#[ test ]
fn part_one () -> GenResult <()> {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("58", puzzle.part_one (& EXAMPLE [0 .. 1]));
	assert_eq_ok! ("43", puzzle.part_one (& EXAMPLE [1 .. 2]));
	assert_eq_ok! ("101", puzzle.part_one (EXAMPLE));
}

#[ test ]
fn part_two () -> GenResult <()> {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("34", puzzle.part_two (& EXAMPLE [0 .. 1]));
	assert_eq_ok! ("14", puzzle.part_two (& EXAMPLE [1 .. 2]));
	assert_eq_ok! ("48", puzzle.part_two (EXAMPLE));
}
