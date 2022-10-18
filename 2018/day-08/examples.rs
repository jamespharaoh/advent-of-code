#![ cfg (test) ]

use super::*;

const EXAMPLE: & [& str] = & [
	"2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2",
];

#[ test ]
fn part_one () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("138", puzzle.part_one (EXAMPLE));
}

#[ test ]
fn part_two () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("66", puzzle.part_two (EXAMPLE));
}
