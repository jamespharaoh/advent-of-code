use super::*;
use input::Input;

pub type Pos = pos::PosRowCol <u8>;
pub type Grid = GridBuf <Vec <bool>, Pos, 2>;

pub fn part_one (input: & Input) -> GenResult <u32> {
	Ok (
		calc_hashes (input).iter ()
			.map (|hash| hash.iter_vals ()
				.map (u8::count_ones)
				.sum::<u32> ())
			.sum ()
	)
}

pub fn part_two (input: & Input) -> GenResult <u32> {

	// calculate hashes and construct a grid

	let grid = Grid::wrap (
		calc_hashes (input).iter_vals ()
			.flat_map (move |hash| -> [bool; 128] {
				array::from_fn (|addr| {
					let idx = addr >> 3_u32;
					let mask = 0x80 >> (addr & 0x7);
					hash [idx] & mask != 0
				})
			})
			.collect (),
		Pos::ZERO,
		Pos::new (input.params.num_rows.pan_u8 (), 128));

	// iterate over positions, look for ones, track which we have seen already

	let mut seen = Grid::new (Pos::ZERO, grid.size ());
	let mut regions = 0;
	for pos in grid.iter ().map (|(pos, _)| pos) {
		if seen.get (pos).unwrap () { continue }
		if ! grid.get (pos).unwrap () { continue }
		seen.set (pos, true);

		// recursively iterate over adjacent unseen positions which are also ones, also mark them
		// as seen

		let mut todo = Vec::new ();
		todo.push (pos);
		while let Some (pos) = todo.pop () {
			for adj_pos in pos.adjacent_4 () {
				if seen.get (adj_pos).unwrap_or (true) { continue }
				seen.set (adj_pos, true);
				if ! grid.get (adj_pos).unwrap_or (false) { continue }
				todo.push (adj_pos);
			}
		}

		regions += 1;

	}

	Ok (regions)

}

fn calc_hashes (input: & Input) -> Vec <[u8; 16]> {
	(0_u32 .. input.params.num_rows)
		.map (|row| knot::calculate_rounds (
			format! ("{}-{}", input.key, row).as_bytes (),
			input.params.num_rounds))
		.collect ()
}
