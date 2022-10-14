//! Iterative search algorithms for solutions in a problem space

use aoc_grid::prelude::*;
use aoc_misc::prelude::*;
use aoc_nums::NumResult;

pub mod pairs_map;
pub use pairs_map::*;

pub mod permutations;
pub use permutations::*;

pub mod priority;
pub use priority::*;

/// Standard prelude for wildcard imports.
///
pub mod prelude {
	pub use crate::pairs_map::PairsMap;
	pub use crate::permutations::PermutationsHelper;
	pub use crate::priority::PrioritySearchAdder;
	pub use crate::priority::PrioritySearch;
}

