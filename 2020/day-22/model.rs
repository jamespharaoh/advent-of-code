use super::*;

pub type Card = u8;
pub type Deck = VecDeque <u8>;

#[ derive (Clone, Copy, Debug) ]
pub enum Winner { One, Two }

pub struct Pool {
	decks: Vec <Deck>,
	seens: Vec <HashSet <State>>,
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
	fn new_seen (& mut self) -> HashSet <State> {
		let mut seen = self.seens.pop ().unwrap_or_default ();
		seen.clear ();
		seen
	}

	#[ inline ]
	fn free_seen (& mut self, seen: HashSet <State>) {
		self.seens.push (seen);
	}

}

#[ derive (Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd) ]
pub struct State {
	decks: [Card; 64],
}

impl State {

	#[ inline ]
	#[ must_use ]
	pub fn new (deck_1: & Deck, deck_2: & Deck) -> Self {
		let mut decks = [0; 64];
		let mut idx = 0;
		for & val in deck_1 { decks [idx] = val; idx += 1; }
		idx += 1;
		for & val in deck_2 { decks [idx] = val; idx += 1; }
		Self { decks }
	}

}

pub struct Game {
	pub start_state: State,
	pub deck_1: Deck,
	pub deck_2: Deck,
	pub card_1: Card,
	pub card_2: Card,
	pub seen: HashSet <State>,
}

impl Game {

	#[ inline ]
	pub fn new <'card> (
		pool: & mut Pool,
		deck_1: impl Iterator <Item = & 'card Card>,
		deck_2: impl Iterator <Item = & 'card Card>,
	) -> Self {
		let deck_1 = pool.new_deck (deck_1.copied ());
		let deck_2 = pool.new_deck (deck_2.copied ());
		let start_state = State::new (& deck_1, & deck_2);
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

}
