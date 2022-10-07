//! Iterative search algorithms for solutions in a problem space

use aoc_grid::GridPos;
use aoc_grid::prelude::*;
use aoc_misc::*;
use aoc_nums::NumResult;

mod permutations;
mod priority;

pub mod prelude {
	pub use super::PermutationsHelper;
}

pub use permutations::*;
pub use priority::*;
