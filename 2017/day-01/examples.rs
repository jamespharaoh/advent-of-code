#![ cfg (test) ]

use super::*;

#[ test ]
fn part_one () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("3", puzzle.part_one (& ["1122"]));
	assert_eq_ok! ("4", puzzle.part_one (& ["1111"]));
	assert_eq_ok! ("0", puzzle.part_one (& ["1234"]));
	assert_eq_ok! ("9", puzzle.part_one (& ["91212129"]));
}

#[ test ]
fn part_two () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("6", puzzle.part_two (& ["1212"]));
	assert_eq_ok! ("0", puzzle.part_two (& ["1221"]));
	assert_eq_ok! ("4", puzzle.part_two (& ["123425"]));
	assert_eq_ok! ("12", puzzle.part_two (& ["123123"]));
	assert_eq_ok! ("4", puzzle.part_two (& ["12131415"]));
}
