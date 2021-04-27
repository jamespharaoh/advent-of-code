use rustyline::Editor;

use std::collections::VecDeque;

pub struct Machine {
	mem: Mem,
	pos: usize,
	input_buffer: VecDeque <i64>,
	input_readline: Option <Editor <()>>,
	output_buffer: Option <VecDeque <i64>>,
}

impl Machine {

	pub fn new (mem: Mem) -> Machine {
		Machine {
			mem: mem,
			pos: 0,
			input_buffer: VecDeque::new (),
			input_readline: None,
			output_buffer: None,
		}
	}

	pub fn with_input_buffer (mut self, input_buffer: VecDeque <i64>) -> Machine {
		self.input_buffer = input_buffer;
		self
	}

	pub fn with_input_readline (mut self, input_readline: Editor <()>) -> Machine {
		self.input_readline = Some (input_readline);
		self
	}

	pub fn with_output_buffer (mut self) -> Machine {
		self.output_buffer = Some (VecDeque::new ());
		self
	}

	pub fn input_buffer (& mut self) -> & mut VecDeque <i64> {
		& mut self.input_buffer
	}

	pub fn output_buffer (& mut self) -> & mut VecDeque <i64> {
		self.output_buffer.as_mut ().unwrap ()
	}

	pub fn queue_input (& mut self, value: i64) {
		self.input_buffer.push_back (value);
	}

	pub fn run (& mut self) {
		loop {
			let opcode: Opcode = self.mem [self.pos].into ();
			match opcode.instr {
				Instruction::Add => {
					let param_0 = self.param_get (0);
					let param_1 = self.param_get (1);
					self.param_set (2, param_0 + param_1);
					self.pos += 4;
				},
				Instruction::Multiply => {
					let param_0 = self.param_get (0);
					let param_1 = self.param_get (1);
					self.param_set (2, param_0 * param_1);
					self.pos += 4;
				},
				Instruction::Input => {
					let value = self.get_input ();
					self.param_set (0, value);
					self.pos += 2;
				},
				Instruction::Output => {
					let value = self.param_get (0);
					if let Some (output_buffer) = self.output_buffer.as_mut () {
						output_buffer.push_back (value);
					} else {
						println! ("Output: {}", value);
					}
					self.pos += 2;
				},
				Instruction::JumpIfTrue => {
					let value = self.param_get (0);
					let dest = self.param_get (1);
					if value != 0 {
						self.pos = dest as usize;
					} else {
						self.pos += 3;
					}
				},
				Instruction::JumpIfFalse => {
					let value = self.param_get (0);
					let dest = self.param_get (1);
					if value == 0 {
						self.pos = dest as usize;
					} else {
						self.pos += 3;
					}
				},
				Instruction::LessThan => {
					let param_0 = self.param_get (0);
					let param_1 = self.param_get (1);
					let value = if param_0 < param_1 { 1 } else { 0 };
					self.param_set (2, value);
					self.pos += 4;
				},
				Instruction::Equals => {
					let param_0 = self.param_get (0);
					let param_1 = self.param_get (1);
					let value = if param_0 == param_1 { 1 } else { 0 };
					self.param_set (2, value);
					self.pos += 4;
				},
				Instruction::Halt => {
					return;
				},
			}
		}
	}

	fn get_input (& mut self) -> i64 {
		if let Some (value) = self.input_buffer.pop_front () {
			return value;
		} else if let Some (input_readline) = self.input_readline.as_mut () {
			let value_str = input_readline.readline ("Input: ").unwrap ();
			return value_str.parse ().unwrap ();
		} else {
			panic! ("No input");
		}
	}

	fn param_get (& mut self, num: usize) -> i64 {
		let opcode: Opcode = self.mem [self.pos].into ();
		let param = self.mem [self.pos + 1 + num];
		match opcode.modes [num] {
			Mode::Position => self.mem [param as usize],
			Mode::Immediate => param,
		}
	}

	fn param_set (& mut self, num: usize, value: i64) {
		let opcode: Opcode = self.mem [self.pos].into ();
		let param = self.mem [self.pos + 1 + num];
		match opcode.modes [num] {
			Mode::Position => self.mem [param as usize] = value,
			Mode::Immediate => panic! (),
		}
	}

}

pub fn from_str (source: & str) -> Mem {
	source.split (',').map (
		|item_str| item_str.trim ().parse ().unwrap (),
	).collect ()
}

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

#[ derive (Clone, Copy, Eq, PartialEq) ]
enum Mode { Position, Immediate }

impl From <i64> for Mode {

	fn from (value: i64) -> Mode {
		match value {
			0 => Mode::Position,
			1 => Mode::Immediate,
			_ => panic! ("Invalid mode: {}", value),
		}
	}

}

#[ derive (Clone, Copy) ]
struct Opcode {
	instr: Instruction,
	modes: [Mode; 3],
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
