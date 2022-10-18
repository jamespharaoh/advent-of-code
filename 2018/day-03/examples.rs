#![ cfg (test) ]

use super::*;

const EXAMPLE: & [& str] = & [
	"#1 @ 1,3: 4x4",
	"#2 @ 3,1: 4x4",
	"#3 @ 5,5: 2x2",
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
