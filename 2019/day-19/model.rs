use super::*;

pub type Coord = i16;
pub type Cpu = intcode::Machine <Val>;
pub type Pos = pos::PosYX <Coord>;
pub type Val = i64;
