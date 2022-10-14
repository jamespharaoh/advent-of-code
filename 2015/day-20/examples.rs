#![ cfg (test) ]

use super::*;

#[ test ]
fn part_one () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("1", puzzle.part_one (& [ "1" ]));
	assert_eq_ok! ("1", puzzle.part_one (& [ "10" ]));
	assert_eq_ok! ("6", puzzle.part_one (& [ "100" ]));
	assert_eq_ok! ("48", puzzle.part_one (& [ "1000" ]));
	assert_eq_ok! ("360", puzzle.part_one (& [ "10000" ]));
	assert_eq_ok! ("3120", puzzle.part_one (& [ "100000" ]));
	assert_eq_ok! ("27720", puzzle.part_one (& [ "1000000" ]));
}

#[ test ]
fn part_two () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("1", puzzle.part_two (& [ "1" ]));
	assert_eq_ok! ("1", puzzle.part_two (& [ "10" ]));
	assert_eq_ok! ("6", puzzle.part_two (& [ "100" ]));
	assert_eq_ok! ("36", puzzle.part_two (& [ "1000" ]));
	assert_eq_ok! ("336", puzzle.part_two (& [ "10000" ]));
	assert_eq_ok! ("2880", puzzle.part_two (& [ "100000" ]));
	assert_eq_ok! ("25200", puzzle.part_two (& [ "1000000" ]));
}
