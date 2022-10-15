#![ cfg (test) ]

use super::*;

const EXAMPLE_ONE: & [& str] = & [
	"The first floor contains a hydrogen-compatible microchip and a lithium-compatible microchip.",
	"The second floor contains a hydrogen generator.",
	"The third floor contains a lithium generator.",
	"The fourth floor contains nothing relevant.",
];

const EXAMPLE_TWO: & [& str] = & [
	"The first floor contains a hydrogen generator.",
	"The second floor contains a lithium generator.",
	"The third floor contains a hydrogen-compatible microchip and a lithium-compatible microchip.",
	"The fourth floor contains nothing relevant.",
];

#[ test ]
fn part_one () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("11", puzzle.part_one (EXAMPLE_ONE));
}

#[ test ]
fn part_two () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("29", puzzle.part_two (EXAMPLE_TWO));
}
