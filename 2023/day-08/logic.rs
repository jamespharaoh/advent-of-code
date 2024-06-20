#![ allow (clippy::match_on_vec_items) ]

use super::*;

use input::Input;
use input::InputParams;
use input::Step;

pub fn part_one (input: & Input) -> GenResult <u64> {
	check_input (input) ?;
	let nodes: HashMap <_, _> =
		input.nodes.iter ()
			.map (|node| (node.name.as_str (), (node.left.as_str (), node.right.as_str ())))
			.collect ();
	if ! nodes.contains_key ("AAA") || ! nodes.contains_key ("ZZZ") {
		return Err ("Nodes AAA and ZZZ must exist".into ());
	}
	let mut current = "AAA";
	let mut num_steps = 0;
	let mut step_idx = 0;
	while current != "ZZZ" {
		match input.steps [step_idx] {
			Step::Left => current = nodes [current].0,
			Step::Right => current = nodes [current].1,
		}
		step_idx += 1;
		if step_idx == input.steps.len () { step_idx = 0; }
		num_steps += 1;
		if num_steps == input.params.max_steps {
			return Err ("Exceeded max steps".into ());
		}
	}
	Ok (num_steps)
}

pub fn part_two (input: & Input) -> GenResult <u64> {
	check_input (input) ?;
	let nodes: HashMap <& str, (& str, & str)> =
		input.nodes.iter ()
			.map (|node| (node.name.as_str (), (node.left.as_str (), node.right.as_str ())))
			.collect ();
	let ghosts: Vec <_> =
		nodes.keys ().copied ()
			.filter (|key| key.ends_with ('A'))
			.map (|start| Ghost::build (& input.params, & input.steps, & nodes, start))
			.try_collect () ?;
	if 10 < ghosts.len () {
		return Err ("Input must have no more than 10 nodes ending in A".into ());
	}
	let ghost =
		ghosts.into_iter ()
			.try_reduce (|ghost_0, ghost_1| Ghost::merge (& input.params, ghost_0, ghost_1)) ?
			.ok_or ("Input must have nodes ending in A") ?;
	Ok (ghost.next ())
}

fn check_input (input: & Input) -> GenResult <()> {
	if input.steps.is_empty () {
		return Err ("Input must have at least one step".into ());
	}
	let node_names: HashSet <_> =
		input.nodes.iter ()
			.map (|node| node.name.as_str ())
			.collect ();
	if ! input.nodes.iter ()
			.flat_map (|node| [node.left.as_str (), node.right.as_str ()])
			.all (|name| node_names.contains (name)) {
		return Err ("Nodes referenced must exist".into ());
	}
	Ok (())
}

#[ derive (Clone, Debug) ]
struct Ghost {
	prefix: Vec <u64>,
	repeat: Vec <u64>,
	prefix_idx: usize,
	repeat_idx: usize,
	remain: u64,
}

impl Ghost {

	fn build (
		params: & InputParams,
		steps: & [Step],
		nodes: & HashMap <& str, (& str, & str)>,
		start: & str,
	) -> GenResult <Self> {
		let mut history = Vec::new ();
		let mut index = HashMap::new ();
		let mut current = start;
		let mut step_idx = 0;
		let mut num_steps = 0;
		let mut num_iters = 0;
		let repeat_idx = loop {
			if num_iters == params.max_iters {
				return Err ("Exceeded max iters".into ());
			}
			num_iters += 1;
			if current.ends_with ('Z') {
				let history_idx = history.len ();
				let state = (current, step_idx);
				if let Some (& prev_idx) = index.get (& state) { break prev_idx }
				history.push (num_steps);
				index.insert (state, history_idx);
				num_steps = 0;
			}
			match steps [step_idx] {
				Step::Left => current = nodes [current].0,
				Step::Right => current = nodes [current].1,
			}
			step_idx += 1;
			if step_idx == steps.len () { step_idx = 0; }
			num_steps += 1;
			if num_steps == params.max_steps {
				return Err ("Exceeded max steps".into ());
			}
		};
		if repeat_idx == 0 {
			Ok (Self {
				prefix: Vec::new (),
				repeat: history [ .. ].to_vec (),
				prefix_idx: 0,
				repeat_idx: 1 % history.len (),
				remain: history [0],
			})
		} else {
			Ok (Self {
				prefix: history [1 .. repeat_idx].to_vec (),
				repeat: history [repeat_idx .. ].to_vec (),
				prefix_idx: 0,
				repeat_idx: 0,
				remain: history [0],
			})
		}
	}

	fn next (& self) -> u64 {
		self.remain
	}

	fn advance (& mut self, mut steps: u64) {
		while self.remain < steps {
			steps -= self.remain;
			if self.prefix_idx < self.prefix.len () {
				self.remain = self.prefix [self.prefix_idx];
				self.prefix_idx += 1;
			} else {
				self.remain = self.repeat [self.repeat_idx];
				self.repeat_idx += 1;
				if self.repeat_idx == self.repeat.len () {
					self.repeat_idx = 0;
				}
			}
		}
		self.remain -= steps;
	}

	fn merge (
		params: & InputParams,
		mut ghost_0: Self,
		mut ghost_1: Self,
	) -> GenResult <Self> {
		let mut num_steps = 0;
		let mut history = Vec::new ();
		let mut index = HashMap::new ();
		let mut num_iters = 0;
		let repeat_idx = loop {
			if num_iters == params.max_iters {
				return Err ("Exceeded max iters".into ());
			}
			num_iters += 1;
			let adv_steps = cmp::max (ghost_0.next (), ghost_1.next ());
			if 0 < adv_steps {
				ghost_0.advance (adv_steps);
				ghost_1.advance (adv_steps);
				num_steps += adv_steps;
				continue;
			}
			let history_idx = history.len ();
			history.push (num_steps);
			if ghost_0.prefix_idx == ghost_0.prefix.len ()
					&& ghost_1.prefix_idx == ghost_1.prefix.len () {
				let state = (ghost_0.prefix_idx, ghost_1.prefix_idx);
				if let Some (& prev_idx) = index.get (& state) {
					if 0 < prev_idx { break prev_idx }
				}
				index.insert (state, history_idx);
			}
			ghost_0.advance (1);
			ghost_1.advance (1);
			num_steps = 1;
		};
		Ok (Self {
			prefix: history [1 .. repeat_idx].to_vec (),
			repeat: history [repeat_idx .. ].to_vec (),
			prefix_idx: 0,
			repeat_idx: 0,
			remain: history [0],
		})
	}

}
