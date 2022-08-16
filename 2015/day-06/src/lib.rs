//! Advent of Code 2015: Day 6: Probably a Fire Hazard
//!
//! [https://adventofcode.com/2015/day/6](https://adventofcode.com/2015/day/6)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;
use aoc_pos as pos;

puzzle_info! {
	name = "Probably a Fire Hazard";
	year = 2015;
	day = 6;
	parse = |input| model::parse_input (input);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
}

pub mod logic {

	use super::*;
	use model::Action;
	use model::Coord;
	use model::Input;
	use model::Step;
	use nums::Int;
	use nums::IntConv;

	pub type ModeFn = fn (Action, u8) -> NumResult <u8>;

	pub fn part_one (input: & Input) -> GenResult <u32> {
		calc_result (input, mode_fn_one)
	}

	pub fn part_two (input: & Input) -> GenResult <u32> {
		calc_result (input, mode_fn_two)
	}

	#[ allow (clippy::unnecessary_wraps) ]
	const fn mode_fn_one (action: Action, old_active: u8) -> NumResult <u8> {
		Ok (match action {
			Action::On => 1,
			Action::Off => 0,
			Action::Toggle => if old_active == 0 { 1 } else { 0 },
		})
	}

	fn mode_fn_two (action: Action, old_active: u8) -> NumResult <u8> {
		Ok (match action {
			Action::On => u8::add_2 (old_active, 1) ?,
			Action::Off => u8::saturating_sub (old_active, 1),
			Action::Toggle => u8::add_2 (old_active, 2) ?,
		})
	}

	fn calc_result (steps: & [Step], mode_fn: ModeFn) -> GenResult <u32> {
		let steps =
			steps.iter ().copied ()
				.enumerate ()
				.sorted_by_key (|& (_, step)| step.origin)
				.collect::<Vec <_>> ();
		let rows =
			steps.iter ().copied ()
				.flat_map (|(_, step)| [ step.origin.row, step.peak.row ])
				.sorted ()
				.dedup ()
				.collect::<Vec <_>> ();
		let mut cur_steps: Vec <(usize, Step)> = Vec::new ();
		let mut sum = 0;
		let mut prev_row = Coord::MIN;
		let mut prev_active = 0;
		let mut steps_iter = steps.into_iter ().peekable ();
		let mut row_data: Vec <(Coord, u8)> = Vec::new ();
		let mut row_data_temp = Vec::new ();
		for row in rows {
			sum = u32::checked_add (sum,
				u32::checked_mul (
					u16::checked_sub (row, prev_row).unwrap ().as_u32 (),
					prev_active.as_u32 (),
				).unwrap (),
			).unwrap ();
			cur_steps.retain (|& (_, step)| row < step.peak.row);
			while let Some (& (_, step)) = steps_iter.peek () {
				if step.origin.row != row { break }
				cur_steps.push (steps_iter.next ().unwrap ());
			}
			cur_steps.sort_by_key (|& (idx, _)| idx);
			row_data.clear ();
			{
				let mut steps = & * cur_steps;
				trait RowIter: Iterator <Item = NumResult <(Coord, u8)>> {}
				impl <SomeIter: Iterator <Item = NumResult <(Coord, u8)>>> RowIter for SomeIter {}
				#[ inline ]
				fn update_once (iter: impl RowIter, step: Step, mode_fn: ModeFn) -> impl RowIter {
					UpdateLineIter::new (iter, step.action, step.origin.col, step.peak.col, mode_fn)
				}
				#[ inline ]
				fn update_twice (iter: impl RowIter, steps: & [(usize, Step)], mode_fn: ModeFn) -> impl RowIter {
					update_once (update_once (iter, steps [0].1, mode_fn), steps [1].1, mode_fn)
				}
				#[ inline ]
				fn update_four_x (iter: impl RowIter, steps: & [(usize, Step)], mode_fn: ModeFn) -> impl RowIter {
					update_twice (update_twice (iter, & steps [0 .. 2], mode_fn), & steps [2 .. 4], mode_fn)
				}
				#[ inline ]
				fn update_eight_x (iter: impl RowIter, steps: & [(usize, Step)], mode_fn: ModeFn) -> impl RowIter {
					update_four_x (update_four_x (iter, & steps [0 .. 4], mode_fn), & steps [4 .. 8], mode_fn)
				}
				while steps.len () >= 8 {
					mem::swap (& mut row_data, & mut row_data_temp);
					assert! (row_data.is_empty ());
					for item in
						update_eight_x (
							row_data_temp.drain ( .. ).map (Ok),
							& steps [0 .. 8 ],
							mode_fn) {
						row_data.push (item ?);
					}
					steps = & steps [ 8 .. ];
				}
				while ! steps.is_empty () {
					mem::swap (& mut row_data, & mut row_data_temp);
					assert! (row_data.is_empty ());
					for item in 
						update_once (
							row_data_temp.drain ( .. ).map (Ok),
							steps [0].1,
							mode_fn) {
						row_data.push (item ?);
					}
					steps = & steps [ 1 .. ];
				}
			}
			/* old implementation:
			mem::swap (& mut row_data, & mut row_data_temp);
			assert! (row_data.is_empty ());
			let mut iter: Box <dyn RowIter> = Box::new (row_data.drain ( .. ));
			for step in steps.iter ().copied () {
				iter = Box::new (UpdateLineIter::new (iter, step.action, step.origin.col, step.peak.col));
			}
			row_data = iter.collect ();
			*/
			prev_row = row;
			prev_active =
				row_data.iter ().copied ()
					.tuple_windows::<(_, _)> ()
					.map (|((start, val), (end, _))|
						Int::mul_2 (
							Int::sub_2 (end, start) ?.as_u32 (),
							val.as_u32 (),
						))
					.fold (Ok (0), |sum, val| Int::add_2 (sum ?, val ?)) ?;
			assert! (row_data.last ().copied ().map_or (0, |(_, val)| val) == 0);
		}
		Ok (sum)
	}

