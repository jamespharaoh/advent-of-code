#![ cfg (test) ]

use super::*;

const EXAMPLE: & [& str] = & [
	"        ...#",
	"        .#..",
	"        #...",
	"        ....",
	"...#.......#",
	"........#...",
	"..#....#....",
	"..........#.",
	"        ...#....",
	"        .....#..",
	"        .#......",
	"        ......#.",
	"",
	"10R5L5R10L4R5L5",
];

#[ test ]
fn part_one () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("6032", puzzle.part_one (EXAMPLE));
}

#[ test ]
fn part_two () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("5031", puzzle.part_two (EXAMPLE));
}
