#![ cfg (test) ]

use super::*;

const EXAMPLE: & [& str] = & [
	"5483143223",
	"2745854711",
	"5264556173",
	"6141336146",
	"6357385478",
	"4167524645",
	"2176841721",
	"6882881134",
	"4846848554",
	"5283751526",
];

#[ test ]
fn part_one () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("1656", puzzle.part_one (EXAMPLE));
}

#[ test ]
fn part_two () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("195", puzzle.part_two (EXAMPLE));
}
