use super::*;

use input::Input;
use model::Command;

pub fn part_one (input: & Input) -> GenResult <i32> {
	let mut distance = 0_i32;
	let mut depth = 0_i32;
	for & cmd in & input.commands {
		match cmd {
			Command::Forward (val) => chk! (distance += val.pan_i32 ()) ?,
			Command::Down (val) => chk! (depth += val.pan_i32 ()) ?,
			Command::Up (val) => chk! (depth -= val.pan_i32 ()) ?,
		}
	}
	Ok (chk! (distance * depth) ?)
}

pub fn part_two (input: & Input) -> GenResult <i32> {
	let mut distance = 0_i32;
	let mut depth = 0_i32;
	let mut aim = 0_i32;
	for & cmd in & input.commands {
		match cmd {
			Command::Forward (val) => {
				chk! (distance += val.pan_i32 ()) ?;
				chk! (depth += aim * val.pan_i32 ()) ?;
			},
			Command::Down (val) => aim += val.pan_i32 (),
			Command::Up (val) => aim -= val.pan_i32 (),
		}
	}
	Ok (chk! (distance * depth) ?)
}
