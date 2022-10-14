#![ cfg (test) ]

use super::*;

#[ test ]
fn part_one () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("491", puzzle.part_one (& [ "Hit Points: 40", "Damage: 8" ]));
	assert_eq_ok! ("787", puzzle.part_one (& [ "Hit Points: 50", "Damage: 8" ]));
	assert_eq_ok! ("1249", puzzle.part_one (& [ "Hit Points: 60", "Damage: 8" ]));
	assert_eq_ok! ("734", puzzle.part_one (& [ "Hit Points: 40", "Damage: 9" ]));
	assert_eq_ok! ("900", puzzle.part_one (& [ "Hit Points: 50", "Damage: 9" ]));
	assert_eq_ok! ("1269", puzzle.part_one (& [ "Hit Points: 60", "Damage: 9" ]));
	assert_eq_ok! ("754", puzzle.part_one (& [ "Hit Points: 40", "Damage: 10" ]));
	assert_eq_ok! ("900", puzzle.part_one (& [ "Hit Points: 50", "Damage: 10" ]));
	assert_eq_ok! ("1309", puzzle.part_one (& [ "Hit Points: 60", "Damage: 10" ]));
}

#[ test ]
fn part_two () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("734", puzzle.part_two (& [ "Hit Points: 40", "Damage: 8" ]));
	assert_eq_ok! ("900", puzzle.part_two (& [ "Hit Points: 50", "Damage: 8" ]));
	assert_eq_ok! ("1309", puzzle.part_two (& [ "Hit Points: 60", "Damage: 8" ]));
	assert_eq_ok! ("754", puzzle.part_two (& [ "Hit Points: 40", "Damage: 9" ]));
	assert_eq_ok! ("920", puzzle.part_two (& [ "Hit Points: 50", "Damage: 9" ]));
	assert_eq_ok! ("1309", puzzle.part_two (& [ "Hit Points: 60", "Damage: 9" ]));
	assert_eq_ok! ("794", puzzle.part_two (& [ "Hit Points: 40", "Damage: 10" ]));
	assert_eq_ok! ("1256", puzzle.part_two (& [ "Hit Points: 50", "Damage: 10" ]));
	assert_eq_ok! ("1442", puzzle.part_two (& [ "Hit Points: 60", "Damage: 10" ]));
}
