use super::*;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub hands: Vec <Hand>,
	pub params: InputParams,
}

struct_parser_display! {
	Input { hands, params } = [ params, @lines hands ]
}

#[ derive (Clone, Copy, Debug) ]
pub struct Hand {
	pub cards: [Card; 5],
	pub bid: u32,
}

struct_parser_display! {
	Hand { cards, bid } = [ @array cards, " ", bid ]
}

enum_decl_parser_display! {
	#[ derive (Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd) ]
	pub enum Card {
		Joker = [ "X" ],
		Two = [ "2" ],
		Three = [ "3" ],
		Four = [ "4" ],
		Five = [ "5" ],
		Six = [ "6" ],
		Seven = [ "7" ],
		Eight = [ "8" ],
		Nine = [ "9" ],
		Ten = [ "T" ],
		Jack = [ "J" ],
		Queen = [ "Q" ],
		King = [ "K" ],
		Ace = [ "A" ],
	}
}

impl Card {
	pub const ALL: [Self; 14] = [
		Self::Joker,
		Self::Two,
		Self::Three,
		Self::Four,
		Self::Five,
		Self::Six,
		Self::Seven,
		Self::Eight,
		Self::Nine,
		Self::Ten,
		Self::Jack,
		Self::Queen,
		Self::King,
		Self::Ace,
	];
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
	}
}
