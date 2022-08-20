use super::*;

pub type Regs = cpu::Regs <Val, 4>;
pub type Val = u16;

pub use cpu::ArgType;
pub use cpu::Op;
pub use cpu::Opcode;

#[ derive (Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd) ]
pub struct Instr {
	pub op: Val,
	pub arg_a: Val,
	pub arg_b: Val,
	pub arg_c: Val,
}

struct_parser_display! (Instr { op, arg_a, arg_b, arg_c } = [
	op, " ", arg_a, " ", arg_b, " ", arg_c,
]);

#[ derive (Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd) ]
pub struct Sample {
	pub before: Regs,
	pub instr: Instr,
	pub after: Regs,
}

struct_parser_display! (Sample { before, instr, after } = [
	"Before: ", before, "\n", instr, "\nAfter:  ", after,
]);
