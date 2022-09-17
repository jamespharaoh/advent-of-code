#![ cfg (test) ]

use super::*;

const EXAMPLE: & [& str] = & [
	"1-3 a: abcde",
	"1-3 b: cdefg",
	"2-9 c: ccccccccc",
];

#[ test ]
fn part_one () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("2", puzzle.part_one (EXAMPLE));
}

#[ test ]
fn part_two () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("1", puzzle.part_two (EXAMPLE));
}
