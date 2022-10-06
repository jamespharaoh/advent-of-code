use super::*;

pub type Coord = u16;
pub type Grid = GridBuf <GridInner, Pos, 2>;
pub type GridInner = Vec <Region>;
pub type Pos = pos::PosYX <Coord>;

enum_decl_parser_display! {
	#[ derive (Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd) ]
	pub enum Region {
		#[ default ]
		Empty = [ "." ],
		East = [ ">" ],
		South = [ "v" ],
	}
}

impl BitVecNative for Region {
	const BITS: u32 = 2;
	fn encode (self) -> usize {
		match self {
			Self::Empty => 0x0,
			Self::East => 0x1,
			Self::South => 0x2,
		}
	}
	fn decode (encoded: usize) -> Self {
		match encoded {
			0 => Self::Empty,
			1 => Self::East,
			2 => Self::South,
			_ => panic! ("Invalid encoded value for Region: {:#x}", encoded),
		}
	}
}
