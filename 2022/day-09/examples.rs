#![ cfg (test) ]

use super::*;

const EXAMPLE_0: & [& str] = & [
	"R 4",
	"U 4",
	"L 3",
	"D 1",
	"R 4",
	"D 1",
	"L 5",
	"R 2",
];

const EXAMPLE_1: & [& str] = & [
	"R 5",
	"U 8",
	"L 8",
	"D 3",
	"R 17",
	"D 10",
	"L 25",
	"U 20",
];

#[ test ]
fn part_one () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("13", puzzle.part_one (EXAMPLE_0));
	assert_eq_ok! ("88", puzzle.part_one (EXAMPLE_1));
}

#[ test ]
fn part_two () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("1", puzzle.part_two (EXAMPLE_0));
	assert_eq_ok! ("36", puzzle.part_two (EXAMPLE_1));
}
