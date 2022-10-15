#![ cfg (test) ]

use super::*;

const EXAMPLE: & [& str] = & [
	"cpy 41 a",
	"inc a",
	"inc a",
	"dec a",
	"jnz a 2",
	"dec a",
	"dec c",
	"jnz c 3",
	"dec a",
	"dec a",
];

#[ test ]
fn part_one () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("42", puzzle.part_one (EXAMPLE));
}

#[ test ]
fn part_two () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("40", puzzle.part_two (EXAMPLE));
}
