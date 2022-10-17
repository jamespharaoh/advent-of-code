#![ cfg (test) ]

use super::*;

const EXAMPLE: & [& str] = & [
	"123",
];

#[ test ]
fn part_one () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("698", puzzle.part_one (EXAMPLE));
}

#[ test ]
fn part_two () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("29108944", puzzle.part_two (EXAMPLE));
}
