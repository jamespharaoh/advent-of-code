//! Logic for solving the puzzles

use super::*;
use input::Input;
use model::Cpu;
use model::Val;

pub fn part_one (input: & Input) -> GenResult <Val> {
	calc_result (input, 1, 500)
}

pub fn part_two (input: & Input) -> GenResult <Val> {
	calc_result (input, 2, 500_000)
}

fn calc_result (input: & Input, mode: Val, max_ops: u32) -> GenResult <Val> {
	let mut cpu = Cpu::new (input.data.clone ());
	cpu.set_max_ops (max_ops);
	cpu.input (mode);
	Ok (cpu.run ().output () ?.ok_or ("No output from program") ?)
}
