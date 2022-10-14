#![ cfg (test) ]

use super::*;

const EXAMPLE: & [& str] = & [
	"Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8",
	"Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3",
];

#[ test ]
fn part_one () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("62842880", puzzle.part_one (EXAMPLE));
}

#[ test ]
fn part_two () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("57600000", puzzle.part_two (EXAMPLE));
}
