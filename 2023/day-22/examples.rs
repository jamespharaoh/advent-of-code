#![ cfg (test) ]

use super::*;

const EXAMPLE: & [& str] = & [
	"1,0,1~1,2,1",
	"0,0,2~2,0,2",
	"0,2,3~2,2,3",
	"0,0,4~0,2,4",
	"2,0,5~2,2,5",
	"0,1,6~2,1,6",
	"1,1,8~1,1,9",
];

#[ test ]
fn part_one () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("5", puzzle.part_one (EXAMPLE));
}

#[ test ]
fn part_two () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("7", puzzle.part_two (EXAMPLE));
}
