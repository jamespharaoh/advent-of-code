use super::*;

use model::State;

#[ derive (Debug, clap::Parser) ]
pub struct RunArgs {

	#[ clap (long, default_value = "inputs/day-10") ]
	input: String,

	#[ clap (conflicts_with = "input") ]
	state: Option <String>,

	#[ clap (long) ]
	verbose: bool,

	#[ clap (long, default_value = "15") ]
	loops: u32,

	#[ clap (long, default_value = "0") ]
	keep_end: usize,

	#[ clap (long, default_value = "0") ]
	keep_start: usize,

}

#[ allow (clippy::needless_pass_by_value) ]
#[ allow (clippy::print_stdout) ]
pub fn run (args: RunArgs) -> GenResult <()> {
	let mut state = if let Some (state) = args.state.as_ref () {
		State::parse (state) ?
	} else {
		State::parse (
			fs::read_to_string (& args.input) ?
				.trim ()
				.split ('\n')
				.next ().unwrap ()
		) ?
	};
	for idx in 0 .. {
		println! ("{:2} {:4} {}", idx, state.len (), state);
		if idx == args.loops { break }
		state = logic::one_round (& state);
		if (args.keep_start > 0 || args.keep_end > 0)
				&& state.len () > (args.keep_start + args.keep_end) {
			state =
				state [ .. args.keep_start].iter ().copied ()
					.chain (state [state.len () - args.keep_end .. ].iter ().copied ())
					.collect ();
		}
	}
	Ok (())
}

#[ derive (Debug, clap::Parser) ]
pub struct InternalsArgs;

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
