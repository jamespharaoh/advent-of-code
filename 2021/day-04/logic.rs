use super::*;

use input::Input;
use model::Board;
use model::Dir;
use model::Grid;
use model::Pos;

pub fn part_one (input: & Input) -> GenResult <u32> {
	check_input (input) ?;
	Ok (
		input.boards.iter ()
			.filter_map (|board| calc_win_turn (input, board).map (|turn| (board, turn)))
			.min_by_key (|& (_, turn)| turn)
			.map (|(board, turn)| calc_score (input, board, turn))
			.ok_or ("No solution found") ?
	)
}

pub fn part_two (input: & Input) -> GenResult <u32> {
	check_input (input) ?;
	Ok (
		input.boards.iter ()
			.filter_map (|board| calc_win_turn (input, board).map (|turn| (board, turn)))
			.max_by_key (|& (_, turn)| turn)
			.map (|(board, turn)| calc_score (input, board, turn))
			.ok_or ("No solution found") ?
	)
}

fn check_input (input: & Input) -> GenResult <()> {
	if input.boards.is_empty () {
		return Err ("Must have at least one board".into ());
	}
	for board in & input.boards {
		if board.size ().row < 2 || board.size ().col < 2 {
			return Err ("Board size must be at least 2×2".into ());
		}
	}
	Ok (())
}

fn calc_win_turn (input: & Input, board: & Board) -> Option <u16> {
	let call_order = & input.call_order;
	let turns: Grid <Option <u16>> =
		board.map (|cur| {
			let num = cur.get (board);
			call_order.iter ()
				.position (|& called_num| called_num == num)
				.map (usize::pan_u16)
		});
	let turns = & turns;
	let down = turns.offset (Dir::Down).unwrap ();
	let right = turns.offset (Dir::Right).unwrap ();
	let cur = turns.cursor (Pos::ZERO).unwrap ();
	[ (down, right), (right, down) ].into_iter ()
		.flat_map (|(dir_0, dir_1)| cur.walk (dir_0)
			.map (move |cur| cur.walk (dir_1)
				.map (|cur| cur.get (turns))
				.max ()
				.unwrap ()))
		.flatten ()
		.min ()
}

fn calc_score (input: & Input, board: & Board, turn: u16) -> u32 {
	let called = & input.call_order [ .. turn.pan_usize () + 1];
	let uncalled_sum: u32 =
		board.values ()
			.filter (|& num| ! called.contains (& num))
			.map (u8::pan_u32)
			.sum ();
	called [called.len () - 1].pan_u32 () * uncalled_sum
}
