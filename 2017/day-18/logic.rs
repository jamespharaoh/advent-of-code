//! Logic for solving the puzzles

use super::*;
use cpu::Cpu;
use cpu::CpuError;
use cpu::Instr;
use input::Input;

pub fn part_one (input: & Input) -> GenResult <i64> {
	let mut cpu = Cpu::new (& * input.instrs);
	cpu.set_limit (10_000);
	let mut last_freq = None;
	loop {
		match cpu.execute () {
			Ok (Some (freq)) => last_freq = Some (freq),
			Ok (None) => break,
			Err (CpuError::Receive) if last_freq.is_some () => return Ok (last_freq.unwrap ()),
			Err (err) => return Err (err.into ()),
		}
	}
	Err ("No solution found".into ())
}

pub fn part_two (input: & Input) -> GenResult <u32> {

	use ProgramState::{ Okay, Waiting, Halted };

	// set up both programs

	let instrs = Rc::from (input.instrs.as_slice ());
	let mut prog_0 = Program::new (& instrs, 0);
	let mut prog_1 = Program::new (& instrs, 1);

	// main loop

	let mut num_sends: u32 = 0;
	while prog_0.state == Okay || prog_1.state == Okay {

		// select which program to run, prefering `prog_1`

		let (prog_a, prog_b) = if prog_1.state == Okay {
			(& mut prog_1, & mut prog_0)
		} else {
			(& mut prog_0, & mut prog_1)
		};

		// run until something interesting happens

		match prog_a.cpu.execute () {
			Ok (Some (output)) => {
				if prog_b.cpu.input ().len () == 1000 {
					return Err ("Queue full (1000 items)".into ());
				}
				prog_b.cpu.push_input (output);
				if prog_b.state == Waiting { prog_b.state = Okay; }
				if prog_a.id == 1 { num_sends += 1; }
			},
			Ok (None) => {
				prog_a.state = Halted;
				if prog_a.id == 1 { break }
			},
			Err (CpuError::Receive) => {
				prog_a.state = Waiting;
			},
			Err (err) => {
				return Err (format! ("CPU error: {}", err).into ());
			}
		}

	}

	Ok (num_sends)

}

#[ derive (Debug, Default, Eq, PartialEq) ]
enum ProgramState {
	#[default] Okay,
	Waiting,
	Halted,
}

#[ derive (Debug) ]
struct Program {
	id: u32,
	cpu: Cpu,
	state: ProgramState,
}

impl Program {
	fn new (instrs: & [Instr], id: u32) -> Self {
		let mut cpu = Cpu::new (instrs);
		cpu.set_limit (100_000_u64);
		cpu.set_reg ('p', id.as_i64 ()).unwrap ();
		Self { id, cpu, state: default () }
	}
}
