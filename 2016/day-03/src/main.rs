use std::env;
use std::ffi::OsString;

use aoc_common::*;
use aoc_2016_day_03::*;

fn main () -> GenResult <()> {
	let args: Vec <OsString> = env::args_os ().collect ();
	puzzle_metadata ().invoke (& args)
}
