#![ cfg (test) ]

use super::*;

const EXAMPLE: & [& str] = & [
	"WIDTH=7",
	"HEIGHT=3",
	"rect 3x2",
	"rotate column x=1 by 1",
	"rotate row y=0 by 4",
	"rotate column x=1 by 1",
];

#[ test ]
fn part_one () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("6", puzzle.part_one (EXAMPLE));
}

#[ test ]
fn part_two () {
	let puzzle = puzzle_metadata ();
	assert_err! ("Unrecognised character: 0x402804 << 96 in position 1", puzzle.part_two (EXAMPLE));
}
