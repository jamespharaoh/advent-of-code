use super::*;

use input::Input;

pub use self::core::{ Core, CoreNext };
pub use self::game::{ Game, GameNext };

pub type Coord = i32;
pub type Cpu = intcode::Machine <Val>;
pub type Grid = aoc_grid::Grid <Vec <Tile>, Pos>;
pub type Pos = aoc_pos::PosYX <Coord>;
pub type RunResult = intcode::RunResult <Val>;
pub type Val = i32;

parse_display_enum! {
	#[ derive (Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd) ]
	pub enum Tile {
		#[ default ]
		Empty = "  ",
		Wall = "##",
		Block = "[]",
		Paddle = "==",
		Ball = "()",
	}
}

mod core {

	use super::*;

	pub struct Core {
		cpu: Cpu,
		step_max_ops: u32,
	}

	impl Core {

		#[ must_use ]
		pub fn new (input: & Input, insert_coin: bool) -> Self {
			let mut cpu = Cpu::new (input.data.clone ());
			if insert_coin { cpu.mem_set (0_i32, 2_i32).unwrap (); }
			Self { cpu, step_max_ops: input.params.step_max_ops }
		}

		pub fn input (& mut self, val: Val) {
			self.cpu.input (val);
		}

		#[ allow (clippy::should_implement_trait) ]
		pub fn next (& mut self) -> GenResult <CoreNext> {
			let mut output_buf: ArrayVec <Val, 3> = array_vec! [];
			self.cpu.set_max_ops (self.step_max_ops);
			loop {
				#[ allow (clippy::wildcard_enum_match_arm) ]
				match self.cpu.run () {
					RunResult::Halt => return Ok (CoreNext::Halt),
					RunResult::Input => return Ok (CoreNext::Input),
					RunResult::Output (val) => output_buf.push (val),
					other => return Err (other.into ()),
				}
				if ! output_buf.is_full () { continue }
				let x = output_buf [0];
				let y = output_buf [1];
				let val = output_buf [2];
				let pos = Pos { y, x };
				return Ok (CoreNext::Output (pos, val));
			}
		}

	}

	pub enum CoreNext {
		Input,
		Output (Pos, Val),
		Halt,
	}

}

mod game {

	use super::*;

	pub struct Game {
		core: Core,
		size: Pos,
		score: Val,
		grid: Grid,
		paddle_pos: Pos,
		ball_pos: Pos,
		max_steps: u32,
	}

	impl Game {

		pub fn new (input: & Input, insert_coin: bool, size: Pos) -> GenResult <Self> {
			let grid = Grid::new ([0, 0], [size.y.as_usize (), size.x.as_usize ()]);
			Ok (Self {
				core: Core::new (input, insert_coin),
				size,
				score: 0,
				grid,
				paddle_pos: Pos::ZERO,
				ball_pos: Pos::ZERO,
				max_steps: input.params.game_max_steps,
			})
		}

		#[ must_use ]
		pub const fn grid (& self) -> & Grid { & self.grid }

		#[ must_use ]
		pub const fn score (& self) -> Val { self.score }

		#[ must_use ]
		pub const fn paddle_pos (& self) -> Pos { self.paddle_pos }

		#[ must_use ]
		pub const fn ball_pos (& self) -> Pos { self.ball_pos } 

		pub fn input (& mut self, val: Val) {
			self.core.input (val);
		}

		#[ allow (clippy::should_implement_trait) ]
		pub fn next (& mut self) -> GenResult <GameNext> {
			for _ in 0 .. self.max_steps {
				match self.core.next () ? {
					CoreNext::Input => return Ok (GameNext::Input),
					CoreNext::Output (pos, val) => {
						if pos == (Pos { y: Coord::ZERO, x: Coord::NEG_ONE }) { self.score = val; continue }
						if pos.x < Coord::ZERO || pos.y < Coord::ZERO || self.size.x <= pos.x || self.size.y <= pos.y {
							return Err (format! ("Invalid position: {pos:?}").into ());
						}
						let tile = match val {
							0_i32 => Tile::Empty,
							1_i32 => Tile::Wall,
							2_i32 => Tile::Block,
							3_i32 => Tile::Paddle,
							4_i32 => Tile::Ball,
							_ => return Err (format! ("Invalid tile id: {val}").into ()),
						};
						if matches! (tile, Tile::Paddle) { self.paddle_pos = pos; }
						if matches! (tile, Tile::Ball) { self.ball_pos = pos; }
						self.grid.set (pos, tile);
					},
					CoreNext::Halt => return Ok (GameNext::Halt),
				}
			}
			Err ("Game max steps exceeded".into ())
		}

	}

	pub enum GameNext {
		Input,
		Halt,
	}

}
