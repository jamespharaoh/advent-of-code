use std::collections::HashMap;
use std::collections::VecDeque;

fn main() {
	let start: Vec <u32> = vec! [0, 8, 15, 2, 12, 1, 4];
	let num = Generator::new (start).skip (29_999_999).next ().unwrap ();
	println! ("30,000,000th number: {}", num);
}

struct Generator {
	start: VecDeque <u32>,
	last_num: u32,
	last_pos: Option <u32>,
	positions: HashMap <u32, u32>,
	pos: u32,
}

impl Generator {
	fn new (start: Vec <u32>) -> Generator {
		Generator {
			start: start.into (),
			last_num: 0,
			last_pos: None,
			positions: HashMap::new (),
			pos: 0,
		}
	}
}

impl Iterator for Generator {
	type Item = u32;
	fn next (& mut self) -> Option <u32> {
		let this_num = if let Some (num) = self.start.pop_front () {
			num
		} else {
			if let Some (last_pos) = self.last_pos {
				self.pos - 1 - last_pos
			} else { 0 }
		};
		self.last_pos = self.positions.get (& this_num).cloned ();
		self.positions.insert (this_num, self.pos);
		self.last_num = this_num;
		self.pos += 1;
		Some (this_num)
	}
}

#[ test ]
fn test_0 () {
	assert_eq! (
		vec! [0, 3, 6, 0, 3, 3, 1, 0, 4, 0],
		Generator::new (vec! [0, 3, 6]).take (10).collect::<Vec <u32>> (),
	);
}

#[ test ]
fn test_1 () {
	assert_eq! (175594, Generator::new (vec! [0, 3, 6]).skip (29_999_999).next ().unwrap ());
}

#[ test ]
fn test_2 () {
	assert_eq! (2578, Generator::new (vec! [1, 3, 2]).skip (29_999_999).next ().unwrap ());
}

#[ test ]
fn test_3 () {
	assert_eq! (3544142, Generator::new (vec! [2, 1, 3]).skip (29_999_999).next ().unwrap ());
}

#[ test ]
fn test_4 () {
	assert_eq! (261214, Generator::new (vec! [1, 2, 3]).skip (29_999_999).next ().unwrap ());
}

#[ test ]
fn test_5 () {
	assert_eq! (6895259, Generator::new (vec! [2, 3, 1]).skip (29_999_999).next ().unwrap ());
}

#[ test ]
fn test_6 () {
	assert_eq! (18, Generator::new (vec! [3, 2, 1]).skip (29_999_999).next ().unwrap ());
}

#[ test ]
fn test_7 () {
	assert_eq! (362, Generator::new (vec! [3, 1, 2]).skip (29_999_999).next ().unwrap ());
}
