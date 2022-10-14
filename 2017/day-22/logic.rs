//! Logic for solving the puzzles

use super::*;
use input::Input;
use model::Dir;
use model::Grid;
use model::Node;
use model::Pos;
use model::Turn;

pub fn part_one (input: & Input) -> GenResult <u64> {
	calc_result (
		input,
		input.params.iters_one,
		|state| match state {
			Node::Clean | Node::Weakened => (Turn::Left, Node::Infected, true),
			Node::Infected | Node::Flagged => (Turn::Right, Node::Clean, false),
		},
	)
}

pub fn part_two (input: & Input) -> GenResult <u64> {
	calc_result (
		input,
		input.params.iters_two,
		|state| match state {
			Node::Clean => (Turn::Left, Node::Weakened, false),
			Node::Weakened => (Turn::None, Node::Infected, true),
			Node::Infected => (Turn::Right, Node::Flagged, false),
			Node::Flagged => (Turn::Around, Node::Clean, false),
		},
	)
}

fn calc_result (
	input: & Input,
	num_bursts: u32,
	state_fn: impl Fn (Node) -> (Turn, Node, bool),
) -> GenResult <u64> {
	let mut nodes = input.nodes.clone ();
	let mut pos = Pos::ZERO;
	let mut dir = Dir::Up;
	let mut num_infected = 0;
	for _ in 0 .. num_bursts {
		let cur_state = nodes.get (pos).unwrap_or_else (|| {
			expand (& mut nodes);
			nodes.get (pos).unwrap ()
		});
		let (turn, next_state, count) = state_fn (cur_state);
		dir = dir + turn;
		nodes.set (pos, next_state);
		if count { num_infected += 1; }
		let dir_one = (dir, 1);
		pos = chk! (pos + dir_one) ?;
	}
	Ok (num_infected)
}

fn expand (nodes: & mut Grid) {
	let incr_amt = cmp::min (nodes.size ().row, nodes.size ().col) / 20 * 5 + 5;
	* nodes = nodes.extend_in_place ([(incr_amt, incr_amt); 2]).unwrap ();
}
