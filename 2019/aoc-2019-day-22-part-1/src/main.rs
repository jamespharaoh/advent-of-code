use std::fs;
use std::mem;
use std::str::FromStr;

fn main () {
	let input_string = fs::read_to_string ("input").unwrap ();
	let input_lines: Vec <& str> = input_string.split ('\n').collect ();
	let stack = perform_operations (10007, & input_lines);
	let card_2019_pos = stack.iter ().position (|card| * card == 2019).unwrap ();
	println! ("Card 2019 card is at position {}", card_2019_pos);
}

fn perform_operations <OpRef: AsRef <str>> (stack_size: u64, op_strs: & [OpRef]) -> Vec <u64> {
	let mut stack: Vec <u64> = (0 .. stack_size).collect ();
	for op_str in op_strs {
		let op_str = op_str.as_ref ();
		if op_str.trim ().is_empty () { continue }
		let op: Op = op_str.parse ().unwrap ();
		perform_operation (& mut stack, op);
	}
	stack
}

fn perform_operation (stack: & mut Vec <u64>, op: Op) {
	match op {
		Op::DealIntoNewStack => stack.reverse (),
		Op::Cut (offset) => {
			let mut offset = offset as isize;
			while offset < 0 { offset += stack.len () as isize }
			let mut offset = offset as usize;
			let mut old_stack = Vec::with_capacity (stack.len ());
			mem::swap (& mut old_stack, stack);
			for _ in 0 .. old_stack.len () {
				stack.push (old_stack [offset]);
				offset = (offset + 1) % (old_stack.len ());
			}
		},
		Op::DealWithIncrement (step_by) => {
			let step_by = step_by as usize;
			let mut offset: usize = 0;
			let mut old_stack: Vec <u64> = vec! [0; stack.len ()];
			mem::swap (& mut old_stack, stack);
			for card in old_stack.iter ().cloned () {
				stack [offset] = card;
				offset = (offset + step_by) % (old_stack.len ());
			}
		},
	}
}

#[ derive (Debug) ]
enum Op {
	DealIntoNewStack,
	Cut (i64),
	DealWithIncrement (u64),
}

impl FromStr for Op {
	type Err = String;
	fn from_str (source: & str) -> Result <Op, String> {
		if source == "deal into new stack" {
			Ok (Op::DealIntoNewStack)
		} else if source.starts_with ("cut ") {
			Ok (Op::Cut (source [4 .. ].parse ().map_err (
				|error| format! ("Error parsing cut offset: {}", error),
			) ?))
		} else if source.starts_with ("deal with increment ") {
			Ok (Op::DealWithIncrement (source [20 .. ].parse ().map_err (
				|error| format! ("Error parsing operation: {}: {}", error, source),
			) ?))
		} else {
			Err (format! ("Unrecognised operation: {}", source))
		}
	}
}

#[ test ]
fn test_deal_into_new_stack () {
	assert_eq! (
		vec! [9, 8, 7, 6, 5, 4, 3, 2, 1, 0],
		perform_operations (10, & vec! ["deal into new stack"]),
	);
}

#[ test ]
fn test_cut_positive () {
	assert_eq! (
		vec! [3, 4, 5, 6, 7, 8, 9, 0, 1, 2],
		perform_operations (10, & vec! ["cut 3"]),
	);
}

#[ test ]
fn test_cut_negative () {
	assert_eq! (
		vec! [6, 7, 8, 9, 0, 1, 2, 3, 4, 5],
		perform_operations (10, & vec! ["cut -4"]),
	);
}

#[ test ]
fn test_deal_with_increment () {
	assert_eq! (
		vec! [0, 7, 4, 1, 8, 5, 2, 9, 6, 3],
		perform_operations(10, & vec! ["deal with increment 3"]),
	);
}

#[ test ]
fn test_multi_0 () {
	assert_eq! (
		vec! [0, 3, 6, 9, 2, 5, 8, 1, 4, 7],
		perform_operations (10, & vec! [
			"deal with increment 7",
			"deal into new stack",
			"deal into new stack",
		]),
	);
}

#[ test ]
fn test_multi_1 () {
	assert_eq! (
		vec! [3, 0, 7, 4, 1, 8, 5, 2, 9, 6],
		perform_operations (10, & vec! [
			"cut 6",
			"deal with increment 7",
			"deal into new stack",
		]),
	);
}

#[ test ]
fn test_multi_2 () {
	assert_eq! (
		vec! [6, 3, 0, 7, 4, 1, 8, 5, 2, 9],
		perform_operations (10, & vec! [
			"deal with increment 7",
			"deal with increment 9",
			"cut -2",
		]),
	);
}

#[ test ]
fn test_multi_3 () {
	assert_eq! (
		vec! [9, 2, 5, 8, 1, 4, 7, 0, 3, 6],
		perform_operations (10, & vec! [
			"deal into new stack",
			"cut -2",
			"deal with increment 7",
			"cut 8",
			"cut -4",
			"deal with increment 7",
			"cut 3",
			"deal with increment 9",
			"deal with increment 3",
			"cut -1",
		]),
	);
}
