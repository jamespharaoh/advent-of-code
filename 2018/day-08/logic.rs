//! Logic for solving the puzzles

use super::*;
use input::Input;

pub fn part_one (input: & Input) -> GenResult <u32> {

	struct Frame {
		num_children: u8,
		num_metadata: u8,
	}

	let mut stack: Vec <Frame> = vec! [
		Frame {
			num_children: 1,
			num_metadata: 0,
		},
	];

	let mut data_iter = input.data.iter_vals ();
	let mut metadata_sum: u32 = 0;

	while ! stack.is_empty () {
		let frame = stack.last_mut ().unwrap ();
		if frame.num_children > 0 {
			frame.num_children -= 1;
			stack.push (Frame {
				num_children: data_iter.next ().ok_or ("Invalid data") ?,
				num_metadata: data_iter.next ().ok_or ("Invalid data") ?,
			});
			continue;
		}
		metadata_sum += data_iter.by_ref ()
			.take (frame.num_metadata.as_usize ())
			.map (IntConv::as_u32)
			.sum::<u32> ();
		stack.pop ();
	}

	if data_iter.next ().is_some () {
		return Err ("Invalid data".into ());
	}

	Ok (metadata_sum)

}

pub fn part_two (input: & Input) -> GenResult <u32> {

	struct Frame {
		num_children: u8,
		num_metadata: u8,
		child_values: Vec <u32>,
	}

	let mut stack: Vec <Frame> = vec! [
		Frame {
			num_children: 1,
			num_metadata: 1,
			child_values: Vec::new (),
		},
	];

	let mut data = input.data.clone ();
	data.push (1);

	let mut data_iter = data.iter_vals ();

	let value = loop {
		let frame = stack.last_mut ().unwrap ();
		if frame.num_children > 0 {
			frame.num_children -= 1;
			stack.push (Frame {
				num_children: data_iter.next ().ok_or ("Invalid data") ?,
				num_metadata: data_iter.next ().ok_or ("Invalid data") ?,
				child_values: Vec::new (),
			});
			continue;
		}
		let value = if frame.child_values.is_empty () {
			data_iter.by_ref ()
				.take (frame.num_metadata.as_usize ())
				.map (IntConv::as_u32)
				.sum::<u32> ()
		} else {
			data_iter.by_ref ()
				.take (frame.num_metadata.as_usize ())
				.map (|idx| {
					let idx = usize::sub_2 (idx.as_usize (), 1) ?;
					Ok::<_, Overflow> (frame.child_values.get (idx).copied ())
				})
				.flatten_ok ()
				.map_ok (IntConv::as_u32)
				.fold_ok (0, |sum, item| sum + item) ?
		};
		stack.pop ();
		if let Some (frame) = stack.last_mut () {
			frame.child_values.push (value);
		} else {
			break value;
		}
	};

	if data_iter.next ().is_some () {
		return Err ("Invalid data".into ());
	}

	Ok (value)

}
