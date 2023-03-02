use super::*;

pub type Coord = i8;
pub type Grid = GridBuf <Vec <bool>, Pos, 2>;
pub type Pos = aoc_pos::PosYX <Coord>;
pub type Val = i8;

enum_decl_parser_display! {
	#[ derive (Clone, Copy, Debug) ]
	pub enum Instr {
		AddX (val: Val) = [ "addx ", val ],
		Noop = [ "noop" ],
	}
}
