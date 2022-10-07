#![ cfg (test) ]

use super::*;

const EXAMPLE: & [& str] = & [
	"\"\"",
	"\"abc\"",
	"\"aaa\\\"aaa\"",
	"\"\\x27\"",
];

#[ test ]
fn part_one () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("12", puzzle.part_one (EXAMPLE));
}

#[ test ]
fn part_two () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("19", puzzle.part_two (EXAMPLE));
}
