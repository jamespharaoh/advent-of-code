#![ cfg (test) ]

use super::*;

const EXAMPLE: & [& str] = & [
	"...........",
	".....###.#.",
	".###.##..#.",
	"..#.#...#..",
	"....#.#....",
	".##..S####.",
	".##..#...#.",
	".......##..",
	".##.#.####.",
	".##..##.##.",
	"...........",
];

#[ test ]
fn part_one () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("16", puzzle.part_one (& with_params (["NUM_STEPS_ONE=6"], EXAMPLE)));
}

#[ test ]
fn part_two () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("16", puzzle.part_two (& with_params (["NUM_STEPS_TWO=6", "TEST=1"], EXAMPLE)));
	assert_eq_ok! ("50", puzzle.part_two (& with_params (["NUM_STEPS_TWO=10", "TEST=1"], EXAMPLE)));
	assert_eq_ok! ("1594", puzzle.part_two (& with_params (["NUM_STEPS_TWO=50", "TEST=1"], EXAMPLE)));
	assert_eq_ok! ("6536", puzzle.part_two (& with_params (["NUM_STEPS_TWO=100", "TEST=1"], EXAMPLE)));
	assert_eq_ok! ("167004", puzzle.part_two (& with_params (["NUM_STEPS_TWO=500", "TEST=1"], EXAMPLE)));
	assert_eq_ok! ("668697", puzzle.part_two (& with_params (["NUM_STEPS_TWO=1000", "TEST=1"], EXAMPLE)));
	assert_eq_ok! ("16733044", puzzle.part_two (& with_params (["NUM_STEPS_TWO=5000", "TEST=1"], EXAMPLE)));
}
