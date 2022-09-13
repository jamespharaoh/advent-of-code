#![ cfg (test) ]

use super::*;

const EXAMPLE_0: & [& str] = & [
	"#########",
	"#b.A.@.a#",
	"#########",
];

const EXAMPLE_1: & [& str] = & [
	"########################",
	"#f.D.E.e.C.b.A.@.a.B.c.#",
	"######################.#",
	"#d.....................#",
	"########################",
];

const EXAMPLE_2: & [& str] = & [
	"########################",
	"#...............b.C.D.f#",
	"#.######################",
	"#.....@.a.B.c.d.A.e.F.g#",
	"########################",
];

const EXAMPLE_3: & [& str] = & [
	"#################",
	"#i.G..c...e..H.p#",
	"########.########",
	"#j.A..b...f..D.o#",
	"########@########",
	"#k.E..a...g..B.n#",
	"########.########",
	"#l.F..d...h..C.m#",
	"#################",
];

const EXAMPLE_4: & [& str] = & [
	"########################",
	"#@..............ac.GI.b#",
	"###d#e#f################",
	"###A#B#C################",
	"###g#h#i################",
	"########################",
];

const EXAMPLE_5: & [& str] = & [
	"#######",
	"#a.#Cd#",
	"##...##",
	"##.@.##",
	"##...##",
	"#cB#Ab#",
	"#######",
];

const EXAMPLE_6: & [& str] = & [
	"###############",
	"#d.ABC.#.....a#",
	"######...######",
	"######.@.######",
	"######...######",
	"#b.....#.....c#",
	"###############",
];

const EXAMPLE_7: & [& str] = & [
	"#############",
	"#DcBa.#.GhKl#",
	"#.###...#I###",
	"#e#d#.@.#j#k#",
	"###C#...###J#",
	"#fEbA.#.FgHi#",
	"#############",
];

const EXAMPLE_8: & [& str] = & [
	"#############",
	"#g#f.D#..h#l#",
	"#F###e#E###.#",
	"#dCba...BcIJ#",
	"#####.@.#####",
	"#nK.L...G...#",
	"#M###N#H###.#",
	"#o#m..#i#jk.#",
	"#############",
];

#[ test ]
fn part_one () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("8", puzzle.part_one (EXAMPLE_0));
	assert_eq_ok! ("86", puzzle.part_one (EXAMPLE_1));
	assert_eq_ok! ("132", puzzle.part_one (EXAMPLE_2));
	assert_eq_ok! ("136", puzzle.part_one (EXAMPLE_3));
	assert_eq_ok! ("81", puzzle.part_one (EXAMPLE_4));
	assert_eq_ok! ("26", puzzle.part_one (EXAMPLE_5));
	assert_eq_ok! ("50", puzzle.part_one (EXAMPLE_6));
	assert_eq_ok! ("127", puzzle.part_one (EXAMPLE_7));
	assert_eq_ok! ("114", puzzle.part_one (EXAMPLE_8));
}

#[ test ]
fn part_two () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("8", puzzle.part_two (EXAMPLE_5));
	assert_eq_ok! ("24", puzzle.part_two (EXAMPLE_6));
	assert_eq_ok! ("32", puzzle.part_two (EXAMPLE_7));
	assert_eq_ok! ("72", puzzle.part_two (EXAMPLE_8));
}
