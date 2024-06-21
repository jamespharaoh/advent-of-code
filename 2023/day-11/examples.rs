#![ cfg (test) ]

use super::*;

const EXAMPLE: & [& str] = & [
	"...#......",
	".......#..",
	"#.........",
	"..........",
	"......#...",
	".#........",
	".........#",
	"..........",
	".......#..",
	"#...#.....",
];

#[ test ]
fn part_one () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("374", puzzle.part_one (EXAMPLE));
}

#[ test ]
fn part_two () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("1030", puzzle.part_two (& with_params (["EXPAND_TWO=9"], EXAMPLE)));
	assert_eq_ok! ("8410", puzzle.part_two (& with_params (["EXPAND_TWO=99"], EXAMPLE)));
}
