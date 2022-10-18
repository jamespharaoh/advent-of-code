#![ cfg (test) ]

use super::*;

const EXAMPLE: & [& str] = & [
	"NUM_WORKERS=2",
	"EXTRA_TIME=0",
	"Step C must be finished before step A can begin.",
	"Step C must be finished before step F can begin.",
	"Step A must be finished before step B can begin.",
	"Step A must be finished before step D can begin.",
	"Step B must be finished before step E can begin.",
	"Step D must be finished before step E can begin.",
	"Step F must be finished before step E can begin.",
];

#[ test ]
fn part_one () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("CABDFE", puzzle.part_one (EXAMPLE));
}

#[ test ]
fn part_two () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("15", puzzle.part_two (EXAMPLE));
}
