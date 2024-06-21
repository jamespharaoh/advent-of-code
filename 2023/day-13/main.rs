use std::env;
use std::ffi::OsString;

use aoc_common::*;
use aoc_2023_day_13::*;

fn main () -> GenResult <()> {
	let args: Vec <OsString> = env::args_os ().collect ();
	puzzle_metadata ().invoke (& args)
}
