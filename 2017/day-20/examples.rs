#![ cfg (test) ]

use super::*;

const EXAMPLE_ONE: & [& str] = & [
	"p=<3,0,0>, v=<2,0,0>, a=<-1,0,0>",
	"p=<4,0,0>, v=<0,0,0>, a=<-2,0,0>",
];

const EXAMPLE_TWO: & [& str] = & [
	"p=<-6,0,0>, v=<3,0,0>, a=<0,0,0>",
	"p=<-4,0,0>, v=<2,0,0>, a=<0,0,0>",
	"p=<-2,0,0>, v=<1,0,0>, a=<0,0,0>",
	"p=<3,0,0>, v=<-1,0,0>, a=<0,0,0>",
];

#[ test ]
fn part_one () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("0", puzzle.part_one (EXAMPLE_ONE));
}

#[ test ]
fn part_two () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("1", puzzle.part_two (EXAMPLE_TWO));
}
