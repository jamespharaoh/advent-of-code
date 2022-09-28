use super::*;

#[ derive (Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd) ]
pub enum Instr {
	And (Reg, Reg),
	Or (Reg, Reg),
	Not (Reg, Reg),
}

impl Display for Instr {
	fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
		match * self {
			Self::And (src, dst) => write! (formatter, "AND {src} {dst}"),
			Self::Or (src, dst) => write! (formatter, "OR {src} {dst}"),
			Self::Not (src, dst) => write! (formatter, "NOT {src} {dst}"),
		}
	}
}

#[ derive (Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd) ]
pub struct Prog {
	instrs: Vec <Instr>,
}

impl Prog {

	#[ must_use ]
	pub const fn new () -> Self {
		Self { instrs: Vec::new () }
	}

	#[ must_use ]
	pub fn len (& self) -> usize {
		self.instrs.len ()
	}

	#[ must_use ]
	pub fn is_empty (& self) -> bool {
		self.instrs.is_empty ()
	}

	#[ must_use ]
	pub fn push (& mut self, instr: Instr) -> Option <()> {
		if 15 <= self.instrs.len () { return None }
		self.instrs.push (instr);
		Some (())
	}

	pub fn eval (& self, regs: & mut [bool; 26]) {
		for instr in & self.instrs {
			match * instr {
				Instr::Or (src, dst) =>
					regs [dst.idx ()] = regs [src.idx ()] || regs [dst.idx ()],
				Instr::And (src, dst) =>
					regs [dst.idx ()] = regs [src.idx ()] && regs [dst.idx ()],
				Instr::Not (src, dst) =>
					regs [dst.idx ()] = ! regs [src.idx ()],
			}
		}
	}

	pub fn test <const LEN: usize> (& self, sample: & [bool]) -> Result <(), Vec <usize>> {
		let mut track = Vec::new ();
		let mut idx = 0;
		loop {
			track.push (idx);
			if sample.len () <= idx { return Ok (()) }
			let sample = & sample [idx .. ];
			if ! sample [0] { return Err (track) }
			let mut regs = [false; 26];
			for (reg_idx, reg) in regs.iter_mut ().enumerate () {
				* reg = sample.get (reg_idx + 1).copied ().unwrap_or (true);
			}
			self.eval (& mut regs);
			idx += if regs [Reg::J.idx ()] { 4 } else { 1 };
		}
	}

}

impl Default for Prog {
	fn default () -> Self {
		Self::new ()
	}
}

impl Display for Prog {
	fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
		for instr in & self.instrs {
			Display::fmt (instr, formatter) ?;
			formatter.write_char ('\n') ?;
		}
		Ok (())
	}
}

impl <'prog> IntoIterator for & 'prog Prog {
	type Item = & 'prog Instr;
	type IntoIter = SliceIter <'prog, Instr>;
	fn into_iter (self) -> SliceIter <'prog, Instr> {
		self.instrs.iter ()
	}
}

#[ derive (Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd) ]
pub struct Reg {
	idx: u8,
}

impl Reg {

	#[ must_use ]
	pub fn char (self) -> char {
		assert! (self.idx < 26);
		('A'.pan_u32 () + self.idx.pan_u32 ()).pan_char ()
	}

	#[ must_use ]
	pub fn idx (self) -> usize {
		self.idx.pan_usize ()
	}

	pub const J: Self = Self { idx: 9 };
	pub const T: Self = Self{ idx: 19 };

}

impl Display for Reg {
	fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
		formatter.write_char (self.char ())
	}
}

impl TryFrom <char> for Reg {
	type Error = Overflow;
	fn try_from (ch: char) -> Result <Self, Overflow> {
		if ! ('A' ..= 'Z').contains (& ch) { return Err (Overflow) }
		Ok (Self { idx: (ch.pan_u32 () - 'A'.pan_u32 ()).pan_u8 () })
	}
}

impl TryFrom <usize> for Reg {
	type Error = Overflow;
	fn try_from (idx: usize) -> Result <Self, Overflow> {
		if ! (0 .. 26).contains (& idx) { return Err (Overflow) }
		Ok (Self { idx: idx.pan_u8 () })
	}
}
