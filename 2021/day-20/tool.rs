use super::*;

use std::path::PathBuf;

use input::Input;
use model::Image;
use model::Pixel;

args_decl! {
	pub struct RunArgs {
		input: Option <PathBuf>,
		verbose: Option <bool>,
		loops: Option <usize>,
	}
}

#[ allow (clippy::needless_pass_by_value) ]
#[ allow (clippy::print_stdout) ]
pub fn run (args: RunArgs) -> GenResult <()> {
	let input_string = fs::read_to_string (& args.input.unwrap ()) ?;
	let input_lines: Vec <& str> = input_string.trim ().split ('\n').collect ();
	let input = Input::parse_from_lines (& input_lines) ?;
	let start_image = Image::new_from (input.pixels, Pixel::Dark);
	let end_image = logic::image_iter (input.algorithm, start_image)
		.enumerate ()
		.inspect (|& (ref steps, ref image)| if args.verbose.unwrap_or (false) {
			println! (
				"\nAfter {steps} steps: {num_pixels} pixels {pixel}\n{image}",
				num_pixels = image.num_pixels (),
				pixel = match image.default () { Pixel::Dark => "light", Pixel::Light => "dark" });
		},
	).map (|(_, image)| image).nth (args.loops.unwrap_or (100)).unwrap ();
	println! ("Result: {}", end_image.num_pixels ());
	Ok (())
}
