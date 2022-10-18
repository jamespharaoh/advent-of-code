#![ cfg (test) ]

use super::*;

const EXAMPLE: & [& str] = & [
	".#.#...|#.",
	".....#|##|",
	".|..|...#.",
	"..|#.....#",
	"#.#|||#|#|",
	"...#.||...",
	".|....|...",
	"||...#|.#|",
	"|.||||..|.",
	"...#.|..|.",
];

#[ test ]
fn test_one () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("1147", puzzle.part_one (EXAMPLE));
}

#[ test ]
fn test_two () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("0", puzzle.part_two (EXAMPLE));
}
