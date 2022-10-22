use super::*;

use std::path::PathBuf;

use model::State;

args_decl! {
	pub struct RunArgs {
		input: Option <PathBuf>,
		state: Option <String>,
		verbose: bool,
		loops: Option <u32>,
		keep_end: Option <usize>,
		keep_start: Option <usize>,
	}
}

#[ allow (clippy::needless_pass_by_value) ]
#[ allow (clippy::print_stdout) ]
pub fn run (args: RunArgs) -> GenResult <()> {
	let mut state = if let Some (state) = args.state.as_ref () {
		State::parse (state) ?
	} else {
		let input_path = puzzle_metadata ().find_input_or_arg (& args.input);
		State::parse (
			fs::read_to_string (input_path) ?
				.trim ()
				.split ('\n')
				.next ().unwrap ()
		) ?
	};
	for idx in 0 .. {
		println! ("{:2} {:4} {}", idx, state.len (), state);
		if idx == args.loops.unwrap_or (15) { break }
		state = logic::one_round (& state);
		if (args.keep_start.unwrap_or (0) > 0 || args.keep_end.unwrap_or (0) > 0)
				&& state.len () > (args.keep_start.unwrap_or (0) + args.keep_end.unwrap_or (0)) {
			state =
				state [ .. args.keep_start.unwrap_or (0)].iter ().copied ()
					.chain (state [state.len () - args.keep_end.unwrap_or (0) .. ].iter ().copied ())
					.collect ();
		}
	}
	Ok (())
}

args_decl! {
	pub struct InternalsArgs {}
}

#[ allow (clippy::needless_pass_by_value) ]
#[ allow (clippy::print_stdout) ]
#[ allow (clippy::unnecessary_wraps) ]
pub fn internals (_args: InternalsArgs) -> GenResult <()> {
	println! ("Data structures:");
	fn show_struct <Type> () {
		let name = std::any::type_name::<Type> ();
		let size = mem::size_of::<Type> ();
		let align = mem::align_of::<Type> ();
		println! (" - {} {} bytes (align = {})", name, size, align);
	}
	show_struct::<tracking::Item> ();
	Ok (())
}
