#![ cfg (test) ]

use super::*;

const EXAMPLE: & [& str] = & [
	"A Y",
	"B X",
	"C Z",
];

#[ test ]
fn part_one () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("15", puzzle.part_one (EXAMPLE));
}

#[ test ]
fn part_two () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("12", puzzle.part_two (EXAMPLE));
}
