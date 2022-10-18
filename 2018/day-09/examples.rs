#![ cfg (test) ]

use super::*;

const EXAMPLE: & [& str] = & [
	"9 players; last marble is worth 25 points",
];

#[ test ]
fn part_one () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("32", puzzle.part_one (EXAMPLE));
}

#[ test ]
fn part_two () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("22563", puzzle.part_two (EXAMPLE));
}
