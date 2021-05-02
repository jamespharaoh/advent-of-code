use modinverse::modinverse;
use std::fs;
use std::io;
use std::io::Write;
use std::iter;
use std::str::FromStr;

fn main () {
	let num_cards: u64 = 119315717514047;
	let input_string = fs::read_to_string ("input").unwrap ();
	let input_lines: Vec <& str> = input_string.split ('\n').collect ();
	let ops = parse_operations (& input_lines);
	let ops: Vec <Op> = ops.into_iter ().rev ().map (|op| op.reverse (num_cards)).collect ();
	let mut ops = optimize (& ops, num_cards);
	let mut card: u64 = 2020;
	let mut reps: u64 = 101741582076661;
	while reps > 0 {
		for _ in 0 .. reps & 1023 {
			for op in ops.iter ().cloned () {
				perform_operation (& mut card, num_cards, op);
			}
		}
		ops = iter::repeat_with (|| ops.iter ()).take (1024).flatten ().cloned ().collect ();
		ops = optimize (& ops, num_cards);
		reps /= 1024;
	}
	println! ("Original position: {}", card);
}

fn fresh_desk (num_cards: u64) -> Vec <u64> {
	(0 .. num_cards).collect ()
}

fn parse_operations <OpRef: AsRef <str>> (op_strs: & [OpRef]) -> Vec <Op> {
	let mut ops: Vec <Op> = Vec::new ();
	for op_str in op_strs {
		let op_str = op_str.as_ref ();
		if op_str.trim ().is_empty () { continue }
		let op: Op = op_str.parse ().unwrap ();
		ops.push (op);
	}
	ops
}

fn perform_operations <OpRef: AsRef <str>> (cards: & mut [u64], op_strs: & [OpRef], reverse: bool) {
	let num_cards = cards.len () as u64;
	let mut ops: Vec <Op> = Vec::new ();
	for op_str in op_strs {
		let op_str = op_str.as_ref ();
		if op_str.trim ().is_empty () { continue }
		let mut op: Op = op_str.parse ().unwrap ();
		if reverse { op = op.reverse (num_cards) }
		ops.push (op);
	}
	if reverse { ops.reverse () }
	let ops = optimize (& ops, num_cards);
	for op in ops.iter ().cloned () {
		for card in cards.iter_mut () {
			perform_operation (card, num_cards, op);
		}
	}
}

fn perform_operation (card: & mut u64, num_cards: u64, op: Op) {
	match op {
		Op::NegateModulo => {
			* card = num_cards - * card - 1;
		},
		Op::AddModulo (arg) => {
			* card = u64::checked_add (* card, arg).unwrap ().rem_euclid (num_cards);
		},
		Op::SubModulo (offset) => {
			let mut offset = offset as i64;
			while offset < 0 { offset += num_cards as i64 }
			let offset = num_cards - offset as u64;
			* card = (* card + offset).rem_euclid (num_cards);
		},
		Op::MultiplyModulo (step_by) => {
			let step_by = step_by as u64;
			* card = (
				u128::checked_mul (
					* card as u128,
					step_by as u128,
				).unwrap () % num_cards as u128
			) as u64;
		},
	}
}

fn place_cards (card_positions: & [u64]) -> Vec <u64> {
	let mut stack: Vec <u64> = vec! [0; card_positions.len ()];
	for (card, pos) in card_positions.iter ().enumerate () {
		stack [* pos as usize] = card as u64;
	}
	stack
}

fn optimize (ops: & [Op], num_cards: u64) -> Vec <Op> {
	let num_cards = num_cards as u128;
	let mut multiply: u128 = 1;
	let mut add: u128 = 0;
	let negate: u128 = num_cards as u128 - 1;
	for op in ops {
		match op {
			Op::NegateModulo => {
				multiply = u128::checked_mul (multiply, negate).unwrap ();
				add = u128::checked_mul (add, negate).unwrap ();
				add = u128::checked_add (add, negate).unwrap ();
			},
			Op::AddModulo (arg) => {
				add = u128::checked_add (add, * arg as u128).unwrap ();
			},
			Op::SubModulo (arg) => {
				let arg = (- arg).rem_euclid (num_cards as i64);
				add = u128::checked_add (add, arg as u128).unwrap ();
			},
			Op::MultiplyModulo (arg) => {
				multiply = u128::checked_mul (multiply, * arg as u128).unwrap ();
				add = u128::checked_mul (add, * arg as u128).unwrap ();
			},
		}
		multiply = multiply.rem_euclid (num_cards);
		add = add.rem_euclid (num_cards);
	}
	vec! [
		Op::MultiplyModulo (multiply as u64),
		Op::AddModulo (add as u64),
	]
}

