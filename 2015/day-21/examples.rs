#![ cfg (test) ]

use super::*;

const EXAMPLES: & [& [& str]] = & [
	& [ "Hit Points: 100", "Damage: 3", "Armor: 3" ],
	& [ "Hit Points: 100", "Damage: 3", "Armor: 4" ],
	& [ "Hit Points: 100", "Damage: 3", "Armor: 5" ],
	& [ "Hit Points: 100", "Damage: 4", "Armor: 3" ],
	& [ "Hit Points: 100", "Damage: 4", "Armor: 4" ],
	& [ "Hit Points: 100", "Damage: 4", "Armor: 5" ],
	& [ "Hit Points: 100", "Damage: 5", "Armor: 3" ],
	& [ "Hit Points: 100", "Damage: 5", "Armor: 4" ],
	& [ "Hit Points: 100", "Damage: 5", "Armor: 5" ],
];

#[ test ]
fn part_one () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("23", puzzle.part_one (EXAMPLES [0]));
	assert_eq_ok! ("38", puzzle.part_one (EXAMPLES [1]));
	assert_eq_ok! ("39", puzzle.part_one (EXAMPLES [2]));
	assert_eq_ok! ("38", puzzle.part_one (EXAMPLES [3]));
	assert_eq_ok! ("53", puzzle.part_one (EXAMPLES [4]));
	assert_eq_ok! ("59", puzzle.part_one (EXAMPLES [5]));
	assert_eq_ok! ("53", puzzle.part_one (EXAMPLES [6]));
	assert_eq_ok! ("71", puzzle.part_one (EXAMPLES [7]));
	assert_eq_ok! ("79", puzzle.part_one (EXAMPLES [8]));
}

#[ test ]
fn part_two () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("33", puzzle.part_two (EXAMPLES [0]));
	assert_eq_ok! ("58", puzzle.part_two (EXAMPLES [1]));
	assert_eq_ok! ("108", puzzle.part_two (EXAMPLES [2]));
	assert_eq_ok! ("58", puzzle.part_two (EXAMPLES [3]));
	assert_eq_ok! ("108", puzzle.part_two (EXAMPLES [4]));
	assert_eq_ok! ("133", puzzle.part_two (EXAMPLES [5]));
	assert_eq_ok! ("108", puzzle.part_two (EXAMPLES [6]));
	assert_eq_ok! ("133", puzzle.part_two (EXAMPLES [7]));
	assert_eq_ok! ("158", puzzle.part_two (EXAMPLES [8]));
}
