use super::*;

#[ derive (Default) ]
pub struct Cpu {
	pub instrs: Rc <Vec <Instr>>,
	pub reg_a: i32,
	pub reg_b: i32,
	pub reg_c: i32,
	pub reg_d: i32,
	pub limit: u32,
	pub next: u32,
}

impl Cpu {

	#[ inline ]
	const fn load (& self, arg: Arg) -> i32 {
		match arg {
			Arg::RegA => self.reg_a,
			Arg::RegB => self.reg_b,
			Arg::RegC => self.reg_c,
			Arg::RegD => self.reg_d,
			Arg::Imm (val) => val,
		}
	}

	#[ inline ]
	fn store (& mut self, arg: Arg, val: i32) {
		match arg {
			Arg::RegA => self.reg_a = val,
			Arg::RegB => self.reg_b = val,
			Arg::RegC => self.reg_c = val,
			Arg::RegD => self.reg_d = val,
			Arg::Imm (_) => (),
		}
	}

	#[ allow (clippy::missing_inline_in_public_items) ]
	pub fn exec (& mut self) -> GenResult <Option <i32>> {
		while self.next.qck_usize () < self.instrs.len () {
			if self.limit == 0 { return Err ("Reached ops limit".into ()) }
			self.limit -= 1;
			#[ allow (clippy::match_on_vec_items) ]
			match self.instrs [self.next.qck_usize ()] {
				Instr::Cpy (src, dst) => self.store (dst, self.load (src)),
				Instr::Inc (reg) => self.store (reg, chk! (self.load (reg) + 1_i32) ?),
				Instr::Dec (reg) => self.store (reg, chk! (self.load (reg) - 1_i32) ?),
				Instr::Jnz (src, dst) => { self.jnz (src, dst) ?; continue }
				Instr::Tgl (dst) => {
					let idx = u32::add_signed (self.next, self.load (dst)) ?;
					if let Some (instr) =
						Rc::make_mut (& mut self.instrs)
							.get_mut (idx.qck_usize ()) {
						* instr = match * instr {
							Instr::Cpy (src, dst) => Instr::Jnz (src, dst),
							Instr::Jnz (src, dst) => Instr::Cpy (src, dst),
							Instr::Inc (arg) => Instr::Dec (arg),
							Instr::Dec (arg) | Instr::Tgl (arg) | Instr::Out (arg) =>
								Instr::Inc (arg),
						};
					}
				},
				Instr::Out (arg) => {
					self.next += 1;
					return Ok (Some (self.load (arg)));
				},
			}
			self.next += 1;
		}
		Ok (None)
	}

	fn jnz (& mut self, src: Arg, dst: Arg) -> GenResult <()> {

		// ignore if src is zero

		if self.load (src) == 0_i32 {
			self.next += 1;
			return Ok (());
		}

		// check if this can be optimized

		if self.next >= 2 {
			match <[Instr; 3]>::try_from (
				& self.instrs [self.next.qck_usize () - 2 .. self.next.qck_usize () + 1],
			).unwrap () {
				[
					Instr::Dec (arg),
					Instr::Inc (dst),
					Instr::Jnz (arg_0, Arg::Imm (-2_i32)),
				] | [
					Instr::Inc (dst),
					Instr::Dec (arg),
					Instr::Jnz (arg_0, Arg::Imm (-2_i32)),
				] if arg == arg_0 && dst != arg && arg.is_reg () && dst.is_reg () => {
					self.store (dst, chk! (self.load (dst) + self.load (arg)) ?);
					self.store (arg, 0);
					return Ok (());
				},
				_ => (),
			}
		}

		if self.next >= 4 {
			match <[Instr; 5]>::try_from (
				& self.instrs [self.next.qck_usize () - 4 .. self.next.qck_usize () + 1],
			).unwrap () {
				[
					Instr::Jnz (arg, Arg::Imm (2_i32)),
					Instr::Jnz (Arg::Imm (1_i32), Arg::Imm (4_i32)),
					Instr::Dec (dst),
					Instr::Dec (arg_0),
					Instr::Jnz (Arg::Imm (1_i32), Arg::Imm (-4_i32)),
				] if arg == arg_0 && arg != dst && arg.is_reg () && dst.is_reg () => {
					self.store (dst, chk! (self.load (dst) - self.load (arg)) ?);
					self.store (arg, 0);
				},
				_ => (),
			}
		}

		if self.next >= 5 {
			match <[Instr; 6]>::try_from (
				& self.instrs [self.next.qck_usize () - 5 .. self.next.qck_usize () + 1],
			).unwrap () {
				[
					Instr::Cpy (arg, tmp),
					Instr::Inc (dst),
					Instr::Dec (tmp_0),
					Instr::Jnz (tmp_1, Arg::Imm (-2_i32)),
					Instr::Dec (ctr),
					Instr::Jnz (ctr_0, Arg::Imm (-5_i32)),
				] if tmp == tmp_0 && tmp == tmp_1 && ctr == ctr_0 && arg != tmp && arg != dst
						&& arg != ctr && tmp != dst && tmp != ctr && dst != ctr && dst.is_reg ()
						&& tmp.is_reg () && ctr.is_reg () => {
					self.store (dst, chk! (self.load (dst) + self.load (arg) * self.load (ctr)) ?);
					self.store (tmp, 0);
					self.store (ctr, 0);
					return Ok (());
				},
				_ => (),
			}
		}

		if self.next >= 7 {
			match <[Instr; 8]>::try_from (
				& self.instrs [self.next.qck_usize () - 7 .. self.next.qck_usize () + 1],
			).unwrap () {
				[
					Instr::Cpy (Arg::Imm (div), rem),
					Instr::Jnz (mul, Arg::Imm (2_i32)),
					Instr::Jnz (Arg::Imm (1_i32), Arg::Imm (6_i32)),
					Instr::Dec (mul_0),
					Instr::Dec (rem_0),
					Instr::Jnz (rem_1, Arg::Imm (-4_i32)),
					Instr::Inc (out),
					Instr::Jnz (Arg::Imm (1_i32), Arg::Imm (-7_i32)),
				] if rem != mul && rem != out && mul != out && mul == mul_0 && rem == rem_0
						&& rem == rem_1 && rem.is_reg () && mul.is_reg () && out.is_reg () => {
					self.store (out, chk! (self.load (out) + self.load (mul) / div) ?);
					let rem_val = self.load (mul) % div;
					self.store (rem, if rem_val == 0 { div } else { rem_val });
					self.store (mul, 0);
					self.next += 1;
					return Ok (())
				},
				_ => (),
			}
		}

		// if not then jump as described

		self.next = u32::add_signed (self.next, self.load (dst)) ?;
		Ok (())

	}

}
