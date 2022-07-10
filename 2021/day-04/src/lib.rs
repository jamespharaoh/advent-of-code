use aoc_common::*;

puzzle! {
	name = "Giant Squid";
	year = 2021;
	day = 4;
	part_one = |lines| logic::calc_result_part_one (lines);
	part_two = |lines| logic::calc_result_part_two (lines);
}

mod logic {

	use super::*;
	use model::Board;
	use model::Input;

	pub fn calc_result_part_one (lines: & [& str]) -> GenResult <i64> {
		let input = Input::parse (lines) ?;
		Ok (scores_iter (& input).next ().unwrap ())
	}

	pub fn calc_result_part_two (lines: & [& str]) -> GenResult <i64> {
		let input = Input::parse (lines) ?;
		Ok (scores_iter (& input).last ().unwrap ())
	}

	fn scores_iter (input: & Rc <Input>) -> ScoresIter {
		ScoresIter {
			input: input.clone (),
			boards: iter::repeat (false).take (input.boards.len ()).collect (),
			call_idx: 0,
			board_idx: 0,
		}
	}

	struct ScoresIter {
		input: Rc <Input>,
		boards: Vec <bool>,
		call_idx: usize,
		board_idx: usize,
	}

	impl Iterator for ScoresIter {
		type Item = i64;
		fn next (& mut self) -> Option <i64> {
			loop {
				if self.board_idx == self.boards.len () {
					self.board_idx = 0;
					self.call_idx += 1;
					continue;
				}
				if self.call_idx == self.input.call_order.len () {
					return None;
				}
				if self.boards [self.board_idx] {
					self.board_idx += 1;
					continue;
				}
				if let Some (score) = calc_score (
					& self.input.call_order [0 .. self.call_idx + 1],
					& self.input.boards [self.board_idx],
				) {
					self.boards [self.board_idx] = true;
					self.board_idx += 1;
					return Some (score as i64
						* self.input.call_order [self.call_idx] as i64,
					)
				}
				self.board_idx += 1;
			}
		}
	}

	fn calc_score (called: & [u8], board: & Board) -> Option <u16> {
		fn check <I: Iterator <Item = u8>> (called: & [u8], mut nums: I) -> bool {
			nums.all (|num| called.contains (& num))
		}
		let mut winner = false;
		for row in 0 .. 5 {
			if check (called, board.iter ().skip (row * 5).take (5).cloned ()) { winner = true }
		}
		for col in 0 .. 5 {
			if check (called, board.iter ().skip (col).step_by (5).cloned ()) { winner = true }
		}
		if winner {
			Some (board.iter ().cloned ().filter (
				|num| ! called.contains (& num),
			).map (
				|num| num as u16,
			).sum ())
		} else { None }
	}

}

mod model {

	use super::*;

	pub type Board = [u8; 25];

	pub struct Input {
		pub call_order: Vec <u8>,
		pub boards: Vec <Board>,
	}

	impl Input {
		pub fn parse (lines: & [& str]) -> GenResult <Rc <Input>> {
			let call_order: Vec <u8> = lines [0].split (",").map (
				|num_str| num_str.parse::<u8> ().unwrap (),
			).collect ();
			let boards: Vec <Board> = lines [2 ..].chunks (6).map (
				|board_lines| board_lines.iter ().take (5).map (
					|board_line| board_line.split_whitespace ().map (
						|num_str| num_str.parse::<u8> ().unwrap (),
					).collect::<Vec <u8>> (),
				).flatten ().collect::<Vec <u8>> ().try_into ().unwrap (),
			).collect ();
			Ok (Input { call_order, boards }.into ())
		}
	}

}

#[ cfg (test) ]
mod examples {

	use super::*;

	const EXAMPLE: & [& str] = & [
		"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1",
		"",
		"22 13 17 11  0",
		" 8  2 23  4 24",
		"21  9 14 16  7",
		" 6 10  3 18  5",
		" 1 12 20 15 19",
		"",
		" 3 15  0  2 22",
		" 9 18 13 17  5",
		"19  8  7 25 23",
		"20 11 10 24  4",
		"14 21 16 12  6",
		"",
		"14 21 17 24  4",
		"10 16 15  9 19",
		"18  8 23 26 20",
		"22 11 13  6  5",
		" 2  0 12  3  7",
	];

	#[ test ]
	fn part_one () -> GenResult <()> {
		assert_eq! (4512, logic::calc_result_part_one (EXAMPLE) ?);
		Ok (())
	}

	#[ test ]
	fn part_two () -> GenResult <()> {
		assert_eq! (1924, logic::calc_result_part_two (EXAMPLE) ?);
		Ok (())
	}

}
