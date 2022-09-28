//! Logic for solving the puzzles

use super::*;

use input::Input;
use model::Dir;
use model::Pos;
use model::Step;

pub fn part_one (input: & Input) -> GenResult <u32> {
	let mut pos = Pos::ZERO;
	let mut dir = Dir::East;
	for & step in & input.steps {
		match step {
			Step::North (arg) => pos = (pos + (Dir::North, arg)) ?,
			Step::South (arg) => pos = (pos + (Dir::South, arg)) ?,
			Step::East (arg) => pos = (pos + (Dir::East, arg)) ?,
			Step::West (arg) => pos = (pos + (Dir::West, arg)) ?,
			Step::Left (90_i32) | Step::Right (270_i32) => dir = dir.left (),
			Step::Right (90_i32) | Step::Left (270_i32) => dir = dir.right (),
			Step::Left (180_i32) | Step::Right (180_i32) => dir = dir.around (),
			Step::Forwards (arg) => pos = (pos + (dir, arg)) ?,
			Step::Left (_) | Step::Right (_) =>
				return Err (format! ("Invalid step: {step:?}").into ()),
		}
	}
	Ok (pos.n.unsigned_abs ().pan_u32 () + pos.e.unsigned_abs ().pan_u32 ())
}

pub fn part_two (input: & Input) -> GenResult <u32> {
	let mut pos = Pos::ZERO;
	let mut way = Pos { n: 1_i32, e: 10_i32 };
	for & step in & input.steps {
		match step {
			Step::North (arg) => way = (way + (Dir::North, arg)) ?,
			Step::South (arg) => way = (way + (Dir::South, arg)) ?,
			Step::East (arg) => way = (way + (Dir::East, arg)) ?,
			Step::West (arg) => way = (way + (Dir::West, arg)) ?,
			Step::Left (90_i32) | Step::Right (270_i32) => way = way.left (),
			Step::Right (90_i32) | Step::Left (270_i32) => way = way.right (),
			Step::Left (180_i32) | Step::Right (180_i32) => way = way.around (),
			Step::Forwards (arg) => pos = pos.try_add (way.try_mul (arg) ?) ?,
			Step::Left (_) | Step::Right (_) =>
				return Err (format! ("Invalid step: {step:?}").into ()),
		}
	}
	Ok (pos.n.unsigned_abs ().pan_u32 () + pos.e.unsigned_abs ().pan_u32 ())
}
