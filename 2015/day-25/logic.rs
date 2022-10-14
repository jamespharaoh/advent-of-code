//! Logic for solving the puzzles.

use super::*;

use input::Input;

pub fn part_one (input: & Input) -> GenResult <u64> {
	let diag_num = chk! (input.row + input.col - 2) ?;
	let diag_seq = chk! (diag_num * (diag_num + 1_u64) / 2) ?;
	let mut cell_seq = chk! (diag_seq + input.col - 1) ?;
	let mut code = 20_151_125_u64;
	let mut mul = 252_533;
	while cell_seq != 0 {
		if cell_seq & 1 == 1 { code = chk! (code * mul % 33_554_393) ?; }
		cell_seq >>= 1_i32;
		mul = chk! (mul * mul % 33_554_393) ?;
	}
	Ok (code)
}
