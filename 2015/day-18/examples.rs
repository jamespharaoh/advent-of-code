#![ cfg (test) ]

use super::*;

const EXAMPLE_ONE: & [& str] = & [
	"NUM_STEPS=4",
	".#.#.#",
	"...##.",
	"#....#",
	"..#...",
	"#.#..#",
	"####.."
];

const EXAMPLE_TWO: & [& str] = & [
	"NUM_STEPS=5",
	".#.#.#",
	"...##.",
	"#....#",
	"..#...",
	"#.#..#",
	"####.."
];

#[ test ]
fn part_one () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("4", puzzle.part_one (EXAMPLE_ONE));
}

#[ test ]
fn part_two () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("17", puzzle.part_two (EXAMPLE_TWO));
}
