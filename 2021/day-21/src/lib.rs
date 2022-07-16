//! Advent of Code 2021: Day 21: Dirac Dice
//!
//! [https://adventofcode.com/2021/day/21](https://adventofcode.com/2021/day/21)

use aoc_common::*;

puzzle_info! {
	name = "Dirac Dice";
	year = 2021;
	day = 21;
	part_one = |lines| logic::part_one (lines);
	part_two = |lines| logic::part_two (lines);
}

mod logic {

	use super::*;

	pub fn part_one (lines: & [& str]) -> GenResult <u64> {
		let (start_1, start_2) = model::parse_input (lines) ?;
		let mut die_state: u16 = 0;
		let mut die_count: u64 = 0;
		let mut die_roll = || {
			let roll = die_state + 1;
			die_state = (die_state + 1) % 100;
			die_count += 1;
			roll
		};
		struct Player { score: u16, pos: u8 }
		let mut players = [
			Player { score: 0, pos: start_1 - 1 },
			Player { score: 0, pos: start_2 - 1 },
		];
		let winner = 'OUTER: loop {
			for (player_idx, player) in players.iter_mut ().enumerate () {
				let roll = iter::from_fn (|| Some (die_roll ())).take (3).sum::<u16> ();
				player.pos = ((player.pos as u16 + roll as u16) % 10) as u8;
				player.score += player.pos as u16 + 1;
				if player.score >= 1000 { break 'OUTER player_idx }
			}
		};
		let loser = (winner + 1) % 2;
		Ok (die_count * players [loser].score as u64)
	}

	pub fn part_two (lines: & [& str]) -> GenResult <u128> {
		let (start_1, start_2) = model::parse_input (lines) ?;
		#[ derive (Clone, Copy, Debug, Default, Eq, Hash, PartialEq) ]
		struct Player { score: u8, pos: u8 }
		#[ derive (Clone, Copy, Debug, Default, Eq, Hash, PartialEq) ]
		struct Game { players: [Player; 2], turn: u8 }
		struct Frame { game: Game, counts: [u128; 2], progress: usize }
		let start_game = Game {
			players: [ Player { score: 0, pos: start_1 }, Player { score: 0, pos: start_2 } ],
			turn: 0,
		};
		let mut solved: HashMap <Game, [u128; 2]> = HashMap::new ();
		let mut stack: Vec <Frame> = Vec::new ();
		stack.push (Frame { game: start_game, counts: [0; 2], progress: 0 });
		let roll_freqs: [(u8, usize); 7] = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];
		while let Some (frame) = stack.last_mut () {
			if frame.progress == roll_freqs.len () {
				solved.insert (frame.game, frame.counts);
				stack.pop ().unwrap ();
				continue;
			}
			let old_player = frame.game.players [frame.game.turn as usize];
			let (roll_val, roll_freq) = roll_freqs [frame.progress];
			let mut new_game = frame.game;
			let new_player = & mut new_game.players [frame.game.turn as usize];
			new_player.pos = ((old_player.pos - 1) + roll_val) % 10 + 1;
			new_player.score = old_player.score + new_player.pos;
			if new_player.score >= 21 {
				frame.counts [frame.game.turn as usize] = u128::checked_add (
					frame.counts [frame.game.turn as usize], roll_freq as u128).unwrap ();
				frame.progress += 1;
				continue;
			}
			new_game.turn = (frame.game.turn + 1) % 2;
			if let Some (solved_counts) = solved.get (& new_game) {
				frame.counts = [
					u128::checked_add (frame.counts [0],
						u128::checked_mul (solved_counts [0], roll_freq as u128).unwrap (),
					).unwrap (),
					u128::checked_add (frame.counts [1],
						u128::checked_mul (solved_counts [1], roll_freq as u128).unwrap (),
					).unwrap (),
				];
				frame.progress += 1;
				continue;
			}
			let new_frame = Frame {
				game: new_game,
				counts: [0; 2],
				progress: 0,
			};
			stack.push (new_frame);
		}
		Ok (solved [& start_game].into_iter ().max ().unwrap ())
	}

}

mod model {

	use super::*;

	pub fn parse_input (lines: & [& str]) -> GenResult <(u8, u8)> {
		let err = move |line_idx| move |char_idx|
			format! ("Invalid input: line {}: {}", line_idx + 1, char_idx + 1);
		if lines.len () != 2 { Err (err (lines.len ()) (0)) ?; }
		let mut parser = parser::Parser::new (lines [0], err (0));
		let start_1: u8 = parser.expect ("Player 1 starting position: ") ?.int () ?;
		let mut parser = parser::Parser::new (lines [1], err (1));
		let start_2: u8 = parser.expect ("Player 2 starting position: ") ?.int () ?;
		Ok ((start_1, start_2))
	}

}

#[ cfg (test) ]
mod examples {

	use super::*;

	const EXAMPLE: & 'static [& 'static str] = & [
		"Player 1 starting position: 4",
		"Player 2 starting position: 8",
	];

	#[ test ]
	fn part_one () -> GenResult <()> {
		assert_eq! (739785, logic::part_one (EXAMPLE) ?);
		Ok (())
	}

	#[ test ]
	fn part_two () -> GenResult <()> {
		assert_eq! (444356092776315, logic::part_two (EXAMPLE) ?);
		Ok (())
	}

}
