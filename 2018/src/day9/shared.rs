use parse_display_derive::FromStr;
use std::collections::VecDeque;
use std::iter;

#[ derive (Clone, Copy, FromStr) ]
#[ display ("{num_players} players; last marble is worth {last_marble} points") ]
pub struct GameParams {
	pub num_players: u32,
	pub last_marble: u32,
}

pub fn play (params: & GameParams) -> u32 {
	let mut scores: Vec <u32> = vec! [0; params.num_players as usize];
	let mut circle: VecDeque <u32> = vec! [0].into ();
	for (player, marble) in Iterator::zip (
		iter::repeat (0 .. params.num_players).flatten (),
		1 ..= params.last_marble,
	) {
		if marble % 23 == 0 {
			let score = & mut scores [player as usize];
			* score += marble;
			circle.rotate_right (7);
			* score += circle.pop_front ().unwrap ();
		} else {
			circle.rotate_left (2 % circle.len ());
			circle.push_front (marble);
		}
	}
	scores.into_iter ().max ().unwrap ()
}

#[test]
fn test_play () {
	for (input, expected) in vec! [
		("9 players; last marble is worth 25 points", 32),
		("10 players; last marble is worth 1618 points", 8317),
		("13 players; last marble is worth 7999 points", 146373),
		("17 players; last marble is worth 1104 points", 2764),
		("21 players; last marble is worth 6111 points", 54718),
		("30 players; last marble is worth 5807 points", 37305),
	] {
		let params: GameParams = input.parse ().unwrap ();
		let actual = play (& params);
		assert_eq! (expected, actual);
	}
}
