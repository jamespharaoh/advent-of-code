use std::io;
use std::io::Write as _;

use aoc_common::*;
use aoc_2021::*;

fn main () -> GenResult <()> {
	let name_len =
		puzzle_metadata ().iter ().map (|puzzle| puzzle.name ().len ()).max ().unwrap ();
	let flush = || io::stdout ().flush ();
	for puzzle in puzzle_metadata () {
		print! ("{:02}  {:name_len$}", puzzle.day (), puzzle.name (), name_len = name_len);
		for part in 0 .. puzzle.num_parts () {
			if part == 0 {
				print! ("  Part one: "); flush () ?;
				let result = puzzle.invoke_part_one () ?;
				print! ("{:14}", result);
			}
			if part == 1 {
				print! ("  Part two: "); flush () ?;
				let result = puzzle.invoke_part_two () ?;
				print! ("{:16}", result);
			}
		}
		print! ("\n");
	}
	Ok (())
}
