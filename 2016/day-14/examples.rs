#![ cfg (test) ]

use super::*;

const EXAMPLE: & [& str] = & [
	"NUM_KEYS=16",
	"NUM_NEXT=1000",
	"HASH_REPS=16",
	"MAX_THREADS=2",
	"abc",
];

#[ test ]
fn part_one () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("1144", puzzle.part_one (EXAMPLE));
}

#[ test ]
fn part_two () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("4684", puzzle.part_two (EXAMPLE));
}
