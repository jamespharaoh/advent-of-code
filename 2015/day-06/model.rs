use super::*;

use input::Input;

pub type Coord = u16;
pub type Pos = pos::PosRowCol <Coord>;

wrapper_deref_mut! {
	#[ derive (Clone, Debug) ]
	pub struct Steps {
		steps: Vec <Step>,
	}
}

impl Steps {
	pub fn build (input: & Input) -> NumResult <Self> {
		Ok (Self {
			steps: input.steps.iter ()
				.map (|step| Ok (Step {
					action: step.action,
					origin: step.origin,
					peak: chk! (step.peak + Pos::new (1, 1)) ?,
				}))
				.try_collect () ?,
		})
	}
}

#[ derive (Clone, Copy, Debug, Eq, PartialEq) ]
pub struct Step {
	pub action: Action,
	pub origin: Pos,
	pub peak: Pos,
}

enum_decl_parser_display! {
	#[ derive (Clone, Copy, Debug, Eq, PartialEq) ]
	pub enum Action {
		On = [ "turn on" ],
		Off = [ "turn off" ],
		Toggle = [ "toggle" ],
	}
}