	struct UpdateLineIter <Inner: Iterator <Item = NumResult <(Coord, u8)>>> {
		inner: Inner,
		next: Option <(Coord, u8)>,
		action: Action,
		step: ArrayVec <Coord, 2>,
		old_active: u8,
		cur_active: u8,
		in_step: bool,
		mode_fn: ModeFn,
	}

	impl <Inner> UpdateLineIter <Inner>
			where Inner: Iterator <Item = NumResult <(Coord, u8)>> {
		#[ inline ]
		fn new (inner: Inner, action: Action, start: Coord, end: Coord, mode_fn: ModeFn) -> Self {
			Self {
				inner,
				next: None,
				action,
				step: [ end, start ].into_iter ().collect (),
				old_active: 0,
				cur_active: 0,
				in_step: false,
				mode_fn,
			}
		}
	}

	impl <Inner> Iterator for UpdateLineIter <Inner>
		where Inner: Iterator <Item = NumResult <(Coord, u8)>> {
		type Item = NumResult <(Coord, u8)>;
		fn next (& mut self) -> Option <NumResult <(Coord, u8)>> {
			loop {
				if self.next.is_none () {
					self.next = match self.inner.next () {
						Some (Ok (val)) => Some (val),
						Some (Err (err)) => return Some (Err (err)),
						None => None,
					}
				}
				let (pos, old_val, step) = match (self.next, self.step.last ().copied ()) {
					(Some ((pos, val)), None) => (pos, Some (val), false),
					(Some ((pos_0, val)), Some (pos_1)) if pos_0 < pos_1 => (pos_0, Some (val), false),
					(Some ((pos_0, _)), Some (pos_1)) if pos_1 < pos_0 => (pos_1, None, true),
					(Some ((pos_0, val)), Some (pos_1)) if pos_0 == pos_1 => (pos_0, Some (val), true),
					(None, Some (pos)) => (pos, None, true),
					(None, None) => return None,
					_ => unreachable! (),
				};
				if let Some (val) = old_val { self.old_active = val; self.next = None; }
				if step { self.in_step = ! self.in_step; self.step.pop ().unwrap (); }
				let want_active = if self.in_step {
					match (self.mode_fn) (self.action, self.old_active) {
						Ok (val) => val,
						Err (err) => return Some (Err (err)),
					}
				} else { self.old_active };
				if self.cur_active != want_active {
					self.cur_active = want_active;
					return Some (Ok ((pos, want_active)));
				}
			}
		}
	}

	#[ cfg (test) ]
	mod tests {

		use super::*;
		use model::Action::*;
		use model::Pos;

		const STEPS: & [Step] = & [
			Step { action: Toggle, origin: Pos { row: 0, col: 0 }, peak: Pos { row: 2, col: 9 }},
			Step { action: Off, origin: Pos { row: 4, col: 3 }, peak: Pos { row: 5, col: 8 }},
			Step { action: On, origin: Pos { row: 2, col: 2 }, peak: Pos { row: 9, col: 3 }},
			Step { action: Toggle, origin: Pos { row: 2, col: 4 }, peak: Pos { row: 9, col: 5 }},
			Step { action: Toggle, origin: Pos { row: 3, col: 3 }, peak: Pos { row: 8, col: 6 }},
			Step { action: Toggle, origin: Pos { row: 6, col: 3 }, peak: Pos { row: 9, col: 8 }},
			Step { action: Off, origin: Pos { row: 1, col: 0 }, peak: Pos { row: 1, col: 3 }},
			Step { action: Off, origin: Pos { row: 2, col: 2 }, peak: Pos { row: 5, col: 5 }},
			Step { action: Toggle, origin: Pos { row: 5, col: 5 }, peak: Pos { row: 9, col: 8 }},
			Step { action: Off, origin: Pos { row: 3, col: 4 }, peak: Pos { row: 7, col: 8 }},
			Step { action: On, origin: Pos { row: 4, col: 0 }, peak: Pos { row: 8, col: 8 }},
			Step { action: Off, origin: Pos { row: 1, col: 1 }, peak: Pos { row: 3, col: 5 }},
			Step { action: On, origin: Pos { row: 2, col: 4 }, peak: Pos { row: 6, col: 8 }},
			Step { action: On, origin: Pos { row: 1, col: 4 }, peak: Pos { row: 7, col: 8 }},
			Step { action: On, origin: Pos { row: 0, col: 3 }, peak: Pos { row: 6, col: 6 }},
		];

