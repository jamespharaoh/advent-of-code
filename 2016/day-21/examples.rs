#![ cfg (test) ]

use super::*;

const EXAMPLE: & [& str] = & [
	"START_ONE=abcde",
	"START_TWO=fbdecgha",
	"swap position 4 with position 0",
	"swap letter d with letter b",
	"reverse positions 0 through 4",
	"rotate left 1 step",
	"move position 1 to position 4",
	"move position 3 to position 0",
	"rotate based on position of letter b",
	"rotate based on position of letter d",
];

#[ test ]
fn part_one () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("decab", puzzle.part_one (EXAMPLE));
}

#[ test ]
fn part_two () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("abcdefgh", puzzle.part_two (EXAMPLE));
}
