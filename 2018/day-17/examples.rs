#![ cfg (test) ]

use super::*;

const EXAMPLE: & [& str] = & [
	"x=495, y=2..7",
	"y=7, x=495..501",
	"x=501, y=3..7",
	"x=498, y=2..4",
	"x=506, y=1..2",
	"x=498, y=10..13",
	"x=504, y=10..13",
	"y=13, x=498..504",
];

#[ test ]
fn test_one () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("57", puzzle.part_one (EXAMPLE));
}

#[ test ]
fn test_two () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("29", puzzle.part_two (EXAMPLE));
}