		#[ test ]
		fn calc_result () {
			assert_eq_ok! (59, logic::calc_result (STEPS, mode_fn_one));
			assert_eq_ok! (203, logic::calc_result (STEPS, mode_fn_two));
		}

	}

}

pub mod model {

	use super::*;
	use pos::PosRowCol;

	pub type Coord = u16;
	pub type Pos = PosRowCol <Coord>;
	pub type Input = Vec <Step>;

	#[ derive (Clone, Copy, Debug, Eq, PartialEq) ]
	pub struct Step {
		pub action: Action,
		pub origin: Pos,
		pub peak: Pos,
	}

	impl Step {

		pub fn parse (input: & str) -> GenResult <Self> {
			Parser::wrap (input, Self::parse_real)
				.map_parse_err (|_, col_idx|
					format! ("Invalid input: col {}: {}", col_idx + 1, input)
				)
		}

		fn parse_real (parser: & mut Parser) -> ParseResult <Self> {
			parser.set_ignore_whitespace (true);
			let action = match parser.word () ? {
				"turn" => match parser.word () ? {
					"on" => Action::On,
					"off" => Action::Off,
					_ => return Err (parser.err ()),
				},
				"toggle" => Action::Toggle,
				_ => return Err (parser.err ()),
			};
			let origin = Pos {
				row: parser.int () ?,
				col: parser.expect (",") ?.int () ?,
			};
			let peak = Pos {
				row: parser.expect ("through") ?.int::<Coord> () ? + 1,
				col: parser.expect (",") ?.int::<Coord> () ? + 1,
			};
			Ok (Self { action, origin, peak })
		}

	}

	#[ derive (Clone, Copy, Debug, Eq, PartialEq) ]
	pub enum Action { On, Off, Toggle }

	pub fn parse_input (input: & [& str]) -> GenResult <Input> {
		input.iter ().enumerate ()
			.map (|(line_idx, line)|
				Parser::wrap (line, Step::parse_real)
					.map_parse_err (|_, char_idx|
						format! ("Invalid input: line {}: col {}: {}",
							line_idx + 1, char_idx + 1, line)))
			.collect::<GenResult <_>> ()
	}

	#[ cfg (test) ]
	mod tests {

		use super::*;

		const STEPS: & [Step] = & [
			Step {
				action: Action::On,
				origin: Pos { row: 1, col: 2 },
				peak: Pos { row: 3, col: 4 },
			},
			Step {
				action: Action::Off,
				origin: Pos { row: 0, col: 200 },
				peak: Pos { row: 100, col: 500 },
			},
			Step {
				action: Action::Toggle,
				origin: Pos { row: 10, col: 1 },
				peak: Pos { row: 90, col: 79 },
			},
		];

		const STEP_TEXTS: & [& str] = & [
			"turn on 1,2 through 2,3",
			"turn off 0,200 through 99,499",
			"toggle 10,1 through 89,78",
		];

		#[ test ]
		fn step_parse () {
			assert_eq_ok! (STEPS [0], Step::parse (STEP_TEXTS [0]));
			assert_eq_ok! (STEPS [1], Step::parse (STEP_TEXTS [1]));
			assert_eq_ok! (STEPS [2], Step::parse (STEP_TEXTS [2]));
			assert_err! ("Invalid input: col 4: go on 1,2 through 2,3",
				Step::parse ("go on 1,2 through 2,3"));
			assert_err! ("Invalid input: col 10: turn red 1,2 through 2,3",
				Step::parse ("turn red 1,2 through 2,3"));
			assert_err! ("Invalid input: col 10: turn on 1:2 through 2,3",
				Step::parse ("turn on 1:2 through 2,3"));
		}

		#[ test ]
		fn parse_input () {
			assert_eq_ok! (STEPS, model::parse_input (STEP_TEXTS));
			assert_err! ("Invalid input: line 2: col 10: turn on 1:2 through 2,3",
				model::parse_input (& [STEP_TEXTS [0], "turn on 1:2 through 2,3" ]));
		}

	}

}

#[ cfg (test) ]
mod examples {

	use super::*;

	const EXAMPLE_0: & [& str] = & [
		"turn on 0,0 through 999,999",
		"toggle 0,0 through 999,0",
		"turn off 499,499 through 500,500",
	];

	const EXAMPLE_1: & [& str] = & [
		"turn on 0,0 through 0,0",
		"toggle 0,0 through 999,999",
	];

	#[ test ]
	fn part_one () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("998996", puzzle.part_one (EXAMPLE_0));
		assert_eq_ok! ("999999", puzzle.part_one (EXAMPLE_1));
	}

	#[ test ]
	fn part_two () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("1001996", puzzle.part_two (EXAMPLE_0));
		assert_eq_ok! ("2000001", puzzle.part_two (EXAMPLE_1));
	}

}
