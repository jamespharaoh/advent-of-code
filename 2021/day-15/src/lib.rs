use aoc_common::*;

puzzle! {
	name = "Chiton";
	year = 2021;
	day = 15;
	part_one = |lines| logic::part_one (lines);
	part_two = |lines| logic::part_two (lines);
}

mod logic {

	use super::*;
	use model::Cave;
	use model::Pos;

	pub fn part_one (lines: & [& str]) -> GenResult <u64> {
		let cave = Cave::parse (lines) ?;
		calc_result (& cave)
	}

	pub fn part_two (lines: & [& str]) -> GenResult <u64> {
		let mut cave = Cave::parse (lines) ?;
		let width = cave.end.x + 1;
		let height = cave.end.y + 1;
		cave.risks = cave.risks.into_iter ().flat_map (|(pos, risk)| {
			let mut result = ArrayVec::<(Pos, u8), 25>::new ();
			for (y_incr, y) in (pos.y .. height * 5).step_by (height as usize).enumerate () {
				for (x_incr, x) in (pos.x .. width * 5).step_by (width as usize).enumerate () {
					result.push ((Pos { x, y }, (risk + x_incr as u8 + y_incr as u8 - 1) % 9 + 1));
				}
			}
			result
		}).collect ();
		cave.end = Pos { x: width * 5 - 1, y: height * 5 - 1 };
		calc_result (& cave)
	}

	pub fn calc_result (cave: & Cave) -> GenResult <u64> {
		let max_x = cave.risks.keys ().map (|pos| pos.x).max ().unwrap_or (0);
		let max_y = cave.risks.keys ().map (|pos| pos.y).max ().unwrap_or (0);
		let max = max_x + max_y;
		let mut todo = VecDeque::from ([ (cave.start, 0) ]);
		let mut best = HashMap::from ([ (cave.start, 0) ]);
		for loop_max in 0 ..= max {
			let mut todo_later = Vec::new ();
			while let Some ((pos, path_risk)) = todo.pop_front () {
				if let Some (& best_path_risk) = best.get (& pos) {
					if best_path_risk < path_risk { continue }
				}
				if pos.x + pos.y > loop_max {
					todo_later.push ((pos, path_risk));
					continue;
				}
				for adj_pos in pos.adjacent () {
					if let Some (& adj_risk) = cave.risks.get (& adj_pos) {
						let adj_path_risk = path_risk + adj_risk as u64;
						if adj_path_risk < best.get (& adj_pos).cloned ().unwrap_or (u64::MAX) {
							best.insert (adj_pos, adj_path_risk);
							todo.push_back ((adj_pos, adj_path_risk));
						}
					}
				}
			}
			todo = todo_later.into ();
		}
		Ok (best [& cave.end])
	}

}

mod model {

	use super::*;

	pub struct Cave {
		pub risks: HashMap <Pos, u8>,
		pub start: Pos,
		pub end: Pos,
	}

	impl Cave {
		pub fn parse (lines: & [& str]) -> GenResult <Cave> {
			let mut risks = HashMap::new ();
			let mut start = None;
			let mut end = None;
			for (line_idx, line) in lines.iter ().enumerate () {
				let line_err = || format! ("Invalid input on line {}: {}", line_idx + 1, line);
				let y = line_idx as i64;
				for (char_idx, letter) in line.chars ().enumerate () {
					let x = char_idx as i64;
					let pos = Pos { x, y };
					risks.insert (pos, letter.to_digit (10).ok_or_else (line_err) ? as u8);
					if start.is_none () { start = Some (pos); }
					end = Some (pos);
				}
			}
			let start = start.ok_or (format! ("Invalid input")) ?;
			let end = end.ok_or (format! ("Invalid input")) ?;
			Ok (Cave { risks, start, end })
		}
	}

	#[ derive (Clone, Copy, Eq, Hash, PartialEq) ]
	pub struct Pos { pub x: i64, pub y: i64 }

	impl Pos {
		pub fn adjacent (& self) -> Vec <Pos> {
			vec! [
				Pos { x: self.x - 1, y: self.y },
				Pos { x: self.x + 1, y: self.y },
				Pos { x: self.x, y: self.y - 1 },
				Pos { x: self.x, y: self.y + 1 },
			]
		}
	}

}

#[ cfg (test) ]
mod examples {

	use super::*;

	const EXAMPLE: & [& str] = & [
		"1163751742",
		"1381373672",
		"2136511328",
		"3694931569",
		"7463417111",
		"1319128137",
		"1359912421",
		"3125421639",
		"1293138521",
		"2311944581",
	];

	#[ test ]
	fn part_one () -> GenResult <()> {
		assert_eq! (40, logic::part_one (EXAMPLE) ?);
		Ok (())
	}

	#[ test ]
	fn part_two () -> GenResult <()> {
		assert_eq! (315, logic::part_two (EXAMPLE) ?);
		Ok (())
	}

}
