//! Logic for solving the puzzles

use super::*;

use input::Input;
use model::Seat;

pub fn part_one (input: & Input) -> GenResult <u16> {
	Ok (input.seats.iter ().map (Seat::id).max ().ok_or ("Input is empty") ?)
}

pub fn part_two (input: & Input) -> GenResult <u16> {
	let seats: Vec <u16> =
		input.seats.iter ()
			.map (Seat::id)
			.sorted ()
			.collect ();
	let min = * seats.first ().ok_or ("Input is empty") ?;
	let max = * seats.last ().ok_or ("Input is empty") ?;
	Ok (
		seats.iter ().copied ()
			.zip (min ..= max)
			.find (|& (actual, expected)| actual != expected)
			.map (|(_, expected)| expected)
			.ok_or ("No solution found") ?
	)
}
