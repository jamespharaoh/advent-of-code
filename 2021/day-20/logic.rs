use super::*;

use input::Input;
use model::Algorithm;
use model::Image;
use model::Pixel;
use model::Pixels;
use model::Pos;

pub fn part_one (input: & Input) -> GenResult <i64> {
	calc_result (input, input.params.num_times_one)
}

pub fn part_two (input: & Input) -> GenResult <i64> {
	calc_result (input, input.params.num_times_two)
}

pub fn calc_result (input: & Input, loops: u32) -> GenResult <i64> {
	let image = Image::new_from (input.pixels.clone (), Pixel::Dark);
	Ok (
		image_iter (input.algorithm, image)
			.nth (loops.pan_usize ())
			.unwrap ()
			.num_pixels ().pan_i64 ()
	)
}

pub struct ImageIter {
	algorithm: Algorithm,
	image: Rc <Image>,
	first: bool,
}

#[ allow (clippy::large_types_passed_by_value) ]
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

#[ must_use ]
pub fn calc_next (algorithm: & Algorithm, image: & Image) -> Image {
	let (origin, peak) = image.range ();
	let new_pixels = (origin.y - 1 .. peak.y + 1)
		.flat_map (|y| (origin.x - 3 .. peak.x + 1)
			.map (move |x| [
				image.get (Pos { y: y - 1, x: x + 1 }),
				image.get (Pos { y, x: x + 1 }),
				image.get (Pos { y: y + 1, x: x + 1 }),
			])
			.scan ([Pixel::Dark; 9], |state, next| {
				* state = [
					state [1], state [2], next [0],
					state [4], state [5], next [1],
					state [7], state [8], next [2],
				];
				Some (* state)
			})
			.skip (2)
			.map (|bits| {
				let algorithm_idx = bits.into_iter ()
					.fold (0, |val, bit| (val << 1_i32) | usize::from (bit));
				algorithm [algorithm_idx]
			}))
		.collect ();
	let new_size = image.size () + Pos::new (2, 2);
	let new_pixels = Pixels::wrap_size (new_pixels, new_size);
	let new_default = algorithm [if image.default () == Pixel::Light { 0x1ff } else { 0 }];
	Image::new_from (new_pixels, new_default)
}
