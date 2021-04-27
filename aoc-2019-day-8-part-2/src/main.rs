use std::iter;
use std::fs;

fn main () {

	let width = 25;
	let height = 6;

	let input_str = fs::read_to_string ("input").unwrap ();
	let input_raw: Vec <u8> = input_str.trim ().chars ().map (
		|ch| ch as u8 - '0' as u8,
	).collect ();
	let layers: Vec <Vec <u8>> = input_raw.chunks (width * height).map (
		|chunk| chunk.to_owned (),
	).collect ();

	let mut least_zeros_layer = Vec::new ();
	let mut least_zeros_count = usize::MAX;
	for layer in layers.iter () {
		let num_zeros = layer.iter ().filter (
			|val| ** val == 0,
		).count ();
		if num_zeros < least_zeros_count {
			least_zeros_layer = layer.clone ();
			least_zeros_count = num_zeros;
		}
	}

	let mut combined: Vec <u8> = iter::repeat (2).take (width * height).collect ();
	for layer in layers {
		for (index, value) in layer.iter ().enumerate () {
			if combined [index] != 2 {
				continue;
			}
			combined [index] = * value;
		}
	}

	for row in 0 .. height {
		for col in 0 .. width {
			match combined [row * width + col] {
				0 => print! (" "),
				1 => print! ("█"),
				_ => panic! (),
			}
		}
		print! ("\n");
	}

}
