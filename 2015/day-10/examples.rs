#![ cfg (test) ]

use super::*;

use model::State;

#[ test ]
fn one_round () -> GenResult <()> {
	let mut state = State::parse ("1") ?;
	for expect in [ "11", "21", "1211", "111221", "312211" ] {
		state = logic::one_round (& state);
		let expect = State::parse (expect) ?;
		assert_eq! (expect, state);
	}
	Ok (())
}
