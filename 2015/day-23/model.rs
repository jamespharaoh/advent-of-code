//! Representation of the puzzle input, etc.

use super::*;

pub type Val = u32;
pub type Offset = i32;

enum_decl_parser_display! {
	#[ derive (Clone, Copy, Debug, Eq, PartialEq) ]
	pub enum Instr {
		Hlf (reg: Reg) = [ "hlf ", reg ],
		Tpl (reg: Reg) = [ "tpl ", reg ],
		Inc (reg: Reg) = [ "inc ", reg ],
		Jmp (off: Offset) = [ "jmp ", off ],
		Jie (reg: Reg, off: Offset) = [ "jie ", reg, ", ", off ],
		Jio (reg: Reg, off: Offset) = [ "jio ", reg, ", ", off ],
	}
}

enum_decl_parser_display! {
	#[ derive (Clone, Copy, Debug, Eq, PartialEq) ]
	pub enum Reg {
		A = [ "a" ],
		B = [ "b" ],
	}
}
