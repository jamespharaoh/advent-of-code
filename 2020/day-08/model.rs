use super::*;

pub type Val = i32;

parse_display_enum! {
	#[ derive (Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd) ]
	pub enum Op { Acc = "acc", Jmp = "jmp", Nop = "nop" }
}

#[ derive (Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd) ]
pub struct Instr { pub op: Op, pub arg: Val }

struct_parser_display! {
	Instr { op, arg } = [ op, " ", arg ]
}
