#![ cfg (test) ]

use super::*;

#[ test ]
fn part_one () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("6", puzzle.part_one (& ["[1,2,3]"]));
	assert_eq_ok! ("6", puzzle.part_one (& ["{\"a\":2,\"b\":4}"]));
	assert_eq_ok! ("3", puzzle.part_one (& ["[[[3]]]"]));
	assert_eq_ok! ("3", puzzle.part_one (& ["{\"a\":{\"b\":4},\"c\":-1}"]));
	assert_eq_ok! ("0", puzzle.part_one (& ["{\"a\":[-1,1]}"]));
	assert_eq_ok! ("0", puzzle.part_one (& ["[-1,{\"a\":1}]"]));
	assert_eq_ok! ("0", puzzle.part_one (& ["[]"]));
	assert_eq_ok! ("0", puzzle.part_one (& ["{}"]));
}

#[ test ]
fn part_two () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("6", puzzle.part_two (& ["[1,2,3]"]));
	assert_eq_ok! ("4", puzzle.part_two (& ["[1,{\"c\":\"red\",\"b\":2},3]"]));
	assert_eq_ok! ("0", puzzle.part_two (& ["{\"d\":\"red\",\"e\":[1,2,3,4],\"f\":5}"]));
	assert_eq_ok! ("6", puzzle.part_two (& ["[1,\"red\",5]"]));
}
