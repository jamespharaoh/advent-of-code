//! Logic for solving the puzzles

use super::*;

use input::Input;
use model::Field;
use model::Ticket;
use model::Val;

pub fn part_one (input: & Input) -> GenResult <Val> {
	check_input (input) ?;
	let matches = get_matches (input);
	Ok (
		input.nearby_tickets.iter ()
			.flat_map (|ticket| ticket.iter ())
			.filter (|&& val| matches [val.as_usize ()] == 0)
			.sum ()
	)
}

pub fn part_two (input: & Input) -> GenResult <u64> {
	check_input (input) ?;
	let matches = get_matches (input);
	let tickets: Vec <& Ticket> =
		input.nearby_tickets.iter ()
			.filter (|ticket| ticket.iter ().all (|& val| matches [val.as_usize ()] != 0))
			.collect ();
	let field_idxes = get_field_idxes (& input.fields, & matches, & tickets);
	Ok (
		get_field_mappings (& field_idxes) ?.iter ()
			.enumerate ()
			.filter (|& (_, name)| name.starts_with ("departure "))
			.map (|(idx, _)| input.your_ticket [idx].as_u64 ())
			.try_fold (1, |prod, val| chk! (prod * val)) ?
	)
}

fn check_input (input: & Input) -> GenResult <()> {
	if 32 <= input.fields.len () {
		return Err ("Don't know how to handle more than 32 fields".into ());
	}
	for field in & input.fields { check_field (field) ?; }
	check_ticket (& input.your_ticket, input.fields.len ()) ?;
	for ticket in & input.nearby_tickets { check_ticket (ticket, input.fields.len ()) ?; }
	Ok (())
}

fn check_field (field: & Field) -> GenResult <()> {
	if ! (field.first.0 < field.first.1 && field.second.0 < field.second.1) {
		return Err ("Field value ranges must be from lower to higher value".into ());
	}
	Ok (())
}

fn check_ticket (ticket: & Ticket, num_fields: usize) -> GenResult <()> {
	if ticket.len () != num_fields {
		return Err ("Tickets must all have the same number of fields".into ());
	}
	if ticket.iter ().any (|& val| 1000 <= val) {
		return Err ("Field values must be less than 1000".into ());
	}
	Ok (())
}

fn get_field_mappings <'inp> (
	field_idxes: & [(InpStr <'inp>, u32)],
) -> GenResult <Vec <InpStr <'inp>>> {
	let mut pending = field_idxes.to_vec ();
	let mut ordered: Vec <Option <InpStr>> = vec! [ None; field_idxes.len () ];
	let mut remaining: u32 = u32::MAX >> (u32::BITS - field_idxes.len ().as_u32 ());
	while remaining != 0 {
		let mut progress = false;
		let mut error = false;
		pending.retain_mut (|& mut (ref name, ref mut idxes)| {
			* idxes &= remaining;
			if * idxes == 0 { error = true; return true }
			if 1 < idxes.count_ones () { return true }
			let idx = idxes.trailing_zeros ();
			ordered [idx.as_usize ()] = Some (name.clone ());
			remaining &= ! (1 << idx);
			progress = true;
			false
		});
		if ! progress || error { return Err ("No solution found".into ()) }
	}
	Ok (ordered.into_iter ().flatten ().collect ())
}

fn get_field_idxes <'inp> (
	fields: & [Field <'inp>],
	matches: & [u32],
	tickets: & [& Ticket],
) -> Vec <(InpStr <'inp>, u32)> {
	let mut temp = vec! [ u32::MAX >> (u32::BITS - fields.len ().as_u32 ()); fields.len () ];
	for ticket in tickets {
		for (& val, temp) in ticket.iter ().zip (temp.iter_mut ()) {
			* temp &= matches [val.as_usize ()];
		}
	}
	fields.iter ()
		.map (|field| field.name.clone ())
		.zip ((0 .. matches.len ())
			.map (|idx| temp.iter ().enumerate ()
				.filter (|& (_, & temp)| temp & 1 << idx != 0)
				.fold (0, |idxes, (idx, & _)| idxes | 1 << idx)))
		.collect ()
}

fn get_matches (input: & Input) -> Vec <u32> {
	let mut matches_temp = vec! [ (0, 0), (1000, 0) ];
	for (idx, field) in input.fields.iter ().enumerate () {
		let bit = 1 << idx;
		for (min, max) in [ field.first, field.second ] {
			let idx = match matches_temp.binary_search_by_key (& min, |& (val, _)| val) {
				Ok (idx) => { matches_temp [idx].1 |= bit; idx },
				Err (idx) => { matches_temp.insert (idx, (min, matches_temp [idx - 1].1 | bit)); idx },
			};
			let mut prev = matches_temp [idx].1;
			for (offset, & mut (val, ref mut fields)) in & mut matches_temp [idx .. ].iter_mut ().enumerate () {
				let idx = idx + offset;
				if val <= max { * fields |= bit; }
				if max < val {
					if val != max + 1 { matches_temp.insert (idx, (max + 1, prev & ! bit)); }
					break;
				}
				prev = * fields;
			}
		}
	}
	matches_temp.into_iter ()
		.tuple_windows ()
		.flat_map (|((val_0, fld_0), (val_1, _))|
			iter::repeat (fld_0).take ((val_1 - val_0).as_usize ()))
		.collect ()
}
