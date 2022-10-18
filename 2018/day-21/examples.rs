#![ cfg (test) ]

use super::*;

const EXAMPLE: & [& str] = & [
	"#ip 5",
	"seti 9 0 1",
	"addi 1 1 1", // incr
	"eqri 1 20 2",
	"addr 5 2 5",
	"addi 5 2 5", // goto check
	"seti 9 0 1",
	"seti 0 0 5", // goto incr
	"eqrr 1 0 2", // check
	"addr 5 2 5",
	"seti 0 0 5", // goto incr
];

#[ test ]
fn part_one () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("10", puzzle.part_one (EXAMPLE));
}

#[ test ]
fn part_two () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("19", puzzle.part_two (EXAMPLE));
}
