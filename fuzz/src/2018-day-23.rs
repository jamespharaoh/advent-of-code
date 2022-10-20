#![ no_main ]

use libfuzzer_sys::fuzz_target;

use aoc_2018::day_23::*;
use aoc_common::*;
use aoc_fuzz::*;

use input::Input;
use model::Coord;
use model::Nanobot;
use model::Pos;

fuzz_target! (|input_str: & str| {
	let input_vec: Vec <& str> = input_str.trim_end ().split ('\n').collect ();
	if let Ok (mut input) = Input::parse_from_lines (& input_vec) {
		input.params.max_iters.bounds_assign ( ..= 3000);
		let _ = logic::part_one (& input);
		let _ = logic::part_two (& input);
	}
});

aoc_fuzz_mutator! {

	transform_lifetimes = <'inp>;
	input_type = Input;

	transform add (1000 * 1, 200 * 10, 40 * 100) = |input, rng| {
		let nanobot = Nanobot {
			pos: Pos {
				x: rng.gen_range (Coord::MIN ..= Coord::MAX),
				y: rng.gen_range (Coord::MIN ..= Coord::MAX),
				z: rng.gen_range (Coord::MIN ..= Coord::MAX),
			},
			radius: rng.gen_range (0 ..= Coord::MAX),
		};
		let idx = rng.gen_range (0 ..= input.nanobots.len ());
		input.nanobots.insert (idx, nanobot);
	}

	pub transform remove (1000 * 1, 200 * 10, 40 * 100) = |input, rng| {
		if input.nanobots.is_empty () { return Some (()) }
		let idx = rng.gen_range (0 .. input.nanobots.len ());
		input.nanobots.remove (idx);
	}

	transform shuffle (1) = |input, rng| {
		input.nanobots.shuffle (rng);
	}

	transform sort (1) = |input, _rng| {
		input.nanobots.sort ();
	}

}
