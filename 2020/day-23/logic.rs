//! Logic for solving the puzzles

use super::*;

use input::Input;
use model::Card;
use model::Deck;

pub fn part_one (input: & Input) -> GenResult <String> {
	let mut deck =
		calc_final_deck (
			input,
			Card::new (9).unwrap (),
			input.params.iters_one) ?;
	deck.move_after (Card::new (1).unwrap ());
	let mut result = String::new ();
	for _ in 0_u32 .. 8 {
		let next = deck.pick ();
		result.push (char::from_digit (next.get (), 10).unwrap ());
	}
	Ok (result)
}

pub fn part_two (input: & Input) -> GenResult <u64> {
	let mut deck =
		calc_final_deck (
			input,
			Card::new (input.params.deck_size_two).unwrap (),
			input.params.iters_two) ?;
	deck.move_after (Card::new (1).unwrap ());
	Ok (chk! (deck.pick ().get ().as_u64 () * deck.pick ().get ().as_u64 ()) ?)
}

fn calc_final_deck (input: & Input, deck_size: Card, num_rounds: u32) -> GenResult <Deck> {
	let mut deck = build_deck (input, deck_size) ?;
	for _ in 0 .. num_rounds {
		play_one_round (& mut deck, deck_size);
	}
	Ok (deck)
}

fn build_deck (input: & Input, deck_size: Card) -> GenResult <Deck> {
	if input.start.chars ().duplicates ().next ().is_some () {
		return Err ("Duplicate cards".into ());
	}
	Ok (
		input.start.chars ()
			.map (|ch| ch.to_digit (10).unwrap ().as_u32 ())
			.chain (10 ..= deck_size.get ())
			.map (|val| Card::new (val).unwrap ())
			.take (deck_size.get ().as_usize ())
			.collect ()
	)
}

fn play_one_round (deck: & mut Deck, deck_size: Card) {
	let cur = deck.pick ();
	let picked: [Card; 3] = array::from_fn (|_| deck.pick ());
	let mut prev = get_prev (cur, deck_size);
	while prev == picked [0] || prev == picked [1] || prev == picked [2] {
		prev = get_prev (prev, deck_size);
	}
	deck.place_after (prev, & picked);
	deck.place (cur);
}

fn get_prev (card: Card, deck_size: Card) -> Card {
	Card::new (card.get () - 1).unwrap_or (deck_size)
}
