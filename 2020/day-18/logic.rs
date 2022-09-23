//! Logic for solving the puzzles

#![ allow (clippy::wildcard_enum_match_arm) ]

use super::*;

use input::Input;
use input::InputToken::{ self, ParenOpen, ParenClose, Number, Add, Mul };

pub type TokensIter <'tok> = Peekable <iter::Copied <SliceIter <'tok, InputToken>>>;

pub fn part_one (input: & Input) -> GenResult <u64> {
	input.exprs.iter ()
		.map (|expr| eval_one (& mut expr.iter ().copied ().peekable ()))
		.try_fold (0_u64, |sum, val| val.and_then (|val| Ok (chk! (sum + val) ?)))
}

pub fn part_two (input: & Input) -> GenResult <u64> {
	input.exprs.iter ()
		.map (|expr| eval_two (& mut expr.iter ().copied ().peekable ()))
		.try_fold (0_u64, |sum, val| {
			let val = val ?;
			Ok::<_, GenError> (chk! (sum + val) ?)
		})
}

fn eval_one (iter: & mut TokensIter) -> GenResult <u64> {

	fn level_0 (iter: & mut TokensIter, depth: u32) -> GenResult <u64> {
		if 1000 < depth { return Err ("Too much nesting".into ()) }
		let mut val = level_1 (iter, depth + 1) ?;
		loop {
			if matches! (iter.peek (), None | Some (& ParenClose)) { return Ok (val) }
			match iter.next ().unwrap () {
				Add => chk! (val += level_1 (iter, depth + 1) ?) ?,
				Mul => chk! (val *= level_1 (iter, depth + 1) ?) ?,
				other => return Err (format! ("Unexpected token: {other:?}").into ()),
			}
		}
	}

	fn level_1 (iter: & mut TokensIter, depth: u32) -> GenResult <u64> {
		if 1000 < depth { return Err ("Too much nesting".into ()) }
		match iter.next ().ok_or ("Unexpected end of token stream") ? {
			ParenOpen => {
				let result = level_0 (iter, depth + 1) ?;
				let next = iter.next ().ok_or ("Unexpected end of token stream") ?;
				if next != ParenClose { return Err ("Expected closing parenthesis".into ()) }
				Ok (result)
			},
			Number (val) => Ok (val.as_u64 ()),
			other => Err (format! ("Unexpected token: {other:?}").into ()),
		}
	}

	let val = level_0 (iter, 0) ?;
	if let Some (token) = iter.next () {
		return Err (format! ("Unexpected token: {token:?}").into ());
	}
	Ok (val)

}

fn eval_two (iter: & mut TokensIter) -> GenResult <u64> {

	fn level_0 (iter: & mut TokensIter, depth: u32) -> GenResult <u64> {
		if 1000 < depth { return Err ("Too much nesting".into ()) }
		let mut val = level_1 (iter, depth + 1) ?;
		loop {
			if matches! (iter.peek (), None | Some (& ParenClose)) { return Ok (val) }
			match iter.next ().unwrap () {
				InputToken::Mul => chk! (val *= level_1 (iter, depth + 1) ?) ?,
				other => return Err (format! ("Unexpected token: {other:?}").into ()),
			}
		}
	}

	fn level_1 (iter: & mut TokensIter, depth: u32) -> GenResult <u64> {
		if 1000 < depth { return Err ("Too much nesting".into ()) }
		let mut val = level_2 (iter, depth + 1) ?;
		loop {
			if matches! (iter.peek (), None | Some (& ParenClose | & Mul)) { return Ok (val) }
			match iter.next ().unwrap () {
				InputToken::Add => chk! (val += level_2 (iter, depth + 1) ?) ?,
				other => return Err (format! ("Unexpected token: {other:?}").into ()),
			}
		}
	}

	fn level_2 (iter: & mut TokensIter, depth: u32) -> GenResult <u64> {
		if 1000 < depth { return Err ("Too much nesting".into ()) }
		match iter.next ().ok_or ("Unexpected end of token stream") ? {
			ParenOpen => {
				let result = level_0 (iter, depth + 1) ?;
				let next = iter.next ().ok_or ("Unexpected end of token stream") ?;
				if next != ParenClose { return Err ("Expected closing parenthesis".into ()) }
				Ok (result)
			},
			Number (val) => Ok (val.as_u64 ()),
			other => Err (format! ("Unexpected token: {other:?}").into ()),
		}
	}

	let val = level_0 (iter, 0) ?;
	if let Some (token) = iter.next () {
		return Err (format! ("Unexpected token: {token:?}").into ());
	}
	Ok (val)

}
