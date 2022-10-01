#![ cfg (test) ]

use super::*;

const EXAMPLE: & [& str] = & [
	"2199943210",
	"3987894921",
	"9856789892",
	"8767896789",
	"9899965678",
];

#[ test ]
fn part_one () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("15", puzzle.part_one (EXAMPLE));
}

#[ test ]
fn part_two () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("1134", puzzle.part_two (EXAMPLE));
}
