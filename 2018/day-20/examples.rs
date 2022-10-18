#![ cfg (test) ]

use super::*;

const EXAMPLES: & [& str] = & [
	"^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$",
	"^ESSWWN(E|NNENN(EESS(WNSE|)SSS|WWWSSSSE(SW|NNNE)))$",
	"^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$",
];

#[ test ]
fn part_one () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("18", puzzle.part_one (& [ EXAMPLES [0] ]));
	assert_eq_ok! ("23", puzzle.part_one (& [ EXAMPLES [1] ]));
	assert_eq_ok! ("31", puzzle.part_one (& [ EXAMPLES [2] ]));
}

#[ test ]
fn part_two () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("13", puzzle.part_two (& [ "DIST_TWO=10", EXAMPLES [0] ]));
	assert_eq_ok! ("25", puzzle.part_two (& [ "DIST_TWO=10", EXAMPLES [1] ]));
	assert_eq_ok! ("39", puzzle.part_two (& [ "DIST_TWO=10", EXAMPLES [2] ]));
}
