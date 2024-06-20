#![ cfg (test) ]

use super::*;

const EXAMPLE: & [& str] = & [
	"#.######",
	"#>>.<^<#",
	"#.<..<<#",
	"#>v.><>#",
	"#<^v^^>#",
	"######.#",
];

#[ test ]
fn part_one () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("18", puzzle.part_one (EXAMPLE));
}

#[ test ]
fn part_two () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("54", puzzle.part_two (EXAMPLE));
}
