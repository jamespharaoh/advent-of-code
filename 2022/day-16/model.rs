use super::*;

use input::Input;
use input::InputValve;

#[ derive (Clone, Debug) ]
pub struct Volcano <'inp> {
	pub caverns: Vec <Cavern <'inp>>,
}

impl <'inp> Volcano <'inp> {

	pub fn build (input: & 'inp Input <'inp>) -> GenResult <Self> {
		let input_valves: HashMap <InpStr, & InputValve> =
			input.valves.iter ()
				.map (|valve| (valve.name.clone (), valve))
				.collect ();
		let distances = Self::build_distances (& input_valves) ?;
		let cavern_names: Vec <InpStr> =
			input_valves.values ()
				.filter (|input_valve| 0 < input_valve.flow_rate)
				.map (|input_valve| input_valve.name.clone ())
				.sorted ()
				.collect ();
		let caverns =
			cavern_names.iter ().enumerate ()
				.map (|(cavern_idx, cavern_name)| {
					let input_valve = input_valves [cavern_name];
					let initial_travel_time =
						distances [& (cavern_name.clone (), InpStr::borrow ("AA"))];
					let travel_times = cavern_names.iter ()
						.map (|other_cavern_name| distances [
							& (cavern_name.clone (), other_cavern_name.clone ())])
						.collect ();
					Cavern {
						idx: cavern_idx.pan_u16 (),
						name: cavern_name.clone (),
						flow_rate: input_valve.flow_rate.pan_u32 (),
						initial_travel_time,
						travel_times,
					}
				})
				.collect ();
		Ok (Volcano { caverns })
	}

	fn build_distances (
		input_valves: & HashMap <InpStr <'inp>, & InputValve <'inp>>,
	) -> GenResult <HashMap <(InpStr <'inp>, InpStr <'inp>), u32>> {
		let mut distances = HashMap::new ();
		for input_valve in input_valves.values () {
			let mut todo = VecDeque::new ();
			todo.push_back ((input_valve.name.clone (), 0));
			distances.insert ((input_valve.name.clone (), input_valve.name.clone ()), 0);
			while let Some ((cur_valve_name, time)) = todo.pop_front () {
				let cur_valve = input_valves [& cur_valve_name];
				for next_valve_name in cur_valve.tunnels.iter ().sorted ().dedup_consecutive () {
					let key = (input_valve.name.clone (), next_valve_name.clone ());
					if distances.contains_key (& key) { continue }
					distances.insert (key, time + 1);
					todo.push_back ((next_valve_name.clone (), time + 1));
				}
			}
		}
		if distances.len () != input_valves.len () * input_valves.len () {
			return Err ("Valves are not fully connected".into ());
		}
		Ok (distances)
	}

}

#[ derive (Clone, Debug) ]
pub struct Cavern <'inp> {
	pub idx: u16,
	pub name: InpStr <'inp>,
	pub flow_rate: u32,
	pub initial_travel_time: u32,
	pub travel_times: Vec <u32>,
}
