use rustyline::Editor;

use std::io::BufRead as _;
use std::io::BufReader;
use std::fs::File;

type Mem = Vec <i64>;

#[ derive (Clone, Copy, Eq, PartialEq) ]
enum Instruction { Add, Multiply, Input, Output, Halt }

#[ derive (Clone, Copy, Eq, PartialEq) ]
enum Mode { Position, Immediate }

struct Opcode {
	instr: Instruction,
	modes: [Mode; 3],
}

fn main () {
	let mut readline = Editor::new ();
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
				let left = mem_get (mem, opcode.modes [0], mem [pos + 1]);
				let right = mem_get (mem, opcode.modes [1], mem [pos + 2]);
				mem_set (mem, opcode.modes [2], mem [pos + 3], left + right);
				pos += 4;
			},
			Instruction::Multiply => {
				let left = mem_get (mem, opcode.modes [0], mem [pos + 1]);
				let right = mem_get (mem, opcode.modes [1], mem [pos + 2]);
				mem_set (mem, opcode.modes [2], mem [pos + 3], left * right);
				pos += 4;
			},
			Instruction::Input => {
				let value_str = readline.readline ("Input: ").unwrap ();
				let value = value_str.parse ().unwrap ();
				mem_set (mem, opcode.modes [0], mem [pos + 1], value);
				pos += 2;
			},
			Instruction::Output => {
				let value = mem_get (mem, opcode.modes [0], mem [pos + 1]);
				println! ("Output: {}", value);
				pos += 2;
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

fn mem_get (mem: & Mem, mode: Mode, param: i64) -> i64 {
	match mode {
		Mode::Position => mem [param as usize],
		Mode::Immediate => param,
	}
}

fn mem_set (mem: & mut Mem, mode: Mode, param: i64, value: i64) {
	match mode {
		Mode::Position => mem [param as usize] = value,
		Mode::Immediate => panic! (),
	}
}
