use super::*;

use input::Input;

const DIRAC_FREQS: [(u8, u128); 7] = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

pub fn part_one (input: & Input) -> GenResult <u64> {
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
		Player { score: 0, pos: input.player_1 - 1 },
		Player { score: 0, pos: input.player_2 - 1 },
	];
	let winner = 'OUTER: loop {
		for (player_idx, player) in players.iter_mut ().enumerate () {
			let roll = iter::from_fn (|| Some (die_roll ())).take (3).sum::<u16> ();
			player.pos = ((player.pos.pan_u16 () + roll.pan_u16 ()) % 10).pan_u8 ();
			player.score += player.pos.pan_u16 () + 1;
			if player.score >= 1000 { break 'OUTER player_idx }
		}
	};
	let loser = (winner + 1) % 2;
	Ok (die_count * players [loser].score.pan_u64 ())
}

pub fn part_two (input: & Input) -> GenResult <u128> {

	// array indexed by game state to track the number of "universes" in which each occurs

	let mut games: Vec <u128> = iter::repeat (0_u128).take (1 << 19).collect ();

	// seed the state with the single starting game in one universe

	let start_game = Game::start (input);
	games [start_game.idx ()] = 1;

	// `num_wins` is used to track universes where each player wins

	let mut num_wins = [0_u128; 2];

	// iterate over game states adding future game states which branch from each

	for this_game_idx in start_game.idx () .. games.len () {
		let this_times = games [this_game_idx];
		if this_times == 0 { continue }
		let this_game = Game::from_idx (this_game_idx);
		for (new_game, new_player, freq) in this_game.iter_next () {
			if 21 <= new_player.score {
				num_wins [this_game.turn.pan_usize ()] += this_times * freq;
			} else {
				games [new_game.idx ()] += this_times * freq;
			}
		}
	}

	// return the higher of the two numbers of winning universes

	Ok (num_wins.iter ().copied ().max ().unwrap ())

}

#[ derive (Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd) ]
struct Player {
	score: u8,
	pos: u8,
}

#[ derive (Clone, Copy, Debug, Default, Eq, Hash, PartialEq) ]
struct Game {
	players: [Player; 2],
	turn: u8,
}

impl Game {

	const fn start (input: & Input) -> Self {
		Self {
			players: [
				Player { score: 0, pos: input.player_1 },
				Player { score: 0, pos: input.player_2 },
			],
			turn: 0,
		}
	}

	fn idx (self) -> usize {
		self.players [0].score.qck_usize () << 14_u32
			| self.players [1].score.qck_usize () << 9_u32
			| self.players [0].pos.qck_usize () << 5_u32
			| self.players [1].pos.qck_usize () << 1_u32
			| self.turn.qck_usize ()
	}

	fn from_idx (idx: usize) -> Self {
		Self {
			players: [
				Player {
					score: (idx >> 14_u32).qck_u8 () & 0x1f,
					pos: (idx >> 5_u32).qck_u8 () & 0xf,
				},
				Player {
					score: (idx >> 9_u32).qck_u8 () & 0x1f,
					pos: (idx >> 1_u32).qck_u8 () & 0xf,
				},
			],
			turn: idx.qck_u8 () & 0x1,
		}
	}

	fn iter_next (self) -> impl Iterator <Item = (Self, Player, u128)> {
		DIRAC_FREQS.into_iter ().map (move |(roll_val, roll_freq)| {
			let mut new_game = self;
			let new_player = & mut new_game.players [self.turn.pan_usize ()];
			new_player.pos += roll_val;
			if 10 < new_player.pos { new_player.pos -= 10; }
			new_player.score += new_player.pos;
			let new_player = * new_player;
			new_game.turn ^= 1;
			debug_assert! (new_game < self);
			(new_game, new_player, roll_freq)
		})
	}

}

impl Ord for Game {
	fn cmp (& self, other: & Self) -> Ordering {
		Ordering::Equal
			.then (self.players [0].score.cmp (& other.players [0].score))
			.then (self.players [1].score.cmp (& other.players [1].score))
			.then (self.players [0].pos.cmp (& other.players [0].pos))
			.then (self.players [1].pos.cmp (& other.players [1].pos))
			.then (self.turn.cmp (& other.turn))
			.reverse ()
	}
}

impl PartialOrd for Game {
	fn partial_cmp (& self, other: & Self) -> Option <Ordering> {
		Some (self.cmp (other))
	}
}
