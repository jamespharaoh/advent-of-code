#![ allow (clippy::missing_inline_in_public_items) ]

use super::*;

use input::Input;
use model::Pos;

#[ derive (clap::Parser) ]
pub struct RunArgs {

	#[ clap (long, value_parser, default_value = "inputs/day-13") ]
	input: String,

}

#[ allow (clippy::print_stdout) ]
pub fn run (args: RunArgs) -> GenResult <()> {
	let input_string = fs::read_to_string (args.input) ?;
	let input_lines: Vec <& str> = input_string.trim ().split ('\n').collect ();
	let input = Input::parse_from_lines (& input_lines) ?;
	let dots = logic::fold_multi (& input.folds, input.dots.iter ().copied ());
	print! ("{}", ocr::DrawDots (dots.iter ().map (|& Pos { y, x }| (y, x))));
	Ok (())
}
