use super::*;

use input::Input;
use input::Step;

pub fn part_one (input: & Input) -> GenResult <u32> {
	Ok (input.steps.iter ()
		.map (|step| hash (step.to_string ().as_bytes ()).pan_u32 ())
		.sum ())
}

pub fn part_two (input: & Input) -> GenResult <u32> {
	let mut boxes: Vec <Vec <(& str, u8)>> = vec! [Vec::new (); 256];
	for step in & input.steps {
		match * step {
			Step::Insert (ref name, lens) => {
				let name = name.as_str ();
				let contents = & mut boxes [hash (name.as_bytes ()).pan_usize ()];
				if let Some (idx) =
						contents.iter ().position (|& (box_name, _)| box_name == name) {
					contents [idx].1 = lens;
				} else {
					contents.push ((name, lens));
				}
			},
			Step::Remove (ref name) => {
				let name = name.as_str ();
				let contents = & mut boxes [hash (name.as_bytes ()).pan_usize ()];
				contents.retain (|& (box_name, _)| box_name != name);
			},
		}
	}
	Ok (boxes.iter ().enumerate ()
		.map (|(box_idx, contents)| {
			let box_num = box_idx.pan_u32 () + 1;
			contents.iter ().enumerate ()
				.map (|(lens_idx, & (_, lens))| {
					let slot_num = lens_idx.pan_u32 () + 1;
					let lens = lens.pan_u32 ();
					box_num * slot_num * lens
				})
				.sum::<u32> ()
		})
		.sum::<u32> ())
}

fn hash (bytes: & [u8]) -> u8 {
	bytes.iter ().fold (0, |prev, & val| prev.wrapping_add (val).wrapping_mul (17))
}
