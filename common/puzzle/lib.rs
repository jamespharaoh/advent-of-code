use aoc_args::*;
use aoc_misc::prelude::*;
use aoc_nums::*;

pub mod command;
pub mod puzzle;
pub mod run;
pub mod year;

pub mod prelude {
	pub use crate::puzzle_info;
	pub use crate::puzzle::Puzzle;
	pub use crate::command::PuzzleCommand;
}
