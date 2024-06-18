#![ cfg (test) ]

use super::*;

const EXAMPLE: & [& str] = & [
	"root: pppw + sjmn",
	"dbpl: 5",
	"cczh: sllz + lgvd",
	"zczc: 2",
	"ptdq: humn - dvpt",
	"dvpt: 3",
	"lfqf: 4",
	"humn: 5",
	"ljgn: 2",
	"sjmn: drzm * dbpl",
	"sllz: 4",
	"pppw: cczh / lfqf",
	"lgvd: ljgn * ptdq",
	"drzm: hmdt - zczc",
	"hmdt: 32",
];

#[ test ]
fn part_one () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("152", puzzle.part_one (EXAMPLE));
}

#[ test ]
fn part_two () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("301", puzzle.part_two (EXAMPLE));
}
