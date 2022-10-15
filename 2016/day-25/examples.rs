#![ cfg (test) ]

use super::*;

const EXAMPLE: & [& str] = & [
	"cpy 10 b",
	"dec a",
	"dec b",
	"jnz b -2",
	"jnz a 4",
	"out 0",
	"out 1",
	"jnz 1 -2",
];

#[ test ]
fn part_one () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("10", puzzle.part_one (EXAMPLE));
}
