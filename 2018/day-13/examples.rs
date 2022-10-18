#![ cfg (test) ]

use super::*;

const EXAMPLE_ONE: & [& str] = & [
	r"/->-\        ",
	r"|   |  /----\",
	r"| /-+--+-\  |",
	r"| | |  | v  |",
	r"\-+-/  \-+--/",
	r"  \------/   ",
];

const EXAMPLE_TWO: & [& str] = & [
	r"/>-<\  ",
	r"|   |  ",
	r"| /<+-\",
	r"| | | v",
	r"\>+</ |",
	r"  |   ^",
	r"  \<->/",
];

#[ test ]
fn part_one () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("7,3", puzzle.part_one (EXAMPLE_ONE));
}

#[ test ]
fn part_two () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("6,4", puzzle.part_two (EXAMPLE_TWO));
}
