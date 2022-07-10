use aoc_common::*;

puzzle! {
	name = "Trench Map";
	year = 2021;
	day = 20;
	part_one = |lines| logic::calc_result_part_one (lines);
	part_two = |lines| logic::calc_result_part_two (lines);
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
					image.len (),
					if image.inverted () { "inactive" } else { "active" },
					image.dump (),
				);
			},
		).map (|(_, image)| image).skip (args.loops).next ().unwrap ();
		println! ("Result: {}", end_image.len ());
		Ok (())
	}

}

mod logic {

	use super::*;
	use model::Algorithm;
	use model::Image;
	use model::Input;
	use model::Pos;

	pub fn calc_result_part_one (lines: & [& str]) -> GenResult <i64> {
		calc_result (lines, 2)
	}

	pub fn calc_result_part_two (lines: & [& str]) -> GenResult <i64> {
		calc_result (lines, 50)
	}

	pub fn calc_result (lines: & [& str], loops: usize) -> GenResult <i64> {
		let input = Input::parse (lines) ?;
		Ok (
			image_iter (input.algorithm, Image::new_from (input.pixels, false))
				.skip (loops).next ().unwrap ().len () as i64
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
		let mut new_image = Image::new (algorithm [if image.inverted () { 0x1ff } else { 0 }]);
		for row in origin.row - 1 .. peak.row + 1 {
			for col in origin.col - 1 .. peak.col + 1 {
				let pos = Pos { row, col };
				let algorithm_idx = {
					let mut algorithm_idx: usize = 0;
					for bit_pos in [
						Pos { row: row - 1, col: col - 1 },
						Pos { row: row - 1, col: col },
						Pos { row: row - 1, col: col + 1 },
						Pos { row: row, col: col - 1 },
						Pos { row: row, col: col },
						Pos { row: row, col: col + 1 },
						Pos { row: row + 1, col: col - 1 },
						Pos { row: row + 1, col: col },
						Pos { row: row + 1, col: col + 1 },
					] {
						let bit = if image.get (bit_pos) { 1 } else { 0 };
						algorithm_idx = algorithm_idx << 1 | bit;
					}
					algorithm_idx
				};
				new_image.set (pos, algorithm [algorithm_idx]);
			}
		}
		new_image
	}

}

mod model {

	use super::*;

	pub type Algorithm = [bool; 512];

	pub struct Input {
		pub algorithm: Algorithm,
		pub pixels: HashSet <Pos>,
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
				let row = line_idx as i16 - 2;
				line.chars ().enumerate ().map (move |(col, letter)| {
					let col = col as i16;
					let pos = Pos { row, col };
					match letter {
						'#' => Ok (Some (pos)),
						'.' => Ok (None),
						_ => Err (err (line_idx, format! ("Invalid character"))),
					}
				})
			}).flatten ().filter_map_ok (|pos| pos).collect::<Result <_, _>> () ?;
			Ok (Input { algorithm, pixels })
		}
	}

	#[ derive (Clone, Copy, Debug, Eq, Hash, PartialEq) ]
	pub struct Pos { pub row: i16, pub col: i16 }

	impl Pos {
		const ZERO: Pos = Pos { row: 0, col: 0 };
	}

	#[ derive (Debug) ]
	pub struct Image {
		pixels: HashSet <Pos>,
		inverted: bool,
	}

	impl Image {
		pub fn new (inverted: bool) -> Image {
			let pixels = HashSet::new ();
			Image { pixels, inverted }
		}
		pub fn new_from (pixels: HashSet <Pos>, inverted: bool) -> Image {
			Image { pixels, inverted }
		}
		pub fn len (& self) -> usize { self.pixels.len () }
		pub fn inverted (& self) -> bool { self.inverted }
		pub fn get (& self, pos: Pos) -> bool {
			self.pixels.contains (& pos) ^ self.inverted
		}
		pub fn set (& mut self, pos: Pos, val: bool) {
			if val ^ self.inverted {
				self.pixels.insert (pos);
			} else {
				self.pixels.remove (& pos);
			}
		}
		pub fn range (& self) -> (Pos, Pos) {
			self.pixels.iter ().fold ((Pos::ZERO, Pos::ZERO),
				|(origin, peak), pixel| (
					Pos {
						row: cmp::min (origin.row, pixel.row),
						col: cmp::min (origin.col, pixel.col),
					},
					Pos {
						row: cmp::max (peak.row, pixel.row + 1),
						col: cmp::max (peak.col, pixel.col + 1),
					},
				),
			)
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
			for row in origin.row .. peak.row {
				for col in origin.col .. peak.col {
					let pos = Pos { row, col };
					write! (formatter, "{}", if image.get (pos) { '#' } else { '.' }) ?;
				}
				write! (formatter, "\n") ?;
			}
			Ok (())
		}
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
		assert_eq! (35, logic::calc_result_part_one (EXAMPLE) ?);
		Ok (())
	}

	#[ test ]
	fn part_two () -> GenResult <()> {
		assert_eq! (3351, logic::calc_result_part_two (EXAMPLE) ?);
		Ok (())
	}

}
