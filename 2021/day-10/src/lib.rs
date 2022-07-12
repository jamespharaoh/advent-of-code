use aoc_common::*;

puzzle_info! {
	name = "Syntax Scoring";
	year = 2021;
	day = 10;
	part_one = |lines| logic::part_one (lines);
	part_two = |lines| logic::part_two (lines);
}

mod logic {

	use super::*;
	use model::Delim;
	use model::Mode;

	pub fn part_one (lines: & [& str]) -> GenResult <u64> {
		let input = model::parse_input (lines) ?;
		let score = input.iter ().map (|line| {
			let mut stack: Vec <Delim> = Vec::new ();
			line.iter ().copied ().map (move |(delim, mode)| match mode {
				Mode::Open => { stack.push (delim); 0 },
				Mode::Close => {
					if stack.pop ().unwrap () != delim {
						delim.mismatched_points ()
					} else { 0 }
				},
			})
		}).flatten ().sum ();
		Ok (score)
	}

	pub fn part_two (lines: & [& str]) -> GenResult <u64> {
		let input = model::parse_input (lines) ?;
		let mut scores: Vec <u64> = input.iter ().filter_map (|line| {
			let mut stack: Vec <Delim> = Vec::new ();
			for (delim, mode) in line.iter ().copied () {
				match mode {
					Mode::Open => stack.push (delim),
					Mode::Close => {
						let expect = stack.pop ().unwrap ();
						if delim != expect { return None }
					},
				}
			}
			Some (
				stack.into_iter ().rev ().map (Delim::not_closed_points)
					.fold (0, |total, value| total * 5 + value)
			)
		}).collect ();
		scores.sort ();
		Ok (scores [(scores.len () - 1) / 2])
	}

}

mod model {

	use super::*;

	type Token = (Delim, Mode);

	pub fn parse_input (lines: & [& str]) -> GenResult <Vec <Vec <Token>>> {
		lines.iter ().enumerate ().map (|(line_idx, line)| {
			let line_err = || format! ("Invalid input: {}: {}", line_idx + 1, line);
			line.chars ().map (
				|letter| decode_char (letter).ok_or_else (|| line_err ().into ()),
			).collect::<GenResult <Vec <Token>>> ()
		}).collect::<GenResult <Vec <Vec <Token>>>> ()
	}

	#[ derive (Clone, Copy, Debug, Eq, PartialEq) ]
	pub enum Delim { Round, Square, Curly, Angle }

	impl Delim {
		pub fn mismatched_points (self) -> u64 {
			match self {
				Delim::Round => 3,
				Delim::Square => 57,
				Delim::Curly => 1197,
				Delim::Angle => 25137,
			}
		}
		pub fn not_closed_points (self) -> u64 {
			match self {
				Delim::Round => 1,
				Delim::Square => 2,
				Delim::Curly => 3,
				Delim::Angle => 4,
			}
		}
	}

	#[ derive (Clone, Copy, Debug, Eq, PartialEq) ]
	pub enum Mode { Open, Close }

	pub fn decode_char (letter: char) -> Option <Token> {
		match letter {
			'(' => Some ((Delim::Round, Mode::Open)),
			')' => Some ((Delim::Round, Mode::Close)),
			'[' => Some ((Delim::Square, Mode::Open)),
			']' => Some ((Delim::Square, Mode::Close)),
			'{' => Some ((Delim::Curly, Mode::Open)),
			'}' => Some ((Delim::Curly, Mode::Close)),
			'<' => Some ((Delim::Angle, Mode::Open)),
			'>' => Some ((Delim::Angle, Mode::Close)),
			_ => None,
		}
	}

}

#[ cfg (test) ]
mod examples {

	use super::*;

	const EXAMPLE: & [& str] = & [
		"[({(<(())[]>[[{[]{<()<>>",
		"[(()[<>])]({[<{<<[]>>(",
		"{([(<{}[<>[]}>{[]{[(<()>",
		"(((({<>}<{<{<>}{[]{[]{}",
		"[[<[([]))<([[{}[[()]]]",
		"[{[{({}]{}}([{[{{{}}([]",
		"{<[[]]>}<{[{[{[]{()[[[]",
		"[<(<(<(<{}))><([]([]()",
		"<{([([[(<>()){}]>(<<{{",
		"<{([{{}}[<[[[<>{}]]]>[]]",
	];

	#[ test ]
	fn part_one () -> GenResult <()> {
		assert_eq! (26397, logic::part_one (EXAMPLE) ?);
		Ok (())
	}

	#[ test ]
	fn part_two () -> GenResult <()> {
		assert_eq! (288957, logic::part_two (EXAMPLE) ?);
		Ok (())
	}

}
