use super::*;

pub type Card = std::num::NonZeroU32;

#[ derive (Debug) ]
pub struct Deck {
	first: Option <Card>,
	last: Option <Card>,
	next: Vec <Option <Card>>,
}

impl Deck {

	#[ inline ]
	#[ must_use ]
	pub const fn new () -> Self {
		Self {
			first: None,
			last: None,
			next: Vec::new (),
		}
	}

	#[ inline ]
	pub fn pick (& mut self) -> Card {
		let card = self.first.take ().unwrap ();
		self.first = self.next [card.get ().as_usize () - 1].take ();
		if self.first.is_none () { self.last = None }
		card
	}

	#[ inline ]
	pub fn place (& mut self, card: Card) {
		debug_assert! (! self.contains (card));
		if let Some (last) = self.last {
			self.next [last.get ().as_usize () - 1] = Some (card);
		} else {
			self.first = Some (card);
		}
		self.last = Some (card);
	}

	#[ inline ]
	pub fn extend (& mut self, iter: impl IntoIterator <Item = Card>) {
		let mut iter = iter.into_iter ();
		let (size_min, size_max) = iter.size_hint ();
		self.next.resize (size_max.unwrap_or (size_min), None);
		let mut prev = if let Some (prev) = self.last { prev } else {
			let card = some_or! (iter.next (), return);
			debug_assert! (! self.contains (card));
			self.first = Some (card);
			card
		};
		for card in iter {
			debug_assert! (! self.contains (card));
			self.next [prev.get ().as_usize () - 1].replace (card);
			prev = card;
		}
		self.last = Some (prev);
	}

	#[ inline ]
	pub fn place_after (& mut self, after: Card, cards: & [Card]) {
		let mut cards_iter = cards.iter ().copied ().peekable ();
		let first = some_or! (cards_iter.next (), return);
		let resume = self.next [after.get ().as_usize () - 1].replace (first);
		let mut prev = first;
		for card in cards_iter {
			self.next [prev.get ().as_usize () - 1] = Some (card);
			prev = card;
		}
		self.next [prev.get ().as_usize () - 1] = resume;
		if resume.is_none () { self.last = Some (prev); }
	}

	#[ inline ]
	pub fn move_after (& mut self, card: Card) {
		loop {
			let scan = self.pick ();
			self.place (scan);
			if scan == card { return }
		}
	}

	#[ inline ]
	#[ must_use ]
	pub fn contains (& self, card: Card) -> bool{
		self.next [card.get ().as_usize () - 1].is_some () || self.last == Some (card)
	}

}

impl Default for Deck {
	fn default () -> Self {
		Self::new ()
	}
}

impl FromIterator <Card> for Deck {
	fn from_iter <SomeIter> (iter: SomeIter) -> Self
			where SomeIter: IntoIterator <Item = Card> {
		let mut deck = Self::new ();
		deck.extend (iter);
		deck
	}
}
