#![ cfg (test) ]

use super::*;

const EXAMPLE: & [& str] = & [
	"Begin in state A.",
	"Perform a diagnostic checksum after 6 steps.",
	"",
	"In state A:",
	"  If the current value is 0:",
	"    - Write the value 1.",
	"    - Move one slot to the right.",
	"    - Continue with state B.",
	"  If the current value is 1:",
	"    - Write the value 0.",
	"    - Move one slot to the left.",
	"    - Continue with state B.",
	"",
	"In state B:",
	"  If the current value is 0:",
	"    - Write the value 1.",
	"    - Move one slot to the left.",
	"    - Continue with state A.",
	"  If the current value is 1:",
	"    - Write the value 1.",
	"    - Move one slot to the right.",
	"    - Continue with state A.",
];

#[ test ]
fn part_one () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("3", puzzle.part_one (EXAMPLE));
}
