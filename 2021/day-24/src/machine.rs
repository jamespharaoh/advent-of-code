use aoc_common::*;
use nums::IntConv;

pub fn parse_prog (lines: & [& str]) -> GenResult <Vec <Instr>> {
	lines.iter ().enumerate ().map (|(line_idx, line)| -> GenResult <Instr> {
		let err = || format! ("Invalid input: {}: {}", line_idx + 1, line);
		let parse_reg = |input| Ok (match input {
			"w" => Reg::W,
			"x" => Reg::X,
			"y" => Reg::Y,
			"z" => Reg::Z,
			_ => return Err (err ()),
		});
		let parse_reg_or_int = |input| -> GenResult <RegOrInt> {
			Ok (match input {
				"w" => RegOrInt::W,
				"x" => RegOrInt::X,
				"y" => RegOrInt::Y,
				"z" => RegOrInt::Z,
				_ => RegOrInt::Int (input.parse::<i64> ()
					.map_err (|_err| err ()) ?),
			})
		};
		let line_parts: Vec <& str> = line.split (' ').collect ();
		let instr_str = line_parts [0];
		let instr_args = & line_parts [1 .. ];
		Ok (match (instr_str, instr_args.len ()) {
			("inp", 1) => Instr::Inp (parse_reg (instr_args [0]) ?),
			("add", 2) => Instr::Add (parse_reg (instr_args [0]) ?, parse_reg_or_int (instr_args [1]) ?),
			("mul", 2) => Instr::Mul (parse_reg (instr_args [0]) ?, parse_reg_or_int (instr_args [1]) ?),
			("div", 2) => Instr::Div (parse_reg (instr_args [0]) ?, parse_reg_or_int (instr_args [1]) ?),
			("mod", 2) => Instr::Mod (parse_reg (instr_args [0]) ?, parse_reg_or_int (instr_args [1]) ?),
			("eql", 2) => Instr::Eql (parse_reg (instr_args [0]) ?, parse_reg_or_int (instr_args [1]) ?),
			_ => Err (err ()) ?,
		})
	}).collect ()
}

#[ must_use ]
pub fn machine_input (input: [u8; 14]) -> [i64; 14] {
	let mut result = [0; 14];
	for idx in 0 .. 14 { result [idx] = input [idx].as_i64 (); }
	result
}

#[ derive (Clone, Debug, Default, Eq, Hash, PartialEq) ]
pub struct Machine {
	pub regs: MachineRegs,
}

#[ derive (Clone, Copy, Debug, Eq, PartialEq) ]
pub enum MachineError {
	NoMoreInput,
	DivideByZero,
	NegativeModulo,
}

impl Machine {

	#[ inline ]
	#[ must_use ]
	pub fn new () -> Self {
		Self { regs: default () }
	}

	pub fn step (& mut self, prog: & [Instr], input: & [i64]) -> Result <bool, MachineError> {
		let instr = match prog.get (self.regs.pc) {
			Some (& instr) => instr,
			None => return Ok (true),
		};
		match instr {
			Instr::Inp (dest) => {
				let val = match input.get (self.regs.ic) {
					Some (& val) => val,
					None => return Err (MachineError::NoMoreInput),
				};
				self.regs.ic += 1;
				self.regs.store (dest, val);
			},
			Instr::Add (dest, src) => {
				let src_val = self.regs.retrieve_or_int (src);
				let dst_val = self.regs.retrieve (dest);
				self.regs.store (dest, dst_val + src_val);
			},
			Instr::Mul (dest, src) => {
				let src_val = self.regs.retrieve_or_int (src);
				let dst_val = self.regs.retrieve (dest);
				self.regs.store (dest, dst_val * src_val);
			},
			Instr::Div (dest, src) => {
				let src_val = self.regs.retrieve_or_int (src);
				if src_val == 0 { Err (MachineError::DivideByZero) ? }
				let dst_val = self.regs.retrieve (dest);
				self.regs.store (dest, dst_val / src_val);
			},
			Instr::Mod (dest, src) => {
				let src_val = self.regs.retrieve_or_int (src);
				if src_val == 0 { Err (MachineError::DivideByZero) ? }
				let dst_val = self.regs.retrieve (dest);
				self.regs.store (dest, dst_val % src_val);
			},
			Instr::Eql (dest, src) => {
				let src_val = self.regs.retrieve_or_int (src);
				let dst_val = self.regs.retrieve (dest);
				self.regs.store (dest, if dst_val == src_val { 1 } else { 0 });
			},
		}
		self.regs.pc += 1;
		Ok (false)
	}
	#[ allow (dead_code) ]
	fn execute (& mut self, prog: & [Instr], input: & [i64]) -> Result <(), MachineError> {
		loop {
			match self.step (prog, input) {
				Ok (true) => break,
				Ok (false) => continue,
				Err (err) => Err (err) ?,
			}
		}
		Ok (())
	}
}

