#![ cfg (test) ]

use super::*;

const EXAMPLES: & [& [& str]] = & [
	& [ "0,3,6" ],
	& [ "1,3,2" ],
    & [ "2,1,3" ],
    & [ "1,2,3" ],
    & [ "2,3,1" ],
    & [ "3,2,1" ],
    & [ "3,1,2" ],
];

#[ test ]
fn part_one () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("436", puzzle.part_one (EXAMPLES [0]));
	assert_eq_ok! ("1", puzzle.part_one (EXAMPLES [1]));
	assert_eq_ok! ("10", puzzle.part_one (EXAMPLES [2]));
	assert_eq_ok! ("27", puzzle.part_one (EXAMPLES [3]));
	assert_eq_ok! ("78", puzzle.part_one (EXAMPLES [4]));
	assert_eq_ok! ("438", puzzle.part_one (EXAMPLES [5]));
	assert_eq_ok! ("1836", puzzle.part_one (EXAMPLES [6]));
}

#[ test ]
fn part_two_0 () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("175594", puzzle.part_two (EXAMPLES [0]));
}

#[ test ]
fn part_two_1 () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("2578", puzzle.part_two (EXAMPLES [1]));
}

#[ test ]
fn part_two_2 () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("3544142", puzzle.part_two (EXAMPLES [2]));
}

#[ test ]
fn part_two_3 () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("261214", puzzle.part_two (EXAMPLES [3]));
}

#[ test ]
fn part_two_4 () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("6895259", puzzle.part_two (EXAMPLES [4]));
}

#[ test ]
fn part_two_5 () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("18", puzzle.part_two (EXAMPLES [5]));
}

#[ test ]
fn part_two_6 () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("362", puzzle.part_two (EXAMPLES [6]));
}
