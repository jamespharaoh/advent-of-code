#![ cfg (test) ]

use super::*;

#[ test ]
fn part_one () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("5158916779", puzzle.part_one (& ["9"]));
	assert_eq_ok! ("0124515891", puzzle.part_one (& ["5"]));
	assert_eq_ok! ("9251071085", puzzle.part_one (& ["18"]));
	assert_eq_ok! ("5941429882", puzzle.part_one (& ["2018"]));
}

#[ test ]
fn part_two () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("9", puzzle.part_two (& ["51589"]));
	assert_eq_ok! ("5", puzzle.part_two (& ["01245"]));
	assert_eq_ok! ("18", puzzle.part_two (& ["92510"]));
	assert_eq_ok! ("2018", puzzle.part_two (& ["59414"]));
}
