//! Logic for solving the puzzles

use super::*;
use input::Input;
use model::Cpu;
use model::Val;

pub fn part_one (input: & Input) -> GenResult <Val> {
	calc_result (input, 12, 2)
}

pub fn part_two (input: & Input) -> GenResult <Val> {
	for noun in 0_i32 .. 100_i32 {
		for verb in 0_i32 .. 100_i32 {
			if calc_result (input, noun, verb) ? != 19_690_720_i32 { continue }
			return Ok (noun * 100_i32 + verb)
		}
	}
	Err ("No solution found".into ())
}

fn calc_result (input: & Input, noun: Val, verb: Val) -> GenResult <Val> {
	let mut cpu = Cpu::new (input.data.clone ());
	cpu.set_max_ops (input.params.max_ops);
	cpu.mem_set (1_i32, noun).unwrap ();
	cpu.mem_set (2_i32, verb).unwrap ();
	cpu.run ().halt () ?;
	Ok (cpu.mem_get (0_i32).unwrap ())
}
