#![ cfg (test) ]

use super::*;

const EXAMPLE: & [& str] = & [
	"aaaaa-bbb-z-y-x-123[abxyz]",
	"a-b-c-d-e-f-g-h-987[abcde]",
	"not-a-real-room-404[oarel]",
	"totally-real-room-200[decoy]",
];

#[ test ]
fn part_one () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("1514", puzzle.part_one (EXAMPLE));
}

#[ test ]
fn part_two () {
	let puzzle = puzzle_metadata ();
	assert_err! ("No solution found", puzzle.part_two (EXAMPLE));
}
