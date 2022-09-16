#![ cfg (test) ]

use super::*;

const EXAMPLE: & [& str] = & [
	"1721",
	"979",
	"366",
	"299",
	"675",
	"1456",
];

#[ test ]
fn part_one () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("514579", puzzle.part_one (EXAMPLE));
}

#[ test ]
fn part_two () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("241861950", puzzle.part_two (EXAMPLE));
}
