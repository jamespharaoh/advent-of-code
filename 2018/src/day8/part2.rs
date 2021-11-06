use std::error::Error;
use std::iter;

pub fn aoc2018_day8_part2 (input: & str) -> Result <(), Box <dyn Error>> {

	let data: Vec <u64> = input.trim ().split (" ").map (
		|datum| Ok (datum.parse () ?),
	).collect::<Result <_, Box <dyn Error>>> () ?;

	struct Frame {
		num_children: u64,
		num_metadata: u64,
		child_values: Vec <u64>,
	}

	let mut stack: Vec <Frame> = vec! [
		Frame {
			num_children: 1,
			num_metadata: 1,
			child_values: Vec::new (),
		},
	];
	let data = {
		let mut data = data;
		data.push (1);
		data
	};

	let mut data_iter = data.iter ().cloned ();

	let value = loop {
		let frame = stack.last_mut ().unwrap ();
		if frame.num_children > 0 {
			frame.num_children -= 1;
			stack.push (Frame {
				num_children: data_iter.next ().unwrap (),
				num_metadata: data_iter.next ().unwrap (),
				child_values: Vec::new (),
			});
			continue;
		}
		let value = if frame.child_values.is_empty () {
			iter::repeat_with (
				|| data_iter.next ().unwrap (),
			).take (frame.num_metadata as usize).sum::<u64> ()
		} else {
			iter::repeat_with (
				|| data_iter.next ().unwrap (),
			).take (frame.num_metadata as usize).map (
				|idx| frame.child_values.get (idx as usize - 1).unwrap_or (& 0)
			).sum::<u64> ()
		};
		stack.pop ();
		if let Some (frame) = stack.last_mut () {
			frame.child_values.push (value);
		} else {
			break value;
		}
	};

	if data_iter.next ().is_some () {
		panic! ();
	}

	println! ("Puzzle answer: {}", value);

	Ok (())

}
