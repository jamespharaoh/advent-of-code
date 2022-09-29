#![ cfg (test) ]

use super::*;

const EXAMPLE: & [& str] = & [
	"00100", "11110", "10110", "10111", "10101", "01111",
	"00111", "11100", "10000", "11001", "00010", "01010",
];

#[ test ]
fn part_one () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("198", puzzle.part_one (EXAMPLE));
}

#[ test ]
fn part_two () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("230", puzzle.part_two (EXAMPLE));
}
