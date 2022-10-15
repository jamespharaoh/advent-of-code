#![ cfg (test) ]

use super::*;

#[ test ]
fn part_one () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("0", puzzle.part_one (& [ "1" ]));
	assert_eq_ok! ("3", puzzle.part_one (& [ "12" ]));
	assert_eq_ok! ("2", puzzle.part_one (& [ "23" ]));
	assert_eq_ok! ("31", puzzle.part_one (& [ "1024" ]));
}

#[ test ]
fn part_two () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("2", puzzle.part_two (& [ "1" ]));
	assert_eq_ok! ("23", puzzle.part_two (& [ "12" ]));
	assert_eq_ok! ("25", puzzle.part_two (& [ "23" ]));
	assert_eq_ok! ("1968", puzzle.part_two (& [ "1024" ]));
}
