use aoc_common::*;

pub type Mem <Val> = Vec <Val>;

#[ derive (Clone) ]
pub struct Machine <Val: Int> {
	mem: Mem <Val>,
	pos: Val,
	rel: Val,
	input_buffer: VecDeque <Val>,
	max_ops: u32,
	mem_limit: u32,
}

impl <Val: Int> Machine <Val> {

	#[ inline ]
	#[ must_use ]
	pub fn new (mem: Mem <Val>) -> Self {
		Self {
			mem,
			pos: Val::ZERO,
			rel: Val::ZERO,
			input_buffer: VecDeque::new (),
			max_ops: u32::MAX,
			mem_limit: 1024 * 1024,
		}
	}

	#[ inline ]
	pub fn set_max_ops (& mut self, max_ops: u32) {
		self.max_ops = max_ops;
	}

	#[ inline ]
	pub fn set_mem_limit (& mut self, mem_limit: u32) {
		self.mem_limit = mem_limit;
	}

	#[ inline ]
	pub fn input (& mut self, value: Val) {
		self.input_buffer.push_back (value);
	}

	#[ inline ]
	pub fn input_char (& mut self, value: char) {
		self.input (Val::from_char (value).unwrap ());
	}

	#[ inline ]
	pub fn input_str (& mut self, value: & str) {
		for ch in value.chars () {
			self.input_buffer.push_back (Val::from_char (ch).unwrap ());
		}
	}

	#[ inline ]
	pub fn input_line (& mut self, value: & str) {
		for ch in value.chars () {
			self.input_char (ch);
		}
		self.input_char ('\n');
	}

	#[ allow (clippy::missing_inline_in_public_items) ]
	pub fn run (& mut self) -> RunResult <Val> {
		loop {
			if self.max_ops == 0 { return RunResult::MaxOps }
			self.max_ops -= 1;
			let opcode_val = some_or! (self.mem_get (self.pos), return RunResult::Memory);
			let opcode_err = RunResult::Instr (self.pos, opcode_val);
			let opcode = some_or! (Opcode::from_int (opcode_val), return opcode_err);
			match opcode.instr {
				Instr::Add => {
					let param_0 = some_or! (self.param_get (0), return RunResult::Memory);
					let param_1 = some_or! (self.param_get (1), return RunResult::Memory);
					let result = some_or! (
						Val::add_2 (param_0, param_1).ok (),
						return RunResult::Overflow);
					some_or! (self.param_set (2, result), return opcode_err);
					self.pos += Val::FOUR;
				},
				Instr::Multiply => {
					let param_0 = some_or! (self.param_get (0), return RunResult::Memory);
					let param_1 = some_or! (self.param_get (1), return RunResult::Memory);
					let result = some_or! (
						Val::mul_2 (param_0, param_1).ok (),
						return RunResult::Overflow);
					some_or! (self.param_set (2, result), return opcode_err);
					self.pos += Val::FOUR;
				},
				Instr::Input => {
					if let Some (value) = self.input_buffer.pop_front () {
						some_or! (self.param_set (0, value), return opcode_err);
						self.pos += Val::TWO;
					} else {
						return RunResult::Input;
					}
				},
				Instr::Output => {
					let value = some_or! (self.param_get (0), return RunResult::Memory);
					self.pos += Val::TWO;
					return RunResult::Output (value);
				},
				Instr::JumpIfTrue => {
					let value = some_or! (self.param_get (0), return RunResult::Memory);
					let dest = some_or! (self.param_get (1), return RunResult::Memory);
					if value != Val::ZERO {
						self.pos = dest;
					} else {
						self.pos += Val::THREE;
					}
				},
				Instr::JumpIfFalse => {
					let value = some_or! (self.param_get (0), return RunResult::Memory);
					let dest = some_or! (self.param_get (1), return RunResult::Memory);
					if value == Val::ZERO {
						self.pos = dest;
					} else {
						self.pos += Val::THREE;
					}
				},
				Instr::LessThan => {
					let param_0 = some_or! (self.param_get (0), return RunResult::Memory);
					let param_1 = some_or! (self.param_get (1), return RunResult::Memory);
					let value = if param_0 < param_1 { Val::ONE } else { Val::ZERO };
					some_or! (self.param_set (2, value), return opcode_err);
					self.pos += Val::FOUR;
				},
				Instr::Equals => {
					let param_0 = some_or! (self.param_get (0), return RunResult::Memory);
					let param_1 = some_or! (self.param_get (1), return RunResult::Memory);
					let value = if param_0 == param_1 { Val::ONE } else { Val::ZERO };
					some_or! (self.param_set (2, value), return opcode_err);
					self.pos += Val::FOUR;
				},
				Instr::AdjustRelBase => {
					let value = some_or! (self.param_get (0), return RunResult::Memory);
					self.rel = some_or! (Val::add_2 (self.rel, value).ok (), return RunResult::Overflow);
					self.pos += Val::TWO;
				},
				Instr::Halt => {
					return RunResult::Halt;
				},
			}
		}
	}

