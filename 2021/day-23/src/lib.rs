//! Advent of Code 2021: Day 23: Amphipod
//!
//! [https://adventofcode.com/2021/day/23](https://adventofcode.com/2021/day/23)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;
use aoc_search as search;

puzzle_info! {
	name = "Amphipod";
	year = 2021;
	day = 23;
	part_one = |lines| logic::part_one (lines);
	part_two = |lines| logic::part_two (lines);
	commands = [
		( name = "run"; method = tools::run; ),
		( name = "internals"; method = tools::internals; ),
	];
}

mod logic {

	use super::*;
	use model::Amph;
	use model::Place;
	use model::State;
	use model::StateCompact;
	use search::PrioritySearch;
	use search::PrioritySearchAdder;

	pub fn part_one (lines: & [& str]) -> GenResult <i64> {
		let input = State::parse (lines) ?;
		calc_result (& input)
	}

	pub fn part_two (lines: & [& str]) -> GenResult <i64> {
		let lines_modified = modify_input_for_part_two (lines);
		let input = State::parse (& lines_modified) ?;
		calc_result (& input)
	}

	pub fn modify_input_for_part_two <'lin> (lines: & [& 'lin str]) -> Vec <& 'lin str> {
		vec! [
			lines [0],
			lines [1],
			lines [2],
			"  #D#C#B#A#",
			"  #D#B#A#C#",
			lines [3],
			lines [4],
		]
	}

	pub fn calc_result (input: & State) -> GenResult <i64> {
		Ok (
			iterator (input)
				.filter (|& (ref state_compact, _)| state_compact.is_finished ())
				.map (|(_, score)| score)
				.next ()
				.ok_or ("Failed to find solution") ?
		)
	}

	pub fn iterator (input: & State) -> impl Iterator <Item = (StateCompact, i64)> {
		let mut search = PrioritySearch::with_hash_map (
			|state: StateCompact, score: i64, mut adder: PrioritySearchAdder <'_, _, _, _>| {
				for (next_state, next_cost) in calc_next_states (state) {
					let next_score = score + next_cost;
					adder.add (next_state, next_score);
				}
				(state, score)
			},
		);
		search.push (input.compact (), 0);
		search
	}

	pub fn calc_next_states (state_compact: StateCompact) -> ArrayVec <(StateCompact, i64), 28> {
		let state = state_compact.expand ();

		let out_cost = |room| state.room_size () - state.room (room).len () + 1;
		let in_cost = |room| state.room_size () - state.room (room).len ();
		let hall_cost = |room: Amph, hall: Place|
			usize::abs_diff (2 + room.idx () * 2, hall.idx ());

		let next_moves = calc_next_moves (& state);
		if next_moves.is_empty () { return ArrayVec::new () }

		let blocking = (state.hall () [3], state.hall () [5], state.hall () [7]);
		use Amph::{ Amber, Bronze, Copper, Desert };
		let sections = [
			! matches! (blocking,
				(Some (Copper | Desert), _, _) |
				(_, Some (Bronze | Copper | Desert), _) |
				(_, _, Some (Desert))),
			! matches! (blocking,
				(Some (Amber), _, _) |
				(_, Some (Copper | Desert), _) |
				(_, _, Some (Desert))),
			! matches! (blocking,
				(Some (Amber), _, _) |
				(_, Some (Amber | Bronze), _) |
				(_, _, Some (Desert))),
			! matches! (blocking,
				(Some (Amber), _, _) |
				(_, Some (Amber | Bronze), _) |
				(_, _, Some (Amber | Bronze | Copper))),
		];

		let mut next_states = ArrayVec::new ();
		for next_move in next_moves.iter ().copied () {
			match next_move {
				Move::Between (amph, from_room, to_room) => {
					if ! sections [from_room.idx ()] || ! sections [to_room.idx ()] { continue }
					let cost = amph.cost () * (out_cost (from_room) + in_cost (to_room)
						+ usize::abs_diff (from_room.idx (), to_room.idx ()) * 2).pan_i64 ();
					let next_state = state.move_between (from_room, to_room);
					return iter::once ((next_state.compact (), cost)).collect ();
				},
				Move::In (amph, from_hall, to_room) => {
					if ! sections [to_room.idx ()] { continue }
					let cost = amph.cost () * (in_cost (to_room) + hall_cost (to_room, from_hall)).pan_i64 ();
					let next_state = state.move_in (from_hall, to_room);
					return iter::once ((next_state.compact (), cost)).collect ();
				},
				Move::Out (..) => (),
			}
		}
		for next_move in next_moves.iter ().copied () {
			if let Move::Out (amph, from_room, to_hall) = next_move {
				if ! sections [from_room.idx ()] { continue }
				let cost = amph.cost () * (out_cost (from_room)
					+ hall_cost (from_room, to_hall)).pan_i64 ();
				let next_state = state.move_out (from_room, to_hall);
				next_states.push ((next_state.compact (), cost));
			}
		}

		next_states

	}

	#[ derive (Clone, Copy) ]
	pub enum Move {
		Out (Amph, Amph, Place),
		In (Amph, Place, Amph),
		Between (Amph, Amph, Amph),
	}

	pub fn calc_next_moves (state: & State) -> ArrayVec <Move, 28> {
		let mut result = ArrayVec::new ();
		let path_clear = |from: Place, to: Place|
			state.hall ().iter ().enumerate ()
				.skip (cmp::min (to.idx (), from.idx ()))
				.take (usize::abs_diff (from.idx (), to.idx ()) + 1)
				.map (|(idx, amph)| (Place::for_idx (idx), amph))
				.filter (|& (hall, _)| hall != from)
				.all (|(_, amph)| amph.is_none ());
		let room_entrance = |room: Amph| Place::for_idx (2 + room.idx () * 2);
		for (idx, amph) in state.hall ().iter ().enumerate ()
				.filter_map (|(idx, amph)| amph.map (|amph| (idx, amph))) {
			let to_room = amph;
			let hall = Place::for_idx (idx);
			if ! state.room_is_happy (to_room) { continue }
			if ! path_clear (hall, room_entrance (to_room)) { continue }
			result.clear ();
			result.push (Move::In (amph, hall, to_room));
			return result;
		}
		for (from_room, amphs) in [
			(Amph::Amber, state.room (Amph::Amber)),
			(Amph::Bronze, state.room (Amph::Bronze)),
			(Amph::Copper, state.room (Amph::Copper)),
			(Amph::Desert, state.room (Amph::Desert)),
		] {
			if let Some (& amph) = amphs.last () {
				let to_room = amph;
				if state.room_is_happy (from_room) { continue }
				if state.room_is_happy (to_room) {
					if ! path_clear (room_entrance (from_room), room_entrance (to_room)) { continue }
					result.clear ();
					result.push (Move::Between (amph, from_room, to_room));
					return result;
				}
				for hall in
					iter::successors (
							Some (room_entrance (from_room)),
							|prev_hall| (prev_hall.idx () > 0).then (||
								Place::for_idx (prev_hall.idx () - 1)))
						.take_while (|& hall| state.get (hall).is_none ())
						.chain (
							iter::successors (
									Some (room_entrance (from_room)),
									|prev_hall| (prev_hall.idx () + 1 < 11).then_some (
										Place::for_idx (prev_hall.idx () + 1)))
								.take_while (|& hall| state.get (hall).is_none ()))
						.filter (|hall| ! hall.entrance ()) {
					if ! path_clear (room_entrance (from_room), hall) { continue }
					result.push (Move::Out (amph, from_room, hall));
				}
			}
		}
		result
	}

}

