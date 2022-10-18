#![ cfg (test) ]

use super::*;

const EXAMPLE: & [& str] = & [ "0/2", "2/2", "2/3", "3/4", "3/5", "0/1", "10/1", "9/10" ];

#[ test ]
fn part_one () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("31", puzzle.part_one (EXAMPLE));
}

#[ test ]
fn part_two () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("19", puzzle.part_two (EXAMPLE));
}
