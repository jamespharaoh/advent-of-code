use aoc_common::*;

pub use logic::Cpu;
pub use logic::CpuError;
pub use model::Instr;
pub use model::SrcArg;
pub use model::DstArg;
pub use model::Reg;
pub use model::Val;

#[ macro_export ]
macro_rules! cpu_optimise {

	(
		$cpu:expr,
		regs = [ $( $reg:ident ),* ],
		instrs = [ $( $instr:ident ($($instr_arg:tt)*) ),* $(,)? ],
		run = { $( $run_body:tt )* },
	) => {
		use aoc_2017_cpu::{ DstArg, Reg, SrcArg };
		let match_len = [ $( stringify! ($instr) ),* ].len ().pan_u64 ();
		struct Args {
			$( $reg: Option <Reg>, )*
		}
		let mut args = Args {
			$( $reg: None, )*
		};
		let matched = loop {
			if ($cpu).instrs ().len ().pan_u64 () < ($cpu).next () + match_len { break false }
			let mut match_idx = ($cpu).next ().pan_usize ();
			$(
				if let cpu_optimise! (@instr_let_args $instr, $($instr_arg)*) =
						($cpu).instrs () [match_idx] {
					cpu_optimise! (@instr_check_args args, $($instr_arg)*);
				} else {
					break false;
				}
				match_idx += 1;
			)*
			break true;
		};
		if matched {
			$( let $reg = args.$reg.unwrap (); )*
			$( $run_body )*
		}
	};

	( @instr_let_args $instr:ident, dst $arg_0:ident, src $arg_1:ident) => {
		::aoc_2017_cpu::Instr::$instr (
			::aoc_2017_cpu::DstArg::Reg ($arg_0),
			::aoc_2017_cpu::SrcArg::Reg ($arg_1))
	};
	( @instr_let_args $instr:ident, dst $arg_0:ident, imm $arg_1:literal) => {
		::aoc_2017_cpu::Instr::$instr (
			DstArg::Reg ($arg_0),
			SrcArg::Imm ($arg_1))
	};
	( @instr_let_args $instr:ident, src $arg_0:ident, imm $arg_1:literal) => {
		::aoc_2017_cpu::Instr::$instr (
			::aoc_2017_cpu::SrcArg::Reg ($arg_0),
			::aoc_2017_cpu::SrcArg::Imm ($arg_1))
	};

	( @instr_check_args $args:ident, dst $arg_0:ident, src $arg_1:ident) => {
		cpu_optimise! (@instr_check_arg $args, reg $arg_0);
		cpu_optimise! (@instr_check_arg $args, reg $arg_1);
	};
	( @instr_check_args $args:ident, dst $arg_0:ident, imm $arg_1:literal) => {
		cpu_optimise! (@instr_check_arg $args, reg $arg_0);
	};
	( @instr_check_args $args:ident, src $arg_0:ident, imm $arg_1:literal) => {
		cpu_optimise! (@instr_check_arg $args, reg $arg_0);
	};
	( @instr_check_arg $args:ident, reg $name:ident) => {
		if let Some (prev) = $args.$name {
			if prev != $name { break false }
		} else {
			$args.$name = Some ($name);
		}
	};

}

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
		pub fn set_next (& mut self, next: u64) {
			self.next = next;
		}

		#[ inline ]
		#[ must_use ]
		pub fn next_instr (& self) -> Option <Instr> {
			self.instrs.get (self.next.qck_usize ()).copied ()
		}

		#[ inline ]
		#[ must_use ]
		pub const fn input (& self) -> & VecDeque <Val> {
			& self.input
		}

		#[ inline ]
		#[ must_use ]
		pub fn can_step (& self) -> bool {
			self.next.qck_usize () < self.instrs.len ()
		}

		#[ inline ]
		#[ must_use ]
		pub const fn limit (& self) -> u64 {
			self.limit
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
		pub fn get_reg (& mut self, reg_ch: char) -> Result <Val, & 'static str> {
			let reg = Reg::try_from (reg_ch).ok ().ok_or ("No such register") ?;
			Ok (self.regs [reg.idx ()])
		}

		#[ inline ]
		pub fn execute (& mut self) -> CpuResult <Option <Val>> {
			while self.can_step () {
				if let Some (output) = self.step () ? {
					return Ok (Some (output));
				}
			}
			Ok (None)
		}

		#[ allow (clippy::missing_inline_in_public_items) ]
		pub fn step (& mut self) -> CpuResult <Option <Val>> {
			if self.limit == 0 { return Err (CpuError::Limit) }
			self.limit -= 1;
			let instr = self.instrs [self.next.qck_usize ()];
			match instr {
				Instr::Snd (src) => {
					self.next += 1;
					return Ok (Some (self.load_src (src)));
				},
				Instr::Set (dst, src) => {
					self.store_dst (dst, self.load_src (src));
				},
				Instr::Add (dst, src) => {
					self.store_dst (dst, Val::add_2 (
						self.load_dst (dst),
						self.load_src (src),
					).ok ().ok_or (CpuError::Overflow) ?);
				},
				Instr::Sub (dst, src) => {
					self.store_dst (dst, Val::sub_2 (
						self.load_dst (dst),
						self.load_src (src),
					).ok ().ok_or (CpuError::Overflow) ?);
				},
				Instr::Mul (dst, src) => {
					self.store_dst (dst, Val::mul_2 (
						self.load_dst (dst),
						self.load_src (src),
					).ok ().ok_or (CpuError::Overflow) ?);
				},
				Instr::Mod (dst, src) => {
					self.store_dst (dst, Val::rem_2 (
						self.load_dst (dst),
						self.load_src (src),
					).ok ().ok_or (CpuError::Overflow) ?);
				},
				Instr::Rcv (dst) => {
					let data = self.input.pop_front ().ok_or (CpuError::Receive) ?;
					self.store_dst (dst, data);
				},
				Instr::Jgz (cond, offset) => {
					if self.load_src (cond) > 0 {
						self.next =
							self.next.add_signed (self.load_src (offset))
								.unwrap_or (u64::MAX);
						return Ok (None);
					}
				},
				Instr::Jnz (cnd, off) => {
					if self.load_src (cnd) != 0 {
						self.next =
							self.next.add_signed (self.load_src (off))
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
		pub fn load_reg (& self, reg: Reg) -> Val {
			self.regs [reg.idx ()]
		}

		#[ inline ]
		pub fn store_reg (& mut self, reg: Reg, val: Val) {
			self.regs [reg.idx ()] = val;
		}

		#[ inline ]
		#[ must_use ]
		pub fn load_dst (& self, dst: DstArg) -> Val {
			match dst {
				DstArg::Reg (reg) => self.load_reg (reg),
			}
		}

		#[ inline ]
		#[ must_use ]
		pub fn load_src (& self, src: SrcArg) -> Val {
			match src {
				SrcArg::Reg (reg) => self.load_reg (reg),
				SrcArg::Imm (val) => val,
			}
		}

		#[ inline ]
		pub fn store_dst (& mut self, dst: DstArg, val: Val) {
			match dst {
				DstArg::Reg (reg) => self.store_reg (reg, val),
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

	pub type CpuResult <Val> = Result <Val, CpuError>;

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
		Rcv (DstArg),
		Set (DstArg, SrcArg),
		Add (DstArg, SrcArg),
		Sub (DstArg, SrcArg),
		Mul (DstArg, SrcArg),
		Mod (DstArg, SrcArg),
		Jgz (SrcArg, SrcArg),
		Jnz (SrcArg, SrcArg),
	}

	impl Display for Instr {

		#[ inline ]
		fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
			match * self {
				Self::Snd (src) => write! (formatter, "snd {}", src),
				Self::Rcv (dst) => write! (formatter, "rcv {}", dst),
				Self::Set (dst, src) => write! (formatter, "set {} {}", dst, src),
				Self::Add (dst, src) => write! (formatter, "add {} {}", dst, src),
				Self::Sub (dst, src) => write! (formatter, "sub {} {}", dst, src),
				Self::Mul (dst, src) => write! (formatter, "mul {} {}", dst, src),
				Self::Mod (dst, src) => write! (formatter, "mod {} {}", dst, src),
				Self::Jgz (cnd, off) => write! (formatter, "jgz {} {}", cnd, off),
				Self::Jnz (cnd, off) => write! (formatter, "jnz {} {}", cnd, off),
			}
		}

	}

	impl <'inp> FromParser <'inp> for Instr {

		#[ allow (clippy::missing_inline_in_public_items) ]
		fn from_parser (parser: & mut Parser <'inp>) -> ParseResult <Self> {
			parser.any ()
				.of (|parser| {
					parse! (parser, "snd ", @confirm, src);
					Ok (Self::Snd (src))
				})
				.of (|parser| {
					parse! (parser, "rcv ", @confirm, dst);
					Ok (Self::Rcv (dst))
				})
				.of (|parser| {
					parse! (parser, "set ", @confirm, dst, " ", src);
					Ok (Self::Set (dst, src))
				})
				.of (|parser| {
					parse! (parser, "add ", @confirm, dst, " ", src);
					Ok (Self::Add (dst, src))
				})
				.of (|parser| {
					parse! (parser, "sub ", @confirm, dst, " ", src);
					Ok (Self::Sub (dst, src))
				})
				.of (|parser| {
					parse! (parser, "mul ", @confirm, dst, " ", src);
					Ok (Self::Mul (dst, src))
				})
				.of (|parser| {
					parse! (parser, "mod ", @confirm, dst, " ", src);
					Ok (Self::Mod (dst, src))
				})
				.of (|parser| {
					parse! (parser, "jgz ", @confirm, cnd, " ", off);
					Ok (Self::Jgz (cnd, off))
				})
				.of (|parser| {
					parse! (parser, "jnz ", @confirm, cnd, " ", off);
					Ok (Self::Jnz (cnd, off))
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
				.of (|parser| { parse! (parser, reg); Ok (Self::Reg (reg)) })
				.of (|parser| { parse! (parser, val); Ok (Self::Imm (val)) })
				.done ()
		}

	}

	#[ derive (Clone, Copy, Debug, Eq, PartialEq) ]
	pub struct Reg (u8);

	impl Reg {

		#[ inline ]
		#[ must_use ]
		pub fn idx (self) -> usize {
			self.0.qck_usize ()
		}

		fn as_char (self) -> char { char::from (self.0 + b'a') }

	}

	impl Display for Reg {

		#[ inline ]
		fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
			formatter.write_char (self.as_char ())
		}

	}

	impl TryFrom <char> for Reg {

		type Error = ();

		#[ inline ]
		fn try_from (ch: char) -> Result <Self, ()> {
			if ! ch.is_ascii_lowercase () { return Err (()) }
			Ok (Self (ch.qck_u8 () - b'a'))
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
