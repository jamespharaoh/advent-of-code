#![ cfg (test) ]

use super::*;

const EXAMPLE: & [& str] = & [
	"abba[mnop]qrst",
	"abcd[bddb]xyyx",
	"aaaa[qwer]tyui",
	"ioxxoj[asdfgh]zxcvbn",
	"aba[bab]xyz",
	"xyx[xyx]xyx",
	"aaa[kek]eke",
	"zazbz[bzb]cdb",
];

#[ test ]
fn part_one () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("2", puzzle.part_one (EXAMPLE));
}

#[ test ]
fn part_two () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("3", puzzle.part_two (EXAMPLE));
}
