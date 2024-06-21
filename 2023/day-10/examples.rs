#![ cfg (test) ]

use super::*;

const EXAMPLE_ONE_0: & [& str] = & [
	"-L|F7",
	"7S-7|",
	"L|7||",
	"-L-J|",
	"L|-JF",
];

const EXAMPLE_ONE_1: & [& str] = & [
	"7-F7-",
	".FJ|7",
	"SJLL7",
	"|F--J",
	"LJ.LJ",
];

const EXAMPLE_TWO: & [& str] = & [
	"FF7FSF7F7F7F7F7F---7",
	"L|LJ||||||||||||F--J",
	"FL-7LJLJ||||||LJL-77",
	"F--JF--7||LJLJ7F7FJ-",
	"L---JF-JLJ.||-FJLJJ7",
	"|F|F-JF---7F7-L7L|7|",
	"|FFJF7L7F-JF7|JL---7",
	"7-L-JL7||F7|L7F-7F7|",
	"L.L7LFJ|||||FJL7||LJ",
	"L7JLJL-JLJLJL--JLJ.L",
];

#[ test ]
fn part_one () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("4", puzzle.part_one (EXAMPLE_ONE_0));
	assert_eq_ok! ("8", puzzle.part_one (EXAMPLE_ONE_1));
}

#[ test ]
fn part_two () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("10", puzzle.part_two (EXAMPLE_TWO));
}
