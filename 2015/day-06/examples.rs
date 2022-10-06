#![ cfg (test) ]

use super::*;

const EXAMPLE_0: & [& str] = & [
	"turn on 0,0 through 999,999",
	"toggle 0,0 through 999,0",
	"turn off 499,499 through 500,500",
];

const EXAMPLE_1: & [& str] = & [
	"turn on 0,0 through 0,0",
	"toggle 0,0 through 999,999",
];

#[ test ]
fn part_one () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("998996", puzzle.part_one (EXAMPLE_0));
	assert_eq_ok! ("999999", puzzle.part_one (EXAMPLE_1));
}

#[ test ]
fn part_two () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("1001996", puzzle.part_two (EXAMPLE_0));
	assert_eq_ok! ("2000001", puzzle.part_two (EXAMPLE_1));
}
