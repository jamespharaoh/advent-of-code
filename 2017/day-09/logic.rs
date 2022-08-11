use super::*;
use input::Input;

pub fn part_one (input: & Input) -> GenResult <u32> {
	let (score, _) = calc_result (input) ?;
	Ok (score)
}

pub fn part_two (input: & Input) -> GenResult <u32> {
	let (_, garbage) = calc_result (input) ?;
	Ok (garbage)
}

fn calc_result (input: & Input) -> GenResult <(u32, u32)> {
	#[ derive (Clone, Copy) ]
	enum State { Normal, Garbage, Cancel }
	use State::{ Normal, Garbage, Cancel };
	let mut depth = 0_u32;
	let mut state = Normal;
	let mut score = 0_u32;
	let mut garbage = 0_u32;
	for ch in input.input.chars () {
		match (state, ch) {
			(Normal, '{') => depth += 1,
			(Normal, '}') => {
				if depth == 0 { return Err ("Unexpected '}'".into ()) }
				score += depth;
				depth -= 1;
			},
			(Normal, '<') => state = Garbage,
			(Normal, _) => (),
			(Garbage, '>') => state = Normal,
			(Garbage, '!') => state = Cancel,
			(Garbage, _) => garbage += 1,
			(Cancel, _) => state = Garbage,
		}
	}
	match state {
		Normal => if depth != 0 { return Err ("Stream ended inside group".into ()) },
		Garbage => return Err ("Stream ended during garbage".into ()),
		Cancel => return Err ("Stream ended during garbage cancel".into ()),
	}
	Ok ((score, garbage))
}
