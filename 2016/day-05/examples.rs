#![ cfg (test) ]

use super::*;

const EXAMPLE: & [& str] = & [
	"NUM_ZEROS=1",
	"MAX_THREADS=1",
	"abc",
];

#[ test ]
fn part_one () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("0500f456", puzzle.part_one (EXAMPLE));
}

#[ test ]
fn part_two () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("36790f5e", puzzle.part_two (EXAMPLE));
}
