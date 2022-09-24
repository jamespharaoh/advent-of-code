#![ cfg (test) ]

use super::*;

const EXAMPLE: & [& str] = & [
	"mxmxvkd kfcds sqjhc nhms (contains dairy, fish)",
	"trh fvjkl sbzzf mxmxvkd (contains dairy)",
	"sqjhc fvjkl (contains soy)",
	"sqjhc mxmxvkd sbzzf (contains fish)",
];

#[ test ]
fn part_one () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("5", puzzle.part_one (EXAMPLE));
}

#[ test ]
fn part_two () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("mxmxvkd,sqjhc,fvjkl", puzzle.part_two (EXAMPLE));
}
