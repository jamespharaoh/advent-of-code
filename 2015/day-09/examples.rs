#![ cfg (test) ]

use super::*;

const EXAMPLE: & [& str] = & [
	"London to Dublin = 464",
	"London to Belfast = 518",
	"Dublin to Belfast = 141",
];

#[ test ]
fn part_one () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("605", puzzle.part_one (EXAMPLE));
}

#[ test ]
fn part_two () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("982", puzzle.part_two (EXAMPLE));
}
