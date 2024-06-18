use super::*;

use input::Input;
use input::Monkey;
use input::MonkeyOp;

type Monkeys <'inp> = HashMap <& 'inp str, & 'inp Monkey <'inp>>;

pub fn part_one (input: & Input) -> GenResult <u64> {
	check_input (input) ?;
	let monkeys: Monkeys = 
		input.monkeys.iter ()
			.map (|monkey| (monkey.id.as_str (), monkey))
			.collect ();
	calc_result (& monkeys, & InpStr::borrow ("root"))
}

pub fn part_two (input: & Input) -> GenResult <u64> {
	check_input (input) ?;
	let monkeys: Monkeys =
		input.monkeys.iter ()
			.map (|monkey| (monkey.id.as_str (), monkey))
			.collect ();
	let [ left, right ] = monkey_op_args (& monkeys ["root"].op).unwrap ();
	let (mut id, mut target) = match (
			contains_humn (& monkeys, left),
			contains_humn (& monkeys, right)) {
		(true, false) => (left, calc_result (& monkeys, right) ?),
		(false, true) => (right, calc_result (& monkeys, left) ?),
		_ => unreachable! (),
	};
	while id != "humn" {
		let op = & monkeys [id].op;
		let [ left, right ] = monkey_op_args (op).unwrap ();
		let left_contains_humn = contains_humn (& monkeys, left);
		let right_contains_humn = contains_humn (& monkeys, right);
		let val = match (left_contains_humn, right_contains_humn) {
			(true, false) => calc_result (& monkeys, right) ?,
			(false, true) => calc_result (& monkeys, left) ?,
			_ => unreachable! (),
		};
		(id, target) = match (left_contains_humn, right_contains_humn, op) {
			(true, false, & MonkeyOp::Add (_, _)) => (left, chk! (target - val) ?),
			(false, true, & MonkeyOp::Add (_, _)) => (right, chk! (target - val) ?),
			(true, false, & MonkeyOp::Sub (_, _)) => (left, chk! (target + val) ?),
			(false, true, & MonkeyOp::Sub (_, _)) => (right, chk! (val - target) ?),
			(true, false, & MonkeyOp::Mul (_, _)) => (left, chk! (target / val) ?),
			(false, true, & MonkeyOp::Mul (_, _)) => (right, chk! (target / val) ?),
			(true, false, & MonkeyOp::Div (_, _)) => (left, chk! (target * val) ?),
			(false, true, & MonkeyOp::Div (_, _)) => (right, chk! (val / target) ?),
			_ => unreachable! (),
		}
	}
	Ok (target)
}

fn check_input (input: & Input) -> GenResult <()> {
	let mut decls = HashSet::new ();
	let mut refs = HashSet::new ();
	for monkey in & input.monkeys {
		if monkey.id == "root" && matches! (monkey.op, MonkeyOp::Number (_)) {
			return Err ("Monkey root must have an operation to perform".into ());
		}
		if monkey.id == "humn" && ! matches! (monkey.op, MonkeyOp::Number (_)) {
			return Err ("Monkey human must have a number to yell".into ());
		}
		if ! decls.insert (monkey.id.as_str ()) {
			return Err (format! ("Monkey declared twice: {}", monkey.id).into ());
		}
		let monkey_refs = match monkey.op {
			MonkeyOp::Number (_) => continue,
			MonkeyOp::Add (ref left, ref right) => [ left.as_str (), right.as_str () ],
			MonkeyOp::Sub (ref left, ref right) => [ left.as_str (), right.as_str () ],
			MonkeyOp::Mul (ref left, ref right) => [ left.as_str (), right.as_str () ],
			MonkeyOp::Div (ref left, ref right) => [ left.as_str (), right.as_str () ],
		};
		for monkey_ref in monkey_refs {
			if ! refs.insert (monkey_ref) {
				return Err (format! ("Monkey referenced twice: {monkey_ref}").into ());
			}
		}
	}
	for ref_ in & refs {
		if ! decls.contains (ref_) {
			return Err (format! ("Monkey referenced does noot exist: {ref_}").into ());
		}
	}
	for decl in & decls {
		if * decl == "root" { continue }
		if ! refs.contains (& ** decl) {
			return Err (format! ("Monkey is never referenced does not exist: {decl}").into ());
		}
	}
	if ! decls.contains ("root") {
		return Err ("Monkey root must exist".into ());
	}
	if ! decls.contains ("humn") {
		return Err ("Monkey humn must exist".into ());
	}
	Ok (())
}

fn calc_result (monkeys: & Monkeys, id: & str) -> GenResult <u64> {
	let monkey = monkeys [id];
	match monkey.op {
		MonkeyOp::Number (val) => Ok (val.pan_u64 ()),
		MonkeyOp::Add (ref left, ref right) =>
			Ok (chk! (calc_result (monkeys, left) ? + calc_result (monkeys, right) ?) ?),
		MonkeyOp::Sub (ref left, ref right) =>
			Ok (chk! (calc_result (monkeys, left) ? - calc_result (monkeys, right) ?) ?),
		MonkeyOp::Mul (ref left, ref right) =>
			Ok (chk! (calc_result (monkeys, left) ? * calc_result (monkeys, right) ?) ?),
		MonkeyOp::Div (ref left, ref right) =>
			Ok (chk! (calc_result (monkeys, left) ? / calc_result (monkeys, right) ?) ?),
	}
}

fn contains_humn (monkeys: & Monkeys, id: & str) -> bool {
	let mut todo = vec! [ id ];
	while let Some (id) = todo.pop () {
		if id == "humn" { return true }
		if let Some ([ left, right ]) = monkey_op_args (& monkeys [id].op) {
			todo.push (left);
			todo.push (right);
		}
	}
	false
}

fn monkey_op_args <'inp> (op: & 'inp MonkeyOp <'inp>) -> Option <[& 'inp str; 2]> {
	match * op {
		MonkeyOp::Number (_) => None,
		MonkeyOp::Add (ref left, ref right) => Some ([ left.as_str (), right.as_str () ]),
		MonkeyOp::Sub (ref left, ref right) => Some ([ left.as_str (), right.as_str () ]),
		MonkeyOp::Mul (ref left, ref right) => Some ([ left.as_str (), right.as_str () ]),
		MonkeyOp::Div (ref left, ref right) => Some ([ left.as_str (), right.as_str () ]),
	}
}
