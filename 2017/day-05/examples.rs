#![ cfg (test) ]

use super::*;

const EXAMPLE: & [& str] = & [
	"0",
	"3",
	"0",
	"1",
	"-3",
];

#[ test ]
fn part_one () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("5", puzzle.part_one (EXAMPLE));
}

#[ test ]
fn part_two () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("10", puzzle.part_two (EXAMPLE));
}
