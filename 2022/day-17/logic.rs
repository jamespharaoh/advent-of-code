use super::*;

use input::Input;
use input::Push;

pub fn part_one (input: & Input) -> GenResult <u64> {
	calc_result (input, input.params.num_drops_one)
}

pub fn part_two (input: & Input) -> GenResult <u64> {
	calc_result (input, input.params.num_drops_two)
}

fn calc_result (input: & Input, num_drops: u64) -> GenResult <u64> {

	if input.pushes.is_empty () {
		return Err ("Must have at least one push direction".into ());
	}

	let mut model = Model::new (input.pushes.clone ());
	let mut states = HashMap::new ();
	let mut piece_idx = 0;
	let mut num_dropped = 0;
	let mut num_iters = 0;

	while num_dropped < num_drops {

		if input.params.max_iters <= num_iters {
			return Err ("Exceeded max iterations".into ());
		}
		num_iters += 1;

		model.drop_piece (PIECES [piece_idx]);
		num_dropped += 1;

		piece_idx += 1;
		if piece_idx == PIECES.len () { piece_idx = 0; }

		let state = State {
			data: model.data.iter ().copied ().collect (),
			piece_idx,
			push_idx: model.push_idx,
		};

		if let Some (& (prev_dropped, prev_height)) = states.get (& state) {
			let diff_placed = num_dropped - prev_dropped;
			let diff_height = model.height - prev_height;
			let reps = (num_drops - num_dropped) / diff_placed;
			num_dropped += diff_placed * reps;
			model.height += diff_height * reps;
		} else {
			states.insert (state, (num_dropped, model.height));
		}

	}

	Ok (model.height)

}

#[ derive (Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd) ]
struct State {
	data: Vec <u8>,
	piece_idx: usize,
	push_idx: usize,
}

struct Model {
	pushes: Vec <Push>,
	push_idx: usize,
	data: VecDeque <u8>,
	height: u64,
}

impl Model {

	fn new (pushes: Vec <Push>) -> Self {
		Self {
			pushes,
			push_idx: 0,
			data: VecDeque::new (),
			height: 0,
		}
	}

	fn can_place (& self, piece: [u8; 5], offset: usize) -> bool {
		if self.data.len () < offset + 5 { return false }
		iter::zip (piece, self.data.iter ().skip (offset))
			.all (|(piece_line, data_line)| piece_line & data_line == 0)
	}

	fn drop_piece (& mut self, mut piece: [u8; 5]) {

		// extend the board up eight units for the piece and three spaces

		for _ in 0_u32 .. 8 {
			self.data.push_front (0b_00000001);
		}
		self.height += 8;

		// work out where the piece will be placed

		let mut offset = 0;
		loop {

			// push if possible

			let push = self.pushes [self.push_idx];
			self.push_idx += 1;
			if self.push_idx == self.pushes.len () { self.push_idx = 0; }

			let pushed_piece = match push {
				Push::Left => piece.map (|line| line.rotate_left (1)),
				Push::Right => piece.map (|line| line.rotate_right (1)),
			};

			if self.can_place (pushed_piece, offset) {
				piece = pushed_piece;
			}

			// drop if possible

			if ! self.can_place (piece, offset + 1) { break }
			offset += 1;

		}

		// update board

		for (line_idx, line) in piece.iter ().enumerate () {
			self.data [offset + line_idx] |= line;
		}

		self.simplify ();

	}

	fn simplify (& mut self) {

		// trim from the top

		while self.data.front () == Some (& 0b_00000001) {
			self.data.pop_front ().unwrap ();
			self.height -= 1;
		}

		// fill in holes and overhangs

		let mut prev_line = 0b_00000001_u8;
		for cur_line in & mut self.data {
			prev_line &= prev_line.rotate_left (1);
			prev_line &= prev_line.rotate_right (1);
			* cur_line |= prev_line;
			prev_line = * cur_line;
		}

		// trim from the bottom

		while self.data.back () == Some (& 0b_11111111) {
			self.data.pop_back ().unwrap ();
		}

	}

}

const PIECES: & [[u8; 5]] = & [
	[
		0b_00000000,
		0b_00000000,
		0b_00000000,
		0b_00000000,
		0b_00111100,
	],
	[
		0b_00000000,
		0b_00000000,
		0b_00010000,
		0b_00111000,
		0b_00010000,
	],
	[
		0b_00000000,
		0b_00000000,
		0b_00001000,
		0b_00001000,
		0b_00111000,
	],
	[
		0b_00000000,
		0b_00100000,
		0b_00100000,
		0b_00100000,
		0b_00100000,
	],
	[
		0b_00000000,
		0b_00000000,
		0b_00000000,
		0b_00110000,
		0b_00110000,
	],
];
