use std::error::Error;
use std::iter;

pub fn aoc2018_day8_part1 (input: & str) -> Result <String, Box <dyn Error>> {

	let data: Vec <u32> = input.trim ().split (" ").map (
		|datum| Ok (datum.parse () ?),
	).collect::<Result <_, Box <dyn Error>>> () ?;

	struct Frame {
		num_children: u32,
		num_metadata: u32,
	}

	let mut stack: Vec <Frame> = vec! [
		Frame {
			num_children: 1,
			num_metadata: 0,
		},
	];

	let mut data_iter = data.iter ().cloned ();
	let mut metadata_sum: u32 = 0;

	while ! stack.is_empty () {
		let frame = stack.last_mut ().unwrap ();
		if frame.num_children > 0 {
			frame.num_children -= 1;
			stack.push (Frame {
				num_children: data_iter.next ().unwrap (),
				num_metadata: data_iter.next ().unwrap (),
			});
			continue;
		}
		metadata_sum += iter::repeat_with (
			|| data_iter.next ().unwrap (),
		).take (frame.num_metadata as usize).sum::<u32> ();
		stack.pop ();
	}

	if data_iter.next ().is_some () {
		panic! ();
	}

	Ok (format! ("{}", metadata_sum))

}
