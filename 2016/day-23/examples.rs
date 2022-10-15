#![ cfg (test) ]

use super::*;

const EXAMPLE: & [& str] = & [
	"cpy 2 a",
	"tgl a",
	"tgl a",
	"tgl a",
	"cpy 1 a",
	"dec a",
	"dec a",
];

#[ test ]
fn part_one () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("3", puzzle.part_one (EXAMPLE));
}
