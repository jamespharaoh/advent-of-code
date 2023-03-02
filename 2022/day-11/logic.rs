use super::*;

use input::Input;
use model::Item;

pub fn part_one (input: & Input) -> GenResult <u64> {
	check_input (input) ?;
	calc_result (input, input.params.rounds_one, input.params.div_one)
}

pub fn part_two (input: & Input) -> GenResult <u64> {
	check_input (input) ?;
	calc_result (input, input.params.rounds_two, input.params.div_two)
}

fn calc_result (input: & Input, num_rounds: u32, div: Item) -> GenResult <u64> {
	let modulo =
		div * input.monkeys.iter ()
			.fold (1, |modulo, monkey| Item::lcm (modulo, monkey.divisible_by));
	let mut monkey_items: Vec <Vec <Item>> =
		input.monkeys.iter ()
			.map (|monkey| monkey.start_items.clone ())
			.collect ();
	let mut inspections: Vec <u32> =
		vec! [0; monkey_items.len ()];
	for _ in 0 .. num_rounds {
		for monkey_id in 0 .. monkey_items.len ().pan_u8 () {
			let monkey = & input.monkeys [monkey_id.pan_usize ()];
			let items = mem::take (& mut monkey_items [monkey_id.pan_usize ()]);
			for item in items {
				inspections [monkey_id.pan_usize ()] += 1;
				let item = monkey.operation.apply (item) ? / div % modulo;
				let new_monkey_id =
					if item % monkey.divisible_by == 0 { monkey.throw_true }
					else { monkey.throw_false };
				assert_ne! (monkey_id, new_monkey_id);
				monkey_items [new_monkey_id.pan_usize ()].push (item);
			}
		}
	}
	inspections.sort_by_key (|& num| cmp::Reverse (num));
	Ok (chk! (inspections [0].pan_u64 () * inspections [1].pan_u64 ()) ?)
}

fn check_input (input: & Input) -> GenResult <()> {
	if input.monkeys.len () < 2 {
		return Err ("Must have at least two monkeys".into ());
	}
	for (monkey_idx, monkey) in input.monkeys.iter ().enumerate () {
		let monkey_id = monkey.id;
		if monkey_idx != monkey_id.pan_usize () {
			return Err (format! (
					"Monkey id not in order: index {monkey_idx}, id {monkey_id}")
				.into ());
		}
		if monkey_id == monkey.throw_true || monkey_id == monkey.throw_false {
			return Err (format! ("Monkey {monkey_id} throws to self").into ());
		}
		if input.monkeys.len () <= monkey.throw_true.pan_usize ()
				|| input.monkeys.len () <= monkey.throw_false.pan_usize () {
			return Err (format! ("Monkey {monkey_id} throws to invalid monkey").into ());
		}
	}
	Ok (())
}
