#![ cfg (test) ]

use super::*;

const EXAMPLE_ONE: & [& str] = & [
	"abcdef",
	"bababc",
	"abbcde",
	"abcccd",
	"aabcdd",
	"abcdee",
	"ababab",
];

const EXAMPLE_TWO: & [& str] = & [
	"abcde",
	"fghij",
	"klmno",
	"pqrst",
	"fguij",
	"axcye",
	"wvxyz",
];

#[ test ]
fn part_one () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("12", puzzle.part_one (EXAMPLE_ONE));
}

#[ test ]
fn part_two () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("fgij", puzzle.part_two (EXAMPLE_TWO));
}
