#![ cfg (test) ]

use super::*;

const EXAMPLE: & [& str] = & [
	"30373",
	"25512",
	"65332",
	"33549",
	"35390",
];

#[ test ]
fn part_one () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("21", puzzle.part_one (EXAMPLE));
}

#[ test ]
fn part_two () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("8", puzzle.part_two (EXAMPLE));
}
