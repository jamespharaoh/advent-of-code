use super::*;

pub type Val = i32;

enum_decl_parser_display! {
	#[ derive (Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd) ]
	pub enum Op {
		Acc = [ "acc" ],
		Jmp = [ "jmp" ],
		Nop = [ "nop" ],
	}
}

#[ derive (Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd) ]
pub struct Instr {
	pub op: Op,
	pub arg: Val,
}

struct_parser_display! {
	Instr { op, arg } = [ op, " ", arg ]
}
