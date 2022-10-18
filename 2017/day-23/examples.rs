#![ cfg (test) ]

use super::*;

const EXAMPLE: & [& str] = & [
	"add a 10",
	"set h 1",
	"mul h a",
	"sub a 1",
	"jnz a -2",
];

#[ test ]
fn part_one () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("10", puzzle.part_one (EXAMPLE));
}

#[ test ]
fn part_two () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("39916800", puzzle.part_two (EXAMPLE));
}
