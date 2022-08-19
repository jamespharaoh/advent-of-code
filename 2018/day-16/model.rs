use super::*;

pub type Val = u16;

parse_display_enum! {

	#[ derive (Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd) ]
	pub enum Opcode {
		Addr = "addr", Addi = "addi",
		Mulr = "mulr", Muli = "muli",
		Banr = "banr", Bani = "bani",
		Borr = "borr", Bori = "bori",
		Setr = "setr", Seti = "seti",
		Gtir = "gtir", Gtri = "gtri", Gtrr = "gtrr",
		Eqir = "eqir", Eqri = "eqri", Eqrr = "eqrr",
	}

	#[ derive (Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd) ]
	pub enum Op {
		Add = "add",
		Mul = "mul",
		Ban = "ban",
		Bor = "bor",
		Set = "set",
		Gt = "gt",
		Eq = "eq",
	}

	#[ derive (Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd) ]
	pub enum ArgType { Reg = "r", Imm = "i", Ignore = "x" }

}

impl Opcode {

	#[ inline ]
	#[ must_use ]
	pub const fn data (self) -> (Op, ArgType, ArgType) {
		match self {
			Self::Addr => (Op::Add, ArgType::Reg, ArgType::Reg),
			Self::Addi => (Op::Add, ArgType::Reg, ArgType::Imm),
			Self::Mulr => (Op::Mul, ArgType::Reg, ArgType::Reg),
			Self::Muli => (Op::Mul, ArgType::Reg, ArgType::Imm),
			Self::Banr => (Op::Ban, ArgType::Reg, ArgType::Reg),
			Self::Bani => (Op::Ban, ArgType::Reg, ArgType::Imm),
			Self::Borr => (Op::Bor, ArgType::Reg, ArgType::Reg),
			Self::Bori => (Op::Bor, ArgType::Reg, ArgType::Imm),
			Self::Setr => (Op::Set, ArgType::Reg, ArgType::Ignore),
			Self::Seti => (Op::Set, ArgType::Imm, ArgType::Ignore),
			Self::Gtir => (Op::Gt, ArgType::Imm, ArgType::Reg),
			Self::Gtri => (Op::Gt, ArgType::Reg, ArgType::Imm),
			Self::Gtrr => (Op::Gt, ArgType::Reg, ArgType::Reg),
			Self::Eqir => (Op::Eq, ArgType::Imm, ArgType::Reg),
			Self::Eqri => (Op::Eq, ArgType::Reg, ArgType::Imm),
			Self::Eqrr => (Op::Eq, ArgType::Reg, ArgType::Reg),
		}
	}

	#[ must_use ]
	pub const fn op (self) -> Op { self.data ().0 }

	#[ must_use ]
	pub const fn arg_a (self) -> ArgType { self.data ().1 }

	#[ must_use ]
	pub const fn arg_b (self) -> ArgType { self.data ().2 }

}

#[ derive (Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd) ]
pub struct Regs {
	pub reg_0: Val,
	pub reg_1: Val,
	pub reg_2: Val,
	pub reg_3: Val,
}

impl Regs {

	#[ inline ]
	#[ must_use ]
	pub const fn get (& self, idx: Val) -> Option <Val> {
		Some (match idx {
			0 => self.reg_0,
			1 => self.reg_1,
			2 => self.reg_2,
			3 => self.reg_3,
			_ => return None,
		})
	}

	#[ inline ]
	pub fn set (& mut self, idx: Val, val: Val) -> Option <()> {
		match idx {
			0 => self.reg_0 = val,
			1 => self.reg_1 = val,
			2 => self.reg_2 = val,
			3 => self.reg_3 = val,
			_ => return None,
		}
		Some (())
	}

}

impl Display for Regs {
	fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
		write! (formatter,
			"[{reg_0}, {reg_1}, {reg_2}, {reg_3}]",
			reg_0 = self.reg_0,
			reg_1 = self.reg_1,
			reg_2 = self.reg_2,
			reg_3 = self.reg_3,
		) ?;
		Ok (())
	}
}

impl <'inp> FromParser <'inp> for Regs {
	fn from_parser (parser: & mut Parser <'inp>) -> ParseResult <Self> {
		parse! (parser, "[", reg_0, ", ", reg_1, ", ", reg_2, ", ", reg_3, "]");
		Ok (Self { reg_0, reg_1, reg_2, reg_3 })
	}
}

#[ derive (Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd) ]
pub struct Instr {
	pub op: Val,
	pub arg_a: Val,
	pub arg_b: Val,
	pub arg_c: Val,
}

impl Display for Instr {
	fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
		write! (formatter,
			"{op} {arg_a} {arg_b} {arg_c}",
			op = self.op,
			arg_a = self.arg_a,
			arg_b = self.arg_b,
			arg_c = self.arg_c,
		) ?;
		Ok (())
	}
}

impl <'inp> FromParser <'inp> for Instr {
	fn from_parser (parser: & mut Parser <'inp>) -> ParseResult <Self> {
		parse! (parser, op, " ", arg_a, " ", arg_b, " ", arg_c);
		Ok (Self { op, arg_a, arg_b, arg_c })
	}
}

#[ derive (Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd) ]
pub struct Sample {
	pub before: Regs,
	pub instr: Instr,
	pub after: Regs,
}

impl Display for Sample {
	fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
		write! (formatter,
			"Before: {before}\n{instr}\nAfter:  {after}",
			before = self.before,
			instr = self.instr,
			after = self.after)
	}
}

impl <'inp> FromParser <'inp> for Sample {
	fn from_parser (parser: & mut Parser <'inp>) -> ParseResult <Self> {
		parse! (parser, "Before: ", before, "\n", instr, "\nAfter:  ", after);
		Ok (Self { before, instr, after })
	}
}
