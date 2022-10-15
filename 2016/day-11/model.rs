use super::*;

use input::Component;
use input::Input;

pub type Val = u16;

#[ derive (Clone, Copy, Debug) ]
pub struct State {
	pub comps: [[u8; 2]; 7],
	pub comps_len: u8,
	pub elevator: u8,
}

#[ derive (Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd) ]
pub struct StateCompact {
	data: u32,
}

impl State {

	pub fn from_input <'inp> (input: & Input <'inp>) -> GenResult <(Self, Vec <InpStr <'inp>>)> {
		let names: Vec <InpStr> =
			input.floors.iter ()
				.flat_map (|floor| floor.iter ()
					.map (|comp| match * comp {
						Component::Generator (ref name) => name.clone (),
						Component::Microchip (ref name) => name.clone (),
					}))
				.sorted ()
				.dedup ()
				.collect ();
		if names.is_empty () { return Err ("Must have at least one component type".into ()) }
		if names.len () > 7 { return Err ("Only support seven types of component".into ()) }
		let comp_floor = |comp|
			input.floors.iter ()
				.enumerate ()
				.flat_map (|(floor_idx, floor)| floor.iter ()
					.map (move |comp| ((floor_idx + 1).pan_u8 (), comp)))
				.filter (|& (_, some_comp)| * some_comp == comp)
				.map (|(floor, _)| floor)
				.exactly_one ()
				.map_err (|_err| "Must have exactly one generator of each type");
		Ok ((Self {
			comps: names.iter ()
				.map (|name| Ok::<_, GenError> ([
					comp_floor (Component::Generator (name.clone ())) ?,
					comp_floor (Component::Microchip (name.clone ())) ?,
				]))
				.chain (iter::from_fn (|| Some (Ok ([1, 1]))))
				.take (7)
				.try_array () ?,
			comps_len: names.len ().pan_u8 (),
			elevator: 1,
		}, names))
	}

	pub fn next_states (& self, results: & mut Vec <StateCompact>) {
		let mut next_floors = ArrayVec::<u8, 2>::new ();
		if self.elevator > 1 { next_floors.push (self.elevator - 1); }
		if self.elevator < 4 { next_floors.push (self.elevator + 1); }
		for name_idx_0 in 0 .. self.comps_len.pan_usize () {
			for comp_idx_0 in 0 .. 2 {
				if self.comps [name_idx_0] [comp_idx_0] != self.elevator { continue }
				for & next_floor in next_floors.iter () {
					let mut next_state = * self;
					next_state.elevator = next_floor;
					next_state.comps [name_idx_0] [comp_idx_0] = next_floor;
					if next_state.is_valid () {
						next_state.comps [ .. next_state.comps_len.pan_usize ()].sort ();
						results.push (next_state.compact ());
					}
				}
				for name_idx_1 in name_idx_0 .. self.comps_len.pan_usize () {
					for comp_idx_1 in 0 .. 2 {
						if self.comps [name_idx_1] [comp_idx_1] != self.elevator { continue }
						if (name_idx_0, comp_idx_0) >= (name_idx_1, comp_idx_1) { continue }
						for & next_floor in next_floors.iter () {
							let mut next_state = * self;
							next_state.elevator = next_floor;
							next_state.comps [name_idx_0] [comp_idx_0] = next_floor;
							next_state.comps [name_idx_1] [comp_idx_1] = next_floor;
							if next_state.is_valid () {
								next_state.comps [ .. next_state.comps_len.pan_usize ()].sort ();
								results.push (next_state.compact ());
							}
						}
					}
				}
			}
		}
	}

	fn is_valid (& self) -> bool {
		for name_idx_0 in 0 .. self.comps_len.pan_usize () {
			if self.comps [name_idx_0] [1] == self.comps [name_idx_0] [0] { continue }
			for name_idx_1 in 0 .. self.comps_len.pan_usize () {
				if name_idx_1 == name_idx_0 { continue }
				if self.comps [name_idx_0] [1] == self.comps [name_idx_1] [0] { return false }
			}
		}
		true
	}

	#[ must_use ]
	pub fn is_done (& self) -> bool {
		self.comps.iter ()
			.take (self.comps_len.pan_usize ())
			.all (|& [gen, chip]| gen == 4 && chip == 4)
	}

	#[ must_use ]
	pub fn compact (& self) -> StateCompact {
		let mut data = BitPusher::new ();
		for & [ gen, chip ] in & self.comps {
			data.push (gen.pan_u64 () - 1, 2);
			data.push (chip.pan_u64 () - 1, 2);
		}
		data.push (self.elevator.pan_u64 () - 1, 2);
		StateCompact { data: data.finish () }
	}

}

impl StateCompact {

	#[ must_use ]
	pub fn expand (self, comps_len: u8) -> State {
		let mut data = BitPopper::new (self.data);
		let comps = array::from_fn (|_| [
			data.pop::<u8> (2) + 1,
			data.pop::<u8> (2) + 1,
		]);
		let elevator = data.pop::<u8> (2) + 1;
		State { comps, comps_len, elevator }
	}

}
