#![ cfg (test) ]

use super::*;

const EXAMPLE: & [& str] = & [
	"12",
	"14",
	"1969",
	"100756",
];

#[ test ]
fn part_one () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("34241", puzzle.part_one (EXAMPLE));
}

#[ test ]
fn part_two () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("51316", puzzle.part_two (EXAMPLE));
}
