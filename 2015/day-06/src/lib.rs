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
			for (_, step) in cur_steps.iter ().copied () {
				row_data = update_line (
					row_data,
					step.action,
					step.origin.col,
					step.peak.col,
					& mut row_data_temp);
			}
			prev_row = row;
			prev_active =
				row_data.iter ().copied ()
					.tuple_windows::<(_, _)> ()
					.map (|((start, val), (end, _))| u32::checked_mul (u16::checked_sub (end, start).unwrap () as u32, val as u32).unwrap ())
					.sum ();
			assert! (row_data.last ().copied ().map (|(_, val)| val).unwrap_or (0) == 0);
		}
		Ok (sum)
	}

	pub fn update_line (
		mut line: Vec <(Coord, u8)>,
		action: Action,
		start: Coord,
		end: Coord,
		new_line: & mut Vec <(Coord, u8)>,
	) -> Vec <(Coord, u8)> {
		new_line.clear ();
		new_line.extend (
			itertools::merge_join_by (
					line.drain ( .. ),
					[ (start, 0), (end, 0) ],
					|(left, _), (right, _)| Ord::cmp (left, right))
				.scan ((0, 0, false), |state, items| {
					let old_active = & mut state.0; // value on old line
					let cur_active = & mut state.1; // value on new line
					let in_step = & mut state.2;
					if let Some ((_, val)) = items.as_ref ().left () { * old_active = * val; }
					if items.has_right () { * in_step = ! * in_step; }
					let (pos, _) = * items.as_ref ().reduce (|first, _| first);
					let want_active = match action {
						Action::Toggle => if (* old_active != 0) != * in_step { 1 } else { 0 },
						Action::On => if (* old_active != 0) || * in_step { 1 } else { 0 },
						Action::Off => if (* old_active != 0) && ! * in_step { 1 } else { 0 },
						Action::Up => if * in_step { u8::checked_add (* old_active, 1).unwrap () } else { * old_active },
						Action::Down => if * in_step { u8::saturating_sub (* old_active, 1) } else { * old_active },
						Action::UpTwo => if * in_step { u8::checked_add (* old_active, 2).unwrap () } else { * old_active },
					};
					if * cur_active != want_active {
						* cur_active = want_active;
						Some (Some ((pos, want_active)))
					} else { Some (None) }
				})
				.flatten ()
		);
		mem::swap (new_line, & mut line);
		line
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