	#[ must_use ]
	fn param_get (& mut self, num: u8) -> Option <Val> {
		let opcode_val = self.mem_get (self.pos).unwrap ();
		let opcode = Opcode::from_int (opcode_val).unwrap ();
		let param = self.mem_get (self.pos + Val::ONE + Val::from_u8 (num).unwrap ()) ?;
		match opcode.modes [num.as_usize ()] {
			Mode::Position => self.mem_get (param),
			Mode::Immediate => Some (param),
			Mode::Relative => self.mem_get (param + self.rel),
		}
	}

	#[ must_use ]
	fn param_set (& mut self, num: u8, value: Val) -> Option <()> {
		let opcode_val = self.mem_get (self.pos).unwrap ();
		let opcode = Opcode::from_int (opcode_val).unwrap ();
		let param = self.mem_get (self.pos + Val::ONE + Val::from_u8 (num).unwrap ()) ?;
		match opcode.modes [num.as_usize ()] {
			Mode::Position => self.mem_set (param, value) ?,
			Mode::Immediate => return None,
			Mode::Relative => self.mem_set (param + self.rel, value) ?,
		}
		Some (())
	}

	#[ inline ]
	#[ must_use ]
	pub fn mem_get (& mut self, addr: Val) -> Option <Val> {
		self.mem_extend (addr) ?;
		Some (self.mem [addr.as_usize ()])
	}

	#[ inline ]
	#[ must_use ]
	pub fn mem_set (& mut self, addr: Val, value: Val) -> Option <()> {
		self.mem_extend (addr) ?;
		self.mem [addr.as_usize ()] = value;
		Some (())
	}

	#[ must_use ]
	fn mem_extend (& mut self, addr: Val) -> Option <()> {
		if addr < Val::ZERO || addr == Val::MAX { return None }
		let size = addr.as_usize () + 1;
		if size < self.mem.len () { return Some (()) }
		if self.mem_limit.as_usize () < size { return None }
		self.mem.extend (iter::repeat (Val::ZERO).take (size - self.mem.len ()));
		Some (())
	}

}

#[ derive (Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd) ]
#[ must_use ]
pub enum RunResult <Val> {
	Halt,
	Output (Val),
	Input,
	Instr (Val, Val),
	MaxOps,
	Overflow,
	Memory,
}

impl <Val: Int> RunResult <Val> {

	#[ inline ]
	pub const fn halt (self) -> Result <(), Self> {
		if ! matches! (self, Self::Halt) { return Err (self) }
		Ok (())
	}

	#[ allow (clippy::wildcard_enum_match_arm) ]
	#[ inline ]
	pub const fn output (self) -> Result <Option <Val>, Self> {
		match self {
			Self::Halt => Ok (None),
			Self::Output (val) => Ok (Some (val)),
			other => Err (other),
		}
	}

}

impl <Val: Int> Display for RunResult <Val> {

	#[ inline ]
	fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
		match * self {
			Self::Halt => write! (formatter, "Halted") ?,
			Self::Output (val) => write! (formatter, "Output: {val}") ?,
			Self::Input => write! (formatter, "Waiting for input") ?,
			Self::Instr (addr, val) => write! (formatter, "Invalid instruction {val} at {addr}") ?,
			Self::MaxOps => write! (formatter, "Max operations exceeded") ?,
			Self::Overflow => write! (formatter, "Numeric overflow") ?,
			Self::Memory => write! (formatter, "Memory limit exceeded") ?,
		}
		Ok (())
	}

}

impl <Val: Int> Error for RunResult <Val> {
}

#[ derive (Clone, Copy) ]
struct Opcode {
	instr: Instr,
	modes: [Mode; 3],
}

impl Opcode {

	fn from_int <Val: Int> (value: Val) -> Option <Self> {
		let value = value.to_u32 ().ok () ?;
		Some (Self {
			instr: Instr::from_int (value % 100_u32) ?,
			modes: [
				Mode::from_int ((value / 100) % 10) ?,
				Mode::from_int ((value / 1000) % 10) ?,
				Mode::from_int ((value / 10000) % 10) ?,
			],
		})
	}

}

#[ derive (Clone, Copy, Eq, PartialEq) ]
enum Instr {
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

impl Instr {

	fn from_int <Val: Int> (value: Val) -> Option <Self> {
		let value = value.to_u32 ().ok () ?;
		Some (match value {
			1_u32 => Self::Add,
			2 => Self::Multiply,
			3 => Self::Input,
			4 => Self::Output,
			5 => Self::JumpIfTrue,
			6 => Self::JumpIfFalse,
			7 => Self::LessThan,
			8 => Self::Equals,
			9 => Self::AdjustRelBase,
			99 => Self::Halt,
			_ => return None,
		})
	}

}

#[ derive (Clone, Copy, Eq, PartialEq) ]
enum Mode { Position, Immediate, Relative }

impl Mode {

	fn from_int <Val: Int> (value: Val) -> Option <Self> {
		let value = value.to_u32 ().ok () ?;
		Some (match value {
			0_u32 => Self::Position,
			1 => Self::Immediate,
			2 => Self::Relative,
			_ => return None,
		})
	}

}
