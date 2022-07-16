//! Advent of Code 2021: Day 20: Trench Map
//!
//! [https://adventofcode.com/2021/day/20](https://adventofcode.com/2021/day/20)

use aoc_common::*;

puzzle_info! {
	name = "Trench Map";
	year = 2021;
	day = 20;
	part_one = |lines| logic::part_one (lines);
	part_two = |lines| logic::part_two (lines);
	commands = [
		( name = "run"; method = tool::run; ),
	];
}

mod logic {

	use super::*;
	use model::Algorithm;
	use model::Image;
	use model::Input;
	use model::Pixels;
	use model::Pos;

	pub fn part_one (lines: & [& str]) -> GenResult <i64> {
		calc_result (lines, 2)
	}

	pub fn part_two (lines: & [& str]) -> GenResult <i64> {
		calc_result (lines, 50)
	}

	pub fn calc_result (lines: & [& str], loops: usize) -> GenResult <i64> {
		let input = Input::parse (lines) ?;
		let image = Image::new_from (input.pixels, false);
		Ok (
			image_iter (input.algorithm, image)
				.skip (loops).next ().unwrap ().num_pixels () as i64
		)
	}

	pub struct ImageIter {
		algorithm: Algorithm,
		image: Rc <Image>,
		first: bool,
	}

	pub fn image_iter <IntoImage: Into <Rc <Image>>> (
		algorithm: Algorithm,
		image: IntoImage,
	) -> ImageIter {
		let image = image.into ();
		ImageIter { algorithm, image, first: true }
	}

	impl Iterator for ImageIter {
		type Item = Rc <Image>;
		fn next (& mut self) -> Option <Rc <Image>> {
			if self.first {
				self.first = false;
			} else {
				self.image = Rc::new (calc_next (& self.algorithm, & self.image));
			}
			Some (Rc::clone (& self.image))
		}
	}

	pub fn calc_next (algorithm: & Algorithm, image: & Image) -> Image {
		let (origin, peak) = image.range ();
		let new_pixels = (origin.y - 1 .. peak.y + 1).flat_map (|y|
			(origin.x - 3 .. peak.x + 1).map (move |x| [
				image.get (Pos { y: y - 1, x: x + 1 }),
				image.get (Pos { y: y, x: x + 1 }),
				image.get (Pos { y: y + 1, x: x + 1 }),
			]).scan ([false; 9], |state, next| {
				* state = [
					state [1], state [2], next [0],
					state [4], state [5], next [1],
					state [7], state [8], next [2],
				];
				Some (* state)
			}).skip (2).map (|bits| {
				let algorithm_idx = bits.into_iter ()
					.fold (0, |val, bit| (val << 1) | if bit { 1 } else { 0 });
				algorithm [algorithm_idx]
			})
		).collect ();
		let new_size = [image.height () + 2, image.width () + 2];
		let new_pixels = Pixels::wrap (new_pixels, [0, 0], new_size);
		let new_inverted = algorithm [if image.inverted () { 0x1ff } else { 0 }];
		Image::new_from (new_pixels, new_inverted)
	}

}

mod model {

	use super::*;

	pub type Algorithm = [bool; 512];
	pub type Pixels = grid::Grid <Vec <bool>, Pos>;
	pub type Pos = pos::PosYX <i16>;

	pub struct Input {
		pub algorithm: Algorithm,
		pub pixels: Pixels,
	}

	impl Input {
		pub fn parse (lines: & [& str]) -> GenResult <Input> {
			let err = |idx, msg| format! ("Invalid input: {}: {}", idx + 1, msg);
			let algorithm = {
				let line_0_len = lines [0].chars ().count ();
				if line_0_len != 512 {
					Err (err (0, format! ("Algorithm is {} chars, expected 512", line_0_len))) ?;
				}
				let mut algorithm = [false; 512];
				for (letter_idx, letter) in lines [0].chars ().enumerate () {
					match letter {
						'#' => algorithm [letter_idx] = true,
						'.' => (),
						_ => Err (err (0, format! ("Invalid character in algorithm"))) ?,
					}
				}
				algorithm
			};
			if ! lines [1].is_empty () { Err (err (1, format! ("Expected blank line"))) ?; }
			let pixels = lines.iter ().enumerate ().skip (2).map (|(line_idx, line)| {
				line.chars ().map (move |letter| { match letter {
					'#' => Ok (true),
					'.' => Ok (false),
					_ => Err (err (line_idx, format! ("Invalid character"))),
				}})
			}).flatten ().collect::<Result <_, _>> () ?;
			let size = [lines.len () - 2, lines [2].chars ().count ()];
			let pixels = Pixels::wrap (pixels, [0, 0], size);
			Ok (Input { algorithm, pixels })
		}
	}

