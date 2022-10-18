#![ cfg (test) ]

use super::*;

const EXAMPLE: & [& str] = & [
	"[1518-11-01 00:00] Guard #10 begins shift",
	"[1518-11-01 00:05] falls asleep",
	"[1518-11-01 00:25] wakes up",
	"[1518-11-01 00:30] falls asleep",
	"[1518-11-01 00:55] wakes up",
	"[1518-11-01 23:58] Guard #99 begins shift",
	"[1518-11-02 00:40] falls asleep",
	"[1518-11-02 00:50] wakes up",
	"[1518-11-03 00:05] Guard #10 begins shift",
	"[1518-11-03 00:24] falls asleep",
	"[1518-11-03 00:29] wakes up",
	"[1518-11-04 00:02] Guard #99 begins shift",
	"[1518-11-04 00:36] falls asleep",
	"[1518-11-04 00:46] wakes up",
	"[1518-11-05 00:03] Guard #99 begins shift",
	"[1518-11-05 00:45] falls asleep",
	"[1518-11-05 00:55] wakes up",
];

#[ test ]
fn part_one () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("240", puzzle.part_one (EXAMPLE));
}

#[ test ]
fn part_two () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("4455", puzzle.part_two (EXAMPLE));
}
