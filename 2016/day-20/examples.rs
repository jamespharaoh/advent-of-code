#![ cfg (test) ]

use super::*;

#[ test ]
fn part_one () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("3", puzzle.part_one (& [ "5-8", "0-2", "4-7" ]));
	assert_eq_ok! ("4294967295", puzzle.part_one (& [ "0-4294967294" ]));
	assert_err! ("No solution found", puzzle.part_one (& [ "0-4294967295" ]));
}

#[ test ]
fn part_two () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("4294967288", puzzle.part_two (& [ "5-8", "0-2", "4-7" ]));
	assert_eq_ok! ("1", puzzle.part_two (& [ "0-4294967294" ]));
	assert_eq_ok! ("0", puzzle.part_two (& [ "0-4294967295" ]));
}
