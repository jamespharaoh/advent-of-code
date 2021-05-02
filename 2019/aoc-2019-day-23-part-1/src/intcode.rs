use std::collections::VecDeque;
use std::convert::TryFrom;
use std::convert::TryInto as _;
use std::iter;

pub struct Machine {
	mem: Mem,
	pos: i64,
	rel: i64,
	input_buffer: VecDeque <i64>,
}

impl Machine {

	pub fn new (mem: Mem) -> Machine {
		Machine {
			mem: mem,
			pos: 0,
			rel: 0,
			input_buffer: VecDeque::new (),
		}
	}

	pub fn input (& mut self, value: i64) {
		self.input_buffer.push_back (value);
	}

	pub fn input_str (& mut self, value: & str) {
		for ch in value.chars () {
			self.input_buffer.push_back (ch as i64);
		}
	}

	pub fn run (& mut self) -> RunResult {
		loop {
			let opcode: Opcode = self.mem_get (self.pos).try_into ().map_err (
				|error| format! ("[pos={}] {}", self.pos, error),
			).unwrap ();
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
					if let Some (value) = self.input_buffer.pop_front () {
						self.param_set (0, value);
						self.pos += 2;
					} else {
						return RunResult::Input;
					}
				},
				Instruction::Output => {
					let value = self.param_get (0);
					self.pos += 2;
					return RunResult::Output (value);
				},
				Instruction::JumpIfTrue => {
					let value = self.param_get (0);
					let dest = self.param_get (1);
					if value != 0 {
						self.pos = dest;
					} else {
						self.pos += 3;
					}
				},
				Instruction::JumpIfFalse => {
					let value = self.param_get (0);
					let dest = self.param_get (1);
					if value == 0 {
						self.pos = dest;
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
				Instruction::AdjustRelBase => {
					let value = self.param_get (0);
					self.rel += value;
					self.pos += 2;
				},
				Instruction::Halt => {
					return RunResult::Halt;
				},
			}
		}
	}

	fn param_get (& mut self, num: u8) -> i64 {
		let opcode: Opcode = self.mem_get (self.pos).try_into ().unwrap ();
		let param = self.mem_get (self.pos + 1 + num as i64);
		match opcode.modes [num as usize] {
			Mode::Position => self.mem_get (param),
			Mode::Immediate => param,
			Mode::Relative => self.mem_get (param + self.rel),
		}
	}

	fn param_set (& mut self, num: u8, value: i64) {
		let opcode: Opcode = self.mem_get (self.pos).try_into ().unwrap ();
		let param = self.mem_get (self.pos + 1 + num as i64);
		match opcode.modes [num as usize] {
			Mode::Position => self.mem_set (param, value),
			Mode::Immediate => panic! (),
			Mode::Relative => self.mem_set (param + self.rel, value),
		}
	}

	fn mem_get (& mut self, addr: i64) -> i64 {
		self.mem_extend (addr);
		self.mem [addr as usize]
	}

	fn mem_set (& mut self, addr: i64, value: i64) {
		self.mem_extend (addr);
		self.mem [addr as usize] = value;
	}

	fn mem_extend (& mut self, addr: i64) {
		if addr < 0 || addr == i64::MAX { panic! (); }
		let size = addr as usize + 1;
		if size < self.mem.len () {
			return;
		}
		self.mem.extend (iter::repeat (0).take (size - self.mem.len ()));
	}

}

pub type Mem = Vec <i64>;

pub fn from_str (source: & str) -> Mem {
	source.split (',').map (
		|item_str| item_str.trim ().parse ().unwrap (),
	).collect ()
}

#[ derive (Debug) ]
pub enum RunResult {
	Halt,
	Output (i64),
	Input,
}

#[ derive (Clone, Copy) ]
struct Opcode {
	instr: Instruction,
	modes: [Mode; 3],
}

impl TryFrom <i64> for Opcode {

	type Error = String;

	fn try_from (value: i64) -> Result <Opcode, String> {
		Ok (Opcode {
			instr: (value % 100).try_into () ?,
			modes: [
				((value / 100) % 10).into (),
				((value / 1000) % 10).into (),
				((value / 10000) % 10).into (),
			],
		})
	}

}

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
	AdjustRelBase,
}

impl TryFrom <i64> for Instruction {

	type Error = String;

	fn try_from (value: i64) -> Result <Instruction, String> {
		Ok (match value {
			1 => Instruction::Add,
			2 => Instruction::Multiply,
			3 => Instruction::Input,
			4 => Instruction::Output,
			5 => Instruction::JumpIfTrue,
			6 => Instruction::JumpIfFalse,
			7 => Instruction::LessThan,
			8 => Instruction::Equals,
			9 => Instruction::AdjustRelBase,
			99 => Instruction::Halt,
			_ => return Err (format! ("Invalid instruction: {}", value)),
		})
	}

}

#[ derive (Clone, Copy, Eq, PartialEq) ]
enum Mode { Position, Immediate, Relative }

impl From <i64> for Mode {

	fn from (value: i64) -> Mode {
		match value {
			0 => Mode::Position,
			1 => Mode::Immediate,
			2 => Mode::Relative,
			_ => panic! ("Invalid mode: {}", value),
		}
	}

}
