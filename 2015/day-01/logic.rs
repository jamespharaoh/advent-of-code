use super::*;

use input::Input;

pub fn part_one (input: & Input) -> GenResult <i32> {
	Ok (
		input.dirs.iter ().copied ().enumerate ()
			.scan (0_i32, |floor, (idx, dir)| {
				* floor += dir.val ();
				Some ((idx, * floor))
			})
			.last ()
			.map_or (0_i32, |(_, floor)| floor)
	)
}

pub fn part_two (input: & Input) -> GenResult <usize> {
	Ok (
		input.dirs.iter ().copied ().enumerate ()
			.scan (0_i32, |floor, (idx, dir)| {
				* floor += dir.val ();
				Some ((idx, * floor))
			})
			.find_map (|(idx, floor)| (floor < 0_i32).then_some (idx + 1))
			.ok_or ("Never visited the basement") ?
	)
}

#[ cfg (test) ]
mod tests {

	use super::*;

	use model::Dir::{ self, Down, Up };

	fn make_input (dirs: impl IntoIterator <Item = Dir>) -> Input {
		Input { dirs: dirs.into_iter ().collect (), params: default () }
	}

	#[ test ]
	fn part_one () {
		assert_eq_ok! (3, logic::part_one (& make_input ([Up, Up, Up])));
		assert_eq_ok! (-1, logic::part_one (& make_input ([Up, Down, Down])));
	}

	#[ test ]
	fn part_two () {
		assert_eq_ok! (3, logic::part_two (& make_input ([Up, Down, Down])));
		assert_err! ("Never visited the basement", logic::part_two (& make_input ([Up, Down])));
	}

}
