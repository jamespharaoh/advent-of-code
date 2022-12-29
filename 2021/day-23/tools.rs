use super::*;

use std::path::PathBuf;

use input::Input;
use model::State;
use model::StateCompact;
use search::PrioritySearch;
use search::PrioritySearchAdder;

args_decl! {
	pub struct RunArgs {
		input: Option <PathBuf>,
		verbose: bool,
		dead_ends: bool,
		part_1: bool,
		part_2: bool,
	}
}

#[ allow (clippy::needless_pass_by_value) ]
pub fn run (args: RunArgs) -> GenResult <()> {
	let mut args = args;
	if ! (args.part_1 || args.part_2) { args.part_1 = true; args.part_2 = true; }
	let input_path = puzzle_metadata ().find_input_or_arg (& args.input);
	let input_string = fs::read_to_string (input_path) ?;
	let input_lines: Vec <_> = input_string.trim ().split ('\n').collect ();
	let input = Input::parse_from_lines (& input_lines) ?;
	if args.part_1 {
		run_part (& args, State::new_part_one (& input)) ?;
	}
	if args.part_2 {
		run_part (& args, State::new_part_two (& input)) ?;
	}
	Ok (())
}

#[ allow (clippy::print_stdout) ]
pub fn run_part (args: & RunArgs, initial_state: State) -> GenResult <()> {
	let mut num_loops = 0_i32;
	let mut last_cost = -1_i64;
	let mut prev_states: HashMap <StateCompact, StateCompact> = HashMap::new ();
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
	search.push (initial_state.compact (), 0);
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
				println! ("Evaluating states with cost: {cost}");
				println! ("Number of iterations: {num_loops}");
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
				print_next_states (& state, & next_states, 100);
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
	}
	println! ();
	println! ("Solved with cost: {final_cost}");
	println! ("Number of steps in solution: {}", all_states.len () - 1);
	println! ("Number of iterations: {num_loops}");
	println! ("Total states generated: {}", prev_states.len ());
	println! ();
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
fn print_next_states (
	cur_state: & State,
	next_states: & [(State, i64)],
	term_lines: u32,
) {
	let chunk_size = (term_lines - 16) / 14;
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
	for (chunk_idx, chunk) in next_states.chunks (chunk_size.pan_usize ()).enumerate () {
		print! ("{:^13}  ", if chunk_idx == 0 { "START" } else { "" });
		for & (_, ref cost) in chunk.iter () {
			print! (" {cost:^13}");
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

args_decl! {
	pub struct InternalsArgs {}
}

#[ allow (clippy::needless_pass_by_value) ]
#[ allow (clippy::print_stdout) ]
pub fn internals (_args: InternalsArgs) -> GenResult <()> {
	println! ("Data structures:");
	fn show_struct <Type> () {
		let name = std::any::type_name::<Type> ();
		let size = mem::size_of::<Type> ();
		let align = mem::align_of::<Type> ();
		println! (" - {name} {size} bytes (align = {align})");
	}
	show_struct::<logic::Move> ();
	show_struct::<model::Amph> ();
	show_struct::<model::Place> ();
	show_struct::<model::State> ();
	show_struct::<model::StateCompact> ();
	Ok (())
}
