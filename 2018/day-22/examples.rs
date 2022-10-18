#![ cfg (test) ]

use super::*;

const EXAMPLE: & [& str] = & [
	"depth: 510",
	"target: 10,10",
];

#[ test ]
fn part_one () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("114", puzzle.part_one (EXAMPLE));
}

#[ test ]
fn part_two () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("45", puzzle.part_two (EXAMPLE));
}
