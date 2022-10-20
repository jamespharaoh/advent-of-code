#![ cfg (test) ]

use super::*;

const EXAMPLE: & [& str] = & [
	"DIST_TWO=32",
	"1, 1",
	"1, 6",
	"8, 3",
	"3, 4",
	"5, 5",
	"8, 9",
];

#[ test ]
fn part_one () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("17", puzzle.part_one (EXAMPLE));
}

#[ test ]
fn part_two () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("16", puzzle.part_two (EXAMPLE));
}
