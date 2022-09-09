#![ cfg (test) ]

use super::*;

const EXAMPLE_0: & [& str] = & [
	"R8,U5,L5,D3",
	"U7,R6,D4,L4",
];

const EXAMPLE_1: & [& str] = & [
	"R75,D30,R83,U83,L12,D49,R71,U7,L72",
	"U62,R66,U55,R34,D71,R55,D58,R83",
];

const EXAMPLE_2: & [& str] = & [
	"R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51",
	"U98,R91,D20,R16,D67,R40,U7,R15,U6,R7",
];

#[ test ]
fn part_one () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("6", puzzle.part_one (EXAMPLE_0));
	assert_eq_ok! ("159", puzzle.part_one (EXAMPLE_1));
	assert_eq_ok! ("135", puzzle.part_one (EXAMPLE_2));
}

#[ test ]
fn part_two () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("30", puzzle.part_two (EXAMPLE_0));
	assert_eq_ok! ("610", puzzle.part_two (EXAMPLE_1));
	assert_eq_ok! ("410", puzzle.part_two (EXAMPLE_2));
}
