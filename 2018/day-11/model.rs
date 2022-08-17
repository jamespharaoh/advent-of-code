use super::*;

pub type Coord = u16;
pub type Grid = grid::Grid <Vec <Power>, Pos>;
pub type Power = i32;
pub type Pos = pos::PosYX <Coord>;
