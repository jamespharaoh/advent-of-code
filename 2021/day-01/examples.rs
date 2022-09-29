#![ cfg (test) ]

use super::*;

const EXAMPLE: & [& str] = & [
	"199", "200", "208", "210", "200", "207", "240", "269", "260", "263"
];

#[ test ]
fn part_one () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("7", puzzle.part_one (EXAMPLE));
}

#[ test ]
fn part_two () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("5", puzzle.part_two (EXAMPLE));
}
