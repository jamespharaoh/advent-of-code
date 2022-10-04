#![ cfg (test) ]

use super::*;

const EXAMPLE_0: & [& str] = & [
	"start-A",
	"start-b",
	"A-c",
	"A-b",
	"b-d",
	"A-end",
	"b-end",
];

const EXAMPLE_1: & [& str] = & [
	"dc-end",
	"HN-start",
	"start-kj",
	"dc-start",
	"dc-HN",
	"LN-dc",
	"HN-end",
	"kj-sa",
	"kj-HN",
	"kj-dc",
];

const EXAMPLE_2: & [& str] = & [
	"fs-end",
	"he-DX",
	"fs-he",
	"start-DX",
	"pj-DX",
	"end-zg",
	"zg-sl",
	"zg-pj",
	"pj-he",
	"RW-he",
	"fs-DX",
	"pj-RW",
	"zg-RW",
	"start-pj",
	"he-WI",
	"zg-he",
	"pj-fs",
	"start-RW",
];

#[ test ]
fn part_one () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("10", puzzle.part_one (EXAMPLE_0));
	assert_eq_ok! ("19", puzzle.part_one (EXAMPLE_1));
	assert_eq_ok! ("226", puzzle.part_one (EXAMPLE_2));
}

#[ test ]
fn part_two () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("36", puzzle.part_two (EXAMPLE_0));
	assert_eq_ok! ("103", puzzle.part_two (EXAMPLE_1));
	assert_eq_ok! ("3509", puzzle.part_two (EXAMPLE_2));
}
