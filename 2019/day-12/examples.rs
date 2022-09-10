#![ cfg (test) ]

use super::*;

const EXAMPLE_0: & [& str] = & [
	"NUM_STEPS_ONE=10",
	"<x=-1, y=0, z=2>",
	"<x=2, y=-10, z=-7>",
	"<x=4, y=-8, z=8>",
	"<x=3, y=5, z=-1>",
];

const EXAMPLE_1: & [& str] = & [
	"NUM_STEPS_ONE=100",
	"<x=-8, y=-10, z=0>",
	"<x=5, y=5, z=10>",
	"<x=2, y=-7, z=3>",
	"<x=9, y=-8, z=-3>",
];

#[ test ]
fn part_one () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("179", puzzle.part_one (EXAMPLE_0));
	assert_eq_ok! ("1940", puzzle.part_one (EXAMPLE_1));
}

#[ test ]
fn part_two () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("2772", puzzle.part_two (EXAMPLE_0));
	assert_eq_ok! ("4686774924", puzzle.part_two (EXAMPLE_1));
}
