use super::*;

pub type Coord = i16;
pub type Cursor = GridCursor <Pos, 2>;
pub type Grid <Val> = GridBuf <Vec <Val>, Pos, 2>;
pub type Offset = GridOffset <Pos, 2>;
pub type Pos = pos::PosYX <Coord>;
pub type Risks = Grid <Val>;
pub type Val = u8;
