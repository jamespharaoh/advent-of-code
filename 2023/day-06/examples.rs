#![ cfg (test) ]

use super::*;

const EXAMPLE: & [& str] = & [
	"Time:      7  15   30",
	"Distance:  9  40  200",
];

#[ test ]
fn part_one () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("288", puzzle.part_one (EXAMPLE));
}

#[ test ]
fn part_two () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("71503", puzzle.part_two (EXAMPLE));
}