	#[ derive (Debug) ]
	pub struct Image {
		pixels: Pixels,
		inverted: bool,
	}

	impl Image {
		pub fn new_from (pixels: Pixels, inverted: bool) -> Image {
			Image { pixels, inverted }
		}
		pub fn num_pixels (& self) -> usize {
			self.pixels.values ().filter (|& val| val != self.inverted).count ()
		}
		pub fn height (& self) -> usize { self.pixels.size () [0] }
		pub fn width (& self) -> usize { self.pixels.size () [1] }
		pub fn inverted (& self) -> bool { self.inverted }
		pub fn get (& self, pos: Pos) -> bool {
			self.pixels.get (pos).unwrap_or (self.inverted)
		}
		pub fn range (& self) -> (Pos, Pos) {
			(self.pixels.origin (), self.pixels.peak () + Pos { y: 1, x: 1 })
		}
		pub fn dump (& self) -> ImageDump {
			ImageDump (self)
		}
	}

	pub struct ImageDump <'a> (& 'a Image);

	impl <'a> fmt::Display for ImageDump <'a> {
		fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
			let ImageDump (image) = self;
			let (origin, peak) = image.range ();
			for y in origin.y .. peak.y {
				for x in origin.x .. peak.x {
					let pos = Pos { y, x };
					write! (formatter, "{}", if image.get (pos) { '#' } else { '.' }) ?;
				}
				write! (formatter, "\n") ?;
			}
			Ok (())
		}
	}

}

pub mod tool {

	use super::*;
	use model::Image;
	use model::Input;

	#[ derive (Debug, clap::Parser) ]
	pub struct RunArgs {

		/// File to read algorithm and initial image from
		#[ clap (long, value_parser, default_value = "inputs/day-20") ]
		input: String,

		/// Print the image after each step
		#[ clap (long) ]
		verbose: bool,

		/// Number of times to apply the algorithm
		#[ clap (long, value_parser, default_value_t = 2) ]
		loops: usize,

	}

	pub fn run (args: RunArgs) -> GenResult <()> {
		let input_string = fs::read_to_string (& args.input) ?;
		let input_lines: Vec <& str> = input_string.trim ().split ("\n").collect ();
		let input = Input::parse (& input_lines) ?;
		let start_image = Image::new_from (input.pixels, false);
		let end_image = logic::image_iter (input.algorithm, start_image)
			.enumerate ()
			.inspect (|(steps, image)| if args.verbose {
				print! (
					"After {} steps: {} pixels {}\n{}",
					steps,
					image.num_pixels (),
					if image.inverted () { "inactive" } else { "active" },
					image.dump (),
				);
			},
		).map (|(_, image)| image).skip (args.loops).next ().unwrap ();
		println! ("Result: {}", end_image.num_pixels ());
		Ok (())
	}

}

#[ cfg (test) ]
mod examples {

	use aoc_common::*;
	use crate::logic;

	const EXAMPLE: & 'static [& 'static str] = & [
		"..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###..\
		.####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..\
		#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.....\
		..#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.#\
		##.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##.\
		.####..#...#.#.#...##..#.#..###..#####........#..####......#..#",
		"",
		"#..#.",
		"#....",
		"##..#",
		"..#..",
		"..###",
	];

	#[ test ]
	fn part_one () -> GenResult <()> {
		assert_eq! (35, logic::part_one (EXAMPLE) ?);
		Ok (())
	}

	#[ test ]
	fn part_two () -> GenResult <()> {
		assert_eq! (3351, logic::part_two (EXAMPLE) ?);
		Ok (())
	}

}
