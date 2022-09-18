use std::env;
use std::ffi::OsString;

use aoc_common::*;
use aoc_2020_day_09::*;

fn main () -> GenResult <()> {
	let args: Vec <OsString> = env::args_os ().collect ();
	puzzle_metadata ().invoke (& args)
}
