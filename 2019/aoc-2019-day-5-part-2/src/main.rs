use rustyline::Editor;

use std::io::BufRead as _;
use std::io::BufReader;
use std::fs::File;

type Mem = Vec <i64>;

#[ derive (Clone, Copy, Eq, PartialEq) ]
enum Instruction {
	Add,
	Multiply,
	Input,
	Output,
	Halt,
	JumpIfTrue,
	JumpIfFalse,
	LessThan,
	Equals,
}

#[ derive (Clone, Copy, Eq, PartialEq) ]
enum Mode { Position, Immediate }

#[ derive (Clone, Copy) ]
struct Opcode {
	instr: Instruction,
	modes: [Mode; 3],
}

fn main () {
	let mut readline = Editor::new ().unwrap ();
	let mut mem = load ();
	run (& mut mem, & mut readline);
}

fn load () -> Mem {
	let mut mem: Mem = Vec::new ();
	let file = File::open ("input").unwrap ();
	let mut reader = BufReader::new (file);
	loop {
		let mut buf: Vec <u8> = Vec::new ();
		let num_read = reader.read_until (b',', & mut buf).unwrap ();
		if num_read == 0 { break; }
		while ! buf.is_empty () && ! buf.last ().unwrap ().is_ascii_digit () {
			buf.pop ().unwrap ();
		}
		let str = String::from_utf8 (buf).unwrap ();
		let num: i64 = str.parse ().unwrap ();
		mem.push (num);
	}
	mem
}

fn run (mem: & mut Mem, readline: & mut Editor::<()>) {
	let mut pos = 0;
	loop {
		let opcode: Opcode = mem [pos].into ();
		match opcode.instr {
			Instruction::Add => {
				let param_0 = param_get (mem, pos, opcode, 0);
				let param_1 = param_get (mem, pos, opcode, 1);
				param_set (mem, pos, opcode, 2, param_0 + param_1);
				pos += 4;
			},
			Instruction::Multiply => {
				let param_0 = param_get (mem, pos, opcode, 0);
				let param_1 = param_get (mem, pos, opcode, 1);
				param_set (mem, pos, opcode, 2, param_0 * param_1);
				pos += 4;
			},
			Instruction::Input => {
				let value_str = readline.readline ("Input: ").unwrap ();
				let value = value_str.parse ().unwrap ();
				param_set (mem, pos, opcode, 0, value);
				pos += 2;
			},
			Instruction::Output => {
				let value = param_get (mem, pos, opcode, 0);
				println! ("Output: {}", value);
				pos += 2;
			},
			Instruction::JumpIfTrue => {
				let value = param_get (mem, pos, opcode, 0);
				let dest = param_get (mem, pos, opcode, 1);
				if value != 0 {
					pos = dest as usize;
				} else {
					pos += 3;
				}
			},
			Instruction::JumpIfFalse => {
				let value = param_get (mem, pos, opcode, 0);
				let dest = param_get (mem, pos, opcode, 1);
				if value == 0 {
					pos = dest as usize;
				} else {
					pos += 3;
				}
			},
			Instruction::LessThan => {
				let param_0 = param_get (mem, pos, opcode, 0);
				let param_1 = param_get (mem, pos, opcode, 1);
				let value = if param_0 < param_1 { 1 } else { 0 };
				param_set (mem, pos, opcode, 2, value);
				pos += 4;
			},
			Instruction::Equals => {
				let param_0 = param_get (mem, pos, opcode, 0);
				let param_1 = param_get (mem, pos, opcode, 1);
				let value = if param_0 == param_1 { 1 } else { 0 };
				param_set (mem, pos, opcode, 2, value);
				pos += 4;
			},
			Instruction::Halt => {
				return;
			},
		}
	}
}

impl From <i64> for Instruction {

	fn from (value: i64) -> Instruction {
		match value {
			1 => Instruction::Add,
			2 => Instruction::Multiply,
			3 => Instruction::Input,
			4 => Instruction::Output,
			5 => Instruction::JumpIfTrue,
			6 => Instruction::JumpIfFalse,
			7 => Instruction::LessThan,
			8 => Instruction::Equals,
			99 => Instruction::Halt,
			_ => panic! ("Invalid instruction: {}", value),
		}
	}

}

impl From <i64> for Mode {

	fn from (value: i64) -> Mode {
		match value {
			0 => Mode::Position,
			1 => Mode::Immediate,
			_ => panic! ("Invalid mode: {}", value),
		}
	}

}

impl From <i64> for Opcode {

	fn from (value: i64) -> Opcode {
		Opcode {
			instr: (value % 100).into (),
			modes: [
				((value / 100) % 10).into (),
				((value / 1000) % 10).into (),
				((value / 10000) % 10).into (),
			],
		}
	}

}

fn param_get (mem: & Mem, pos: usize, opcode: Opcode, num: usize) -> i64 {
	let param = mem [pos + 1 + num];
	match opcode.modes [num] {
		Mode::Position => mem [param as usize],
		Mode::Immediate => param,
	}
}

fn param_set (mem: & mut Mem, pos: usize, opcode: Opcode, num: usize, value: i64) {
	let param = mem [pos + 1 + num];
	match opcode.modes [num] {
		Mode::Position => mem [param as usize] = value,
		Mode::Immediate => panic! (),
	}
}
