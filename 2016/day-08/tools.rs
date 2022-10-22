use std::path::PathBuf;

use super::*;

use model::Grid;
use model::Pos;

args_decl! {
	pub struct Args {
		pub input: Option <PathBuf>,
		pub animate: bool,
	}
}

#[ allow (clippy::print_stdout) ]
pub fn run (args: Args) -> GenResult <()> {
	let puzzle = puzzle_metadata ();
	let input_path = puzzle.find_input_or_arg (& args.input);
	let input_string = fs::read_to_string (input_path) ?;
	let input_lines: Vec <& str> = input_string.trim_end ().split ('\n').collect ();
	let input = input::Input::parse_from_lines (& input_lines) ?;
	let grid = if args.animate {
		let mut grid: Grid =
			Grid::new_size (Pos::new (input.params.height, input.params.width));
		println! ("{}", grid.print (|val| if val { "##" } else { "  " }));
		for & step in & input.steps {
			thread::sleep (time::Duration::from_micros (100_000));
			logic::apply_step (& mut grid, step);
			print! ("\x1b[{rows}A{grid}",
				rows = input.params.height,
				grid = grid.print (|val| if val { "##" } else { "  " }));
		}
		grid
	} else {
		let grid = logic::calc_result (& input);
		let dots: Vec <_> = grid.iter ()
			.filter (|& (_, val)| val)
			.map (|(pos, _)| (pos.row, pos.col))
			.collect ();
		print! ("{}", ocr::DrawDots (dots.iter ().copied ()));
		grid
	};
	let dots: Vec <_> = grid.iter ()
		.filter (|& (_, val)| val)
		.map (|(pos, _)| (pos.row, pos.col))
		.collect ();
	let result = ocr::read_fixed (dots.iter ().copied (), (6, 5)) ?;
	println! ("Result: {}", result);
	Ok (())
}
