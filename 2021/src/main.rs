use std::io;
use std::io::Write as _;

use aoc_common::*;
use aoc_2021::*;

fn main () -> GenResult <()> {
	let name_len =
		puzzle_metadata ().iter ().map (|puzzle| puzzle.name ().len ()).max ().unwrap ();
	let flush = || io::stdout ().flush ();
	for puzzle in puzzle_metadata () {
		let input_string = puzzle.load_input () ?;
		let input_lines: Vec <& str> = input_string.trim ().split ('\n').collect ();
		print! ("{:02}  {:name_len$}", puzzle.day (), puzzle.name (), name_len = name_len);
		let start_time = time::Instant::now ();
		for part in 0 .. 2 {
			if puzzle.num_parts () < part + 1 {
				print! ("{:24}", "");
				continue;
			}
			if part == 0 {
				print! ("  One: "); flush () ?;
				let result = puzzle.part_one (& input_lines) ?;
				print! ("{:17}", result);
			}
			if part == 1 {
				print! ("  Two: "); flush () ?;
				let result = puzzle.part_two (& input_lines) ?;
				print! ("{:17}", result);
			}
		}
		let end_time = time::Instant::now ();
		let duration = end_time - start_time;
		print! ("Time: {:>4}.{:02}ms", duration.as_millis (), (duration.as_micros () % 1000) / 10);
		print! ("\n");
	}
	Ok (())
}
