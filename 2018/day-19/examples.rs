#![ cfg (test) ]

use super::*;

const EXAMPLE: & [& str] = & [
	"#ip 0",
	"seti 5 0 1",
	"seti 6 0 2",
	"addi 0 1 0",
	"addr 1 2 3",
	"setr 1 0 0",
	"seti 8 0 4",
	"seti 9 0 5",
];

#[ test ]
fn part_one () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("7", puzzle.part_one (EXAMPLE));
}
