#![ cfg (test) ]

use super::*;

const EXAMPLE: & [& str] = & [
	"jio a, +5",
	"inc a",
	"tpl a",
	"tpl a",
	"jmp +3",
	"inc a",
	"tpl a",
	"jio a, +4",
	"hlf a",
	"inc b",
	"jmp -3",
];

#[ test ]
fn part_one () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("3", puzzle.part_one (EXAMPLE));
}

#[ test ]
fn part_two () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("2", puzzle.part_two (EXAMPLE));
}
