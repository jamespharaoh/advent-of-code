use std::collections::HashMap;

fn main() {
	let start: Vec <u64> = vec! [0, 8, 15, 2, 12, 1, 4];
	let nums = calculate (start, 2020);
	println! ("2020th number: {}", nums [2020 - 1]);
}

fn calculate (start: Vec <u64>, len: u64) -> Vec <u64> {
	let mut nums: Vec <u64> = Vec::new ();
	let mut positions: HashMap <u64, Vec <u64>> = HashMap::new ();
	for (idx, num) in start.iter ().cloned ().enumerate () {
		nums.push (num);
		positions.entry (num).or_insert (Vec::new ()).push (idx as u64);
	}
	for pos in positions.len () as u64 .. len {
		let last_num = nums.last ().cloned ().unwrap ();
		let last_posns = positions.entry (last_num).or_insert (Vec::new ());
		let this_num = if last_posns.len () >= 2 {
			last_posns [last_posns.len () - 1] - last_posns [last_posns.len () - 2]
		} else { 0 };
		let this_posns = positions.entry (this_num).or_insert (Vec::new ());
		nums.push (this_num);
		this_posns.push (pos);
	}
	nums
}

#[ test ]
fn test_0 () {
	let nums = calculate (vec! [0, 3, 6], 10);
	assert_eq! (nums, vec! [0, 3, 6, 0, 3, 3, 1, 0, 4, 0]);
}
