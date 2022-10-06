use super::*;

use input::Input;
use model::Amph::{ Amber, Bronze, Copper, Desert };
use model::Place::{ Hall, Room };

#[ derive (Clone, Debug, Eq, PartialEq) ]
pub struct State {
	room_size: usize,
	hall: ArrayVec <Option <Amph>, 11>,
	amber: ArrayVec <Amph, 4>,
	bronze: ArrayVec <Amph, 4>,
	copper: ArrayVec <Amph, 4>,
	desert: ArrayVec <Amph, 4>,
}

impl State {

	#[ must_use ]
	pub fn new_part_one (input: & Input) -> Self {
		Self {
			room_size: 2,
			hall: array_vec! [None; 11],
			amber: array_vec! [ input.amphs [0] [1], input.amphs [0] [0] ],
			bronze: array_vec! [ input.amphs [1] [1], input.amphs [1] [0] ],
			copper: array_vec! [ input.amphs [2] [1], input.amphs [2] [0] ],
			desert: array_vec! [ input.amphs [3] [1], input.amphs [3] [0] ],
		}
	}

	#[ must_use ]
	pub fn new_part_two (input: & Input) -> Self {
		Self {
			room_size: 4,
			hall: array_vec! [None; 11],
			amber: array_vec! [ input.amphs [0] [1], Desert, Desert, input.amphs [0] [0] ],
			bronze: array_vec! [ input.amphs [1] [1], Bronze, Copper, input.amphs [1] [0] ],
			copper: array_vec! [ input.amphs [2] [1], Amber, Bronze, input.amphs [2] [0] ],
			desert: array_vec! [ input.amphs [3] [1], Copper, Amber, input.amphs [3] [0] ],
		}
	}

	#[ must_use ]
	pub fn from_array (room_size: usize, places: [Option <Amph>; 27]) -> Self {
		let mut state = Self {
			room_size,
			hall: ArrayVec::new (),
			amber: ArrayVec::new (),
			bronze: ArrayVec::new (),
			copper: ArrayVec::new (),
			desert: ArrayVec::new (),
		};
		state.hall = places [0 .. 11].iter ().copied ().collect ();
		for room in Amph::VARIANTS.iter ().copied () {
			state.room_mut (room).extend (
				places [11 + room.idx () * 4 .. ].iter ().copied ().take (room_size).rev ()
					.while_some ());
		}
		state
	}

	#[ must_use ]
	pub fn as_array (& self) -> [Option <Amph>; 27] {
		let mut result = [None; 27];
		for (result, & hall) in result.iter_mut ().zip (& self.hall) {
			* result = hall;
		}
		for (idx, amph) in self.amber.iter ().copied ().enumerate () {
			result [11 + self.room_size - idx - 1] = Some (amph);
		}
		for (idx, amph) in self.bronze.iter ().copied ().enumerate () {
			result [15 + self.room_size - idx - 1] = Some (amph);
		}
		for (idx, amph) in self.copper.iter ().copied ().enumerate () {
			result [19 + self.room_size - idx - 1] = Some (amph);
		}
		for (idx, amph) in self.desert.iter ().copied ().enumerate () {
			result [23 + self.room_size - idx - 1] = Some (amph);
		}
		result
	}

	#[ must_use ]
	pub const fn room_size (& self) -> usize {
		self.room_size
	}

	#[ must_use ]
	pub fn get (& self, place: Place) -> Option <Amph> {
		match place {
			Hall (id) => self.hall [id.pan_usize ()],
			Room (Amber, depth) =>
				self.amber.get (self.room_size - depth.pan_usize () - 1).copied (),
			Room (Bronze, depth) =>
				self.bronze.get (self.room_size - depth.pan_usize () - 1).copied (),
			Room (Copper, depth) =>
				self.copper.get (self.room_size - depth.pan_usize () - 1).copied (),
			Room (Desert, depth) =>
				self.desert.get (self.room_size - depth.pan_usize () - 1).copied (),
		}
	}

	#[ must_use ]
	pub fn hall (& self) -> & [Option <Amph>] {
		& self.hall
	}

	#[ must_use ]
	pub fn room (& self, amph: Amph) -> & [Amph] {
		match amph {
			Amber => & self.amber,
			Bronze => & self.bronze,
			Copper => & self.copper,
			Desert => & self.desert,
		}
	}

	fn room_mut (& mut self, amph: Amph) -> & mut ArrayVec <Amph, 4> {
		match amph {
			Amber => & mut self.amber,
			Bronze => & mut self.bronze,
			Copper => & mut self.copper,
			Desert => & mut self.desert,
		}
	}

