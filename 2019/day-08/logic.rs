//! Logic for solving the puzzles

use super::*;
use input::Input;
use model::Pixel;

pub fn part_one (input: & Input) -> GenResult <u32> {
	let layers: Vec <Vec <Pixel>> =
		input.pixels.chunks (25 * 6)
			.map (<[Pixel]>::to_vec)
			.collect ();
	let most_black_idx =
		layers.iter ().enumerate ()
			.map (|(layer_idx, pixels)| (
				layer_idx,
				pixels.iter ().copied ()
					.filter (|& pixel| pixel == Pixel::Black)
					.count ()))
			.min_by_key (|& (_, num_black)| num_black)
			.map (|(idx, _)| idx)
			.ok_or ("No solution found") ?;
	let most_black_layer = & layers [most_black_idx];
	let (num_white, num_transparent) =
		most_black_layer.iter ().copied ()
			.fold ((0, 0), |(sum_white, sum_transparent), pixel| match pixel {
				Pixel::Black => (sum_white, sum_transparent),
				Pixel::White => (sum_white + 1, sum_transparent),
				Pixel::Transparent => (sum_white, sum_transparent + 1),
			});
	Ok (num_white * num_transparent)
}

pub fn part_two (input: & Input) -> GenResult <String> {
	let start: Vec <Pixel> = iter::repeat (Pixel::Transparent).take (25 * 6).collect ();
	let image: Vec <Pixel> =
		input.pixels.chunks (25 * 6)
			.fold (start, |mut image, layer| {
				for (img_pixel, & lyr_pixel) in Iterator::zip (image.iter_mut (), layer) {
					if * img_pixel != Pixel::Transparent { continue }
					* img_pixel = lyr_pixel;
				}
				image
			});
	let mut result = String::new ();
	for ch_idx in 0 .. 5 {
		let ch_str = ocr::read_auto (
			image.iter ().copied ().enumerate ()
				.filter_map (|(idx, val)| {
					let row = idx / 25;
					let col = idx % 25;
					if col / 5 != ch_idx { return None }
					match val {
						Pixel::Black => None,
						Pixel::White => Some ((row, col)),
						Pixel::Transparent => None,
					}
				})) ?;
		result += & ch_str;
	}
	Ok (result)
}
