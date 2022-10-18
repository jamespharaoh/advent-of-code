#![ cfg (test) ]

use super::*;

const EXAMPLE: & [& str] = & [
	"..#",
	"#..",
	"...",
];

#[ test ]
fn part_one () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("5587", puzzle.part_one (EXAMPLE));
}

#[ test ]
fn part_two () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("2511944", puzzle.part_two (EXAMPLE));
}
