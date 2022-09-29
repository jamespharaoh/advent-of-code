#![ cfg (test) ]

use super::*;

const EXAMPLE: & [& str] = & [ "3,4,3,1,2" ];

#[ test ]
fn part_one () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("5934", puzzle.part_one (EXAMPLE));
}

#[ test ]
fn part_two () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("26984457539", puzzle.part_two (EXAMPLE));
}
