#![ cfg (test) ]

use super::*;

const EXAMPLES_ONE: & [& str] = & [
	"(())",
	"()()",
	"))(((((",
	"())",
	"))(",
	")))",
	")())())",
];

const EXAMPLES_TWO: & [& str] = & [
	")",
	"()())",
];

#[ test ]
fn part_one () -> GenResult <()> {
	let puzzle = puzzle_metadata ();
	assert_eq! ("0", puzzle.part_one (& [EXAMPLES_ONE [0]]) ?);
	assert_eq! ("0", puzzle.part_one (& [EXAMPLES_ONE [1]]) ?);
	assert_eq! ("3", puzzle.part_one (& [EXAMPLES_ONE [2]]) ?);
	assert_eq! ("-1", puzzle.part_one (& [EXAMPLES_ONE [3]]) ?);
	assert_eq! ("-1", puzzle.part_one (& [EXAMPLES_ONE [4]]) ?);
	assert_eq! ("-3", puzzle.part_one (& [EXAMPLES_ONE [5]]) ?);
	assert_eq! ("-3", puzzle.part_one (& [EXAMPLES_ONE [6]]) ?);
	Ok (())
}

#[ test ]
fn part_two () -> GenResult <()> {
	let puzzle = puzzle_metadata ();
	assert_eq! ("1", puzzle.part_two (& [EXAMPLES_TWO [0]]) ?);
	assert_eq! ("5", puzzle.part_two (& [EXAMPLES_TWO [1]]) ?);
	Ok (())
}
