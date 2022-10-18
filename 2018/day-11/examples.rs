#![ cfg (test) ]

use super::*;

#[ test ]
fn part_one () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("33,45", puzzle.part_one (& [ "18" ]));
	assert_eq_ok! ("21,61", puzzle.part_one (& [ "42" ]));
}

#[ test ]
fn part_two () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("90,269,16", puzzle.part_two (& [ "18" ]));
	assert_eq_ok! ("232,251,12", puzzle.part_two (& [ "42" ]));
}
