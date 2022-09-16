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
		match self.run_real () {
			Ok (val) => val,
			Err (RunResult::Instr (_, _)) => {
				let opcode_val = self.mem_get_real (self.pos).unwrap ();
				RunResult::Instr (self.pos, opcode_val)
			},
			Err (val) => val,
		}
	}

	#[ inline ]
	fn run_real (& mut self) -> Result <RunResult <Val>, RunResult <Val>> {
		loop {
			if self.max_ops == 0 { return Err (RunResult::MaxOps) }
			self.max_ops -= 1;
			let opcode = Opcode::from_int (self.mem_get (self.pos).ok_or (RunResult::Memory) ?)
				.ok_or (RunResult::Instr (Val::ZERO, Val::ZERO)) ?;
			match opcode.instr {
				Instr::Add => {
					self.param_set (opcode, 2, Val::add_2 (
						self.param_get (opcode, 0) ?,
						self.param_get (opcode, 1) ?,
					) ?) ?;
					self.pos = Val::add_2 (self.pos, Val::FOUR) ?;
				},
				Instr::Multiply => {
					self.param_set (opcode, 2, Val::mul_2 (
						self.param_get (opcode, 0) ?,
						self.param_get (opcode, 1) ?,
					) ?) ?;
					self.pos = Val::add_2 (self.pos, Val::FOUR) ?;
				},
				Instr::Input => {
					if let Some (value) = self.input_buffer.pop_front () {
						self.param_set (opcode, 0, value) ?;
						self.pos = Val::add_2 (self.pos, Val::TWO) ?;
					} else {
						return Ok (RunResult::Input);
					}
				},
				Instr::Output => {
					let value = self.param_get (opcode, 0) ?;
					self.pos = Val::add_2 (self.pos, Val::TWO) ?;
					return Ok (RunResult::Output (value));
				},
				Instr::JumpIfTrue => {
					if self.param_get (opcode, 0) ? != Val::ZERO {
						self.pos = self.param_get (opcode, 1) ?;
					} else {
						self.pos = Val::add_2 (self.pos, Val::THREE) ?;
					}
				},
				Instr::JumpIfFalse => {
					if self.param_get (opcode, 0) ? == Val::ZERO {
						self.pos = self.param_get (opcode, 1) ?;
					} else {
						self.pos = Val::add_2 (self.pos, Val::THREE) ?;
					}
				},
				Instr::LessThan => {
					let value = self.param_get (opcode, 0) ? < self.param_get (opcode, 1) ?;
					self.param_set (opcode, 2, if value { Val::ONE } else { Val::ZERO }) ?;
					self.pos = Val::add_2 (self.pos, Val::FOUR) ?;
				},
				Instr::Equals => {
					let value = self.param_get (opcode, 0) ? == self.param_get (opcode, 1) ?;
					self.param_set (opcode, 2, if value { Val::ONE } else { Val::ZERO }) ?;
					self.pos = Val::add_2 (self.pos, Val::FOUR) ?;
				},
				Instr::AdjustRelBase => {
					self.rel = Val::add_2 (self.rel, self.param_get (opcode, 0) ?) ?;
					self.pos = Val::add_2 (self.pos, Val::TWO) ?;
				},
				Instr::Halt => return Ok (RunResult::Halt),
			}
		}
	}

	#[ inline ]
	fn param_raw (& self, num: u8) -> Result <Val, RunResult <Val>> {
		let addr = Val::add_3 (self.pos, Val::ONE, Val::from_u8 (num).unwrap ()) ?;
		self.mem_get_real (addr)
	}

	#[ inline ]
	fn param_get (& self, opcode: Opcode, num: u8) -> Result <Val, RunResult <Val>> {
		let raw = self.param_raw (num) ?;
		match opcode.modes [num.as_usize ()] {
			Mode::Position => self.mem_get_real (raw),
			Mode::Immediate => Ok (raw),
			Mode::Relative => self.mem_get_real (Val::add_2 (raw, self.rel) ?),
		}
	}

	#[ inline ]
	fn param_set (& mut self, opcode: Opcode, num: u8, value: Val) -> Result <(), RunResult <Val>> {
		let param = self.param_raw (num) ?;
		match opcode.modes [num.as_usize ()] {
			Mode::Position => self.mem_set_real (param, value),
			Mode::Immediate => Err (RunResult::Instr (Val::ZERO, Val::ZERO)),
			Mode::Relative => self.mem_set_real (param + self.rel, value),
		}
	}

	#[ inline ]
	#[ must_use ]
	pub fn mem_get (& self, addr: Val) -> Option <Val> {
		self.mem_get_real (addr).ok ()
	}

	#[ inline ]
	fn mem_get_real (& self, addr: Val) -> Result <Val, RunResult <Val>> {
		let addr = addr.to_usize ().map_err (|_err| RunResult::Memory) ?;
		Ok (self.mem.get (addr).copied ().unwrap_or (Val::ZERO))
	}

	#[ inline ]
	#[ must_use ]
	pub fn mem_set (& mut self, addr: Val, value: Val) -> Option <()> {
		self.mem_set_real (addr, value).ok ()
	}

	#[ inline ]
	fn mem_set_real (& mut self, addr: Val, value: Val) -> Result <(), RunResult <Val>> {
		self.mem_extend (addr) ?;
		self.mem [addr.as_usize ()] = value;
		Ok (())
	}

	#[ inline ]
	fn mem_extend (& mut self, addr: Val) -> Result <(), RunResult <Val>> {
		if addr < Val::ZERO || addr == Val::MAX { return Err (RunResult::Memory) }
		let size = addr.as_usize () + 1;
		if size < self.mem.len () { return Ok (()) }
		if self.mem_limit.as_usize () < size { return Err (RunResult::Memory) }
		self.mem.resize ((size + 0xff) & ! 0xff, Val::ZERO);
		Ok (())
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

impl <Val: Int> From <Overflow> for RunResult <Val> {

	#[ inline ]
	fn from (_overflow: Overflow) -> Self {
		Self::Overflow
	}

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
			if value < Val::ZERO || multiple * Val::THREE <= value { None }
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
