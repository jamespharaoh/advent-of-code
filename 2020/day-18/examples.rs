#![ cfg (test) ]

use super::*;

const EXAMPLE: & [& str] = & [
	"1 + 2 * 3 + 4 * 5 + 6",
	"1 + (2 * 3) + (4 * (5 + 6))",
	"2 * 3 + (4 * 5)",
	"5 + (8 * 3 + 9 + 3 * 4 * 3)",
	"5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))",
	"((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2",
];

#[ test ]
fn part_one () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("26457", puzzle.part_one (EXAMPLE));
}

#[ test ]
fn part_two () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("694173", puzzle.part_two (EXAMPLE));
}
