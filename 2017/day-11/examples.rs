#![ cfg (test) ]

use super::*;

#[ test ]
fn part_one () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("3", puzzle.part_one (& [ "ne,ne,ne" ]));
	assert_eq_ok! ("0", puzzle.part_one (& [ "ne,ne,sw,sw" ]));
	assert_eq_ok! ("2", puzzle.part_one (& [ "ne,ne,s,s" ]));
	assert_eq_ok! ("3", puzzle.part_one (& [ "se,sw,se,sw,sw" ]));
}

#[ test ]
fn part_two () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("3", puzzle.part_two (& [ "ne,ne,ne" ]));
	assert_eq_ok! ("2", puzzle.part_two (& [ "ne,ne,sw,sw" ]));
	assert_eq_ok! ("2", puzzle.part_two (& [ "ne,ne,s,s" ]));
	assert_eq_ok! ("3", puzzle.part_two (& [ "se,sw,se,sw,sw" ]));
}
