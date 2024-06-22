#![ cfg (test) ]

use super::*;

const EXAMPLE_0: & [& str] = & [
	"broadcaster -> a, b, c",
	"%a -> b",
	"%b -> c",
	"%c -> inv",
	"&inv -> a",
];

const EXAMPLE_1: & [& str] = & [
	"broadcaster -> a",
	"%a -> inv, con",
	"&inv -> b",
	"%b -> con",
	"&con -> output",
];

#[ test ]
fn part_one () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("32000000", puzzle.part_one (EXAMPLE_0));
	assert_eq_ok! ("11687500", puzzle.part_one (EXAMPLE_1));
}

#[ test ]
fn part_two () {
	let puzzle = puzzle_metadata ();
	assert_err! ("No such module: rx", puzzle.part_two (EXAMPLE_0));
	assert_err! ("No such module: rx", puzzle.part_two (EXAMPLE_1));
}
