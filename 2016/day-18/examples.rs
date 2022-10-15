#![ cfg (test) ]

use super::*;

const EXAMPLE: & [& str] = & [
	"NUM_ROWS_ONE=10",
	"NUM_ROWS_TWO=20",
	".^^.^.^^^^",
];

#[ test ]
fn part_one () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("38", puzzle.part_one (EXAMPLE));
}

#[ test ]
fn part_two () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("93", puzzle.part_two (EXAMPLE));
}
