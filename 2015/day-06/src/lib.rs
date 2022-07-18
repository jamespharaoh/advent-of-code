//! Advent of Code 2015: Day 6: Probably a Fire Hazard
//!
//! [https://adventofcode.com/2015/day/6](https://adventofcode.com/2015/day/6)

use aoc_common::*;

puzzle_info! {
	name = "Probably a Fire Hazard";
	year = 2015;
	day = 6;
	part_one = |input| logic::part_one (input);
	part_two = |input| logic::part_two (input);
}

mod logic {

	use super::*;
	use model::Action;
	use model::Coord;
	use model::Step;

	pub fn part_one (input: & [& str]) -> GenResult <u32> {
		let steps = model::parse_input (input, false) ?;
		calc_result (steps)
	}

	pub fn part_two (input: & [& str]) -> GenResult <u32> {
		let steps = model::parse_input (input, true) ?;
		calc_result (steps)
	}

	pub fn calc_result (steps: Vec <Step>) -> GenResult <u32> {
		let steps =
			steps.into_iter ()
				.enumerate ()
				.sorted_by_key (|(_, step)| step.origin)
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
					u16::checked_sub (row, prev_row).unwrap () as u32,
					prev_active as u32,
				).unwrap (),
			).unwrap ();
			cur_steps.retain (|(_, step)| row < step.peak.row);
			while let Some ((_, step)) = steps_iter.peek () {
				if step.origin.row != row { break }
				cur_steps.push (steps_iter.next ().unwrap ());
			}
			cur_steps.sort_by_key (|& (idx, _)| idx);
			row_data.clear ();
			{
				let mut steps = & cur_steps [ .. ];
				trait RowIter: Iterator <Item = (Coord, u8)> {}
				impl <SomeIter: Iterator <Item = (Coord, u8)>> RowIter for SomeIter {}
				fn update_once (iter: impl RowIter, step: Step) -> impl RowIter {
					UpdateLineIter::new (iter, step.action, step.origin.col, step.peak.col)
				}
				fn update_twice (iter: impl RowIter, steps: & [(usize, Step)]) -> impl RowIter {
					update_once (update_once (iter, steps [0].1), steps [1].1)
				}
				fn update_four_x (iter: impl RowIter, steps: & [(usize, Step)]) -> impl RowIter {
					update_twice (update_twice (iter, & steps [0 .. 2]), & steps [2 .. 4])
				}
				fn update_eight_x (iter: impl RowIter, steps: & [(usize, Step)]) -> impl RowIter {
					update_four_x (update_four_x (iter, & steps [0 .. 4]), & steps [4 .. 8])
				}
				while steps.len () >= 8 {
					mem::swap (& mut row_data, & mut row_data_temp);
					assert! (row_data.is_empty ());
					row_data.extend (
						update_eight_x (
							row_data_temp.drain ( .. ),
							& steps [0 .. 8 ]));
					steps = & steps [ 8 .. ];
				}
				while steps.len () >= 1 {
					mem::swap (& mut row_data, & mut row_data_temp);
					assert! (row_data.is_empty ());
					row_data.extend (
						update_once (
							row_data_temp.drain ( .. ),
							steps [0].1));
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
						u32::checked_mul (
							u16::checked_sub (end, start).unwrap () as u32,
							val as u32,
						).unwrap ())
					.sum ();
			assert! (row_data.last ().copied ().map (|(_, val)| val).unwrap_or (0) == 0);
		}
		Ok (sum)
	}

	struct UpdateLineIter <Inner: Iterator> {
		inner: Inner,
		next: Option <Inner::Item>,
		action: Action,
		step: ArrayVec <Coord, 2>,
		old_active: u8,
		cur_active: u8,
		in_step: bool,
	}

	impl <Inner> UpdateLineIter <Inner>
			where Inner: Iterator <Item = (Coord, u8)> {
		fn new (inner: Inner, action: Action, start: Coord, end: Coord) -> Self {
			UpdateLineIter {
				inner,
				next: None,
				action,
				step: [ end, start ].into_iter ().collect (),
				old_active: 0,
				cur_active: 0,
				in_step: false,
			}
		}
	}

	impl <Inner> Iterator for UpdateLineIter <Inner>
			where Inner: Iterator <Item = (Coord, u8)> {
		type Item = (Coord, u8);
		fn next (& mut self) -> Option <(Coord, u8)> {
			loop {
				if self.next.is_none () { self.next = self.inner.next (); }
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
					match self.action {
						Action::Toggle => if self.old_active == 0 { 1 } else { 0 },
						Action::On => 1,
						Action::Off => 0,
						Action::Up => self.old_active + 1,
						Action::Down => u8::saturating_sub (self.old_active, 1),
						Action::UpTwo => u8::checked_add (self.old_active, 2).unwrap (),
					}
				} else { self.old_active };
				if self.cur_active != want_active {
					self.cur_active = want_active;
					return Some ((pos, want_active));
				}
			}
		}
	}

}

mod model {

	use super::*;
	use parser::Parser;
	use pos::PosRowCol;

	pub type Coord = u16;
	pub type Pos = PosRowCol <Coord>;
	pub type Input = Vec <Step>;

	#[ derive (Clone, Copy, Debug) ]
	pub struct Step {
		pub action: Action,
		pub origin: Pos,
		pub peak: Pos,
	}

	#[ derive (Clone, Copy, Debug, Eq, PartialEq) ]
	pub enum Action { On, Off, Toggle, Up, Down, UpTwo }

	pub fn parse_input (input: & [& str], part_two: bool) -> GenResult <Input> {
		input.iter ().enumerate ().map (|(line_idx, line)| {
			let mut parser = Parser::new (line, |char_idx|
				format! ("Invalid input: line {}: col {}: {}", line_idx + 1, char_idx + 1, line));
			let action = match parser.word () ? {
				"turn" => match parser.word () ? {
					"on" => if part_two { Action::Up } else { Action::On },
					"off" => if part_two { Action::Down } else { Action::Off },
					_ => return Err (parser.err ()),
				},
				"toggle" => if part_two { Action::UpTwo } else { Action::Toggle },
				_ => return Err (parser.err ()),
			};
			let origin = Pos {
				row: parser.int () ?,
				col: parser.expect (",") ?.int () ?,
			};
			let peak = Pos {
				row: parser.expect (" through ") ?.int::<Coord> () ? + 1,
				col: parser.expect (",") ?.int::<Coord> () ? + 1,
			};
			Ok (Step { action, origin, peak })
		}).collect::<GenResult <_>> ()
	}
}
