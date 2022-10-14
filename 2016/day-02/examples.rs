#![ cfg (test) ]

use super::*;

const EXAMPLE: & [& str] = & [
	"ULL",
	"RRDDD",
	"LURDL",
	"UUUUD",
];

#[ test ]
fn part_one () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("1985", puzzle.part_one (EXAMPLE));
}

#[ test ]
fn part_two () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("5DB3", puzzle.part_two (EXAMPLE));
}
