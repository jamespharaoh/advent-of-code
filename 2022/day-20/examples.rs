#![ cfg (test) ]

use super::*;

const EXAMPLE: & [& str] = & [
	"1",
	"2",
	"-3",
	"3",
	"-2",
	"0",
	"4",
];

#[ test ]
fn part_one () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("3", puzzle.part_one (EXAMPLE));
}

#[ test ]
fn part_two () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("1623178306", puzzle.part_two (EXAMPLE));
}
