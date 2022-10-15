#![ cfg (test) ]

use super::*;

const EXAMPLE_ONE: & [& str] = & [
	"5\t1\t9\t5",
	"7\t5\t3",
	"2\t4\t6\t8",
];

const EXAMPLE_TWO: & [& str] = & [
	"5\t9\t2\t8",
	"9\t4\t7\t3",
	"3\t8\t6\t5",
];

#[ test ]
fn part_one () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("18", puzzle.part_one (EXAMPLE_ONE));
}

#[ test ]
fn part_two () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("9", puzzle.part_two (EXAMPLE_TWO));
}
