#![ cfg (test) ]

use super::*;

const EXAMPLE: & [& str] = & [
	"###########",
	"#0.1.....2#",
	"#.#######.#",
	"#4.......3#",
	"###########",
];

#[ test ]
fn part_one () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("14", puzzle.part_one (EXAMPLE));
}

#[ test ]
fn part_two () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("20", puzzle.part_two (EXAMPLE));
}
