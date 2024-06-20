use super::*;

use input::Input;

pub fn part_one (input: & Input) -> GenResult <u64> {
	let mut sum = 0;
	for card in & input.cards {
		let mut score = 0;
		let winning: HashSet <u8> = card.winning.iter ().copied ().collect ();
		for & selected in & card.selected {
			if ! winning.contains (& selected) { continue }
			if score == 0 {
				score = 1;
			} else {
				score <<= 1_i32;
			}
		}
		sum += score;
	}
	Ok (sum)
}

pub fn part_two (input: & Input) -> GenResult <u64> {
	let mut sum = 0;
	let mut cards: Vec <_> = input.cards.iter ().map (|card| (card, 1)).collect ();
	for card_idx in 0 .. cards.len () {
		let (card, num) = cards [card_idx];
		sum += num;
		let winning: HashSet <u8> = card.winning.iter ().copied ().collect ();
		let score =
			card.selected.iter ().copied ()
				.filter (|& num| winning.contains (& num))
				.count ()
				.pan_u64 ();
		for & mut (_, ref mut next_num) in
				cards [card_idx + 1 .. ].iter_mut ().take (score.pan_usize ()) {
			* next_num += num;
		}
	}
	Ok (sum)
}
