use super::*;
use parser::*;

pub type Val = i32;

#[ derive (Clone, Debug) ]
pub struct Cpu <'inp> {
	instrs: Rc <[Instr <'inp>]>,
	regs: HashMap <InpStr <'inp>, Val>,
	next: usize,
}

#[ derive (Clone, Copy, Debug, Eq, PartialEq) ]
pub enum ChkOp { Gtr, GtrEq, Lsr, LsrEq, Eq, NotEq }

#[ derive (Clone, Copy, Debug, Eq, PartialEq) ]
pub enum DstOp { Inc, Dec }

#[ derive (Clone, Debug, Eq, PartialEq) ]
pub struct Instr <'inp> {
	pub dst_reg: InpStr <'inp>,
	pub dst_op: DstOp,
	pub dst_amt: Val,
	pub chk_reg: InpStr <'inp>,
	pub chk_op: ChkOp,
	pub chk_amt: Val,
}

impl <'inp> Cpu <'inp> {

	#[ inline ]
	pub fn new (instrs: impl Into <Rc <[Instr <'inp>]>>) -> Self {
		Self {
			instrs: instrs.into (),
			regs: default (),
			next: 0,
		}
	}

	#[ inline ]
	pub fn regs (& self) -> impl Iterator <Item = (& InpStr <'inp>, & Val)> {
		self.regs.iter ()
	}

	pub fn execute (& mut self) {
		while self.next < self.instrs.len () {
			self.step ();
		}
	}

	#[ must_use ]
	pub fn is_ready (& self) -> bool {
		self.next < self.instrs.len ()
	}

	pub fn step (& mut self) -> Option <(InpStr <'inp>, Val)> {
		let instr = self.instrs [self.next].clone ();
		self.next += 1;
		let chk_val = self.load (& instr.chk_reg);
		let chk_result = match instr.chk_op {
			ChkOp::Gtr => chk_val > instr.chk_amt,
			ChkOp::GtrEq => chk_val >= instr.chk_amt,
			ChkOp::Lsr => chk_val < instr.chk_amt,
			ChkOp::LsrEq => chk_val <= instr.chk_amt,
			ChkOp::Eq => chk_val == instr.chk_amt,
			ChkOp::NotEq => chk_val != instr.chk_amt,
		};
		if ! chk_result { return None }
		let mut dst_val = self.load (& instr.dst_reg);
		match instr.dst_op {	
			DstOp::Inc => dst_val += instr.dst_amt,
			DstOp::Dec => dst_val -= instr.dst_amt,
		}
		self.store (& instr.dst_reg, dst_val);
		Some ((instr.dst_reg.clone (), dst_val))
	}

	#[ inline ]
	#[ must_use ]
	pub fn load (& self, name: & InpStr <'inp>) -> Val {
		self.regs.get (name).copied ().unwrap_or (0_i32)
	}

	#[ inline ]
	pub fn store (& mut self, name: & InpStr <'inp>, val: Val) {
		self.regs.insert (name.clone (), val);
	}

}

impl Display for ChkOp {
	fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
		match * self {
			Self::Gtr => write! (formatter, ">") ?,
			Self::GtrEq => write! (formatter, ">=") ?,
			Self::Lsr => write! (formatter, "<") ?,
			Self::LsrEq => write! (formatter, "<=") ?,
			Self::Eq => write! (formatter, "==") ?,
			Self::NotEq => write! (formatter, "!=") ?,
		}
		Ok (())
	}
}

impl <'inp> FromParser <'inp> for ChkOp {
	fn from_parser (parser: & mut Parser <'inp>) -> ParseResult <Self> {
		parser.any ()
			.of (|parser| { parser.expect (">=") ?; Ok (Self::GtrEq) })
			.of (|parser| { parser.expect ("<=") ?; Ok (Self::LsrEq) })
			.of (|parser| { parser.expect ("==") ?; Ok (Self::Eq) })
			.of (|parser| { parser.expect ("!=") ?; Ok (Self::NotEq) })
			.of (|parser| { parser.expect (">") ?; Ok (Self::Gtr) })
			.of (|parser| { parser.expect ("<") ?; Ok (Self::Lsr) })
			.done ()
	}
}

impl Display for DstOp {
	fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
		match * self {
			Self::Inc => write! (formatter, "inc") ?,
			Self::Dec => write! (formatter, "dec") ?,
		}
		Ok (())
	}
}

impl <'inp> FromParser <'inp> for DstOp {
	fn from_parser (parser: & mut Parser <'inp>) -> ParseResult <Self> {
		parser.any ()
			.of (|parser| { parser.expect ("inc") ?; Ok (Self::Inc) })
			.of (|parser| { parser.expect ("dec") ?; Ok (Self::Dec) })
			.done ()
	}
}

impl <'inp> Display for Instr <'inp> {
	fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
		write! (formatter, "{} {} {} if {} {} {}",
			self.dst_reg, self.dst_op, self.dst_amt,
			self.chk_reg, self.chk_op, self.chk_amt) ?;
		Ok (())
	}
}

impl <'inp> FromParser <'inp> for Instr <'inp> {
	fn from_parser (parser: & mut Parser <'inp>) -> ParseResult <Self> {
		let dst_reg = parser.word () ?.into ();
		let dst_op = parser.item () ?;
		let dst_amt = parser.int () ?;
		parser.expect_word ("if") ?;
		let chk_reg = parser.word () ?.into ();
		let chk_op = parser.item () ?;
		let chk_amt = parser.int () ?;
		parser.end () ?;
		Ok (Instr { dst_reg, dst_op, dst_amt, chk_reg, chk_op, chk_amt })
	}
}
