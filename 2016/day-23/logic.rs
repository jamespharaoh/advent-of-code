use super::*;

use cpu::Cpu;
use input::Input;

pub fn part_one (input: & Input) -> GenResult <i32> {
	check_input (input) ?;
	let mut cpu = Cpu {
		instrs: Rc::new (input.instrs.clone ()),
		reg_a: 7,
		limit: 1000,
		.. default ()
	};
	cpu.exec () ?;
	Ok (cpu.reg_a)
}

pub fn part_two (input: & Input) -> GenResult <i32> {
	check_input (input) ?;
	let mut cpu = Cpu {
		instrs: Rc::new (input.instrs.clone ()),
		reg_a: 12,
		limit: 1000,
		.. default ()
	};
	cpu.exec () ?;
	Ok (cpu.reg_a)
}

fn check_input (input: & Input) -> GenResult <()> {
	if let Some (instr) = input.instrs.iter ().find (|instr| ! instr.is_v2 ()) {
		return Err (format! ("Invalid instruction: {instr}").into ());
	}
	Ok (())
}
