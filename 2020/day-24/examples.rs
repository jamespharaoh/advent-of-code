#![ cfg (test) ]

use super::*;

const EXAMPLE: & [& str] = & [
	"sesenwnenenewseeswwswswwnenewsewsw",
	"neeenesenwnwwswnenewnwwsewnenwseswesw",
	"seswneswswsenwwnwse",
	"nwnwneseeswswnenewneswwnewseswneseene",
	"swweswneswnenwsewnwneneseenw",
	"eesenwseswswnenwswnwnwsewwnwsene",
	"sewnenenenesenwsewnenwwwse",
	"wenwwweseeeweswwwnwwe",
	"wsweesenenewnwwnwsenewsenwwsesesenwne",
	"neeswseenwwswnwswswnw",
	"nenwswwsewswnenenewsenwsenwnesesenew",
	"enewnwewneswsewnwswenweswnenwsenwsw",
	"sweneswneswneneenwnewenewwneswswnese",
	"swwesenesewenwneswnwwneseswwne",
	"enesenwswwswneneswsenwnewswseenwsese",
	"wnwnesenesenenwwnenwsewesewsesesew",
	"nenewswnwewswnenesenwnesewesw",
	"eneswnwswnwsenenwnwnwwseeswneewsenese",
	"neswnwewnwnwseenwseesewsenwsweewe",
	"wseweeenwnesenwwwswnew",
];

#[ test ]
fn part_one () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("10", puzzle.part_one (EXAMPLE));
}

#[ test ]
fn part_two () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("2208", puzzle.part_two (EXAMPLE));
}

#[ test ]
fn part_two_bits () {
	let input = input::Input::parse_from_lines (EXAMPLE).unwrap ();
	assert_eq_ok! (2208, logic::part_two_bits (& input));
}
