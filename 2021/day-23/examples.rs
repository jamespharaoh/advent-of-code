#![ cfg (test) ]

use super::*;

const EXAMPLE: & 'static [& 'static str] = & [
	"#############",
	"#...........#",
	"###B#C#B#D###",
	"  #A#D#C#A#",
	"  #########",
];

#[ test ]
fn part_one () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("12521", puzzle.part_one (EXAMPLE));
}

#[ test ]
fn part_two () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("44169", puzzle.part_two (EXAMPLE));
}
