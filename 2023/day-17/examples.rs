#![ cfg (test) ]

use super::*;

const EXAMPLE_0: & [& str] = & [
	"2413432311323",
	"3215453535623",
	"3255245654254",
	"3446585845452",
	"4546657867536",
	"1438598798454",
	"4457876987766",
	"3637877979653",
	"4654967986887",
	"4564679986453",
	"1224686865563",
	"2546548887735",
	"4322674655533",
];

const EXAMPLE_1: & [& str] = & [
	"111111111111",
	"999999999991",
	"999999999991",
	"999999999991",
	"999999999991",
];

#[ test ]
fn part_one () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("102", puzzle.part_one (EXAMPLE_0));
	assert_eq_ok! ("59", puzzle.part_one (EXAMPLE_1));
}

#[ test ]
fn part_two () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("94", puzzle.part_two (EXAMPLE_0));
	assert_eq_ok! ("71", puzzle.part_two (EXAMPLE_1));
}