#[ derive (Clone, Copy, Debug, Default, Eq, Hash, PartialEq) ]
pub struct MachineRegs {
	pub w: i64,
	pub x: i64,
	pub y: i64,
	pub z: i64,
	pub pc: usize,
	pub ic: usize,
}

impl MachineRegs {
	fn store (& mut self, reg: Reg, val: i64) {
		match reg {
			Reg::W => self.w = val,
			Reg::X => self.x = val,
			Reg::Y => self.y = val,
			Reg::Z => self.z = val,
		}
	}
	const fn retrieve (& self, reg: Reg) -> i64 {
		match reg {
			Reg::W => self.w,
			Reg::X => self.x,
			Reg::Y => self.y,
			Reg::Z => self.z,
		}
	}
	const fn retrieve_or_int (& self, reg_or_int: RegOrInt) -> i64 {
		match reg_or_int {
			RegOrInt::W => self.w,
			RegOrInt::X => self.x,
			RegOrInt::Y => self.y,
			RegOrInt::Z => self.z,
			RegOrInt::Int (val) => val,
		}
	}
}

#[ derive (Clone, Copy, Debug, Eq, PartialEq) ]
pub enum Instr {
	Inp (Reg),
	Add (Reg, RegOrInt),
	Mul (Reg, RegOrInt),
	Div (Reg, RegOrInt),
	Mod (Reg, RegOrInt),
	Eql (Reg, RegOrInt),
}

impl fmt::Display for Instr {
	fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
		match * self {
			Self::Inp (arg) => write! (formatter, "inp {}", arg) ?,
			Self::Add (left, right) => write! (formatter, "add {} {}", left, right) ?,
			Self::Mul (left, right) => write! (formatter, "mul {} {}", left, right) ?,
			Self::Div (left, right) => write! (formatter, "div {} {}", left, right) ?,
			Self::Mod (left, right) => write! (formatter, "mod {} {}", left, right) ?,
			Self::Eql (left, right) => write! (formatter, "eql {} {}", left, right) ?,
		}
		Ok (())
	}
}

#[ derive (Clone, Copy, Debug, Eq, PartialEq) ]
pub enum RegOrInt { W, X, Y, Z, Int (i64) }

impl fmt::Display for RegOrInt {
	fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
		match * self {
			Self::W => write! (formatter, "w") ?,
			Self::X => write! (formatter, "x") ?,
			Self::Y => write! (formatter, "y") ?,
			Self::Z => write! (formatter, "z") ?,
			Self::Int (val) => write! (formatter, "{}", val) ?,
		}
		Ok (())
	}
}

#[ derive (Clone, Copy, Debug, Eq, PartialEq) ]
pub enum Reg { W, X, Y, Z }

impl fmt::Display for Reg {
	fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
		match * self {
			Self::W => write! (formatter, "w") ?,
			Self::X => write! (formatter, "x") ?,
			Self::Y => write! (formatter, "y") ?,
			Self::Z => write! (formatter, "z") ?,
		}
		Ok (())
	}
}
