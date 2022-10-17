#![ cfg (test) ]

use super::*;

const EXAMPLE: & [& str] = & [
	"     |          ",
	"     |  +--+    ",
	"     A  |  C    ",
	" F---|----E|--+ ",
	"     |  |  |  D ",
	"     +B-+  +--+ ",
	"                ",
];

#[ test ]
fn part_one () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("ABCDEF", puzzle.part_one (EXAMPLE));
}

#[ test ]
fn part_two () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("38", puzzle.part_two (EXAMPLE));
}
