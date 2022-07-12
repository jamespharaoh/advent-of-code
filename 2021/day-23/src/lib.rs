use aoc_common::*;

puzzle_info! {
	name = "Amphipod";
	year = 2021;
	day = 23;
	part_one = |lines| logic::part_one (lines);
	part_two = |lines| logic::part_two (lines);
}

mod logic {

	use super::*;
	use model::Amph;
	use model::Place;
	use model::PlaceAdjacent;
	use model::State;
	use model::StateCompact;

	pub fn part_one (lines: & [& str]) -> GenResult <i64> {
		let input = State::parse (lines) ?;
		calc_result (input)
	}

	pub fn part_two (lines: & [& str]) -> GenResult <i64> {
		let lines_modified: Vec <& str> = vec! [
			lines [0],
			lines [1],
			lines [2],
			"  #D#C#B#A#",
			"  #D#B#A#C#",
			lines [3],
			lines [4],
		];
		let input = State::parse (& lines_modified) ?;
		calc_result (input)
	}

	pub fn calc_result (input: State) -> GenResult <i64> {

		let mut search = search::PrioritySearch::new (
			|& state_compact: & StateCompact, & score, mut adder| {
				let state = state_compact.expand ();
				for (next_state, next_cost) in calc_next_states (& state) {
					let next_compact = next_state.compact ();
					let next_score = score + next_cost;
					adder.add (next_compact, next_score);
				}
			},
		);

		search.push (input.compact (), 0);

		let final_score = search
			.filter (|(state_compact, _)| state_compact.expand ().is_finished ())
			.map (|(_, score)| score)
			.next ();

		let final_score = final_score.ok_or (format! ("Failed to find solution")) ?;
		Ok (final_score)

	}

	#[ allow (dead_code) ]
	fn print_next_states (cur_state: & State, next_states: & [(State, i64)]) {
		for (chunk_idx, chunk) in next_states.chunks (10).enumerate () {
			print! ("{:^13}  ", if chunk_idx == 0 { "START" } else { "" });
			for (_, cost) in chunk.iter () {
				print! (" {:^13}", cost);
			}
			print! ("\n");
			for line in 0 .. (cur_state.room_size () as usize + 3) {
				print! ("{:13}  ", if chunk_idx == 0 { cur_state.pretty_line (line) } else { String::new () });
				for (next_state, _) in chunk.iter () {
					print! (" {:13}", next_state.pretty_line (line));
				}
				print! ("\n");
			}
		}
	}

	fn calc_next_states (state: & State) -> ArrayVec <(State, i64), 28> {
		let mut next_states = ArrayVec::new ();
		for (cur_place, amph) in state.iter () {
			for (next_place, dist) in calc_routes (& state, cur_place) {
				if ! allowed_move (state, cur_place, next_place) { continue }
				let next_state = state.with_move (cur_place, next_place);
				if ! allowed_state (next_state) { continue }
				next_states.push ((
					state.with_move (cur_place, next_place),
					amph.cost () * dist,
				));
			}
		}
		next_states
	}

