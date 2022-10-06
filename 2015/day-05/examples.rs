#![ cfg (test) ]

use super::*;

const EXAMPLE_ONE: & [& str] = & [
	"ugknbfddgicrmopn",
	"aaa",
	"jchzalrnumimnmhp",
	"haegwjzuvuyypxyu",
	"dvszwmarrgswjxmb",
];

const EXAMPLE_TWO: & [& str] = & [
	"qjhvhtzxzqqjkmpb",
	"xxyxx",
	"uurcxstgmygtbstg",
	"ieodomkazucvgmuy",
];

#[ test ]
fn part_one () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("2", puzzle.part_one (EXAMPLE_ONE));
}

#[ test ]
fn part_two () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("2", puzzle.part_two (EXAMPLE_TWO));
}
