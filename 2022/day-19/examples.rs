#![ cfg (test) ]

use super::*;

const EXAMPLE: & [& str] = & [
	"Blueprint 1: \
	Each ore robot costs 4 ore. \
	Each clay robot costs 2 ore. \
	Each obsidian robot costs 3 ore and 14 clay. \
	Each geode robot costs 2 ore and 7 obsidian.",
	"Blueprint 2: \
	Each ore robot costs 2 ore. \
	Each clay robot costs 3 ore. \
	Each obsidian robot costs 3 ore and 8 clay. \
	Each geode robot costs 3 ore and 12 obsidian.",
];

#[ test ]
fn part_one () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("33", puzzle.part_one (EXAMPLE));
}

#[ test ]
fn part_two () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("3472", puzzle.part_two (EXAMPLE));
}
