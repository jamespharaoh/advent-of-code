use super::*;
use input::Input;
use model::Grouper;

pub fn part_one (input: & Input) -> GenResult <u32> {
	let grouper = Grouper::build (input);
	Ok (grouper.group_size (0).as_u32 ())
}

pub fn part_two (input: & Input) -> GenResult <u32> {
	let grouper = Grouper::build (input);
	Ok (grouper.groups ().count ().as_u32 ())
}
