#![ cfg (test) ]

use super::*;

const EXAMPLE: & [& str] = & [
	"TARGET=25",
	"20",
	"15",
	"10",
	"5",
	"5",
];

#[ test ]
fn part_one () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("4", puzzle.part_one (EXAMPLE));
}

#[ test ]
fn part_two () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("3", puzzle.part_two (EXAMPLE));
}
