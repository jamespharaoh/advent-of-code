#![ cfg (test) ]

use super::*;

const EXAMPLE: & [& str] = & [
	"NUM_REPS_TWO=10",
	"....#",
	"#..#.",
	"#..##",
	"..#..",
	"#....",
];

#[ test ]
fn part_one () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("2129920", puzzle.part_one (EXAMPLE));
}

#[ test ]
fn part_two () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("99", puzzle.part_two (EXAMPLE));
}