	#[ allow (dead_code) ]
	struct CalcStatesIter <'a> {
		cur_state: & 'a State,
		state_iter: Option <SliceIter <'a, (Place, Amph)>>,
		cur_place: Option <Place>,
		routes_iter: Option <CalcRoutesIter <'a>>,
	}

	fn allowed_move (state: & State, from: Place, to: Place) -> bool {

		let amph = state.get (from).unwrap ();

		let misplaced_in_my_room = || state.room_for (amph).rev ().map (
			|some_place| state.get (some_place),
		).take_while (Option::is_some).map (Option::unwrap).any (
			|some_amph| some_amph != amph,
		);

		// don't move in the hallway
		if from.hall () && to.hall () { return false }

		// don't stop at room entrance
		if to.entrance () { return false }

		// don't move inside a room
		if from.room () && to.room () && from.amph () == to.amph () { return false }

		// when entering a room...
		if to.room () && from.amph () != to.amph () {

			// don't enter the wrong one
			if ! amph.belongs (to) { return false }

			// only enter if there are no mismatched amphs
			if misplaced_in_my_room () { return false }

			// don't leave space at the back of a room
			let my_room: ArrayVec <Place, 4> = state.room_for (amph).collect ();
			let my_depth = my_room.iter ().copied ().position (|room| room == to).unwrap ();
			if let Some (& next_room) = my_room.get (my_depth + 1) {
				if state.get (next_room).is_none () { return false }
			}

		}

		// don't leave own room unless you need to get out of the way
		if from.room () && from.amph () != to.amph ()
			&& from.amph () == Some (amph) && ! misplaced_in_my_room () { return false }

		true

	}

	fn allowed_state (state: State) -> bool {
		match (
			state.get (Place::hall_blocking_left ()),
			state.get (Place::hall_blocking_middle ()),
			state.get (Place::hall_blocking_right ()),
		) {
			(Some (Amph::Desert), Some (Amph::Amber), _) => false,
			(Some (Amph::Desert), _, Some (Amph::Amber)) => false,
			(_, Some (Amph::Desert), Some (Amph::Amber)) => false,
			(_, Some (Amph::Desert), Some (Amph::Bronze)) => false,
			(Some (Amph::Copper), Some (Amph::Amber), _) => false,
			_ => true,
		}
	}

	fn calc_routes <'a> (state: & 'a State, start_place: Place) -> CalcRoutesIter {
		let mut todo = ArrayVec::new ();
		todo.push ((0, start_place));
		CalcRoutesIter {
			cur_state: state,
			cur_route: ArrayVec::new (),
			todo,
			state: CalcRoutesIterState::Outer,
		}
	}

	struct CalcRoutesIter <'a> {
		cur_state: & 'a State,
		cur_route: ArrayVec <Place, 12>,
		todo: ArrayVec <(usize, Place), 4>,
		state: CalcRoutesIterState,
	}

	enum CalcRoutesIterState {
		Outer,
		Inner { cur_place: Place, adj_iter: PlaceAdjacent },
		Complete,
		Poison,
	}

	impl <'a> Iterator for CalcRoutesIter <'a> {
		type Item = (Place, i64);
		fn next (& mut self) -> Option <(Place, i64)> {
			loop { match mem::replace (& mut self.state, CalcRoutesIterState::Poison) {
				CalcRoutesIterState::Outer => {
					if let Some ((todo_len, todo_place)) = self.todo.pop () {
						self.cur_route.truncate (todo_len);
						self.cur_route.push (todo_place);
						self.state = CalcRoutesIterState::Inner {
							cur_place: todo_place,
							adj_iter: todo_place.adjacent (),
						};
						continue;
					} else {
						self.state = CalcRoutesIterState::Complete;
						return None;
					}
				},
				CalcRoutesIterState::Inner { cur_place, mut adj_iter } => {
					if let Some (adj_place) = adj_iter.next () {
						if ! self.cur_state.valid_place (adj_place)
								|| self.cur_route.contains (& adj_place)
								|| self.cur_state.get (adj_place).is_some () {
							self.state = CalcRoutesIterState::Inner { cur_place, adj_iter };
							continue;
						}
						self.todo.push ((self.cur_route.len (), adj_place));
						self.state = CalcRoutesIterState::Inner { cur_place, adj_iter };
						return Some ((adj_place, self.cur_route.len () as i64));
					} else {
						self.state = CalcRoutesIterState::Outer;
						continue;
					}
				},
				CalcRoutesIterState::Complete => return None,
				CalcRoutesIterState::Poison => panic! (),
			} }
		}
	}

}

mod model {

	use aoc_common::*;

	#[ derive (Clone, Debug, Eq, PartialEq) ]
	pub struct State {
		room_size: u8,
		places: [Option <Amph>; 27],
	}

	impl State {

		pub fn new (room_size: u8, places: [Option <Amph>; 27]) -> State {
			State { room_size, places }
		}

		pub fn room_size (& self) -> u8 { self.room_size }

		pub fn get (& self, place: Place) -> Option <Amph> {
			self.places [place.idx () as usize]
		}

		pub fn iter (& self) -> impl Iterator <Item = (Place, Amph)> {
			self.places.into_iter ().enumerate ().filter_map (
				|(idx, amph)| amph.map (|amph| (Place { id: idx as u8 }, amph)),
			)
		}

		pub fn is_finished (& self) -> bool {
			self.iter ().all (|(place, amph)| amph.belongs (place))
		}

		pub fn room_for (& self, amph: Amph) -> impl DoubleEndedIterator <Item = Place> {
			let offset = match amph {
				Amph::Amber => 11, Amph::Bronze => 12,
				Amph::Copper => 13, Amph::Desert => 14,
			};
			(0 .. self.room_size).map (move |idx| Place { id: offset + idx * 4 })
		}

		pub fn valid_place (& self, place: Place) -> bool {
			place.hall () || place.room_depth () < self.room_size
		}

		pub fn with_move (& self, from: Place, to: Place) -> State {
			let mut places = self.places;
			let amph = places [from.idx ()].unwrap ();
			if places [to.idx ()].is_some () { panic! () }
			places [to.idx ()] = Some (amph);
			places [from.idx ()] = None;
			State::new (self.room_size, places)
		}