mod model {

	use aoc_common::*;

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
			for room in Amph::ALL.iter ().copied () {
				state.room_mut (room).extend (
					places [11 + room.idx () * 4 .. ].iter ().copied ().take (room_size).rev ()
						.while_some ());
			}
			state
		}

		pub fn as_array (& self) -> [Option <Amph>; 27] {
			let mut result = [None; 27];
			#[ allow (clippy::needless_range_loop) ]
			for idx in 0 .. 11 { result [idx] = self.hall [idx]; }
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

		pub const fn room_size (& self) -> usize { self.room_size }

		pub fn get (& self, place: Place) -> Option <Amph> {
			match place {
				Place::Hall (id) => self.hall [id.pan_usize ()],
				Place::Room (Amph::Amber, depth) =>
					self.amber.get (self.room_size - depth.pan_usize () - 1).copied (),
				Place::Room (Amph::Bronze, depth) =>
					self.bronze.get (self.room_size - depth.pan_usize () - 1).copied (),
				Place::Room (Amph::Copper, depth) =>
					self.copper.get (self.room_size - depth.pan_usize () - 1).copied (),
				Place::Room (Amph::Desert, depth) =>
					self.desert.get (self.room_size - depth.pan_usize () - 1).copied (),
			}
		}

		pub fn hall (& self) -> & [Option <Amph>] { & self.hall }

		pub fn room (& self, amph: Amph) -> & [Amph] {
			match amph {
				Amph::Amber => & self.amber,
				Amph::Bronze => & self.bronze,
				Amph::Copper => & self.copper,
				Amph::Desert => & self.desert,
			}
		}

		fn room_mut (& mut self, amph: Amph) -> & mut ArrayVec <Amph, 4> {
			match amph {
				Amph::Amber => & mut self.amber,
				Amph::Bronze => & mut self.bronze,
				Amph::Copper => & mut self.copper,
				Amph::Desert => & mut self.desert,
			}
		}

		pub fn is_finished (& self) -> bool {
			self.hall.iter ().all (Option::is_none)
				&& self.amber.iter ().all (|& amph| amph == Amph::Amber)
				&& self.bronze.iter ().all (|& amph| amph == Amph::Bronze)
				&& self.copper.iter ().all (|& amph| amph == Amph::Copper)
				&& self.desert.iter ().all (|& amph| amph == Amph::Desert)
		}

		pub fn room_is_happy (& self, room: Amph) -> bool {
			self.room (room).iter ().copied ().all (|amph| amph == room)
		}

		pub fn move_out (& self, room: Amph, hall: Place) -> Self {
			let mut state = self.clone ();
			let amph = state.room_mut (room).pop ().unwrap ();
			assert! (state.get (hall).is_none ());
			state.hall [hall.idx ()] = Some (amph);
			state
		}

		pub fn move_in (& self, hall: Place, room: Amph) -> Self {
			let mut state = self.clone ();
			let amph = state.hall [hall.idx ()].take ().unwrap ();
			state.room_mut (room).push (amph);
			state
		}

		pub fn move_between (& self, from: Amph, to: Amph) -> Self {
			let mut state = self.clone ();
			let amph = state.room_mut (from).pop ().unwrap ();
			let to = match to {
				Amph::Amber => & mut state.amber, Amph::Bronze => & mut state.bronze,
				Amph::Copper => & mut state.copper, Amph::Desert => & mut state.desert,
			};
			assert! (to.len () < self.room_size);
			to.push (amph);
			state
		}

		pub fn pretty_line (& self, line: usize) -> String {
			let print_amph = |amph: Option <Amph>| amph.map_or (' ', Amph::letter);
			if line == 0 {
				"┌───────────┐".to_owned ()
			} else if line == 1 {
				format! ("│{}│",
					self.hall.iter ().copied ().map (print_amph).collect::<String> ())
			} else if line == 2 {
				format! ("└─┐{}╷{}╷{}╷{}┌─┘",
					print_amph (self.get (Place::Room (Amph::Amber, 0))),
					print_amph (self.get (Place::Room (Amph::Bronze, 0))),
					print_amph (self.get (Place::Room (Amph::Copper, 0))),
					print_amph (self.get (Place::Room (Amph::Desert, 0))))
			} else if line < self.room_size.pan_usize () + 2 {
				let depth = (line - 2).pan_u8 ();
				format! ("  │{}│{}│{}│{}│",
					print_amph (self.get (Place::Room (Amph::Amber, depth))),
					print_amph (self.get (Place::Room (Amph::Bronze, depth))),
					print_amph (self.get (Place::Room (Amph::Copper, depth))),
					print_amph (self.get (Place::Room (Amph::Desert, depth))))
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

		#[ allow (dead_code) ]
		pub fn from_str (input: & str) -> Option <Self> {
			let num_chars = input.chars ().count ();
			if ! [23, 27, 31].contains (& num_chars) { return None }
			let room_size = (num_chars - 11) / 5;
			let mut places = [None; 27];
			let mut place_idx = 0;
			for (char_idx, letter) in input.chars ().enumerate () {
				if 11 <= char_idx && (char_idx - 11) % (room_size + 1) == 0 {
					if letter != '/' { return None }
					if 11 < char_idx { place_idx += 4 - room_size; }
					continue;
				}
				places [place_idx] = match Amph::from_letter (letter) {
					Some (amph) => amph,
					None => return None,
				};
				place_idx += 1;
			}
			Some (Self::from_array (room_size, places))
		}

		pub fn parse (lines: & [& str]) -> GenResult <Self> {
			let err = |line_idx| format! ("Invalid input: {}: {}", line_idx, lines [line_idx]);
			if lines.len () < 5 || lines.len () > 7 { Err (err (5)) ?; }
			if lines [0] != "#############" { Err (err (0)) ?; }
			if lines [1] != "#...........#" { Err (err (1)) ?; }
			let mut places = [None; 27];
			for line_idx in 0 .. lines.len () - 1 {
				Parser::wrap (lines [line_idx], |parser| {
					fn parse_amph (parser: & mut Parser) -> ParseResult <Option <Amph>> {
						Amph::from_letter (parser.expect_next () ?)
							.ok_or_else (|| parser.err ())
					}
					if line_idx == 0 {
						parser.expect ("#############") ?.end () ?;
					} else if line_idx == 1 {
						parser.expect ("#") ?;
						#[ allow (clippy::needless_range_loop) ]
						for idx in 0 .. 11 {
							places [idx] = parse_amph (parser) ?;
						}
						parser.expect ("#") ?.end () ?;
					} else if line_idx < lines.len () - 1 {
						if line_idx == 2 {
							parser.expect ("###") ?;
						} else {
							parser.expect ("  #") ?;
						}
						places [11 + (line_idx - 2)] = parse_amph (parser) ?;
						parser.expect ("#") ?;
						places [15 + (line_idx - 2)] = parse_amph (parser) ?;
						parser.expect ("#") ?;
						places [19 + (line_idx - 2)] = parse_amph (parser) ?;
						parser.expect ("#") ?;
						places [23 + (line_idx - 2)] = parse_amph (parser) ?;
						if line_idx == 2 {
							parser.expect ("###") ?;
						} else {
							parser.expect ("#") ?;
						}
						parser.end () ?;
					} else {
						parser.expect ("  ##########") ?.end () ?;
					}
					Ok (())
				}) ?;
			}
			let room_size = lines.len () - 3;
			Ok (Self::from_array (room_size, places))
		}

		pub fn compact (& self) -> StateCompact {
			let mut place_bits: u64 = 0;
			let mut amph_bits: u64 = 0;
			for amph in self.as_array () {
				place_bits <<= 1_u64;
				if let Some (amph) = amph {
					place_bits |= 1;
					amph_bits <<= 2_i32;
					amph_bits |= match amph {
						Amph::Amber => 0, Amph::Bronze => 1,
						Amph::Copper => 2, Amph::Desert => 3,
					}
				}
			}
			assert! (64 - place_bits.leading_zeros () <= 27);
			assert! (64 - amph_bits.leading_zeros () <= 32);
			let data = ((self.room_size.pan_u64 ()) << 59_i32) | (place_bits << 32_i32) | amph_bits;
			StateCompact { data }
		}

	}

	impl fmt::Display for State {
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

	#[ derive (Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd) ]
	pub struct StateCompact {
		data: u64,
	}

	impl StateCompact {
		pub fn expand (self) -> State {
			let mut present_bits = (self.data & 0x_07ff_ffff_0000_0000) >> 32_i32;
			let mut amph_bits = self.data & 0x_0000_0000_ffff_ffff;
			let mut places = [None; 27];
			for place_idx in (0 .. 27).rev () {
				if present_bits & 1 != 0 {
					match amph_bits & 0x3 {
						0 => places [place_idx] = Some (Amph::Amber),
						1 => places [place_idx] = Some (Amph::Bronze),
						2 => places [place_idx] = Some (Amph::Copper),
						3 => places [place_idx] = Some (Amph::Desert),
						_ => unreachable! (),
					}
					amph_bits >>= 2_i32;
				}
				present_bits >>= 1_i32;
			}
			let room_size = (self.data >> 59_i32).pan_usize ();
			State::from_array (room_size, places)
		}
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
		pub fn idx (self) -> usize {
			match self {
				Self::Hall (id) => id.pan_usize (),
				Self::Room (Amph::Amber, depth) => 11 + depth.pan_usize (),
				Self::Room (Amph::Bronze, depth) => 15 + depth.pan_usize (),
				Self::Room (Amph::Copper, depth) => 19 + depth.pan_usize (),
				Self::Room (Amph::Desert, depth) => 23 + depth.pan_usize (),
			}
		}
		pub fn for_idx (idx: usize) -> Self {
			match idx {
				0 ..= 10 => Self::Hall (idx.pan_u8 ()),
				11 ..= 14 => Self::Room (Amph::Amber, idx.pan_u8 () - 11),
				15 ..= 18 => Self::Room (Amph::Bronze, idx.pan_u8 () - 15),
				19 ..= 22 => Self::Room (Amph::Copper, idx.pan_u8 () - 19),
				23 ..= 26 => Self::Room (Amph::Desert, idx.pan_u8 () - 23),
				_ => panic! ("Invalid index: {}", idx),
			}
		}
		pub fn entrance (self) -> bool { [2, 4, 6, 8].contains (& self.idx ()) }
	}

	#[ derive (Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd) ]
	pub enum Amph { Amber, Bronze, Copper, Desert }

	impl Amph {
		pub const fn idx (self) -> usize {
			match self {
				Self::Amber => 0,
				Self::Bronze => 1,
				Self::Copper => 2,
				Self::Desert => 3,
			}
		}
		#[ allow (clippy::option_option) ]
		pub const fn from_letter (letter: char) -> Option <Option <Self>> {
			match letter {
				'A' => Some (Some (Self::Amber)),
				'B' => Some (Some (Self::Bronze)),
				'C' => Some (Some (Self::Copper)),
				'D' => Some (Some (Self::Desert)),
				'.' => Some (None), _ => None,
			}
		}
		pub const fn cost (self) -> i64 { Self::COSTS [self.idx ()] }
		pub const fn letter (self) -> char { Self::LETTERS [self.idx ()] }
		const COSTS: & 'static [i64; 4] = & [1, 10, 100, 1000];
		const LETTERS: & 'static [char; 4] = & ['A', 'B', 'C', 'D'];
		pub const ALL: & 'static [Self] = & [
			Self::Amber, Self::Bronze, Self::Copper, Self::Desert
		];
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

}

pub mod tools {

	use super::*;
	use model::State;
	use model::StateCompact;
	use search::PrioritySearch;
	use search::PrioritySearchAdder;

	#[ derive (Debug, clap::Parser) ]
	#[ allow (clippy::struct_excessive_bools) ]
	pub struct RunArgs {

		#[ clap (long, default_value ("inputs/day-23")) ]
		input: String,

		#[ clap (long) ]
		verbose: bool,

		#[ clap (long) ]
		dead_ends: bool,

		#[ clap (long) ]
		part_1: bool,

		#[ clap (long) ]
		part_2: bool,

	}

	#[ allow (clippy::needless_pass_by_value) ]
	pub fn run (args: RunArgs) -> GenResult <()> {
		let mut args = args;
		if ! (args.part_1 || args.part_2) { args.part_1 = true; args.part_2 = true; }
		let input_string = fs::read_to_string (& args.input) ?;
		let input_lines: Vec <_> = input_string.trim ().split ('\n').collect ();
		if args.part_1 {
			run_part (& args, & input_lines) ?;
		}
		if args.part_2 {
			let input_lines_modified = logic::modify_input_for_part_two (& input_lines);
			run_part (& args, & input_lines_modified) ?;
		}
		Ok (())
	}

	#[ allow (clippy::print_stdout) ]
	pub fn run_part (args: & RunArgs, lines: & [& str]) -> GenResult <()> {
		let input = State::parse (lines) ?;
		let mut num_loops = 0_i32;
		let mut last_cost = -1_i64;
		let mut prev_states = HashMap::new ();
		let mut search = PrioritySearch::with_hash_map (
			|state_compact, score, mut adder: PrioritySearchAdder <_, _, _>| {
				let next_states_compact = logic::calc_next_states (state_compact);
				for (next_state_compact, next_cost) in next_states_compact.iter ().copied () {
					let next_score = score + next_cost;
					adder.add (next_state_compact, next_score);
				}
				(state_compact, score, next_states_compact)
			},
		);
		search.push (input.compact (), 0);
		let final_cost = loop {
			let (state_compact, cost, next_states_compact) = match search.next () {
				Some (val) => val,
				None => break None,
			};
			num_loops += 1_i32;
			let state = state_compact.expand ();
			if state.is_finished () {
				break Some ((state_compact, cost));
			}
			if args.verbose {
				let next_states: Vec <_> =
					next_states_compact.iter ().copied ()
						.map (|(state_compact, cost)| (state_compact.expand (), cost))
						.sorted_by_key (|& (_, cost)| cost)
						.collect ();
				if cost != last_cost {
					println! ();
					println! ("Evaluating states with cost: {}", cost);
					println! ("Number of iterations: {}", num_loops);
					println! ("Size of backlog: {}", search.len ());
				}
				println! ();
				if next_states.is_empty () && args.dead_ends {
					let all_states =
						iter::successors (
								Some (state_compact),
								|state| prev_states.get (state).copied ())
							.map (StateCompact::expand)
							.collect::<Vec <_>> ();
					println! ("  ▒▒▒▒  Dead end:");
					for chunk in all_states.chunks (11) {
						for line in 0 .. state.room_size () + 3 {
							print! ("  ▒▒▒▒  ");
							for (idx, state) in chunk.iter ().enumerate () {
								if idx > 0 { print! (" "); }
								print! ("{:13}", state.pretty_line (line));
							}
							print! ("\n");
						}
					}
				} else {
					print_next_states (& state, & next_states);
				}
			}
			for (next_state_compact, _) in next_states_compact {
				prev_states.insert (next_state_compact, state_compact);
			}
			last_cost = cost;
		};
		let (final_state_compact, final_cost) =
			final_cost.ok_or ("Failed to find a solution") ?;
		let final_state = final_state_compact.expand ();
		let mut all_states =
			iter::successors (
					Some (final_state_compact),
					|state| prev_states.get (state).copied ())
				.map (StateCompact::expand)
				.collect::<Vec <_>> ();
		all_states.reverse ();
		if args.verbose {
			println! ();
			println! ("═══════════════════════════ Found solution ═══════════════════════════");
			println! ();
		}
		println! ("Solved with cost: {}", final_cost);
		println! ("Number of steps in solution: {}", all_states.len () - 1);
		println! ();
		println! ("Number of iterations: {}", num_loops);
		println! ("Total states genereated: {}", prev_states.len ());
		if args.verbose { println! (); }
		for chunk in all_states.chunks (11) {
			for line in 0 .. final_state.room_size () + 3 {
				for (idx, state) in chunk.iter ().enumerate () {
					if idx > 0 { print! (" "); }
					print! ("{:13}", state.pretty_line (line));
				}
				print! ("\n");
			}
		}
		if args.verbose { println! (); }
		Ok (())
	}

	#[ allow (clippy::print_stdout) ]
	pub fn print_next_states (cur_state: & State, next_states: & [(State, i64)]) {
		if next_states.is_empty () {
			println! ("{:^13}", "START");
			for line in 0 .. cur_state.room_size () + 3 {
				print! ("{:13}", cur_state.pretty_line (line));
				if line == (cur_state.room_size () + 3) / 2 {
					print! ("   (dead end)");
				}
				print! ("\n");
			}
			return;
		}
		for (chunk_idx, chunk) in next_states.chunks (10).enumerate () {
			print! ("{:^13}  ", if chunk_idx == 0 { "START" } else { "" });
			for & (_, ref cost) in chunk.iter () {
				print! (" {:^13}", cost);
			}
			print! ("\n");
			for line in 0 .. (cur_state.room_size ().pan_usize () + 3) {
				print! ("{:13}  ", if chunk_idx == 0 { cur_state.pretty_line (line) } else { String::new () });
				for & (ref next_state, _) in chunk.iter () {
					print! (" {:13}", next_state.pretty_line (line));
				}
				print! ("\n");
			}
		}
	}

	#[ derive (Debug, clap::Parser) ]
	pub struct InternalsArgs;

	#[ allow (clippy::needless_pass_by_value) ]
	#[ allow (clippy::print_stdout) ]
	pub fn internals (_args: InternalsArgs) -> GenResult <()> {
		println! ("Data structures:");
		fn show_struct <Type> () {
			let name = std::any::type_name::<Type> ();
			let size = mem::size_of::<Type> ();
			let align = mem::align_of::<Type> ();
			println! (" - {} {} bytes (align = {})", name, size, align);
		}
		show_struct::<logic::Move> ();
		show_struct::<model::Amph> ();
		show_struct::<model::Place> ();
		show_struct::<model::State> ();
		show_struct::<model::StateCompact> ();
		Ok (())
	}

}

#[ cfg (test) ]
mod examples {

	use super::*;

	const EXAMPLE: & 'static [& 'static str] = & [
		"#############",
		"#...........#",
		"###B#C#B#D###",
		"  #A#D#C#A#",
		"  #########",
	];

	#[ test ]
	fn part_one () -> GenResult <()> {
		assert_eq! (12521, logic::part_one (EXAMPLE) ?);
		Ok (())
	}

	#[ test ]
	fn part_two () -> GenResult <()> {
		assert_eq! (44169, logic::part_two (EXAMPLE) ?);
		Ok (())
	}

}
