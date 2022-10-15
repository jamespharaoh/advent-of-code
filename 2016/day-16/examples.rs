#![ cfg (test) ]

use super::*;

const EXAMPLE: & [& str] = & [
	"DISK_SIZE_ONE=20",
	"DISK_SIZE_TWO=80",
	"10000",
];

#[ test ]
fn part_one () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("01100", puzzle.part_one (EXAMPLE));
}

#[ test ]
fn part_two () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("11010", puzzle.part_two (EXAMPLE));
}
