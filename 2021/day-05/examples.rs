#![ cfg (test) ]

use super::*;

const EXAMPLE: & [& str] = & [
	"0,9 -> 5,9",
	"8,0 -> 0,8",
	"9,4 -> 3,4",
	"2,2 -> 2,1",
	"7,0 -> 7,4",
	"6,4 -> 2,0",
	"0,9 -> 2,9",
	"3,4 -> 1,4",
	"0,0 -> 8,8",
	"5,5 -> 8,2",
];

#[ test ]
fn part_one () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("5", puzzle.part_one (EXAMPLE));
}

#[ test ]
fn part_two () -> GenResult <()> {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("12", puzzle.part_two (EXAMPLE));
}
