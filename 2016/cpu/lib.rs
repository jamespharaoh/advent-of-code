//! Advent of Code 2016: CPU
//!
//! CPU used in the following puzzles:
//!
//! - [https://adventofcode.com/2016/day/12](https://adventofcode.com/2016/day/12)
//! - [https://adventofcode.com/2016/day/23](https://adventofcode.com/2016/day/23)
//! - [https://adventofcode.com/2016/day/25](https://adventofcode.com/2016/day/25)

#![ allow (clippy::inline_always) ]

use aoc_common::*;

mod cpu;
mod instr;

pub use cpu::*;
pub use instr::*;