		pub fn pretty_line (& self, line: usize) -> String {
			let print_amph = |amph: Option <Amph>| amph.map (Amph::letter).unwrap_or (' ');
			if line == 0 {
				format! ("┌───────────┐")
			} else if line == 1 {
				format! ("│{}│",
					self.places [0 .. 11].iter ().copied ().map (print_amph).collect::<String> ())
			} else if line == 2 {
				format! ("└─┐{}╷{}╷{}╷{}┌─┘",
					print_amph (self.places [11]), print_amph (self.places [12]),
					print_amph (self.places [13]), print_amph (self.places [14]))
			} else if line < self.room_size as usize + 2 {
				let offset = 11 + (line - 2) * 4;
				format! ("  │{}│{}│{}│{}│",
					print_amph (self.places [offset + 0]), print_amph (self.places [offset + 1]),
					print_amph (self.places [offset + 2]), print_amph (self.places [offset + 3]))
			} else if line == self.room_size as usize + 2 {
				format! ("  └─┴─┴─┴─┘")
			} else {
				panic! ();
			}
		}

		#[ allow (dead_code) ]
		pub fn print (& self) {
			for line in 0 .. self.room_size as usize + 3 {
				println! ("{}", self.pretty_line (line));
			}
		}

		#[ allow (dead_code) ]
		pub fn from_str (input: & str) -> Option <State> {
			let num_chars = input.chars ().count ();
			if ! [19, 23, 27].contains (& num_chars) { return None }
			let room_size = ((num_chars - 11) / 4) as u8;
			let mut places = [None; 27];
			for (idx, letter) in input.chars ().enumerate () {
				places [idx] = match Amph::from_letter (letter) {
					Some (amph) => amph,
					None => return None,
				}
			}
			Some (State::new (room_size, places))
		}

		pub fn parse (lines: & [& str]) -> GenResult <State> {
			let err = |line_idx| format! ("Invalid input: {}: {}", line_idx, lines [line_idx]);
			if lines.len () < 5 || lines.len () > 7 { Err (err (5)) ?; }
			if lines [0] != "#############" { Err (err (0)) ?; }
			if lines [1] != "#...........#" { Err (err (1)) ?; }
			for line_idx in 2 .. lines.len () - 1 {
				if line_idx == 2 && lines [line_idx].len () != 13 { Err (err (line_idx)) ?; }
				if line_idx != 2 && lines [line_idx].len () != 11 { Err (err (line_idx)) ?; }
				if line_idx == 2 && & lines [line_idx] [0 .. 3] != "###" { Err (err (line_idx)) ?; }
				if line_idx != 2 && & lines [line_idx] [0 .. 3] != "  #" { Err (err (line_idx)) ?; }
				if & lines [line_idx] [4 .. 5] != "#" { Err (err (line_idx)) ?; }
				if & lines [line_idx] [6 .. 7] != "#" { Err (err (line_idx)) ?; }
				if & lines [line_idx] [8 .. 9] != "#" { Err (err (line_idx)) ?; }
				if line_idx == 2 && & lines [line_idx] [10 .. 13] != "###" { Err (err (line_idx)) ?; }
				if line_idx != 2 && & lines [line_idx] [10 .. 11] != "#" { Err (err (line_idx)) ?; }
			}
			let line_idx = lines.len () - 1;
			if lines [line_idx] != "  #########" { Err (err (line_idx)) ?; }
			let room_size = (lines.len () - 3) as u8;
			let parse_amph = |line: usize, col| Amph::from_letter (
				lines [line].chars ().skip (col).next ().unwrap (),
			).ok_or (err (2));
			let mut places = [None; 27];
			for idx in 0 .. room_size as usize {
				let line_idx = 2 + idx;
				places [11 + idx * 4] = parse_amph (line_idx, 3) ?;
				places [12 + idx * 4] = parse_amph (line_idx, 5) ?;
				places [13 + idx * 4] = parse_amph (line_idx, 7) ?;
				places [14 + idx * 4] = parse_amph (line_idx, 9) ?;
			}
			Ok (State::new (room_size, places))
		}

		pub fn compact (& self) -> StateCompact {
			let mut mask: u64 = 0;
			let mut places: u64 = 0;
			for (idx, amph) in self.places.into_iter ().enumerate () {
				if idx == 2 || idx == 4 || idx == 6 { continue }
				mask <<= 1;
				if let Some (amph) = amph {
					places <<= 2;
					mask |= 1;
					match amph {
						Amph::Amber => places |= 0,
						Amph::Bronze => places |= 1,
						Amph::Copper => places |= 2,
						Amph::Desert => places |= 3,
					}
				}
			}
			let data = ((self.room_size as u64) << 56) | (mask << 32) | places;
			StateCompact { data }
		}

	}

