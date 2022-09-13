#![ cfg (test) ]

use super::*;

#[ test ]
fn part_one () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("24176176", puzzle.part_one (& [ "80871224585914546619083218645595" ]));
	assert_eq_ok! ("73745418", puzzle.part_one (& [ "19617804207202209144916044189917" ]));
	assert_eq_ok! ("52432133", puzzle.part_one (& [ "69317163492948606335995924319873" ]));
}

#[ test ]
fn part_two () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("84462026", puzzle.part_two (& [ "03036732577212944063491565474664" ]));
	assert_eq_ok! ("78725270", puzzle.part_two (& [ "02935109699940807407585447034323" ]));
	assert_eq_ok! ("53553731", puzzle.part_two (& [ "03081770884921959731165446850517" ]));
}
