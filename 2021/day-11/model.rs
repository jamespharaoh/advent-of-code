use super::*;

pub type Coord = i8;
pub type Grid = GridBuf <Vec <Val>, Pos, 2>;
pub type Pos = aoc_pos::PosYX <Coord>;
pub type Val = u8;
