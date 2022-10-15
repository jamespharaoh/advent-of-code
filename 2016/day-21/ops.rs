use super::*;

use input::ScrambleOp;
use model::Mode;
use model::State;

/// Apply the provided [`ScrambleOp`], in the provided [`Mode`], to the provided [`State`].
///
/// This operates in-place on a mutable reference.
///
/// We also requires the passing of a second, temporary buffer which is used for some operations
/// to allow a simple and efficient algorithm without requiring memory allocation. The contents of
/// this buffer after the operation are undefined.
///
pub fn apply (
	state: & mut State,
	state_temp: & mut State,
	mode: Mode,
	op: ScrambleOp,
) -> GenResult <()> {
	match (mode, op) {
		(_, ScrambleOp::SwapPosns (pos_0, pos_1)) =>
			ops::swap_posns (state, pos_0.pan_usize (), pos_1.pan_usize ()),
		(_, ScrambleOp::SwapChars (ch_0, ch_1)) =>
			ops::swap_chars (state, ch_0, ch_1),
		(Mode::Scramble, ScrambleOp::RotLeft (num)) |
		(Mode::Unscramble, ScrambleOp::RotRight (num)) =>
			ops::rotate (state, state_temp, num.pan_usize ()),
		(Mode::Scramble, ScrambleOp::RotRight (num)) |
		(Mode::Unscramble, ScrambleOp::RotLeft (num)) => {
			let num = chk! (state.len () - num.pan_usize ()) ?;
			ops::rotate (state, state_temp, num)
		},
		(Mode::Scramble, ScrambleOp::RotChar (ch)) =>
			ops::rotate_char_scramble (state, state_temp, ch),
		(Mode::Unscramble, ScrambleOp::RotChar (ch)) =>
			ops::rotate_char_unscramble (state, state_temp, ch),
		(_, ScrambleOp::Reverse (pos_0, pos_1)) =>
			ops::reverse (state, pos_0.pan_usize (), pos_1.pan_usize ()),
		(Mode::Scramble, ScrambleOp::Move (pos_0, pos_1)) |
		(Mode::Unscramble, ScrambleOp::Move (pos_1, pos_0)) =>
			ops::move_shift (state, pos_0.pan_usize (), pos_1.pan_usize ()),
	}
}

pub fn swap_posns (
	state: & mut State,
	pos_0: usize,
	pos_1: usize,
) -> GenResult <()> {
	if pos_0 >= state.len () || pos_1 >= state.len () {
		return Err ("Swap positions must be inside password".into ());
	}
	let ch_0 = state [pos_0];
	let ch_1 = state [pos_1];
	state [pos_0] = ch_1;
	state [pos_1] = ch_0;
	Ok (())
}

pub fn swap_chars (
	state: & mut State,
	ch_0: char,
	ch_1: char,
) -> GenResult <()> {
	for ch in state.iter_mut () {
		* ch = match * ch {
			ch if ch == ch_0 => ch_1,
			ch if ch == ch_1 => ch_0,
			ch => ch,
		}
	}
	Ok (())
}

pub fn rotate (
	state: & mut State,
	state_temp: & mut State,
	num: usize,
) -> GenResult <()> {
	state_temp.clear ();
	state_temp.extend (state.iter ().skip (num).copied ());
	state_temp.extend (state.iter ().take (num).copied ());
	mem::swap (state, state_temp);
	Ok (())
}

pub fn rotate_char_scramble (
	state: & mut State,
	state_temp: & mut State,
	ch: char,
) -> GenResult <()> {
	if let Some (pos) = state.iter ().position (|& some_ch| some_ch == ch) {
		let num = (if pos >= 4 { 2 } else { 1 } + pos) % state.len ();
		state_temp.clear ();
		state_temp.extend (state.iter ().skip (state.len () - num).copied ());
		state_temp.extend (state.iter ().take (state.len () - num).copied ());
		mem::swap (state, state_temp);
	}
	Ok (())
}

pub fn rotate_char_unscramble (
	state: & mut State,
	state_temp: & mut State,
	ch: char,
) -> GenResult <()> {
	if let Some (pos) = state.iter ().position (|& some_ch| some_ch == ch) {
		let num = match pos {
			0 => 1, 1 => 1, 2 => 6, 3 => 2,
			4 => 7, 5 => 3, 6 => 0, 7 => 4,
			_ => unreachable! (),
		};
		state_temp.clear ();
		state_temp.extend (state.iter ().skip (num).copied ());
		state_temp.extend (state.iter ().take (num).copied ());
		mem::swap (state, state_temp);
	}
	Ok (())
}

pub fn reverse (
	state: & mut State,
	mut pos_0: usize,
	mut pos_1: usize,
) -> GenResult <()> {
	if state.len () <= pos_0 || state.len () <= pos_1 {
		return Err ("Reverse range outside of password length".into ());
	}
	while pos_0 < pos_1 {
		let ch_0 = state [pos_0];
		let ch_1 = state [pos_1];
		state [pos_0] = ch_1;
		state [pos_1] = ch_0;
		pos_0 += 1;
		pos_1 -= 1;
	}
	Ok (())
}

pub fn move_shift (
	state: & mut State,
	pos_0: usize,
	pos_1: usize,
) -> GenResult <()> {
	if pos_0 >= state.len () || pos_1 >= state.len () {
		return Err ("Move position outside of password length".into ());
	}
	let ch = state.remove (pos_0);
	state.insert (pos_1, ch);
	Ok (())
}
