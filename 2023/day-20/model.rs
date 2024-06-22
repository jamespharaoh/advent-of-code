use super::*;

use input::Input;
use input::InputModule;
use input::InputModuleType;

#[ derive (Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd) ]
pub enum Signal {
	High,
	Low,
}

impl fmt::Display for Signal {
	fn fmt (& self, fmtr: & mut fmt::Formatter) -> fmt::Result {
		match * self {
			Self::High => fmtr.write_str ("high"),
			Self::Low => fmtr.write_str ("low"),
		}
	}
}

pub struct Runner {
	pub modules: Vec <Module>,
	pub modules_index: HashMap <String, usize>,
	pub num_runs: u64,
	pub num_low: u64,
	pub num_high: u64,
	pub state: Vec <Signal>,
	pub max_signals: usize,
}

impl Runner {

	pub fn build (input: & Input) -> GenResult <Self> {

		let input_modules: HashMap <& str, & InputModule> =
			input.modules.iter ()
				.map (|module| (module.name.as_str (), module))
				.collect ();

		let module_names: Vec <String> =
			input.modules.iter ()
				.flat_map (|module| iter::once (module.name.as_str ())
					.chain (module.dests.iter ().map (InpStr::as_str)))
				.chain (iter::once ("button"))
				.sorted ()
				.dedup_consecutive ()
				.map (str::to_owned)
				.collect ();

		let modules_index: HashMap <String, usize> =
			module_names.iter ().enumerate ()
				.map (|(idx, name)| (name.to_string (), idx))
				.collect ();

		let mut global_inputs = vec! [Vec::new (); module_names.len ()];
		for module in & input.modules {
			let src_idx = modules_index [module.name.as_str ()];
			for dest_name in & module.dests {
				let dest_idx = modules_index [dest_name.as_str ()];
				if dest_idx == src_idx { return Err ("Self-referential module".into ()) }
				let module_inputs = & mut global_inputs [dest_idx];
				if ! module_inputs.contains (& src_idx) { module_inputs.push (src_idx); }
			}
		}

		let mut global_state = Vec::new ();
		let modules: Vec <Module> =
			module_names.iter ().enumerate ()
				.map (|(module_idx, module_name)| {
					let module_inputs = global_inputs [module_idx].clone ();
					if let Some (module) = input_modules.get (module_name.as_str ()) {
						let module_outputs: Vec <usize> =
							module.dests.iter ()
								.map (|dest| modules_index [dest.as_str ()])
								.collect ();
						let (type_, module_state_vals) = match module.type_ {
							InputModuleType::Broadcast =>
								(ModuleType::Broadcast, Vec::new ()),
							InputModuleType::FlipFlop =>
								(ModuleType::FlipFlop, vec! [Signal::High]),
							InputModuleType::Conjunction =>
								(ModuleType::Conjunction, vec! [Signal::Low; module_inputs.len ()]),
						};
						let mut module_state = Vec::new ();
						for state_val in module_state_vals {
							module_state.push (global_state.len ());
							global_state.push (state_val);
						}
						Module {
							index: module_idx,
							type_,
							inputs: module_inputs,
							outputs: module_outputs,
							state: module_state,
							num_low: 0,
							num_high: 0,
						}
					} else {
						Module {
							index: module_idx,
							type_: ModuleType::Output,
							inputs: module_inputs,
							outputs: Vec::new (),
							state: Vec::new (),
							num_low: 0,
							num_high: 0,
						}
					}
				})
				.collect ();

		Ok (Self {
			modules,
			modules_index,
			num_runs: 0,
			num_low: 0,
			num_high: 0,
			state: global_state,
			max_signals: 0,
		})

	}

	pub fn run (& mut self) -> GenResult <Vec <(usize, usize, Signal)>> {
		self.num_runs += 1;
		let mut signals = Vec::with_capacity (self.max_signals);
		let mut queue = VecDeque::new ();
		let button_idx = self.module_for_name ("button") ?.index;
		let broadcaster_idx = self.module_for_name ("broadcaster") ?.index;
		queue.push_back ((button_idx, broadcaster_idx, Signal::Low));
		while let Some ((from_idx, module_idx, input_signal)) = queue.pop_front () {
			signals.push ((from_idx, module_idx, input_signal));
			if 1000 < signals.len () { return Err ("Max signals exceeded".into ()); }
			match input_signal {
				Signal::High => self.num_high += 1,
				Signal::Low => self.num_low += 1,
			}
			let module = & mut self.modules [module_idx];
			match input_signal {
				Signal::High => module.num_high += 1,
				Signal::Low => module.num_low += 1,
			}
			let output_signal = match module.type_ {
				ModuleType::Broadcast => Some (input_signal),
				ModuleType::FlipFlop => {
					(input_signal == Signal::Low).then (|| {
						let state_idx = module.state [0];
						let next = self.state [state_idx];
						self.state [state_idx] = match next {
							Signal::Low => Signal::High,
							Signal::High => Signal::Low,
						};
						next
					})
				},
				ModuleType::Conjunction => {
					let input_idx =
						module.inputs.iter ()
							.position (|& idx| idx == from_idx)
							.unwrap ();
					self.state [module.state [input_idx]] = input_signal;
					if module.state.iter ()
							.map (|& state_idx| self.state [state_idx])
							.all (|sig| sig == Signal::High) {
						Some (Signal::Low)
					} else {
						Some (Signal::High)
					}
				},
				ModuleType::Output => None,
			};
			if let Some (signal) = output_signal {
				for & to_idx in & module.outputs {
					queue.push_back ((module_idx, to_idx, signal));
				}
			}
		}
		self.max_signals = cmp::max (self.max_signals, signals.len ());
		Ok (signals)
	}

	pub fn module_for_name (& self, name: & str) -> GenResult <& Module> {
		let & idx =
			self.modules_index.get (name)
				.ok_or_else (|| format! ("No such module: {name}")) ?;
		Ok (& self.modules [idx])
	}

}

pub struct Module {
	pub index: usize,
	pub type_: ModuleType,
	pub inputs: Vec <usize>,
	pub outputs: Vec <usize>,
	pub state: Vec <usize>,
	pub num_low: u64,
	pub num_high: u64,
}

#[ derive (Clone, Copy, Debug, Eq, PartialEq) ]
pub enum ModuleType {
	Broadcast,
	FlipFlop,
	Conjunction,
	Output,
}
