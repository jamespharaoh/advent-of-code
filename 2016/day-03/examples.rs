#![ cfg (test) ]

use super::*;

const EXAMPLE: & [& str] = & [
	"2 2 3",
	"4 6 8",
	"5 10 25",
];

#[ test ]
fn part_one () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("2", puzzle.part_one (EXAMPLE));
}

#[ test ]
fn part_two () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("1", puzzle.part_two (EXAMPLE));
}
