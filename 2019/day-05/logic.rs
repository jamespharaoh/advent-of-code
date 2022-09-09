//! Logic for solving the puzzles

use super::*;
use input::Input;
use model::Cpu;
use model::Val;

pub fn part_one (input: & Input) -> GenResult <Val> {
	calc_result (input, 1)
}

pub fn part_two (input: & Input) -> GenResult <Val> {
	calc_result (input, 5)
}

fn calc_result (input: & Input, mode: Val) -> GenResult <Val> {
	let mut cpu = Cpu::new (input.data.clone ());
	cpu.set_max_ops (300);
	cpu.input (mode);
	let mut last_output = None;
	while let Some (val) = cpu.run ().output () ? {
		last_output = Some (val);
	}
	Ok (last_output.ok_or ("No diagnostic code received") ?)
}
