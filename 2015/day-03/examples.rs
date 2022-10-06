#![ cfg (test) ]

use super::*;

#[ test ]
fn part_one () -> GenResult <()> {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("2", puzzle.part_one (& [">"]));
	assert_eq_ok! ("4", puzzle.part_one (& ["^>v<"]));
	assert_eq_ok! ("2", puzzle.part_one (& ["^v^v^v^v^v"]));
	Ok (())
}

#[ test ]
fn part_two () -> GenResult <()> {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("3", puzzle.part_two (& ["^v"]));
	assert_eq_ok! ("3", puzzle.part_two (& ["^>v<"]));
	assert_eq_ok! ("11", puzzle.part_two (& ["^v^v^v^v^v"]));
	Ok (())
}
