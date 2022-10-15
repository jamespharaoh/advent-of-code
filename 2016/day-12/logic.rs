use super::*;

use cpu::Cpu;
use input::Input;

pub fn part_one (input: & Input) -> GenResult <i32> {
	check_input (input) ?;
	let mut cpu = Cpu {
		instrs: Rc::new (input.instrs.clone ()),
		limit: input.params.ops_limit,
		.. Cpu::default ()
	};
	cpu.exec () ?;
	Ok (cpu.reg_a)
}

pub fn part_two (input: & Input) -> GenResult <i32> {
	check_input (input) ?;
	let mut cpu = Cpu {
		instrs: Rc::new (input.instrs.clone ()),
		reg_c: 1,
		limit: input.params.ops_limit,
		.. Cpu::default ()
	};
	cpu.exec () ?;
	Ok (cpu.reg_a)
}

fn check_input (input: & Input) -> GenResult <()> {
	if let Some (& instr) = input.instrs.iter ().find (|& instr| ! instr.is_v1 ()) {
		return Err (format! ("Invalid instrucction: {instr:?}").into ());
	}
	Ok (())
}
