use super::*;

use input::Input;
use input::ScrambleOp;
use model::Mode;
use model::State;

pub fn part_one (input: & Input) -> GenResult <String> {
	calc_result (& input.ops, & input.params.start_one, Mode::Scramble)
}

pub fn part_two (input: & Input) -> GenResult <String> {
	calc_result (& input.ops, & input.params.start_two, Mode::Unscramble)
}

fn calc_result (ops: & [ScrambleOp], start: & str, mode: Mode) -> GenResult <String> {

	// sanity checks

	if ! (1 ..= 26).contains (& start.len ()) {
		return Err ("Password must have between one and sixteen characters".into ());
	}
	if ! start.chars ().all (|ch| ch.is_ascii_lowercase ()) {
		return Err ("Password must contains only lowercase ASCII characters".into ());
	}
	if ! start.chars ().all_unique () {
		return Err ("Password must contain only unique characters".into ());
	}
	if mode == Mode::Unscramble && start.chars ().count () != 8 {
		return Err ("Can only unscramble password with length 8".into ());
	}

	// convert password to chars, allocate temp buffer

	let mut state: State = start.chars ().collect ();
	let mut state_temp: State = Vec::with_capacity (state.len ());

	// iterate over operations, in reverse order for unscramble

	match mode {
		Mode::Scramble => {
			for & op in ops {
				ops::apply (& mut state, & mut state_temp, mode, op) ?;
			}
		},
		Mode::Unscramble => {
			for & op in ops.iter ().rev () {
				ops::apply (& mut state, & mut state_temp, mode, op) ?;
			}
		},
	};

	// collect back into string and return

	Ok (state.into_iter ().collect ())

}
