#![ cfg (test) ]

use super::*;

const EXAMPLE: & [& str] = & [
	"1=-0-2",
	"12111",
	"2=0=",
	"21",
	"2=01",
	"111",
	"20012",
	"112",
	"1=-1=",
	"1-12",
	"12",
	"1=",
	"122",
];

#[ test ]
fn part_one () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("2=-1=0", puzzle.part_one (EXAMPLE));
}
