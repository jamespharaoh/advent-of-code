#![ cfg (test) ]

use super::*;

const EXAMPLE_0: & [& str] = & [ "mjqjpqmgbljsphdztnvjfqwrcgsmlb" ];
const EXAMPLE_1: & [& str] = & [ "bvwbjplbgvbhsrlpgdmjqwftvncz" ];
const EXAMPLE_2: & [& str] = & [ "nppdvjthqldpwncqszvftbrmjlhg" ];
const EXAMPLE_3: & [& str] = & [ "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg" ];
const EXAMPLE_4: & [& str] = & [ "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw" ];

#[ test ]
fn part_one () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("7", puzzle.part_one (EXAMPLE_0));
	assert_eq_ok! ("5", puzzle.part_one (EXAMPLE_1));
	assert_eq_ok! ("6", puzzle.part_one (EXAMPLE_2));
	assert_eq_ok! ("10", puzzle.part_one (EXAMPLE_3));
	assert_eq_ok! ("11", puzzle.part_one (EXAMPLE_4));
}

#[ test ]
fn part_two () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("19", puzzle.part_two (EXAMPLE_0));
	assert_eq_ok! ("23", puzzle.part_two (EXAMPLE_1));
	assert_eq_ok! ("23", puzzle.part_two (EXAMPLE_2));
	assert_eq_ok! ("29", puzzle.part_two (EXAMPLE_3));
	assert_eq_ok! ("26", puzzle.part_two (EXAMPLE_4));
}
