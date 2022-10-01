use super::*;

pub type Coord = i16;
pub type Grid = GridBuf <Vec <u8>, Pos, 2>;
pub type Pos = aoc_pos::PosRowCol <Coord>;