	impl fmt::Display for State {
		fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
			for amph in self.places {
				write! (formatter, "{}", amph.map (Amph::letter).unwrap_or ('.')) ?;
			}
			Ok (())
		}
	}

	impl hash::Hash for State {
		fn hash <Hasher: hash::Hasher> (& self, state: & mut Hasher) {
			state.write_u64 (self.compact ().data);
		}
	}

	#[ derive (Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd) ]
	pub struct StateCompact {
		data: u64,
	}

	impl StateCompact {
		pub fn expand (self) -> State {
			let mut mask = (self.data & 0x00ffffff00000000) >> 32;
			let mut place_bits = self.data & 0x00000000ffffffff;
			let mut places = [None; 27];
			for idx in (0 .. 27).rev () {
				if idx == 2 || idx == 4 || idx == 6 { continue }
				if mask & 1 != 0 {
					match place_bits & 0x3 {
						0 => places [idx] = Some (Amph::Amber),
						1 => places [idx] = Some (Amph::Bronze),
						2 => places [idx] = Some (Amph::Copper),
						3 => places [idx] = Some (Amph::Desert),
						_ => unreachable! (),
					}
					place_bits >>= 2;
				}
				mask >>= 1;
			}
			let room_size = (self.data >> 56) as u8;
			State { room_size, places }
		}
	}

	#[ derive (Clone, Copy, Debug, Eq, Hash, PartialEq) ]
	pub struct Place {
		id: u8,
	}

	impl Place {
		pub fn hall_blocking_left () -> Place { Place { id: 3 } }
		pub fn hall_blocking_middle () -> Place { Place { id: 5 } }
		pub fn hall_blocking_right () -> Place { Place { id: 7 } }
		fn idx (self) -> usize { self.id as usize }
		pub fn hall (self) -> bool { (0 .. 11).contains (& self.idx ()) }
		pub fn room (self) -> bool { (11 .. 27).contains (& self.idx ()) }
		pub fn entrance (self) -> bool { [2, 4, 6, 8].contains (& self.idx ()) }
		pub fn amph (self) -> Option <Amph> { match self.id {
			11 | 15 | 19 | 23 => Some (Amph::Amber),
			12 | 16 | 20 | 24 => Some (Amph::Bronze),
			13 | 17 | 21 | 25 => Some (Amph::Copper),
			14 | 18 | 22 | 26 => Some (Amph::Desert),
			_ => None,
		} }
		pub fn adjacent (self) -> PlaceAdjacent {
			PlaceAdjacent { iter: Place::ADJACENT [self.id as usize].iter () }
		}
		pub fn room_depth (self) -> u8 { match self.id {
			11 ..= 14 => 0,
			15 ..= 18 => 1,
			19 ..= 22 => 2,
			23 ..= 26 => 3,
			_ => panic! (),
		} }
		const ADJACENT: & 'static [& 'static [u8]] = & [
			& [1], & [0, 2], & [1, 3, 11], & [2, 4], & [3, 5, 12], & [4, 6], & [5, 7, 13],
			& [6, 8], & [7, 9, 14], & [8, 10], & [9], & [2, 15], & [4, 16], & [6, 17], & [8, 18],
			& [11, 19], & [12, 20], & [13, 21], & [14, 22], & [15, 23], & [16, 24], & [17, 25],
			& [18, 26], & [19], & [20], & [21], & [22],
		];
	}

	pub struct PlaceAdjacent { iter: SliceIter <'static, u8> }
	impl Iterator for PlaceAdjacent {
		type Item = Place;
		fn next (& mut self) -> Option <Place> {
			match self.iter.next () {
				Some (id) => Some (Place { id: * id }),
				None => None,
			}
		}
	}

	#[ derive (Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd) ]
	pub enum Amph { Amber, Bronze, Copper, Desert }

	impl Amph {
		pub fn cost (self) -> i64 { match self {
			Amph::Amber => 1, Amph::Bronze => 10,
			Amph::Copper => 100, Amph::Desert => 1000,
		} }
		pub fn belongs (self, place: Place) -> bool {
			place.amph () == Some (self)
		}
		pub fn letter (self) -> char { match self {
			Amph::Amber => 'A', Amph::Bronze => 'B',
			Amph::Copper => 'C', Amph::Desert => 'D',
		} }
		pub fn from_letter (letter: char) -> Option <Option <Amph>> { match letter {
			'A' => Some (Some (Amph::Amber)), 'B' => Some (Some (Amph::Bronze)),
			'C' => Some (Some (Amph::Copper)), 'D' => Some (Some (Amph::Desert)),
			'.' => Some (None), _ => None,
		} }
	}

	#[ cfg (test) ]
	mod tests {

		use super::*;

		#[ test ]
		fn test_state_finished () {
			assert! (State::from_str ("...........ABCDABCD").unwrap ().is_finished ());
			assert! (! State::from_str ("A...........BCDABCD").unwrap ().is_finished ());
		}

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
