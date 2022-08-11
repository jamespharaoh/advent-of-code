use super::*;
use cpu::Cpu;
use cpu::Val;
use input::Input;

pub fn part_one (input: & Input) -> GenResult <Val> {
	let mut cpu = Cpu::new (& * input.instrs);
	cpu.execute ();
	Ok (
		cpu.regs ()
			.map (|(_, & val)| val)
			.max ()
			.unwrap_or (Val::ZERO)
	)
}

pub fn part_two (input: & Input) -> GenResult <Val> {
	let mut cpu = Cpu::new (& * input.instrs);
	Ok (
		iter::from_fn (|| cpu.is_ready ().then (|| cpu.step ()))
			.flatten ()
			.map (|(_, val)| val)
			.max ()
			.unwrap_or (Val::ZERO)
	)
}
