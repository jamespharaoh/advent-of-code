#![ cfg (test) ]

use super::*;

const EXAMPLE: & [& str] = & [
	"Player 1:",
	"9",
	"2",
	"6",
	"3",
	"1",
	"",
	"Player 2:",
	"5",
	"8",
	"4",
	"7",
	"10",
];

#[ test ]
fn part_one () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("306", puzzle.part_one (EXAMPLE));
}

#[ test ]
fn part_two () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("291", puzzle.part_two (EXAMPLE));
}
