//! Logic for solving the puzzles

use super::*;

use input::Input;
use model::Deck;
use model::Game;
use model::Pool;
use model::State;
use model::Winner;

pub fn part_one (input: & Input) -> GenResult <u32> {
	check_input (input) ?;
	let mut deck_1: Deck = input.player_1.iter ().copied ().collect ();
	let mut deck_2: Deck = input.player_2.iter ().copied ().collect ();
	let mut seen = HashSet::new ();
	while ! deck_1.is_empty () && ! deck_2.is_empty () {
		if ! seen.insert (State::new (& deck_1, & deck_2)) {
			return Err ("Game is looping".into ());
		}
		let card_1 = deck_1.pop_front ().unwrap ();
		let card_2 = deck_2.pop_front ().unwrap ();
		if card_1 < card_2 {
			deck_2.push_back (card_2);
			deck_2.push_back (card_1);
		} else {
			deck_1.push_back (card_1);
			deck_1.push_back (card_2);
		}
	}
	let winner = if deck_1.is_empty () { deck_2 } else { deck_1 };
	Ok (
		winner.iter ().rev ().enumerate ()
			.map (|(idx, card)| (idx.pan_u32 () + 1) * card.pan_u32 ())
			.try_fold (0_u32, |sum, item| chk! (sum + item)) ?
	)
}

pub fn part_two (input: & Input) -> GenResult <u32> {
	check_input (input) ?;
	let mut pool = Pool::new ();
	let mut stack: Vec <Game> = Vec::new ();
	let mut game = Game::new (& mut pool, input.player_1.iter (), input.player_2.iter ());
	let mut games = 0_u32;
	let mut rounds = 0_u32;
	let mut cache: HashMap <State, Winner> = HashMap::new ();
	let winner = 'OUTER: loop {
		if games == 50_000 { return Err ("Giving up after max games".into ()) }
		games += 1;
		let winner = loop {
			if rounds == 2_000_000 { return Err ("Giving up after max rounds".into ()) }
			rounds += 1;
			let state = State::new (& game.deck_1, & game.deck_2);
			if let Some (& winner) = cache.get (& state) { break winner }
			if ! game.seen.insert (state) { break Winner::One }
			game.card_1 = some_or! (game.deck_1.pop_front (), break Winner::Two);
			game.card_2 = some_or! (game.deck_2.pop_front (), break Winner::One);
			if game.card_1.pan_usize () <= game.deck_1.len ()
					&& game.card_2.pan_usize () <= game.deck_2.len () {
				let new_game = Game::new (& mut pool,
					game.deck_1.iter ().take (game.card_1.pan_usize ()),
					game.deck_2.iter ().take (game.card_2.pan_usize ()));
				stack.push (game);
				game = new_game;
				continue 'OUTER;
			}
			if game.card_1 < game.card_2 {
				game.deck_2.push_back (game.card_2);
				game.deck_2.push_back (game.card_1);
			} else {
				game.deck_1.push_back (game.card_1);
				game.deck_1.push_back (game.card_2);
			}
		};
		cache.insert (game.start_state, winner);
		let prev_game = some_or! (stack.pop (), break winner);
		game.free (& mut pool);
		game = prev_game;
		match winner {
			Winner::One => {
				game.deck_1.push_back (game.card_1);
				game.deck_1.push_back (game.card_2);
			},
			Winner::Two => {
				game.deck_2.push_back (game.card_2);
				game.deck_2.push_back (game.card_1);
			},
		}
	};
	let winner = match winner {
		Winner::One => game.deck_1,
		Winner::Two => game.deck_2,
	};
	Ok (
		winner.iter ().rev ().enumerate ()
			.map (|(idx, card)| (idx.pan_u32 () + 1) * card.pan_u32 ())
			.try_fold (0_u32, |sum, item| chk! (sum + item)) ?
	)
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
