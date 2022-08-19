#![ no_main ]

use libfuzzer_sys::fuzz_target;

use aoc_2018::day_17::*;
use aoc_common::*;
use aoc_fuzz::*;

use input::Input;
use model::ClayRange;
use model::Coord;

fuzz_target! (|input_str: & str| {
	let input_vec: Vec <& str> = input_str.trim_end ().split ('\n').collect ();
	if let Ok (input) = Input::parse_from_lines (& input_vec) {
		let _ = logic::part_one (& input);
		let _ = logic::part_two (& input);
	}
});

const Y_START: Coord = 1;
const Y_END: Coord = 200;
const Y_LEN: Coord = 8;
const X_START: Coord = 451;
const X_END: Coord = 549;
const X_LEN: Coord = 20;

aoc_fuzz_mutator! {

	transform_lifetimes = <'inp>;
	input_type = Input;

	transform add (1000 * 1, 200 * 10, 40 * 100) = |input, rng| {
		let clay_range = match rng.gen_range (0 .. 2) {
			0 => {
				let x = rng.gen_range (X_START ..= X_END);
				let y_start = rng.gen_range (Y_START ..= Y_END - Y_LEN + 1);
				let y_end = rng.gen_range (y_start + 1 ..= Y_END);
				ClayRange::Vert { x, y_start, y_end }
			},
			1 => {
				let y = rng.gen_range (Y_START ..= Y_END);
				let x_start = rng.gen_range (X_START ..= X_END - X_LEN + 1);
				let x_end = rng.gen_range (x_start + 1 ..= X_END);
				ClayRange::Horiz { y, x_start, x_end }
			},
			_ => unreachable! (),
		};
		let new_idx = rng.gen_range (0 ..= input.clay_ranges.len ());
		input.clay_ranges.insert (new_idx, clay_range);
	}

	pub transform remove (1000 * 1, 200 * 10, 40 * 100) = |input, rng| {
		if input.clay_ranges.is_empty () { return Some (()) }
		let idx = rng.gen_range (0 .. input.clay_ranges.len ());
		input.clay_ranges.remove (idx);
	}

	pub transform squish (100 * 10, 10 * 100) = |input, rng| {
		if input.clay_ranges.is_empty () { return Some (()) }
		let y_max = input.clay_ranges.iter ().copied ()
			.map (|range| * range.y ().end ())
			.max ()
			.unwrap ();
		if y_max < 1 { return Some (()) }
		let cut_y = rng.gen_range (1 ..= y_max);
		input.clay_ranges = input.clay_ranges.iter ().copied ()
			.filter_map (|range| match range {
				ClayRange::Horiz { y, x_start, x_end } =>
					match Ord::cmp (& y, & cut_y) {
						Ordering::Less => Some (range),
						Ordering::Equal => None,
						Ordering::Greater => Some (ClayRange::Horiz { y: y - 1, x_start, x_end }),
					},
				ClayRange::Vert { x, y_start, y_end } =>
					(cut_y != y_start && cut_y != y_end).then_some (ClayRange::Vert {
						x,
						y_start: if cut_y < y_start { y_start - 1 } else { y_start },
						y_end: if cut_y <= y_start { y_start - 1 } else { y_start },
					}),
			})
			.collect ();
	}

	transform shuffle (1) = |input, rng| {
		input.clay_ranges.shuffle (rng);
	}

	transform sort (1) = |input, _rng| {
		input.clay_ranges.sort ();
	}

}