	#[ must_use ]
	pub fn is_finished (& self) -> bool {
		self.hall.iter ().all (Option::is_none)
			&& self.amber.iter ().all (|& amph| amph == Amber)
			&& self.bronze.iter ().all (|& amph| amph == Bronze)
			&& self.copper.iter ().all (|& amph| amph == Copper)
			&& self.desert.iter ().all (|& amph| amph == Desert)
	}

	#[ must_use ]
	pub fn room_is_happy (& self, room: Amph) -> bool {
		self.room (room).iter ().copied ().all (|amph| amph == room)
	}

	#[ must_use ]
	pub fn move_out (& self, room: Amph, hall: Place) -> Self {
		let mut state = self.clone ();
		let amph = state.room_mut (room).pop ().unwrap ();
		assert! (state.get (hall).is_none ());
		state.hall [hall.idx ()] = Some (amph);
		state
	}

	#[ must_use ]
	pub fn move_in (& self, hall: Place, room: Amph) -> Self {
		let mut state = self.clone ();
		let amph = state.hall [hall.idx ()].take ().unwrap ();
		state.room_mut (room).push (amph);
		state
	}

	#[ must_use ]
	pub fn move_between (& self, from: Amph, to: Amph) -> Self {
		let mut state = self.clone ();
		let amph = state.room_mut (from).pop ().unwrap ();
		let to = match to {
			Amber => & mut state.amber,
			Bronze => & mut state.bronze,
			Copper => & mut state.copper,
			Desert => & mut state.desert,
		};
		assert! (to.len () < self.room_size);
		to.push (amph);
		state
	}

	#[ must_use ]
	pub fn pretty_line (& self, line: usize) -> String {
		let print_amph = |amph: Option <Amph>| amph.map_or (' ', Amph::letter);
		if line == 0 {
			"┌───────────┐".to_owned ()
		} else if line == 1 {
			format! ("│{}│",
				self.hall.iter ().copied ().map (print_amph).collect::<String> ())
		} else if line == 2 {
			format! ("└─┐{}╷{}╷{}╷{}┌─┘",
				print_amph (self.get (Room (Amber, 0))),
				print_amph (self.get (Room (Bronze, 0))),
				print_amph (self.get (Room (Copper, 0))),
				print_amph (self.get (Room (Desert, 0))))
		} else if line < self.room_size.pan_usize () + 2 {
			let depth = (line - 2).pan_u8 ();
			format! ("  │{}│{}│{}│{}│",
				print_amph (self.get (Room (Amber, depth))),
				print_amph (self.get (Room (Bronze, depth))),
				print_amph (self.get (Room (Copper, depth))),
				print_amph (self.get (Room (Desert, depth))))
		} else if line == self.room_size.pan_usize () + 2 {
			"  └─┴─┴─┴─┘".to_owned ()
		} else {
			panic! ();
		}
	}

	#[ allow (clippy::print_stdout) ]
	pub fn print (& self) {
		for line in 0 .. self.room_size.pan_usize () + 3 {
			println! ("{}", self.pretty_line (line));
		}
	}

	#[ must_use ]
	pub fn compact (& self) -> StateCompact {
		let mut place_bits: u64 = 0;
		let mut amph_bits: u64 = 0;
		for amph in self.as_array () {
			place_bits <<= 1_u64;
			if let Some (amph) = amph {
				place_bits |= 1;
				amph_bits <<= 2_i32;
				amph_bits |= match amph {
					Amber => 0,
					Bronze => 1,
					Copper => 2,
					Desert => 3,
				}
			}
		}
		assert! (64 - place_bits.leading_zeros () <= 27);
		assert! (64 - amph_bits.leading_zeros () <= 32);
		let data = ((self.room_size.pan_u64 ()) << 59_i32) | (place_bits << 32_i32) | amph_bits;
		StateCompact { data }
	}

}

impl Display for State {
	fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
		for amph in self.hall.iter ().copied () {
			write! (formatter, "{}", amph.map_or ('.', Amph::letter)) ?;
		}
		for room in [& self.amber, & self.bronze, & self.copper, & self.desert] {
			write! (formatter, "/") ?;
			for amph in iter::repeat (None).take (self.room_size - room.len ())
					.chain (room.iter ().copied ().rev ().map (Some)) {
				write! (formatter, "{}", amph.map_or ('.', Amph::letter)) ?;
			}
		}
		Ok (())
	}
}

impl FromStr for State {

	type Err = ();

