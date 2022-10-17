#![ cfg (test) ]

use super::*;

const EXAMPLE_ONE: & [& str] = & [
	"set a 1",
	"add a 2",
	"mul a a",
	"mod a 5",
	"snd a",
	"set a 0",
	"rcv a",
	"jgz a -1",
	"set a 1",
	"jgz a -2",
];

const EXAMPLE_TWO: & [& str] = & [
	"snd 1",
	"snd 2",
	"snd p",
	"rcv a",
	"rcv b",
	"rcv c",
	"rcv d",
];

#[ test ]
fn part_one () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("4", puzzle.part_one (EXAMPLE_ONE));
}

#[ test ]
fn part_two () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("3", puzzle.part_two (EXAMPLE_TWO));
}
