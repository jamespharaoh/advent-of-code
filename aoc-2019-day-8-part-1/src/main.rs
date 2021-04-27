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
	for layer in layers.into_iter () {
		let num_zeros = layer.iter ().filter (
			|val| ** val == 0,
		).count ();
		if num_zeros < least_zeros_count {
			least_zeros_layer = layer;
			least_zeros_count = num_zeros;
		}
	}

	let num_ones = least_zeros_layer.iter ().filter (
		|val| ** val == 1,
	).count ();
	let num_twos = least_zeros_layer.iter ().filter (
		|val| ** val == 2,
	).count ();
	println! ("Checksum: {}", num_ones * num_twos);

}
