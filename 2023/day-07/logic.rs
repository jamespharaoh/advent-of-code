use super::*;

use input::Card;
use input::Hand;
use input::Input;

pub fn part_one (input: & Input) -> GenResult <u64> {
	Ok (input.hands.iter ().copied ()
		.map (|hand| (hand, calc_hand_type (hand.cards)))
		.sorted_by_key (|& (hand, type_)| (type_, hand.cards))
		.enumerate ()
		.map (|(idx, (hand, _))| (idx.pan_u64 () + 1) * hand.bid.pan_u64 ())
		.sum ())
}

pub fn part_two (input: & Input) -> GenResult <u64> {
	let hands: Vec <Hand> =
		input.hands.iter ()
			.map (|& hand| Hand {
				cards: hand.cards.map (|card| if card == Card::Jack { Card::Joker } else { card }),
				bid: hand.bid,
			})
			.collect ();
	Ok (hands.iter ().copied ()
		.map (|hand| (hand, calc_hand_type (hand.cards)))
		.sorted_by_key (|& (hand, type_)| (type_, hand.cards))
		.enumerate ()
		.map (|(idx, (hand, _))| (idx.pan_u64 () + 1) * hand.bid.pan_u64 ())
		.sum ())
}

fn calc_hand_type (cards: [Card; 5]) -> HandType {
	let nums = Card::ALL
		.map (|card| cards.into_iter ()
		.filter (|& other_card| other_card == card)
		.count ());
	let jokers = nums [0];
	let others = || nums [1 .. ].iter ().copied ();
	if others ().any (|num| 5 <= num + jokers) { return HandType::FiveOfAKind }
	if others ().any (|num| 4 <= num + jokers) { return HandType::FourOfAKind }
	if others ().filter (|& num| 0 < num).count () <= 2 { return HandType::FullHouse }
	if others ().any (|num| 3 <= num + jokers) { return HandType::ThreeOfAKind }
	if jokers == 0 && others ().filter (|& num| num == 2).count () == 2 { return HandType::TwoPair }
	if jokers == 1 && others ().any (|num| num == 2) { return HandType::TwoPair }
	if 2 <= jokers { return HandType::TwoPair }
	if 1 <= jokers { return HandType::OnePair }
	if others ().any (|num| num == 2) { return HandType::OnePair }
	if others ().all (|num| num == 0 || num == 1) { return HandType::HighCard }
	HandType::None
}

#[ derive (Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd) ]
enum HandType {
	None,
	HighCard,
	OnePair,
	TwoPair,
	ThreeOfAKind,
	FullHouse,
	FourOfAKind,
	FiveOfAKind,
}
