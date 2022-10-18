#![ cfg (test) ]

use super::*;

const EXAMPLE_0: & [& str] = & [
	"#######",
	"#.G...#",
	"#...EG#",
	"#.#.#G#",
	"#..G#E#",
	"#.....#",
	"#######",
];

const EXAMPLE_1: & [& str] = & [
	"#######",
	"#G..#E#",
	"#E#E.E#",
	"#G.##.#",
	"#...#E#",
	"#...E.#",
	"#######",
];

const EXAMPLE_2: & [& str] = & [
	"#######",
	"#E..EG#",
	"#.#G.E#",
	"#E.##E#",
	"#G..#.#",
	"#..E#.#",
	"#######",
];

const EXAMPLE_3: & [& str] = & [
	"#######",
	"#E.G#.#",
	"#.#G..#",
	"#G.#.G#",
	"#G..#.#",
	"#...E.#",
	"#######",
];

const EXAMPLE_4: & [& str] = & [
	"#######",
	"#.E...#",
	"#.#..G#",
	"#.###.#",
	"#E#G#G#",
	"#...#G#",
	"#######",
];

const EXAMPLE_5: & [& str] = & [
	"#########",
	"#G......#",
	"#.E.#...#",
	"#..##..G#",
	"#...##..#",
	"#...#...#",
	"#.G...G.#",
	"#.....G.#",
	"#########",
];

#[ test ]
fn part_one () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("27730", puzzle.part_one (EXAMPLE_0));
	assert_eq_ok! ("36334", puzzle.part_one (EXAMPLE_1));
	assert_eq_ok! ("39514", puzzle.part_one (EXAMPLE_2));
	assert_eq_ok! ("27755", puzzle.part_one (EXAMPLE_3));
	assert_eq_ok! ("28944", puzzle.part_one (EXAMPLE_4));
	assert_eq_ok! ("18740", puzzle.part_one (EXAMPLE_5));
}

#[ test ]
fn part_two () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("4988", puzzle.part_two (EXAMPLE_0));
	assert_eq_ok! ("29064", puzzle.part_two (EXAMPLE_1));
	assert_eq_ok! ("31284", puzzle.part_two (EXAMPLE_2));
	assert_eq_ok! ("3478", puzzle.part_two (EXAMPLE_3));
	assert_eq_ok! ("6474", puzzle.part_two (EXAMPLE_4));
	assert_eq_ok! ("1140", puzzle.part_two (EXAMPLE_5));
}
