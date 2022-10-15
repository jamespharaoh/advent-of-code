#![ cfg (test) ]

use super::*;

const EXAMPLE: & [& str] = & [
	"Disc #1 has 5 positions; at time=0, it is at position 4.",
	"Disc #2 has 2 positions; at time=0, it is at position 1.",
];

#[ test ]
fn part_one () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("5", puzzle.part_one (EXAMPLE));
}

#[ test ]
fn part_two () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("85", puzzle.part_two (EXAMPLE));
}
