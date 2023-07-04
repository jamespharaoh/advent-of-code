#![ cfg (test) ]

use super::*;

const EXAMPLE: & [& str] = & [
	"2,2,2",
	"1,2,2",
	"3,2,2",
	"2,1,2",
	"2,3,2",
	"2,2,1",
	"2,2,3",
	"2,2,4",
	"2,2,6",
	"1,2,5",
	"3,2,5",
	"2,1,5",
	"2,3,5",
];

#[ test ]
fn part_one () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("64", puzzle.part_one (EXAMPLE));
}

#[ test ]
fn part_two () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("58", puzzle.part_two (EXAMPLE));
}