#[ derive (Clone, Copy, Debug) ]
enum Op {
	NegateModulo,
	AddModulo (u64),
	SubModulo (i64),
	MultiplyModulo (u64),
}

impl Op {
	fn reverse (& self, num_cards: u64) -> Op {
		match self {
			Op::NegateModulo => Op::NegateModulo,
			Op::AddModulo (arg) => Op::AddModulo (num_cards - arg),
			Op::SubModulo (arg) => Op::SubModulo (- arg),
			Op::MultiplyModulo (incr) => Op::MultiplyModulo (modinverse (* incr as i64, num_cards as i64).unwrap () as u64),
		}
	}
}

impl FromStr for Op {
	type Err = String;
	fn from_str (source: & str) -> Result <Op, String> {
		if source == "deal into new stack" {
			Ok (Op::NegateModulo)
		} else if source.starts_with ("cut ") {
			Ok (Op::SubModulo (source [4 .. ].parse ().map_err (
				|error| format! ("Error parsing cut offset: {}", error),
			) ?))
		} else if source.starts_with ("deal with increment ") {
			Ok (Op::MultiplyModulo (source [20 .. ].parse ().map_err (
				|error| format! ("Error parsing operation: {}: {}", error, source),
			) ?))
		} else {
			Err (format! ("Unrecognised operation: {}", source))
		}
	}
}

#[ test ]
fn test_deal_into_new_stack () {
	let mut cards = place_cards (& vec! [9, 8, 7, 6, 5, 4, 3, 2, 1, 0]);
	perform_operations (& mut cards, & vec! ["deal into new stack"], true);
	assert_eq! (fresh_desk (10), place_cards (& cards));
}

#[ test ]
fn test_cut_positive () {
	let mut cards = place_cards (& vec! [3, 4, 5, 6, 7, 8, 9, 0, 1, 2]);
	perform_operations (& mut cards, & vec! ["cut 3"], true);
	assert_eq! (fresh_desk (10), place_cards (& cards));
}

#[ test ]
fn test_cut_negative () {
	let mut cards = place_cards (& vec! [6, 7, 8, 9, 0, 1, 2, 3, 4, 5]);
	perform_operations (& mut cards, & vec! ["cut -4"], true);
	assert_eq! (fresh_desk (10), place_cards (& cards));
}

#[ test ]
fn test_deal_with_increment () {
	let mut cards = place_cards (& vec! [0, 7, 4, 1, 8, 5, 2, 9, 6, 3]);
	perform_operations (& mut cards, & vec! ["deal with increment 3"], true);
	assert_eq! (fresh_desk (10), place_cards (& cards));
}

#[ test ]
fn test_multi_0 () {
	let mut cards = place_cards (& vec! [0, 3, 6, 9, 2, 5, 8, 1, 4, 7]);
	perform_operations (& mut cards, & vec! [
		"deal with increment 7",
		"deal into new stack",
		"deal into new stack",
	], true);
	assert_eq! (fresh_desk (10), place_cards (& cards));
}

#[ test ]
fn test_multi_1 () {
	let mut cards = place_cards (& vec! [3, 0, 7, 4, 1, 8, 5, 2, 9, 6]);
	perform_operations (& mut cards, & vec! [
		"cut 6",
		"deal with increment 7",
		"deal into new stack",
	], true);
	assert_eq! (fresh_desk (10), place_cards (& cards));
}

#[ test ]
fn test_multi_2 () {
	let mut cards = place_cards (& vec! [6, 3, 0, 7, 4, 1, 8, 5, 2, 9]);
	perform_operations (& mut cards, & vec! [
		"deal with increment 7",
		"deal with increment 9",
		"cut -2",
	], true);
	assert_eq! (fresh_desk (10), place_cards (& cards));
}

#[ test ]
fn test_multi_3 () {
	let mut cards = place_cards (& vec! [9, 2, 5, 8, 1, 4, 7, 0, 3, 6]);
	perform_operations (& mut cards, & vec! [
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
	], true);
	assert_eq! (fresh_desk (10), place_cards (& cards));
}
