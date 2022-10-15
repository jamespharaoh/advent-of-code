#![ cfg (test) ]

use super::*;

const EXAMPLES_ONE: & [& str] = & [
	"ADVENT",
	"A(1x5)BC",
	"(3x3)XYZ",
	"A(2x2)BCD(2x2)EFG",
	"(6x1)(1x3)A",
	"X(8x2)(3x3)ABCY",
];

const EXAMPLES_TWO: & [& str] = & [
	"(3x3)XYZ",
	"X(8x2)(3x3)ABCY",
	"(27x12)(20x12)(13x14)(7x10)(1x12)A",
	"(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN",
];

#[ test ]
fn part_one () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("6", puzzle.part_one (& [& EXAMPLES_ONE [0]]));
	assert_eq_ok! ("7", puzzle.part_one (& [& EXAMPLES_ONE [1]]));
	assert_eq_ok! ("9", puzzle.part_one (& [& EXAMPLES_ONE [2]]));
	assert_eq_ok! ("11", puzzle.part_one (& [& EXAMPLES_ONE [3]]));
	assert_eq_ok! ("6", puzzle.part_one (& [& EXAMPLES_ONE [4]]));
	assert_eq_ok! ("18", puzzle.part_one (& [& EXAMPLES_ONE [5]]));
}

#[ test ]
fn part_two () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("9", puzzle.part_two (& [& EXAMPLES_TWO [0]]));
	assert_eq_ok! ("20", puzzle.part_two (& [& EXAMPLES_TWO [1]]));
	assert_eq_ok! ("241920", puzzle.part_two (& [& EXAMPLES_TWO [2]]));
	assert_eq_ok! ("445", puzzle.part_two (& [& EXAMPLES_TWO [3]]));
}
