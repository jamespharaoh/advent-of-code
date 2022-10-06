use super::*;

#[ must_use ]
pub fn machine_input (input: [u8; 14]) -> [i64; 14] {
	let mut result = [0; 14];
	for idx in 0 .. 14 { result [idx] = input [idx].pan_i64 (); }
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
				self.regs.store (dest, i64::from (dst_val == src_val));
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

enum_decl_parser_display! {
	#[ derive (Clone, Copy, Debug, Eq, PartialEq) ]
	pub enum Instr {
		Inp (dst: Reg) = [ "inp ", dst ],
		Add (dst: Reg, src: RegOrInt) = [ "add ", dst, " ", src ],
		Mul (dst: Reg, src: RegOrInt) = [ "mul ", dst, " ", src ],
		Div (dst: Reg, src: RegOrInt) = [ "div ", dst, " ", src ],
		Mod (dst: Reg, src: RegOrInt) = [ "mod ", dst, " ", src ],
		Eql (dst: Reg, src: RegOrInt) = [ "eql ", dst, " ", src ],
	}
}

enum_decl_parser_display! {
	#[ derive (Clone, Copy, Debug, Eq, PartialEq) ]
	pub enum Reg {
		W = [ "w" ],
		X = [ "x" ],
		Y = [ "y" ],
		Z = [ "z" ],
	}
}

enum_decl_parser_display! {
	#[ derive (Clone, Copy, Debug, Eq, PartialEq) ]
	pub enum RegOrInt {
		W = [ "w" ],
		X = [ "x" ],
		Y = [ "y" ],
		Z = [ "z" ],
		Int (val: i64) = [ val ],
	}
}
