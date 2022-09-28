#![ cfg (test) ]

use super::*;

const EXAMPLE: & [& str] = & [
	"5764801",
	"17807724",
];

#[ test ]
fn part_one () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("14897079", puzzle.part_one (EXAMPLE));
}
