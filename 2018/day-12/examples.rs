#![ cfg (test) ]

use super::*;

const EXAMPLE: & [& str] = & [
	"CHECK_RULES=false",
	"initial state: #..#.#..##......###...###",
	"",
	"...## => #",
	"..#.. => #",
	".#... => #",
	".#.#. => #",
	".#.## => #",
	".##.. => #",
	".#### => #",
	"#.#.# => #",
	"#.### => #",
	"##.#. => #",
	"##.## => #",
	"###.. => #",
	"###.# => #",
	"####. => #",
];

#[ test ]
fn part_one () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("325", puzzle.part_one (EXAMPLE));
}

#[ test ]
fn part_two () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("999999999374", puzzle.part_two (EXAMPLE));
}
