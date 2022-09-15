//! Logic for solving the puzzles

use super::*;

use input::Input;
use model::Cpu;
use model::RunResult;
use model::Val;

use self::node::Node;

pub fn part_one (input: & Input) -> GenResult <Val> {
	let mut nodes: Vec <Node> =
		(0 .. 50)
			.map (|addr| Node::new (input.data.clone (), input.params.max_ops, addr))
			.collect ();
	for _ in 0 .. input.params.max_iters_one {
		for node_idx in 0 .. nodes.len () {
			if let Some ((addr, x, y)) = nodes [node_idx].run () ? {
				if addr == 255 {
					return Ok (y);
				} else if (0 .. 50).contains (& addr) {
					nodes [addr.as_usize ()].deliver (x, y);
				} else {
					return Err (format! ("Invalid address: {addr}").into ());
				}
			}
		}
	}
	Err ("Max iterations".into ())
}

pub fn part_two (input: & Input) -> GenResult <Val> {
	let mut nodes: Vec <Node> =
		(0 .. 50)
			.map (|addr| Node::new (input.data.clone (), input.params.max_ops, addr))
			.collect ();
	let mut cached = None;
	let mut last = None;
	for _ in 0 .. input.params.max_iters_two {
		for node_idx in 0 .. nodes.len () {
			if let Some ((addr, x, y)) = nodes [node_idx].run () ? {
				if addr == 255 {
					cached = Some ((x, y));
				} else if (0 .. 50).contains (& addr) {
					nodes [addr.as_usize ()].deliver (x, y);
				} else {
					return Err (format! ("Invalid address: {addr}").into ());
				}
			}
		}
		if nodes.iter ().all (Node::blocked) {
			let (x, y) = cached.ok_or ("Network idle but no cached packet") ?;
			if last == Some (y) { return Ok (y) }
			nodes [0].deliver (x, y);
			last = Some (y);
		}
	}
	Err ("Max iterations".into ())
}

mod node {

	use super::*;

	pub struct Node {
		cpu: Cpu,
		max_ops: u32,
		num_inputs: u32,
		output_buffer: Vec <Val>,
	}

	impl Node {

		pub fn new (prog: Vec <Val>, max_ops: u32, addr: Val) -> Self {
			let mut cpu = Cpu::new (prog);
			cpu.set_mem_limit (4096);
			cpu.input (addr);
			Self { cpu, max_ops, num_inputs: 0, output_buffer: Vec::new () }
		}

		pub const fn blocked (& self) -> bool {
			1 < self.num_inputs
		}

		pub fn deliver (& mut self, x: Val, y: Val) {
			self.cpu.input (x);
			self.cpu.input (y);
			self.num_inputs = 0;
		}

		#[ allow (clippy::wildcard_enum_match_arm) ]
		pub fn run (& mut self) -> GenResult <Option <(Val, Val, Val)>> {
			self.cpu.set_max_ops (self.max_ops);
			match self.cpu.run () {
				RunResult::Output (val) => {
					self.output_buffer.push (val);
					if self.output_buffer.len () < 3 {
						Ok (None)
					} else {
						let addr = self.output_buffer [0];
						let x = self.output_buffer [1];
						let y = self.output_buffer [2];
						self.output_buffer.clear ();
						Ok (Some ((addr, x, y)))
					}
				},
				RunResult::Input => {
					self.cpu.input (-1);
					self.num_inputs += 1;
					Ok (None)
				},
				other => Err (other.into ()),
			}
		}

	}

}
