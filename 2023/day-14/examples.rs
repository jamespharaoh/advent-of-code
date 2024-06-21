#![ cfg (test) ]

use super::*;

const EXAMPLE: & [& str] = & [
	"O....#....",
	"O.OO#....#",
	".....##...",
	"OO.#O....O",
	".O.....O#.",
	"O.#..O.#.#",
	"..O..#O..O",
	".......O..",
	"#....###..",
	"#OO..#....",
];

#[ test ]
fn part_one () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("136", puzzle.part_one (EXAMPLE));
}

#[ test ]
fn part_two () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("64", puzzle.part_two (EXAMPLE));
}
