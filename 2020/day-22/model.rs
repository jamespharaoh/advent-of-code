use super::*;

use input::Input;

pub type Card = u8;
pub type Deck = VecDeque <u8>;

#[ derive (Clone, Copy, Debug) ]
pub enum Winner { One, Two }

pub struct Pool {
	decks: Vec <Deck>,
	seens: Vec <HashSet <DecksState>>,
}

impl Pool {

	#[ must_use ]
	pub const fn new () -> Self {
		Self {
			decks: Vec::new (),
			seens: Vec::new (),
		}
	}

	#[ inline ]
	fn new_deck (& mut self, iter: impl Iterator <Item = Card>) -> Deck {
		let mut deck = self.decks.pop ().unwrap_or_default ();
		deck.clear ();
		deck.extend (iter);
		deck
	}

	#[ inline ]
	fn free_deck (& mut self, deck: Deck) {
		self.decks.push (deck);
	}

	#[ inline ]
	fn new_seen (& mut self) -> HashSet <DecksState> {
		let mut seen = self.seens.pop ().unwrap_or_default ();
		seen.clear ();
		seen
	}

	#[ inline ]
	fn free_seen (& mut self, seen: HashSet <DecksState>) {
		self.seens.push (seen);
	}

}

#[ derive (Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd) ]
pub struct DecksState {
	decks: [Card; 64],
}

impl DecksState {

	#[ inline ]
	#[ must_use ]
	pub fn new (deck_1: & Deck, deck_2: & Deck) -> Self {
		let mut decks = [0; 64];
		let mut idx = 0;
		for & val in deck_1 { debug_assert! (val != 0); decks [idx] = val; idx += 1; }
		idx += 1;
		for & val in deck_2 { debug_assert! (val != 0); decks [idx] = val; idx += 1; }
		Self { decks }
	}

}

pub struct Game {
	pub start_state: DecksState,
	pub deck_1: Deck,
	pub deck_2: Deck,
	pub card_1: Card,
	pub card_2: Card,
	pub seen: HashSet <DecksState>,
}

impl Game {

	#[ must_use ]
	pub fn new_input (input: & Input) -> Self {
		let deck_1 = input.player_1.iter ().copied ().collect ();
		let deck_2 = input.player_2.iter ().copied ().collect ();
		let start_state = DecksState::new (& deck_1, & deck_2);
		Self {
			start_state,
			deck_1,
			deck_2,
			card_1: 0,
			card_2: 0,
			seen: HashSet::new (),
		}
	}

	#[ inline ]
	pub fn new <'card> (
		pool: & mut Pool,
		deck_1: impl Iterator <Item = & 'card Card>,
		deck_2: impl Iterator <Item = & 'card Card>,
	) -> Self {
		let deck_1 = pool.new_deck (deck_1.copied ());
		let deck_2 = pool.new_deck (deck_2.copied ());
		let start_state = DecksState::new (& deck_1, & deck_2);
		Self {
			start_state,
			deck_1,
			deck_2,
			card_1: 0,
			card_2: 0,
			seen: pool.new_seen (),
		}
	}

	#[ inline ]
	pub fn free (self, pool: & mut Pool) {
		pool.free_deck (self.deck_1);
		pool.free_deck (self.deck_2);
		pool.free_seen (self.seen);
	}

	pub fn draw_cards (& mut self) -> Result <(), Winner> {
		if self.deck_1.is_empty () { return Err (Winner::Two) }
		if self.deck_2.is_empty () { return Err (Winner::One) }
		self.card_1 = self.deck_1.pop_front ().unwrap ();
		self.card_2 = self.deck_2.pop_front ().unwrap ();
		Ok (())
	}

	pub fn replace_cards (& mut self, winner: Winner) {
		match winner {
			Winner::One => {
				self.deck_1.push_back (self.card_1);
				self.deck_1.push_back (self.card_2);
			},
			Winner::Two => {
				self.deck_2.push_back (self.card_2);
				self.deck_2.push_back (self.card_1);
			},
		}
	}

	#[ inline ]
	#[ must_use ]
	pub fn decks_state (& self) -> DecksState {
		DecksState::new (& self.deck_1, & self.deck_2)
	}

}
