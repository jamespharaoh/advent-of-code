//! Logic for solving the puzzles

use super::*;
use input::Input;
use model::Line;
use model::Pos;
use model::Prog;
use model::Step;

pub fn part_one (input: & Input) -> GenResult <String> {
	let trans = Transform::compile (& input.steps);
	let progs = trans.apply (Line::default ());
	Ok (progs.to_string ())
}

pub fn part_two (input: & Input) -> GenResult <String> {
	Ok (calc_result (input, input.params.reps_two))
}

fn calc_result (input: & Input, num_reps: u64) -> String {
	let mut transform = Transform::compile (& input.steps);
	let mut state = Line::default ();
	let mut remain = num_reps;
	while remain != 0 {
		if remain & 1 != 0 {
			state = transform.apply (state);
		}
		transform = transform.combine (transform);
		remain >>= 1_u32;
	}
	state.to_string ()
}

#[ derive (Clone, Copy) ]
struct Transform {
	posns: Line <Pos>,
	swaps: Line <Prog>,
}

/// Efficient abstract to transform [`Line <Prog>`]
///
/// This tracks changes of positions and swaps between programmes separately. Transformations can
/// be applied to a [`Line <Prog>`], and they can also be combined with another `Transform`. A
/// transform can be initialised from a list of [`Step`]s.
///
/// To implement this, we track position changes separately from swaps of named programmes. When
/// applied to a programme line, we first apply the position swaps, then the programmes, which
/// gives the same answer.
///
impl Transform {

	fn compile (steps: & [Step]) -> Self {
		let mut posns = Line::default ();
		let mut swaps = Line::default ();
		for step in steps.iter ().copied () {
			match step {
				Step::Spin (prg) => {
					let size = 16 - prg.idx ();
					posns = Line::from (
						array::from_fn (|idx|
							posns [(idx + size.pan_usize ()) & 0xf]));
				},
				Step::Exchange (pos_0, pos_1) => {
					(posns [pos_0.idx ()], posns [pos_1.idx ()]) =
						(posns [pos_1.idx ()], posns [pos_0.idx ()]);
				},
				Step::Partner (prg_0, prg_1) => {
					for prg in swaps.iter_mut () {
						let cur = * prg;
						if cur == prg_0 { * prg = prg_1; }
						if cur == prg_1 { * prg = prg_0; }
					}
				},
			}
		}
		Self { posns, swaps }
	}

	fn combine (self, other: Self) -> Self {
		let posns = Line::from (self.posns.map (|pos| other.posns [pos.idx ()]));
		let swaps = Line::from (self.swaps.map (|prog| other.swaps [prog.idx ()]));
		Self { posns, swaps }
	}

	fn apply (self, progs: Line <Prog>) -> Line <Prog> {
		Line::from (self.posns.map (|prog|
			self.swaps [progs [prog.idx ()].idx ()]
		))
	}

}
