use aoc_common::*;

pub type CpuResult <Item> = Result <Item, CpuError>;

pub use instr::Instr;

parse_display_enum! {

	#[ derive (Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd) ]
	pub enum Opcode {
		Addr = "addr", Addi = "addi",
		Mulr = "mulr", Muli = "muli",
		Banr = "banr", Bani = "bani",
		Borr = "borr", Bori = "bori",
		Setr = "setr", Seti = "seti",
		Gtir = "gtir", Gtri = "gtri", Gtrr = "gtrr",
		Eqir = "eqir", Eqri = "eqri", Eqrr = "eqrr",
	}

	#[ derive (Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd) ]
	pub enum Op {
		Add = "add",
		Mul = "mul",
		Ban = "ban",
		Bor = "bor",
		Set = "set",
		Gt = "gt",
		Eq = "eq",
	}

	#[ derive (Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd) ]
	pub enum ArgType { Reg = "r", Imm = "i", Ignore = "x" }

	#[ derive (Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd) ]
	pub enum CpuError {
		Halt = "Halt",
		Overflow = "Overflow",
		Register = "Register",
		Internal = "Internal",
	}

}

impl Error for CpuError {}

impl Opcode {

	#[ inline ]
	pub fn apply <Val: Int, const NUM: usize> (
		& self,
		arg_a: Val,
		arg_b: Val,
		arg_c: Val,
		mut regs: Regs <Val, NUM>,
	) -> CpuResult <Regs <Val, NUM>> {

		use CpuError::{ Internal as ErrInt, Register as ErrReg };

		let val_a = match self.arg_a () {
			ArgType::Reg => regs.get (arg_a).ok_or (ErrReg),
			ArgType::Imm => Ok (arg_a),
			ArgType::Ignore => Err (ErrInt),
		};

		let val_b = match self.arg_b () {
			ArgType::Reg => regs.get (arg_b).ok_or (ErrReg),
			ArgType::Imm => Ok (arg_b),
			ArgType::Ignore => Err (ErrInt),
		};

		let val_c = self.op ().apply (val_a, val_b) ?;

		regs.set (arg_c, val_c).ok_or (ErrReg) ?;

		Ok (regs)

	}

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

	#[ inline ]
	#[ must_use ]
	pub const fn op (self) -> Op { self.data ().0 }

	#[ inline ]
	#[ must_use ]
	pub const fn arg_a (self) -> ArgType { self.data ().1 }

	#[ inline ]
	#[ must_use ]
	pub const fn arg_b (self) -> ArgType { self.data ().2 }

}

impl Op {

	#[ inline ]
	pub fn apply <Val: Int> (
		& self,
		val_a: CpuResult <Val>,
		val_b: CpuResult <Val>,
	) -> CpuResult <Val> {
		use CpuError::Overflow as ErrNum;
		Ok (match * self {
			Self::Add => chk! (val_a ? + val_b ?).ok ().ok_or (ErrNum) ?,
			Self::Mul => chk! (val_a ? * val_b ?).ok ().ok_or (ErrNum) ?,
			Self::Ban => val_a ? & val_b ?,
			Self::Bor => val_a ? | val_b ?,
			Self::Set => val_a ?,
			Self::Gt => if val_a ? > val_b ? { Val::ONE } else { Val::ZERO },
			Self::Eq => if val_a ? == val_b ? { Val::ONE } else { Val::ZERO },
		})
	}

}

#[ derive (Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd) ]
pub struct Regs <Val: Int, const NUM: usize> {
	regs: [Val; NUM],
}

impl <Val: Int, const NUM: usize> Regs <Val, NUM> {

	#[ inline ]
	#[ must_use ]
	pub const fn new (regs: [Val; NUM]) -> Self {
		Self { regs }
	}

	#[ inline ]
	#[ must_use ]
	pub fn get (& self, idx: Val) -> Option <Val> {
		self.regs.get (idx.to_usize ().ok () ?).copied ()
	}

	#[ inline ]
	#[ must_use ]
	pub fn set (& mut self, idx: Val, val: Val) -> Option <()> {
		* (self.regs.get_mut (idx.to_usize ().ok () ?) ?) = val;
		Some (())
	}

}

impl <Val: Int, const NUM: usize> Default for Regs <Val, NUM> {

	#[ inline ]
	fn default () -> Self {
		let regs = [Val::default (); NUM];
		Self { regs }
	}

}

impl <Val: Int, const NUM: usize> Deref for Regs <Val, NUM> {

	type Target = [Val];

	#[ inline ]
	fn deref (& self) -> & [Val] {
		& self.regs
	}

}

impl <Val: Int, const NUM: usize> Display for Regs <Val, NUM> {

	#[ inline ]
	fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
		write! (formatter, "[{}]", self.regs.display_delim (", "))
	}

}

impl <'inp, Val: Int + FromParser <'inp>, const NUM: usize> FromParser <'inp> for Regs <Val, NUM> {

	#[ inline ]
	fn from_parser (parser: & mut Parser <'inp>) -> ParseResult <Self> {
		parse! (parser, "[", (@delim ", " regs): Vec <Val>, "]");
		let regs = regs.try_into ().ok ().ok_or_else (|| parser.err ()) ?;
		Ok (Self { regs })
	}

}

mod instr {

	use super::*;

	#[ derive (Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd) ]
	pub struct Instr <Val: Int> {
		pub opcode: Opcode,
		pub arg_a: Val,
		pub arg_b: Val,
		pub arg_c: Val,
	}

	struct_parser_display! (
		params = { Val: Int + FromParser <'inp> }
		Instr <Val> { opcode, arg_a, arg_b, arg_c } = [
			opcode, " ", arg_a, " ", arg_b, " ", arg_c,
		]
	);

	impl <Val: Int> Debug for Instr <Val> {

		#[ inline ]
		fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
			write! (formatter,
				"Instr ({opcode:?}, {arg_a}, {arg_b}, {arg_c})",
				opcode = self.opcode,
				arg_a = self.arg_a,
				arg_b = self.arg_b,
				arg_c = self.arg_c,
			)
		}

	}

	impl <Val: Int> Instr <Val> {

		#[ inline ]
		pub fn apply <const NUM: usize> (& self, regs: Regs <Val, NUM>) -> CpuResult <Regs <Val, NUM>> {
			self.opcode.apply (self.arg_a, self.arg_b, self.arg_c, regs)
		}

	}

	#[ cfg (test) ]
	mod tests {

		use super::*;

		#[ test ]
		fn instr_parse () {
			assert_eq_ok! (
				Instr { opcode: Opcode::Addi, arg_a: 3, arg_b: 16, arg_c: 3 },
				Instr::parse_from_str ("addi 3 16 3"));
		}

	}

}
