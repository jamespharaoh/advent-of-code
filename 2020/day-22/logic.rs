//! Logic for solving the puzzles

use super::*;

use input::Input;
use input::InputParams;
use model::Deck;
use model::DecksState;
use model::Game;
use model::Pool;
use model::Winner;

pub fn part_one (input: & Input) -> GenResult <u32> {
	check_input (input) ?;
	let mut game = Game::new_input (input);
	let winner = loop {
		let decks_state = game.decks_state ();
		if ! game.seen.insert (decks_state) {
			return Err ("Infinite loop".into ());
		}
		if let Some (winner) = game.draw_cards ().err () {
			break match winner {
				Winner::One => game.deck_1,
				Winner::Two => game.deck_2,
			}
		}
		game.replace_cards (
			if game.card_1 < game.card_2 { Winner::Two }
			else { Winner::One });
	};
	Ok (
		winner.iter ().rev ().enumerate ()
			.map (|(idx, card)| (idx.pan_u32 () + 1) * card.pan_u32 ())
			.try_fold (0_u32, |sum, item| chk! (sum + item)) ?
	)
}

pub fn part_two (input: & Input) -> GenResult <u32> {
	check_input (input) ?;
	let winner = play_recursive (input) ?;
	Ok (
		winner.iter ().rev ().enumerate ()
			.map (|(idx, card)| chk! ((idx.pan_u32 () + 1_u32) * card.pan_u32 ()))
			.try_fold (0_u32, |sum, item| chk! (sum + item ?)) ?
	)
}

struct RecursiveGame <'dat> {
	params: & 'dat InputParams,
	num_games: u32,
	num_rounds: u32,
	pool: Pool,
	cache: HashMap <DecksState, Winner>,
	stack: Vec <Game>,
}

fn play_recursive (input: & Input) -> GenResult <Deck> {
	let mut rec_game = RecursiveGame {
		params: & input.params,
		num_games: 1,
		num_rounds: 0,
		pool: Pool::new (),
		cache: HashMap::new (),
		stack: Vec::new (),
	};
	let mut game = Game::new_input (input);
	Ok (match rec_game.play_real (& mut game) ? {
		Winner::One => game.deck_1,
		Winner::Two => game.deck_2,
	})
}

impl <'dat> RecursiveGame <'dat> {

	fn play_real (& mut self, game: & mut Game) -> GenResult <Winner> {
		loop {
			self.inc_num_rounds () ?;

			let decks_state = game.decks_state ();
			if let Some (winner) = None
					.or_else (|| self.cache.get (& decks_state).copied ())
					.or_else (|| (! game.seen.insert (decks_state)).then_some (Winner::One))
					.or_else (|| game.draw_cards ().err ()) {

				// game has been won, unrecurse

				self.cache.insert (game.start_state, winner);
				let prev_game = some_or! (self.stack.pop (), return Ok (winner));
				mem::replace (game, prev_game).free (& mut self.pool);
				game.replace_cards (winner);

			} else if game.deck_1.len () < game.card_1.pan_usize ()
					|| game.deck_2.len () < game.card_2.pan_usize () {

				// not enough cards to recurse, select winner based on value

				game.replace_cards (
					if game.card_1 < game.card_2 { Winner::Two }
					else { Winner::One });

			} else {

				// enough cards to recurse, start a new recursive game

				self.inc_num_games () ?;

				self.stack.push (mem::replace (game, Game::new (
					& mut self.pool,
					game.deck_1.iter ().take (game.card_1.pan_usize ()),
					game.deck_2.iter ().take (game.card_2.pan_usize ()))));

			}

		}
	}

	#[ inline ]
	fn inc_num_rounds (& mut self) -> GenResult <()> {
		if self.num_rounds == self.params.max_rounds {
			return Err ("Giving up after max rounds".into ());
		}
		self.num_rounds += 1;
		Ok (())
	}

	#[ inline ]
	fn inc_num_games (& mut self) -> GenResult <()> {
		if self.num_games == self.params.max_games {
			return Err ("Giving up after max games".into ());
		}
		self.num_games += 1;
		Ok (())
	}

}

fn check_input (input: & Input) -> GenResult <()> {
	if ! input.player_1.iter ().chain (& input.player_2).all_unique () {
		return Err ("No two cards can be the same".into ());
	}
	if 63 < input.player_1.len () + input.player_2.len () {
		return Err ("Can't play with more than 63 cards".into ());
	}
	for player in [ & input.player_1, & input.player_2 ] {
		if player.iter ().any (|& card| card == 0) {
			return Err ("Card must not be zero".into ());
		}
	}
	Ok (())
}
