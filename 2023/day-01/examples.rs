#![ cfg (test) ]

use super::*;

const EXAMPLE_ONE: & [& str] = & [
	"1abc2",
	"pqr3stu8vwx",
	"a1b2c3d4e5f",
	"treb7uchet",
];

const EXAMPLE_TWO: & [& str] = & [
	"two1nine",
	"eightwothree",
	"abcone2threexyz",
	"xtwone3four",
	"4nineeightseven2",
	"zoneight234",
	"7pqrstsixteen",
];

#[ test ]
fn part_one () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("142", puzzle.part_one (EXAMPLE_ONE));
}

#[ test ]
fn part_two () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("281", puzzle.part_two (EXAMPLE_TWO));
}
