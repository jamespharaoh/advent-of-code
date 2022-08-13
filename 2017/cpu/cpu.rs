use aoc_common::*;

pub use logic::Cpu;
pub use logic::CpuError;
pub use model::Instr;
pub use model::SrcArg;
pub use model::DstArg;
pub use model::Reg;

mod logic {

	use super::*;
	use model::Instr;
	use model::Val;

	#[ derive (Clone, Debug) ]
	pub struct Cpu {
		instrs: Rc <[Instr]>,
		regs: [Val; 26],
		next: u64,
		limit: u64,
		input: VecDeque <Val>,
	}

	impl Cpu {

		#[ inline ]
		pub fn new (instrs: impl Into <Rc <[Instr]>>) -> Self {
			Self {
				instrs: instrs.into (),
				.. default ()
			}
		}

		#[ inline ]
		#[ must_use ]
		pub const fn instrs (& self) -> & Rc <[Instr]> {
			& self.instrs
		}

		#[ inline ]
		#[ must_use ]
		pub const fn next (& self) -> u64 {
			self.next
		}

		#[ inline ]
		#[ must_use ]
		pub const fn input (& self) -> & VecDeque <Val> {
			& self.input
		}

		#[ inline ]
		#[ must_use ]
		pub fn can_step (& self) -> bool {
			self.next.as_usize () < self.instrs.len ()
		}

		#[ inline ]
		pub fn set_limit (& mut self, limit: u64) -> & mut Self {
			self.limit = limit;
			self
		}

		#[ inline ]
		pub fn push_input (& mut self, input: Val) -> & mut Self {
			self.input.push_back (input);
			self
		}

		#[ inline ]
		pub fn set_reg (& mut self, reg_ch: char, val: Val) -> Result <& mut Self, & 'static str> {
			let reg = Reg::try_from (reg_ch).ok ().ok_or ("No such register") ?;
			self.regs [reg.idx ()] = val;
			Ok (self)
		}

		#[ inline ]
		pub fn execute (& mut self) -> Result <Option <Val>, CpuError> {
			while self.can_step () {
				if let Some (output) = self.step () ? {
					return Ok (Some (output));
				}
			}
			Ok (None)
		}

		#[ allow (clippy::missing_inline_in_public_items) ]
		pub fn step (& mut self) -> Result <Option <Val>, CpuError> {
			if self.limit == 0 { return Err (CpuError::Limit) }
			self.limit -= 1;
			let instr = self.instrs [self.next.as_usize ()];
			match instr {
				Instr::Snd (src) => {
					self.next += 1;
					return Ok (Some (self.load_src (src)));
				},
				Instr::Set (dst, src) => {
					self.store (dst, self.load_src (src));
				},
				Instr::Add (dst, src) => {
					self.store (dst, Val::add_2 (
						self.load_dst (dst),
						self.load_src (src),
					).ok ().ok_or (CpuError::Overflow) ?);
				},
				Instr::Mul (dst, src) => {
					self.store (dst, Val::mul_2 (
						self.load_dst (dst),
						self.load_src (src),
					).ok ().ok_or (CpuError::Overflow) ?);
				},
				Instr::Mod (dst, src) => {
					self.store (dst, Val::rem_2 (
						self.load_dst (dst),
						self.load_src (src),
					).ok ().ok_or (CpuError::Overflow) ?);
				},
				Instr::Rcv (dst) => {
					let data = self.input.pop_front ().ok_or (CpuError::Receive) ?;
					self.store (dst, data);
				},
				Instr::Jgz (cond, offset) => {
					if self.load_src (cond) > 0 {
						self.next =
							self.next.add_signed (self.load_src (offset))
								.unwrap_or (u64::MAX);
						return Ok (None);
					}
				},
			}
			self.next += 1;
			Ok (None)
		}

		#[ inline ]
		#[ must_use ]
		pub fn load_dst (& self, dst: DstArg) -> Val {
			match dst {
				DstArg::Reg (reg) => self.regs [reg.idx ()],
			}
		}

		#[ inline ]
		#[ must_use ]
		pub fn load_src (& self, src: SrcArg) -> Val {
			match src {
				SrcArg::Reg (reg) => self.regs [reg.idx ()],
				SrcArg::Imm (val) => val,
			}
		}

		#[ inline ]
		pub fn store (& mut self, dst: DstArg, val: Val) {
			match dst {
				DstArg::Reg (reg) => self.regs [reg.idx ()] = val,
			}
		}

	}

	impl Default for Cpu {

		#[ inline ]
		fn default () -> Self {
			Self {
				instrs: Rc::from ([]),
				regs: [Val::ZERO; 26],
				next: 0_u64,
				limit: u64::MAX,
				input: VecDeque::new (),
			}
		}

	}

	#[ derive (Clone, Copy, Debug) ]
	pub enum CpuError {
		Limit,
		Overflow,
		Receive,
	}

	impl Display for CpuError {

		#[ inline ]
		fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
			match * self {
				Self::Limit => write! (formatter, "Instruction limit reached") ?,
				Self::Overflow => write! (formatter, "Numeric overflow") ?,
				Self::Receive => write! (formatter, "Tried to receive with no input") ?,
			}
			Ok (())
		}

	}

	impl Error for CpuError {
	}

}

mod model {

	use super::*;

	pub type Val = i64;

	#[ derive (Clone, Copy, Debug, Eq, PartialEq) ]
	pub enum Instr {
		Snd (SrcArg),
		Set (DstArg, SrcArg),
		Add (DstArg, SrcArg),
		Mul (DstArg, SrcArg),
		Mod (DstArg, SrcArg),
		Rcv (DstArg),
		Jgz (SrcArg, SrcArg),
	}

