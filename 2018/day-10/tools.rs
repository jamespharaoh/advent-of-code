use super::*;

use std::path::PathBuf;

args_decl! {
	pub struct RunArgs {
		input: Option <PathBuf>,
	}
}

#[ allow (clippy::print_stdout) ]
pub fn run (args: RunArgs) -> GenResult <()> {
	let input_path = puzzle_metadata ().find_input_or_arg (args.input);
	let input_string = fs::read_to_string (input_path) ?;
	let input_lines: Vec <& str> = input_string.trim_end ().split ('\n').collect ();
	let input = input::Input::parse_from_lines (& input_lines) ?;
	let (points, num_iters) = logic::find_smallest (& input) ?;
	println! ("{}", ocr::DrawDots (points.iter ()
		.map (|& point| (point.pos.y, point.pos.x))));
	println! ("Number of iterations: {num_iters}");
	let message = ocr::read_dots (points.iter ()
		.map (|& point| (point.pos.y, point.pos.x))) ?;
	println! ("Decoded message: {message}");
	Ok (())
}
