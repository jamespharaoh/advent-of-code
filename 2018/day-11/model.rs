use super::*;

pub type Coord = i16;
pub type Grid = GridBuf <Vec <Power>, Pos, 2>;
pub type Power = i32;
pub type Pos = aoc_pos::PosYX <Coord>;
