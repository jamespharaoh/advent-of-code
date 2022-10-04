#![ cfg (test) ]

use super::*;

const EXAMPLE: & [& str] = & [
	"1163751742", "1381373672", "2136511328", "3694931569", "7463417111",
	"1319128137", "1359912421", "3125421639", "1293138521", "2311944581",
];

#[ test ]
fn part_one () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("40", puzzle.part_one (EXAMPLE));
}

#[ test ]
fn part_two () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("315", puzzle.part_two (EXAMPLE));
}
