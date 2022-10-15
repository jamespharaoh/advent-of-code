#![ cfg (test) ]

use super::*;

const EXAMPLE: & [& str] = & [
	"value 5 goes to bot 2",
	"bot 2 gives low to bot 1 and high to bot 0",
	"value 3 goes to bot 1",
	"bot 1 gives low to output 1 and high to bot 0",
	"bot 0 gives low to output 2 and high to output 0",
	"value 2 goes to bot 2",
];

#[ test ]
fn part_one () {
	use parser::with_params;
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("0", puzzle.part_one (& with_params (["LOW=3", "HIGH=5"], EXAMPLE)));
	assert_eq_ok! ("1", puzzle.part_one (& with_params (["LOW=2", "HIGH=3"], EXAMPLE)));
	assert_eq_ok! ("2", puzzle.part_one (& with_params (["LOW=2", "HIGH=5"], EXAMPLE)));
	assert_err! ("No solution found", puzzle.part_one (EXAMPLE));
}

#[ test ]
fn part_two () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("30", puzzle.part_two (EXAMPLE));
	assert_err! ("No solution found", puzzle.part_one (& EXAMPLE [1 .. ]));
}
