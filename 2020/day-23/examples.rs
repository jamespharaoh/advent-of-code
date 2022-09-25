#![ cfg (test) ]

use super::*;

const EXAMPLE: & [& str] = & [
	"389125467",
];

#[ test ]
fn part_one () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("67384529", puzzle.part_one (EXAMPLE));
}

#[ test ]
fn part_two () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("149245887792", puzzle.part_two (EXAMPLE));
}
