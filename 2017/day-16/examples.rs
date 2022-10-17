#![ cfg (test) ]

use super::*;

const EXAMPLE: & [& str] = & [
	"s1,x3/4,pe/b",
];

#[ test ]
fn part_one () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("paedcbfghijklmno", puzzle.part_one (EXAMPLE));
}

#[ test ]
fn part_two () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("ghidjklmnopabcef", puzzle.part_two (EXAMPLE));
}
