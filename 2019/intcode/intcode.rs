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
					let param_0 = some_or! (self.param_get (opcode, 0), return RunResult::Memory);
					let param_1 = some_or! (self.param_get (opcode, 1), return RunResult::Memory);
					let result = some_or! (
						Val::add_2 (param_0, param_1).ok (),
						return RunResult::Overflow);
					some_or! (self.param_set (opcode, 2, result), return opcode_err);
					self.pos = ok_or! (Val::add_2 (self.pos, Val::FOUR), return RunResult::Overflow);
				},
				Instr::Multiply => {
					let param_0 = some_or! (self.param_get (opcode, 0), return RunResult::Memory);
					let param_1 = some_or! (self.param_get (opcode, 1), return RunResult::Memory);
					let result = some_or! (
						Val::mul_2 (param_0, param_1).ok (),
						return RunResult::Overflow);
					some_or! (self.param_set (opcode, 2, result), return opcode_err);
					self.pos = ok_or! (Val::add_2 (self.pos, Val::FOUR), return RunResult::Overflow);
				},
				Instr::Input => {
					if let Some (value) = self.input_buffer.pop_front () {
						some_or! (self.param_set (opcode, 0, value), return opcode_err);
						self.pos = ok_or! (Val::add_2 (self.pos, Val::TWO), return RunResult::Overflow);
					} else {
						return RunResult::Input;
					}
				},
				Instr::Output => {
					let value = some_or! (self.param_get (opcode, 0), return RunResult::Memory);
					self.pos = ok_or! (Val::add_2 (self.pos, Val::TWO), return RunResult::Overflow);
					return RunResult::Output (value);
				},
				Instr::JumpIfTrue => {
					let value = some_or! (self.param_get (opcode, 0), return RunResult::Memory);
					let dest = some_or! (self.param_get (opcode, 1), return RunResult::Memory);
					if value != Val::ZERO {
						self.pos = dest;
					} else {
						self.pos = ok_or! (Val::add_2 (self.pos, Val::THREE), return RunResult::Overflow);
					}
				},
				Instr::JumpIfFalse => {
					let value = some_or! (self.param_get (opcode, 0), return RunResult::Memory);
					let dest = some_or! (self.param_get (opcode, 1), return RunResult::Memory);
					if value == Val::ZERO {
						self.pos = dest;
					} else {
						self.pos = ok_or! (Val::add_2 (self.pos, Val::THREE), return RunResult::Overflow);
					}
				},
				Instr::LessThan => {
					let param_0 = some_or! (self.param_get (opcode, 0), return RunResult::Memory);
					let param_1 = some_or! (self.param_get (opcode, 1), return RunResult::Memory);
					let value = if param_0 < param_1 { Val::ONE } else { Val::ZERO };
					some_or! (self.param_set (opcode, 2, value), return opcode_err);
					self.pos = ok_or! (Val::add_2 (self.pos, Val::FOUR), return RunResult::Overflow);
				},
				Instr::Equals => {
					let param_0 = some_or! (self.param_get (opcode, 0), return RunResult::Memory);
					let param_1 = some_or! (self.param_get (opcode, 1), return RunResult::Memory);
					let value = if param_0 == param_1 { Val::ONE } else { Val::ZERO };
					some_or! (self.param_set (opcode, 2, value), return opcode_err);
					self.pos = ok_or! (Val::add_2 (self.pos, Val::FOUR), return RunResult::Overflow);
				},
				Instr::AdjustRelBase => {
					let value = some_or! (self.param_get (opcode, 0), return RunResult::Memory);
					self.rel = some_or! (Val::add_2 (self.rel, value).ok (), return RunResult::Overflow);
					self.pos = ok_or! (Val::add_2 (self.pos, Val::TWO), return RunResult::Overflow);
				},
				Instr::Halt => {
					return RunResult::Halt;
				},
			}
		}
	}

	#[ inline ]
	#[ must_use ]
	fn param_get (& mut self, opcode: Opcode, num: u8) -> Option <Val> {
		let param = self.mem_get (self.pos + Val::ONE + Val::from_u8 (num).unwrap ()) ?;
		match opcode.modes [num.as_usize ()] {
			Mode::Position => self.mem_get (param),
			Mode::Immediate => Some (param),
			Mode::Relative => self.mem_get (param + self.rel),
		}
	}

	#[ inline ]
	#[ must_use ]
	fn param_set (& mut self, opcode: Opcode, num: u8, value: Val) -> Option <()> {
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
		if addr < Val::ZERO { return None }
		Some (self.mem.get (addr.as_usize ()).copied ().unwrap_or (Val::ZERO))
	}

	#[ inline ]
	#[ must_use ]
	pub fn mem_set (& mut self, addr: Val, value: Val) -> Option <()> {
		self.mem_extend (addr) ?;
		self.mem [addr.as_usize ()] = value;
		Some (())
	}

	#[ inline ]
	#[ must_use ]
	pub fn mem_extend (& mut self, addr: Val) -> Option <()> {
		if addr < Val::ZERO || addr == Val::MAX { return None }
		let size = addr.as_usize () + 1;
		if size < self.mem.len () { return Some (()) }
		if self.mem_limit.as_usize () < size { return None }
		self.mem.resize ((size + 0xff) & ! 0xff, Val::ZERO);
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
		#[ inline ]
		fn decode_mode <Val: Int> (value: Val, multiple: Val) -> Option <(Val, Val)> {
			if multiple * Val::THREE <= value { None }
			else if multiple * Val::TWO <= value { Some ((value - multiple * Val::TWO, Val::TWO)) }
			else if multiple * Val::ONE <= value { Some ((value - multiple * Val::ONE, Val::ONE)) }
			else { Some ((value, Val::ZERO)) }
		}
		let value = value.to_u32 ().ok () ?;
		let (value, mode_0) = decode_mode (value, 10_000) ?;
		let (value, mode_1) = decode_mode (value, 1_000) ?;
		let (value, mode_2) = decode_mode (value, 100) ?;
		Some (Self {
			instr: Instr::from_int (value) ?,
			modes: [
				Mode::from_int (mode_2) ?,
				Mode::from_int (mode_1) ?,
				Mode::from_int (mode_0) ?,
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
		if value == Val::ONE { Some (Self::Add) }
		else if value == Val::TWO { Some (Self::Multiply) }
		else if value == Val::THREE { Some (Self::Input) }
		else if value == Val::FOUR { Some (Self::Output) }
		else if value == Val::FIVE { Some (Self::JumpIfTrue) }
		else if value == Val::SIX { Some (Self::JumpIfFalse) }
		else if value == Val::SEVEN { Some (Self::LessThan) }
		else if value == Val::EIGHT { Some (Self::Equals) }
		else if value == Val::NINE { Some (Self::AdjustRelBase) }
		else {
			let value = value.to_u32 ().ok () ?;
			if value == 99 { Some (Self::Halt) }
			else { None }
		}
	}

}

#[ derive (Clone, Copy, Eq, PartialEq) ]
enum Mode { Position, Immediate, Relative }

impl Mode {

	fn from_int <Val: Int> (value: Val) -> Option <Self> {
		if value == Val::ZERO { Some (Self::Position) }
		else if value == Val::ONE { Some (Self::Immediate) }
		else if value == Val::TWO { Some (Self::Relative) }
		else { None }
	}

}
