#![ cfg (test) ]

use super::*;

const EXAMPLE_ONE: & [& str] = & [
	"3,4,1,5",
];

const EXAMPLE_TWO: & [& str] = & [
	"3,4,1,5",
];

#[ test ]
fn part_one () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("2", puzzle.part_one (EXAMPLE_ONE));
}

#[ test ]
fn part_two () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("4a19451b02fb05416d73aea0ec8c00c0", puzzle.part_two (EXAMPLE_TWO));
}
