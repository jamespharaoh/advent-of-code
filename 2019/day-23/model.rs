use super::*;

pub type Cpu = intcode::Machine <Val>;
pub type RunResult = intcode::RunResult <Val>;
pub type Val = i64;
