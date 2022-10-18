#![ cfg (test) ]

use super::*;

#[ test ]
fn part_one () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("3", puzzle.part_one (& [ "+1", "-2", "+3", "+1" ]));
	assert_eq_ok! ("3", puzzle.part_one (& [ "+1", "+1", "+1" ]));
	assert_eq_ok! ("0", puzzle.part_one (& [ "+1", "+1", "-2" ]));
	assert_eq_ok! ("-6", puzzle.part_one (& [ "-1", "-2", "-3" ]));
}

#[ test ]
fn part_two () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("0", puzzle.part_two (& [ "+1", "-1" ]));
	assert_eq_ok! ("10", puzzle.part_two (& [ "+3", "+3", "+4", "-2", "-4" ]));
	assert_eq_ok! ("5", puzzle.part_two (& [ "-6", "+3", "+8", "+5", "-6" ]));
	assert_eq_ok! ("14", puzzle.part_two (& [ "+7", "+7", "-2", "-7", "-4" ]));
}
