use intcode::Machine;
use intcode::Mem;
use intcode::RunResult;
use std::fs;
use std::iter;
use std::mem;

mod intcode;

fn main () {
	let input_string = fs::read_to_string ("input").unwrap ();
	let input_programme = intcode::from_str (& input_string);
	let mut nodes: Vec <Node> = (0 .. 50).map (
		|node_id| Node::new (input_programme.clone (), node_id),
	).collect ();
	let mut queues: Vec <Vec <(i64, i64)>> = iter::repeat (Vec::new ()).take (50).collect ();
	let mut nat_received: Option <(i64, i64)> = None;
	let mut nat_sent: Option <(i64, i64)> = None;
	loop {
		let mut working = false;
		for (node_id, node) in nodes.iter_mut ().enumerate () {
			let mut input: Vec <(i64, i64)> = Vec::new ();
			mem::swap (& mut input, & mut queues [node_id]);
			match node.run (input) {
				NodeRunResult::Working => { working = true },
				NodeRunResult::Idle => (),
				NodeRunResult::Send (a, x, y) => {
					if a == 255 {
						nat_received = Some ((x, y));
					} else if a >= 0 && a < 50 {
						queues [a as usize].push ((x, y));
					} else {
						panic! ();
					}
				},
			}
		}
		if ! working {
			if let Some ((x, y)) = nat_received {
				if nat_sent == Some ((x, y)) {
					println! ("Sent twice: x={}, y={}", x, y);
					return;
				}
				nat_sent = Some ((x, y));
				queues [0].push ((x, y));
				println! ("NAT send x={}, y={}", x, y);
			} else {
				println! ("NAT (nothing to send)");
			}
		}
	}
}

struct Node {
	machine: Machine,
	buf: Vec <i64>,
}

impl Node {
	fn new (mem: Mem, node_id: i64) -> Node {
		let mut machine = Machine::new (mem);
		machine.input (node_id as i64);
		Node {
			machine,
			buf: Vec::new (),
		}
	}
	fn run (& mut self, input: Vec <(i64, i64)>) -> NodeRunResult {
		for (x, y) in input.into_iter () {
			self.machine.input (x);
			self.machine.input (y);
		}
		let mut idle = false;
		match self.machine.run () {
			RunResult::Output (value) => self.buf.push (value),
			RunResult::Input => { self.machine.input (-1); idle = true },
			RunResult::Halt => panic! (),
		}
		if self.buf.len () == 3 {
			let (a, x, y) = (self.buf [0], self.buf [1], self.buf [2]);
			self.buf.clear ();
			NodeRunResult::Send (a, x, y)
		} else if idle {
			NodeRunResult::Idle
		} else {
			NodeRunResult::Working
		}
	}
}

enum NodeRunResult { Working, Idle, Send (i64, i64, i64) }
