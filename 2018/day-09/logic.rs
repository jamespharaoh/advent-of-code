//! Logic for solving the puzzles

use super::*;
use input::Input;

pub fn part_one (input: & Input) -> GenResult <u32> {
	Ok (play (input) ?)
}

pub fn part_two (input: & Input) -> GenResult <u32> {
	let input = Input { last_marble: input.last_marble * 100, .. * input };
	Ok (play (& input) ?)
}

fn play (input: & Input) -> NumResult <u32> {
	let mut scores: Vec <u32> = vec! [0; input.num_players.pan_usize ()];
	let mut circle: VecDeque <u32> = vec! [0].into ();
	for (player, marble) in Iterator::zip (
		iter::repeat (0 .. input.num_players).flatten (),
		1 ..= input.last_marble,
	) {
		if marble % 23 == 0 {
			let score = & mut scores [player.pan_usize ()];
			* score = u32::add_2 (* score, marble) ?;
			circle.rotate_right (7);
			* score = u32::add_2 (* score, circle.pop_front ().unwrap ()) ?;
		} else {
			circle.rotate_left (2 % circle.len ());
			circle.push_front (marble);
		}
	}
	Ok (scores.into_iter ().max ().unwrap ())
}

#[ cfg (test) ]
mod tests {

	use super::*;

	#[ test ]
	fn play () {
		for (expected, input) in vec! [
			(8317, Input { num_players: 10, last_marble: 1618, params: default () }),
			(146373, Input { num_players: 13, last_marble: 7999, params: default () }),
			(2764, Input { num_players: 17, last_marble: 1104, params: default () }),
			(54718, Input { num_players: 21, last_marble: 6111, params: default () }),
			(37305, Input { num_players: 30, last_marble: 5807, params: default () }),
		] {
			assert_eq_ok! (expected, logic::play (& input));
		}
	}

}
