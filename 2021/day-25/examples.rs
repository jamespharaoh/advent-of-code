#![ cfg (test) ]

use super::*;

const EXAMPLE_0: & [& str] = & [
	"v...>>.vv>",
	".vv>>.vv..",
	">>.>v>...v",
	">>v>>.>.v.",
	"v>v.vv.v..",
	">.>>..v...",
	".vv..>.>v.",
	"v.v..>>v.v",
	"....v..v.>",
];

#[ test ]
fn part_one () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("58", puzzle.part_one (EXAMPLE_0));
}
