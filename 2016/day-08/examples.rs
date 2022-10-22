#![ cfg (test) ]

use super::*;

const EXAMPLE: & [& str] = & [
	"WIDTH=5",
	"HEIGHT=6",
	"rect 1x6",
	"rotate row y=5 by 1",
	"rotate column x=0 by 1",
	"rotate row y=5 by 1",
	"rotate column x=0 by 1",
	"rotate row y=5 by 1",
	"rotate column x=0 by 1",
	"rect 1x3",
];

#[ test ]
fn part_one () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("9", puzzle.part_one (EXAMPLE));
}

#[ test ]
fn part_two () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("L", puzzle.part_two (EXAMPLE));
}
