use aoc_common::*;

puzzle! {
	name = "Hydrothermal Venture";
	year = 2021;
	day = 5;
	part_one = |lines| logic::calc_result_part_one (lines);
	part_two = |lines| logic::calc_result_part_two (lines);
}

mod logic {

	use super::*;
	use model::Pos;

	pub fn calc_result_part_one (lines: & [& str]) -> GenResult <i64> {
		calc_result (lines, false)
	}

	pub fn calc_result_part_two (lines: & [& str]) -> GenResult <i64> {
		calc_result (lines, true)
	}

	pub fn calc_result (lines: & [& str], include_diagonal: bool) -> GenResult <i64> {
		let mut vents = model::parse_input (lines) ?;
		if ! include_diagonal {
			vents.retain (|vent| ! vent.is_diagonal ());
		}
		let mut points: HashMap <Pos, u16> = HashMap::new ();
		for vent in vents {
			let step = Pos {
				x: (vent.end.x - vent.start.x).signum (),
				y: (vent.end.y - vent.start.y).signum (),
			};
			let mut pos = vent.start;
			loop {
				* points.entry (pos).or_insert (0) += 1;
				if pos == vent.end { break }
				pos.x += step.x;
				pos.y += step.y;
			}
		}
		Ok (points.values ().cloned ().filter (|& num| num > 1).count () as i64)
	}

}

mod model {

	use super::*;

	pub fn parse_input (lines: & [& str]) -> GenResult <Vec <Vent>> {
		let mut vents: Vec <Vent> = Vec::new ();
		for line in lines {
			let line_parts: Vec <& str> = line.split (" -> ").collect ();
			if line_parts.len () != 2 {
				Err (format! ("Invalid input: {}", line)) ?;
			}
			vents.push (Vent {
				start: parse_pos (line_parts [0]) ?,
				end: parse_pos (line_parts [1]) ?,
			});
		}
		Ok (vents)
	}

	pub fn parse_pos (input: & str) -> GenResult <Pos> {
		let input_parts: Vec <& str> = input.split (",").collect ();
		if input_parts.len () != 2 {
			Err (format! ("Input position: {}", input)) ?;
		}
		Ok (Pos {
			x: input_parts [0].parse () ?,
			y: input_parts [1].parse () ?,
		})
	}

	#[ derive (Clone, Copy, Debug, Eq, Hash, PartialEq) ]
	pub struct Pos { pub x: i16, pub y: i16 }

	#[ derive (Clone, Copy, Debug, Eq, Hash, PartialEq) ]
	pub struct Vent { pub start: Pos, pub end: Pos }

	impl Vent {
		pub fn is_diagonal (self) -> bool {
			self.start.x != self.end.x && self.start.y != self.end.y
		}
	}

}

#[ cfg (tes) ]
mod examples {

	const EXAMPLE: & [& str] = & [
		"0,9 -> 5,9",
		"8,0 -> 0,8",
		"9,4 -> 3,4",
		"2,2 -> 2,1",
		"7,0 -> 7,4",
		"6,4 -> 2,0",
		"0,9 -> 2,9",
		"3,4 -> 1,4",
		"0,0 -> 8,8",
		"5,5 -> 8,2",
	];

	#[ test ]
	fn part_one () -> GenResult <()> {
		assert_eq! (12, logic::calc_result_part_one (EXAMPLE) ?);
		Ok (())
	}
	#[ test ]
	fn part_two () -> GenResult <()> {
		assert_eq! (12, logic::calc_result_part_two (EXAMPLE) ?);
		Ok (())
	}

}
