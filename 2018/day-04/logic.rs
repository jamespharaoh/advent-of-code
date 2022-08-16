//! Logic for solving the puzzles

use super::*;
use input::Input;
use model::Event;

pub fn part_one (input: & Input) -> GenResult <u32> {
	let guard_min_sleep = analyse (input) ?;
	let mut guard_sleep: HashMap <u32, u32> = HashMap::new ();
	for (& (guard, _), & mins) in guard_min_sleep.iter () {
		* guard_sleep.entry (guard).or_insert (0) += mins;
	}
	let sleepiest_guard =
		guard_sleep.iter ()
			.max_by_key (|& (_, mins)| mins)
			.map (|(& guard, _)| guard)
			.ok_or ("No solution found") ?;
	let sleepiest_min =
		guard_min_sleep.iter ()
			.filter (|& (& (guard, _), _)| guard == sleepiest_guard)
			.max_by_key (|& (_, & mins)| mins)
			.map (|(& (_, min), _)| min)
			.unwrap ();
	Ok (sleepiest_guard * sleepiest_min)
}

pub fn part_two (input: & Input) -> GenResult <u32> {
	let guard_min_sleep = analyse (input) ?;
	let (guard, min) =
		guard_min_sleep.iter ()
			.max_by_key (|& (_, & mins)| mins)
			.map (|(& (guard, min), _)| (guard, min))
			.unwrap ();
	Ok (guard * min)
}

fn analyse (input: & Input) -> GenResult <HashMap <(u32, u32), u32>> {
	let mut entries = input.entries.clone ();
	entries.sort_by_key (|entry| (entry.date, entry.time));
	if entries.iter ()
			.tuple_windows::<(_, _)> ()
			.any (|(left, right)| (left.date, left.time) == (right.date, right.time)) {
		return Err ("Two entries share the same time".into ());
	}
	let mut guard_id = None;
	let mut fell_asleep_at = None;
	let mut result: HashMap <(u32, u32), u32> = HashMap::new ();
	for entry in entries.iter () {
		match entry.event {
			Event::BeginsShift (id) => {
				if fell_asleep_at.is_some () {
					return Err ("Begin shift while previous guard is asleep".into ());
				}
				guard_id = Some (id);
			},
			Event::FallsAsleep => {
				if guard_id.is_none () {
					return Err ("Falls asleep while no guard on duty".into ());
				}
				if fell_asleep_at.is_some () {
					return Err ("Falls asleep while already asleep".into ());
				}
				if entry.time.hour () != 0 {
					return Err ("Falls asleep after 01:00".into ());
				}
				fell_asleep_at = Some ((entry.date, entry.time));
			},
			Event::WakesUp => {
				if guard_id.is_none () {
					return Err ("Falls asleep while no guard on duty".into ());
				}
				let guard_id = guard_id.unwrap ();
				if fell_asleep_at.is_none () {
					return Err ("Wakes up while not asleep".into ());
				}
				if entry.time.hour () != 0 {
					return Err ("Wakes up after 01:00".into ());
				}
				let (fell_asleep_date, fell_asleep_time) = fell_asleep_at.take ().unwrap ();
				if fell_asleep_date != entry.date {
					return Err ("Falls asleep and wakes up on different days".into ());
				}
				for min in fell_asleep_time.minute () .. entry.time.minute () {
					* result.entry ((guard_id, min)).or_insert (0) += 1;
				}
			},
		}
	}
	Ok (result)
}