	fn from_str (input: & str) -> Result <Self, ()> {
		let num_chars = input.chars ().count ();
		if ! [23, 27, 31].contains (& num_chars) { return Err (()) }
		let room_size = (num_chars - 11) / 5;
		let mut places = [None; 27];
		let mut place_idx = 0;
		for (char_idx, letter) in input.chars ().enumerate () {
			if 11 <= char_idx && (char_idx - 11) % (room_size + 1) == 0 {
				if letter != '/' { return Err (()) }
				if 11 < char_idx { place_idx += 4 - room_size; }
				continue;
			}
			places [place_idx] = match Amph::from_letter (letter) {
				Some (amph) => amph,
				None => return Err (()),
			};
			place_idx += 1;
		}
		Ok (Self::from_array (room_size, places))
	}

}

#[ derive (Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd) ]
pub struct StateCompact {
	data: u64,
}

impl StateCompact {

	#[ must_use ]
	pub fn expand (self) -> State {
		let mut present_bits = (self.data & 0x_07ff_ffff_0000_0000) >> 32_i32;
		let mut amph_bits = self.data & 0x_0000_0000_ffff_ffff;
		let mut places = [None; 27];
		for place_idx in (0 .. 27).rev () {
			if present_bits & 1 != 0 {
				match amph_bits & 0x3 {
					0 => places [place_idx] = Some (Amber),
					1 => places [place_idx] = Some (Bronze),
					2 => places [place_idx] = Some (Copper),
					3 => places [place_idx] = Some (Desert),
					_ => unreachable! (),
				}
				amph_bits >>= 2_i32;
			}
			present_bits >>= 1_i32;
		}
		let room_size = (self.data >> 59_i32).pan_usize ();
		State::from_array (room_size, places)
	}

	#[ must_use ]
	pub fn is_finished (self) -> bool {
		let mut present_bits = (self.data & 0x_07ff_ffff_0000_0000) >> 32_i32;
		if present_bits & 0x_07ff_0000 != 0 { return false }
		let mut amph_bits = self.data & 0x_0000_0000_ffff_ffff;
		for idx in (0 .. 4).rev () {
			for _ in 0_i32 .. 4_i32 {
				if present_bits & 1 != 0 {
					if amph_bits & 0x3 != idx { return false }
					amph_bits >>= 2_i32;
				}
				present_bits >>= 1_i32;
			}
		}
		true
	}

}

#[ derive (Clone, Copy, Debug, Eq, Hash, PartialEq) ]
pub enum Place {
	Hall (u8),
	Room (Amph, u8),
}

impl Place {

	#[ must_use ]
	pub fn idx (self) -> usize {
		match self {
			Hall (id) => id.pan_usize (),
			Room (Amber, depth) => 11 + depth.pan_usize (),
			Room (Bronze, depth) => 15 + depth.pan_usize (),
			Room (Copper, depth) => 19 + depth.pan_usize (),
			Room (Desert, depth) => 23 + depth.pan_usize (),
		}
	}

	#[ must_use ]
	pub fn for_idx (idx: usize) -> Self {
		match idx {
			0 ..= 10 => Hall (idx.pan_u8 ()),
			11 ..= 14 => Room (Amber, idx.pan_u8 () - 11),
			15 ..= 18 => Room (Bronze, idx.pan_u8 () - 15),
			19 ..= 22 => Room (Copper, idx.pan_u8 () - 19),
			23 ..= 26 => Room (Desert, idx.pan_u8 () - 23),
			_ => panic! ("Invalid index: {}", idx),
		}
	}

	#[ must_use ]
	pub fn entrance (self) -> bool {
		[2, 4, 6, 8].contains (& self.idx ())
	}

}

parse_display_enum! {
	#[ derive (Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd) ]
	pub enum Amph { Amber = "A", Bronze = "B", Copper = "C", Desert = "D" }
}

impl Amph {

	#[ allow (clippy::option_option) ]
	#[ must_use ]
	pub const fn from_letter (letter: char) -> Option <Option <Self>> {
		match letter {
			'A' => Some (Some (Amber)),
			'B' => Some (Some (Bronze)),
			'C' => Some (Some (Copper)),
			'D' => Some (Some (Desert)),
			'.' => Some (None),
			_ => None,
		}
	}

	#[ must_use ]
	pub const fn cost (self) -> i64 {
		Self::COSTS [self.idx ()]
	}

	#[ must_use ]
	pub const fn letter (self) -> char {
		Self::LETTERS [self.idx ()]
	}

	const COSTS: & 'static [i64; 4] = & [1, 10, 100, 1000];
	const LETTERS: & 'static [char; 4] = & ['A', 'B', 'C', 'D'];

}

#[ cfg (test) ]
mod tests {

	use super::*;

	#[ test ]
	fn test_state_finished () {
		assert! (State::from_str (".........../AA/BB/CC/DD").unwrap ().is_finished ());
		assert! (! State::from_str ("A........../.A/BB/CC/DD").unwrap ().is_finished ());
	}

}
