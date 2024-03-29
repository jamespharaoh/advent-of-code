use super::*;

use input::Input;

pub use self::core::Core;
pub use self::core::CoreStep;

pub type Coord = i16;
pub type Cpu = intcode::Machine <Val>;
pub type Dir = aoc_pos::DirGeo;
pub type Grid = GridBuf <Vec <Tile>, Pos, 2>;
pub type Pos = aoc_pos::PosGeo <Coord>;
pub type RunResult = intcode::RunResult <Val>;
pub type Val = i32;

enum_decl_parser_display! {
	#[ derive (Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd) ]
	pub enum Tile {
		#[ default ]
		Unknown = [ " " ],
		Empty = [ "." ],
		Wall = [ "#" ],
		OxygenSystem = [ "S" ],
		Oxygen = [ "O" ],
	}
}

mod core {

	use super::*;

	#[ derive (Clone) ]
	pub struct Core {
		cpu: Cpu,
	}

	impl Core {

		#[ inline ]
		#[ must_use ]
		pub fn new (input: & Input) -> Self {
			let mut cpu = Cpu::new (input.data.clone ());
			cpu.set_mem_limit (4 * 1024);
			Self { cpu }
		}

		#[ allow (clippy::wildcard_enum_match_arm) ]
		#[ inline ]
		pub fn step (& mut self, dir: Dir) -> Result <CoreStep, RunResult> {
			self.cpu.input (match dir {
				Dir::North => Val::ONE,
				Dir::South => Val::TWO,
				Dir::West => Val::THREE,
				Dir::East => Val::FOUR,
			});
			self.cpu.set_max_ops (100);
			match self.cpu.run () {
				RunResult::Output (Val::ZERO) => Ok (CoreStep::Blocked),
				RunResult::Output (Val::ONE) => Ok (CoreStep::Moved),
				RunResult::Output (Val::TWO) => Ok (CoreStep::Found),
				other => Err (other),
			}
		}

	}

	pub enum CoreStep {
		Blocked,
		Moved,
		Found,
	}

}