	impl Display for Instr {

		#[ inline ]
		fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
			match * self {
				Self::Snd (arg) => write! (formatter, "snd {}", arg),
				Self::Set (arg_0, arg_1) => write! (formatter, "set {} {}", arg_0, arg_1),
				Self::Add (arg_0, arg_1) => write! (formatter, "add {} {}", arg_0, arg_1),
				Self::Mul (arg_0, arg_1) => write! (formatter, "mul {} {}", arg_0, arg_1),
				Self::Mod (arg_0, arg_1) => write! (formatter, "mod {} {}", arg_0, arg_1),
				Self::Rcv (arg) => write! (formatter, "rcv {}", arg),
				Self::Jgz (arg_0, arg_1) => write! (formatter, "jgz {} {}", arg_0, arg_1),
			}
		}

	}

	impl <'inp> FromParser <'inp> for Instr {

		#[ allow (clippy::missing_inline_in_public_items) ]
		fn from_parser (parser: & mut Parser <'inp>) -> ParseResult <Self> {
			parser.any ()
				.of (|parser| {
					let arg = parser.expect ("snd ") ?.confirm ().item () ?;
					Ok (Self::Snd (arg))
				})
				.of (|parser| {
					let arg_0 = parser.expect ("set ") ?.confirm ().item () ?;
					let arg_1 = parser.expect (" ") ?.item () ?;
					Ok (Self::Set (arg_0, arg_1))
				})
				.of (|parser| {
					let arg_0 = parser.expect ("add ") ?.confirm ().item () ?;
					let arg_1 = parser.expect (" ") ?.item () ?;
					Ok (Self::Add (arg_0, arg_1))
				})
				.of (|parser| {
					let arg_0 = parser.expect ("mul ") ?.confirm ().item () ?;
					let arg_1 = parser.expect (" ") ?.item () ?;
					Ok (Self::Mul (arg_0, arg_1))
				})
				.of (|parser| {
					let arg_0 = parser.expect ("mod ") ?.confirm ().item () ?;
					let arg_1 = parser.expect (" ") ?.item () ?;
					Ok (Self::Mod (arg_0, arg_1))
				})
				.of (|parser| {
					let arg = parser.expect ("rcv ") ?.confirm ().item () ?;
					Ok (Self::Rcv (arg))
				})
				.of (|parser| {
					let arg_0 = parser.expect ("jgz ") ?.confirm ().item () ?;
					let arg_1 = parser.expect (" ") ?.item () ?;
					Ok (Self::Jgz (arg_0, arg_1))
				})
				.done ()
		}

	}

	#[ derive (Clone, Copy, Debug, Eq, PartialEq) ]
	pub enum DstArg {
		Reg (Reg),
	}

	impl Display for DstArg {

		#[ inline ]
		fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
			match * self {
				Self::Reg (reg) => write! (formatter, "{}", reg) ?,
			}
			Ok (())
		}

	}

	impl <'inp> FromParser <'inp> for DstArg {

		#[ inline ]
		fn from_parser (parser: & mut Parser <'inp>) -> ParseResult <Self> {
			parser.any ()
				.of (|parser| {
					let reg = parser.item::<Reg> () ?;
					Ok (Self::Reg (reg))
				})
				.done ()
		}

	}

	#[ derive (Clone, Copy, Debug, Eq, PartialEq) ]
	pub enum SrcArg {
		Reg (Reg),
		Imm (Val),
	}

	impl Display for SrcArg {

		#[ inline ]
		fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
			match * self {
				Self::Reg (reg) => write! (formatter, "{}", reg) ?,
				Self::Imm (val) => write! (formatter, "{}", val) ?,
			}
			Ok (())
		}

	}

	impl <'inp> FromParser <'inp> for SrcArg {

		#[ inline ]
		fn from_parser (parser: & mut Parser <'inp>) -> ParseResult <Self> {
			parser.any ()
				.of (|parser| {
					let reg = parser.item::<Reg> () ?;
					Ok (Self::Reg (reg))
				})
				.of (|parser| {
					let val = parser.int () ?;
					Ok (Self::Imm (val))
				})
				.done ()
		}

	}

	#[ derive (Clone, Copy, Debug, Eq, PartialEq) ]
	pub struct Reg (u8);

	impl Reg {

		#[ inline ]
		#[ must_use ]
		pub fn idx (self) -> usize {
			self.0.as_usize ()
		}

		fn as_char (self) -> char { (self.0 + 'a'.as_u8 ()).as_char () }

	}

	impl Display for Reg {

		#[ inline ]
		fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
			write! (formatter, "{}", self.as_char ()) ?;
			Ok (())
		}

	}

	impl TryFrom <char> for Reg {

		type Error = ();

		#[ inline ]
		fn try_from (ch: char) -> Result <Self, ()> {
			if ! ch.is_ascii_lowercase () { return Err (()) }
			Ok (Self (ch.as_u8 () - 'a'.as_u8 ()))
		}

	}

	impl <'inp> FromParser <'inp> for Reg {

		#[ inline ]
		fn from_parser (parser: & mut Parser) -> ParseResult <Self> {
			parser.any ()
				.of (|parser| {
					let ch = parser.expect_next () ?;
					let reg = Self::try_from (ch).map_err (|()| parser.err ()) ?;
					Ok (reg)
				})
				.done ()
		}

	}

}
