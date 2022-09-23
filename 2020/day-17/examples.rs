#![ cfg (test) ]

use super::*;

const EXAMPLE: & [& str] = & [
	".#.",
	"..#",
	"###",
];

#[ test ]
fn part_one () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("112", puzzle.part_one (EXAMPLE));
}

#[ test ]
fn part_two () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("848", puzzle.part_two (EXAMPLE));
}
