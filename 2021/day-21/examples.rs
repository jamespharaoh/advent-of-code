#![ cfg (test) ]

use super::*;

const EXAMPLE: & 'static [& 'static str] = & [
	"Player 1 starting position: 4",
	"Player 2 starting position: 8",
];

#[ test ]
fn part_one () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("739785", puzzle.part_one (EXAMPLE));
}

#[ test ]
fn part_two () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("444356092776315", puzzle.part_two (EXAMPLE));
}
