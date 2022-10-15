use super::*;

enum_decl_parser_display! {
	#[ derive (Clone, Copy, Debug, Eq, PartialEq) ]
	pub enum Arg {
		RegA = [ "a" ],
		RegB = [ "b" ],
		RegC = [ "c" ],
		RegD = [ "d" ],
		Imm (val: i32) = [ val ],
	}
}

impl Arg {

	#[ inline (always) ]
	#[ must_use ]
	pub const fn is_reg (& self) -> bool {
		matches! (* self, Self::RegA | Self::RegB | Self::RegC | Self::RegD)
	}

}

enum_decl_parser_display! {
	#[ derive (Clone, Copy, Debug, Eq, PartialEq) ]
	pub enum Instr {
		Cpy (arg_0: Arg, arg_1: Arg) = [ "cpy ", arg_0, " ", arg_1 ],
		Inc (arg: Arg) = [ "inc ", arg ],
		Dec (arg: Arg) = [ "dec ", arg ],
		Jnz (arg_0: Arg, arg_1: Arg) = [ "jnz ", arg_0, " ", arg_1 ],
		Tgl (arg: Arg) = [ "tgl ", arg ],
		Out (arg: Arg) = [ "out ", arg ],
	}
}

impl Instr {

	#[ inline (always) ]
	#[ must_use ]
	pub const fn is_v1 (& self) -> bool {
		use Instr::{ Cpy, Dec, Inc, Jnz };
		matches! (* self, Cpy (_, _) | Inc (_) | Dec (_) | Jnz (_, _))
	}

	#[ inline (always) ]
	#[ must_use ]
	pub const fn is_v2 (& self) -> bool {
		use Instr::{ Cpy, Dec, Inc, Jnz, Tgl };
		matches! (* self, Cpy (_, _) | Inc (_) | Dec (_) | Jnz (_, _) | Tgl (_))
	}

	#[ inline (always) ]
	#[ must_use ]
	pub const fn is_v3 (& self) -> bool {
		use Instr::{ Cpy, Dec, Inc, Jnz, Out };
		matches! (* self, Cpy (_, _) | Inc (_) | Dec (_) | Jnz (_, _) | Out (_))
	}

}
