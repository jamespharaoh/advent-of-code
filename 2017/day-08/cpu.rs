use super::*;

pub type Val = i32;

#[ derive (Clone, Debug) ]
pub struct Cpu <'inp> {
	instrs: Rc <[Instr <'inp>]>,
	regs: HashMap <InpStr <'inp>, Val>,
	next: usize,
}

enum_decl_parser_display! {
	#[ derive (Clone, Copy, Debug, Eq, PartialEq) ]
	pub enum ChkOp {
		GtrEq = [ ">=" ],
		LsrEq = [ "<=" ],
		Eq = [ "==" ],
		NotEq = [ "!=" ],
		Gtr = [ ">" ],
		Lsr = [ "<" ],
	}
}

enum_decl_parser_display! {
	#[ derive (Clone, Copy, Debug, Eq, PartialEq) ]
	pub enum DstOp {
		Inc = [ "inc" ],
		Dec = [ "dec" ],
	}
}

#[ derive (Clone, Debug, Eq, PartialEq) ]
pub struct Instr <'inp> {
	pub dst_reg: InpStr <'inp>,
	pub dst_op: DstOp,
	pub dst_amt: Val,
	pub chk_reg: InpStr <'inp>,
	pub chk_op: ChkOp,
	pub chk_amt: Val,
}

struct_parser_display! {
	input_lifetime = 'inp;
	Instr <'inp> { dst_reg, dst_op, dst_amt, chk_reg, chk_op, chk_amt } = [
		@str dst_reg = (|ch| { ch.is_ascii_lowercase () }, 1 ..= 8), " ",
		dst_op, " ",
		dst_amt, " ",
		"if ",
		@str chk_reg = (|ch| { ch.is_ascii_lowercase () }, 1 ..= 8), " ",
		chk_op, " ",
		chk_amt,
	]
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
