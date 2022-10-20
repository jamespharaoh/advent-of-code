#![ allow (clippy::missing_inline_in_public_items) ]

use super::*;

use std::path::PathBuf;

use input::Input;
use model::Pos;

args_decl! {
	pub struct RunArgs {
		input: Option <PathBuf>,
	}
}

#[ allow (clippy::print_stdout) ]
pub fn run (args: RunArgs) -> GenResult <()> {
	let input_string = fs::read_to_string (
		puzzle_metadata ().find_input_or_arg (args.input)) ?;
	let input_lines: Vec <& str> = input_string.trim ().split ('\n').collect ();
	let input = Input::parse_from_lines (& input_lines) ?;
	let dots = logic::fold_multi (& input.folds, input.dots.iter ().copied ());
	print! ("{}", ocr::DrawDots (dots.iter ().map (|& Pos { y, x }| (y, x))));
	Ok (())
}
