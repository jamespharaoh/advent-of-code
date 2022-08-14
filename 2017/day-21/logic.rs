//! Logic for solving the puzzles

use super::*;
use input::Input;
use input::InputRule;
use model::TwoByTwo;
use model::ThreeByThree;
use model::FourByFour;
use model::SixBySix;

pub fn part_one (input: & Input) -> GenResult <u64> {
	calc_result (input, input.params.iters_one)
}

pub fn part_two (input: & Input) -> GenResult <u64> {
	calc_result (input, input.params.iters_two)
}

fn calc_result (input: & Input, num_iters: u32) -> GenResult <u64> {
	let enhancer = Enhancer::build (input) ?;
	let mut state = State::default ();
	for _ in 0 .. num_iters {
		state = enhancer.next (& state);
	}
	Ok (state.num_active ())
}

#[ derive (Clone, Debug) ]
struct Enhancer {
	two_to_three: Rc <[ThreeByThree; 16]>,
	three_to_four: Rc <[FourByFour; 512]>,
}

impl Enhancer {

	fn build (input: & Input) -> GenResult <Self> {
		let mut two_to_three: [ThreeByThree; 16] = default ();
		let mut two_to_three_found = [false; 16];
		let mut three_to_four: [FourByFour; 512] = array::from_fn (|_| default ());
		let mut three_to_four_found = [false; 512];
		for rule in input.rules.iter () {
			match * rule {
				InputRule::TwoToThree (from, to) => {
					let mut from = TwoByTwo::try_from (from).unwrap ();
					let to = ThreeByThree::try_from (to).unwrap ();
					if input.params.check_rules && two_to_three_found [from.idx ()] {
						return Err (format! ("Duplicated rule: {from}").into ());
					}
					loop {
						two_to_three [from.idx ()] = to;
						two_to_three_found [from.idx ()] = true;
						from = from.rotate ();
						if two_to_three_found [from.idx ()] { break }
					}
				},
				InputRule::ThreeToFour (from, to) => {
					let mut from = ThreeByThree::try_from (from).unwrap ();
					let to = FourByFour::from (to);
					if input.params.check_rules && three_to_four_found [from.idx ()] {
						return Err (format! ("Duplicated rule: {from}").into ());
					}
					loop {
						three_to_four [from.idx ()] = to;
						three_to_four_found [from.idx ()] = true;
						from = from.rotate ();
						if three_to_four_found [from.idx ()] {
							from = from.flip ();
							if three_to_four_found [from.idx ()] { break }
						}
					}
				},
			}
		}
		if input.params.check_rules {
			if let Some (rule) = two_to_three_found.iter ().position (|found| ! found) {
				return Err (format! ("Missing rule: 0x{rule:01x}").into ());
			}
			if let Some (rule) = three_to_four_found.iter ().position (|found| ! found) {
				return Err (format! ("Missing rule: 0x{rule:03x}").into ());
			}
		}
		let two_to_three = Rc::new (two_to_three);
		let three_to_four = Rc::new (three_to_four);
		Ok (Self { two_to_three, three_to_four })
	}

	fn next (& self, prev: & State) -> State {
		match * prev {
			State::ThreeByThree (ref old_counts) => {
				let mut new_counts = Vec::new ();
				for (old_square, old_count) in old_counts.iter ().copied () {
					let new_square = self.three_to_four [old_square.idx ()];
					new_counts.push ((new_square, old_count));
				}
				State::FourByFour (State::merge_counts (new_counts))
			},
			State::FourByFour (ref old_counts) => {
				let mut new_counts = Vec::new ();
				for (old_square, old_count) in old_counts.iter ().copied () {
					let new_square = SixBySix::join (
						old_square.split ()
							.map (|old_square| self.two_to_three [old_square.idx ()]));
					new_counts.push ((new_square, old_count));
				}
				State::SixBySix (State::merge_counts (new_counts))
			},
			State::SixBySix (ref old_counts) => {
				let mut new_counts = Vec::new ();
				for (old_square, old_count) in old_counts.iter ().copied () {
					for old_square in old_square.split () {
						let new_square = self.two_to_three [old_square.idx ()];
						new_counts.push ((new_square, old_count));
					}
				}
				State::ThreeByThree (State::merge_counts (new_counts))
			},
		}
	}

}

#[ derive (Debug) ]
enum State {
	ThreeByThree (Vec <(ThreeByThree, u64)>),
	FourByFour (Vec <(FourByFour, u64)>),
	SixBySix (Vec <(SixBySix, u64)>),
}

impl State {

	fn merge_counts <Square: Copy + Ord> (
		mut counts: Vec <(Square, u64)>,
	) -> Vec <(Square, u64)> {
		counts.sort ();
		counts.iter ().copied ()
			.coalesce (|left, right|
				if left.0 == right.0 {
					Ok ((left.0, left.1 + right.1))
				} else {
					Err ((left, right))
				})
			.collect ()
	}

	fn default () -> Self {
		Self::ThreeByThree (vec! [
			(ThreeByThree::try_from (0x1e2).unwrap (), 1),
		])
	}

	fn num_active (& self) -> u64 {
		match * self {
			Self::ThreeByThree (ref counts) =>
				counts.iter ()
					.map (|& (square, count)| square.num_active () * count)
					.sum (),
			Self::FourByFour (ref counts) =>
				counts.iter ()
					.map (|& (square, count)| square.num_active () * count)
					.sum (),
			Self::SixBySix (ref counts) =>
				counts.iter ()
					.map (|& (square, count)| square.num_active () * count)
					.sum (),
		}
	}

}
