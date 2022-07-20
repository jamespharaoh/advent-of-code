#![ allow (clippy::print_with_newline) ]
#![ allow (clippy::vec_init_then_push) ]

use super::*;

#[ derive (Debug, clap::Parser) ]
pub struct Args {

	#[ clap (required = true) ]
	state: String,

	#[ clap (long, default_value = "1000") ]
	loops: u16,

	#[ clap (long, default_value = "65536") ]
	max_length: usize,

	#[ clap (long, default_value = "10") ]
	min_points: u16,

}

pub fn run (args: Args) -> GenResult <()> {
	let mut states = Vec::new ();
	states.push (State::parse (& args.state) ?);
	for cur_gen in 0u16 .. {
		if cur_gen == args.loops { break }
		let cur_state = states.last ().unwrap ();
		let next_state = {
			let mut items_iter = cur_state.items.iter ().copied ().enumerate ().multipeek ();
			let mut buffer = None;
			let mut item_idx = 0;
			iter::from_fn (move || {
				items_iter.reset_peek ();
				let item_0 = items_iter.peek ().copied ();
				let item_1 = items_iter.peek ().copied ();
				// deterine num to output
				let num = match (buffer, item_0) {
					(Some (buf), _) => buf,
					(None, Some ((_, Item { run, .. }))) => run,
					(None, None) => return None,
				};
				// measure size of run
				let (next_0, next_1) = match item_0 {
					Some ((_, Item { run, num, .. })) => (Some (run), Some (num)),
					None => (None, None),
				};
				let (next_2, next_3) = match item_1 {
					Some ((_, Item { run, num, .. })) => (Some (run), Some (num)),
					None => (None, None),
				};
				let buffer_run = buffer.iter ().count ();
				let next_run =
					[next_0, next_1, next_2, next_3].into_iter ()
						.take_while (|& next| next == Some (num))
						.count ();
				let run: u8 = (buffer_run + next_run).try_into ().unwrap ();
				//println! ("buf={:?} next=[{:?} {:?} {:?} {:?}] num={} run={}+{}={}", buffer, next_0, next_1, next_2, next_3, num, buffer_run, next_run, run);
				// work out gen and index
				let (gen, idx) = if buffer.is_some () {
					(cur_gen + 1, item_idx)
				} else {
					let (_, item_0) = item_0.unwrap ();
					(item_0.gen, item_0.idx)
				};
				// update state
				let (iter_adv, buffer_new) = match next_run {
					0 => (0, None),
					1 => (1, Some (next_1.unwrap ())),
					2 => (1, None),
					3 => (2, Some (next_3.unwrap ())),
					_ => panic! ("Found run of {} + {} = {}", buffer_run, next_run, run),
				};
				for _ in 0 .. iter_adv { items_iter.next ().unwrap (); }
				buffer = buffer_new;
				item_idx += 1;
				// produce value
				Some (Item { run, num, gen, idx })
			})
		}.take (args.max_length * 2 / 3).collect ();
		states.push (next_state);
	}
	// print states
	for (state_gen, state) in states.iter ().enumerate () {
		let state_gen = state_gen as u16;
		let mut align_iter =
			states.last ().unwrap ().items.iter ().copied ()
				.map (|Item { gen, idx, .. }| (gen, idx))
				.enumerate ()
				.map (|(col, (gen, idx))| ((gen, idx), col * 3))
				.filter (|& ((gen, _), _)| gen <= state_gen);
		let (mut align_key, mut align_col) = align_iter.next ().unwrap ();
		let mut col = 0;
		for & Item { gen, idx, run, num } in state.items.iter () {
			let item_key = (gen, idx);
			if item_key == align_key {
				while col < align_col { print! (" "); col += 1; }
				print! ("|"); col += 1;
				(align_key, align_col) = align_iter.next ().unwrap_or (((0, 0), 0));
			}
			//print! ("[{}:{}]", gen, idx);
			print! ("{}{}", run, num); col += 2;
		}
		print! ("\n");
	}
	Ok (())
}

pub struct State {
	items: Vec <Item>,
}

impl State {
	pub fn parse (input: & str) -> GenResult <Self> {
		let state = model::State::parse (input) ?.try_into () ?;
		Ok (state)
	}
}

impl FromIterator <Item> for State {
	fn from_iter <IntoIter> (iter: IntoIter) -> State
			where IntoIter: IntoIterator <Item = Item> {
		State {
			items: iter.into_iter ().collect (),
		}
	}
}

impl TryFrom <model::State> for State {
	type Error = GenError;
	fn try_from (state: model::State) -> GenResult <State> {
		if state.len () % 2 != 0 {
			Err ("TrackingState requires an even numbers of items") ?;
		}
		Ok (State {
			items: state.iter ().copied ()
				.tuples::<(_, _)> ()
				.enumerate ()
				.map (|(idx, (run, num))|
					Ok (Item {
						gen: 0,
						idx: idx.try_into () ?,
						run,
						num,
					}))
				.collect::<GenResult <_>> () ?,
		})
	}
}

#[ derive (Clone, Copy, Debug, Eq, PartialEq) ]
pub struct Item { run: u8, num: u8, gen: u16, idx: u32 }
