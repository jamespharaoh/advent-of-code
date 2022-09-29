use super::*;

pub type Board = Grid <u8>;
pub type Coord = i8;
pub type Dir = pos::Dir2d;
pub type Grid <Val> = GridBuf <Vec <Val>, Pos, 2>;
pub type Pos = pos::PosRowCol <i8>;
