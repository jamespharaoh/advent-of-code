use super::*;

use std::path::PathBuf;

use input::Input;
use model::Image;
use model::Pixel;

#[ derive (Debug, clap::Parser) ]
pub struct RunArgs {

	/// File to read algorithm and initial image from
	#[ clap (long, from_global, value_parser = PathBuf) ]
	input: PathBuf,

	/// Print the image after each step
	#[ clap (long) ]
	verbose: bool,

	/// Number of times to apply the algorithm
	#[ clap (long, value_parser, default_value_t = 2) ]
	loops: usize,

}

#[ allow (clippy::needless_pass_by_value) ]
#[ allow (clippy::print_stdout) ]
pub fn run (args: RunArgs) -> GenResult <()> {
	let input_string = fs::read_to_string (& args.input) ?;
	let input_lines: Vec <& str> = input_string.trim ().split ('\n').collect ();
	let input = Input::parse_from_lines (& input_lines) ?;
	let start_image = Image::new_from (input.pixels, Pixel::Dark);
	let end_image = logic::image_iter (input.algorithm, start_image)
		.enumerate ()
		.inspect (|& (ref steps, ref image)| if args.verbose {
			println! (
				"\nAfter {steps} steps: {num_pixels} pixels {pixel}\n{image}",
				num_pixels = image.num_pixels (),
				pixel = match image.default () { Pixel::Dark => "light", Pixel::Light => "dark" });
		},
	).map (|(_, image)| image).nth (args.loops).unwrap ();
	println! ("Result: {}", end_image.num_pixels ());
	Ok (())
}
