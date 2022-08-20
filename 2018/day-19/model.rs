use super::*;

pub type Instr = cpu::Instr <Val>;
pub type Regs = cpu::Regs <Val, 6>;
pub type Val = u64;

pub use cpu::ArgType;
pub use cpu::CpuError;
pub use cpu::Op;
pub use cpu::Opcode;
