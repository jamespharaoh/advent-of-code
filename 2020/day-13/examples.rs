#![ cfg (test) ]

use super::*;

const EXAMPLE: & [& str] = & [
	"939",
	"7,13,x,x,59,x,31,19",
];

#[ test ]
fn part_one () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("295", puzzle.part_one (EXAMPLE));
}

#[ test ]
fn part_two () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("1068781", puzzle.part_two (EXAMPLE));
}
