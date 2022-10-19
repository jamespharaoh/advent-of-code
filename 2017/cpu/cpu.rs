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
					self.store_dst (dst, chk! (self.load_dst (dst) + self.load_src (src)) ?);
				},
				Instr::Sub (dst, src) => {
					self.store_dst (dst, chk! (self.load_dst (dst) - self.load_src (src)) ?);
				},
				Instr::Mul (dst, src) => {
					self.store_dst (dst, chk! (self.load_dst (dst) * self.load_src (src)) ?);
				},
				Instr::Mod (dst, src) => {
					self.store_dst (dst, chk! (self.load_dst (dst) % self.load_src (src)) ?);
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

	impl From <Overflow> for CpuError {
		#[ inline ]
		fn from (_overflow: Overflow) -> Self {
			Self::Overflow
		}
	}

}

mod model {

	use super::*;

	pub type Val = i64;

	enum_decl_parser_display! {
		#[ derive (Clone, Copy, Debug, Eq, PartialEq) ]
		pub enum Instr {
			Snd (src: SrcArg) = [ "snd ", src ],
			Rcv (dst: DstArg) = [ "rcv ", dst ],
			Set (dst: DstArg, src: SrcArg) = [ "set ", dst, " ", src ],
			Add (dst: DstArg, src: SrcArg) = [ "add ", dst, " ", src ],
			Sub (dst: DstArg, src: SrcArg) = [ "sub ", dst, " ", src ],
			Mul (dst: DstArg, src: SrcArg) = [ "mul ", dst, " ", src ],
			Mod (dst: DstArg, src: SrcArg) = [ "mod ", dst, " ", src ],
			Jgz (cnd: SrcArg, off: SrcArg) = [ "jgz ", cnd, " ", off ],
			Jnz (cnd: SrcArg, off: SrcArg) = [ "jnz ", cnd, " ", off ],
		}
	}

	enum_decl_parser_display! {
		#[ derive (Clone, Copy, Debug, Eq, PartialEq) ]
		pub enum DstArg {
			Reg (reg: Reg) = [ reg ],
		}
	}

	enum_decl_parser_display! {
		#[ derive (Clone, Copy, Debug, Eq, PartialEq) ]
		pub enum SrcArg {
			Reg (reg: Reg) = [ reg ],
			Imm (val: Val) = [ val ],
		}
	}

	#[ derive (Clone, Copy, Debug, Eq, PartialEq) ]
	pub struct Reg (u8);

	struct_parser_display! {
		Reg (id) = [
			@display { let id = (id + b'a').pan_char (); },
			id = 'a' ..= 'z',
			@parse { let id = id.pan_u8 () - b'a'; },
		]
	}

	impl Reg {

		#[ inline ]
		#[ must_use ]
		pub fn idx (self) -> usize {
			self.0.qck_usize ()
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

}
