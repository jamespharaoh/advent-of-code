#![ allow (clippy::todo) ]
#![ allow (dead_code) ]
#![ allow (unused_imports) ]
#![ allow (unused_mut) ]
#![ allow (unused_variables) ]

use super::*;
use model::State;

args_decl! {
	pub struct Args {
		iterations: Option <usize>,
		max_length: Option <usize>,
	}
}

#[ derive (Clone, Eq, Hash, PartialEq) ]
pub struct Span (Rc <[u8]>);

impl Span {
	/*
	fn split (& self) -> Vec <Span> {
		iter::empty ()
			.chain (iter::once (0))
			.chain (self.iter ().skip (1)
				.tuples::<(_, _)> ()
				.enumerate ()
				.filter_map (|(idx, (left, right))| (left != right).then_some (idx + 1)))
			.chain (iter::once (self.len ()))
			.tuple_windows::<(_, _)> ()
			.map (|(start, end)| Span::from (& self [start .. end]))
			.collect ()
	}
	fn can_split (& self) -> bool {
		iter::empty ()
			.chain (self.iter ().skip (1)
				.tuples::<(_, _)> ()
				.enumerate ()
				.filter_map (|(idx, (left, right))| (left != right).then_some (idx + 1)))
			.next ()
			.is_some ()
	}
	*/
}

impl Debug for Span {
	fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
		write! (formatter, "Span (\"{self}\")") ?;
		Ok (())
	}
}

impl Display for Span {
	fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
		for & val in self.0.iter () {
			write! (formatter, "{}", char::from_digit (val.pan_u32 (), 10).unwrap ()) ?;
		}
		Ok (())
	}
}

impl Borrow <[u8]> for Span {
	fn borrow (& self) -> & [u8] { & self.0 }
}

impl From <& [u8]> for Span {
	fn from (other: & [u8]) -> Self {
		Self (other.into ())
	}
}

impl TryFrom <& str> for Span {
	type Error = GenError;
	fn try_from (src: & str) -> GenResult <Self> {
		Ok (Self::from (model::State::parse (src) ?.as_slice ()))
	}
}

impl PartialEq <[u8]> for Span {
	fn eq (& self, other: & [u8]) -> bool {
		self.0.as_ref () == other
	}
}

impl PartialOrd for Span {
	fn partial_cmp (& self, other: & Self) -> Option <Ordering> {
		Some (self.cmp (other))
	}
}

impl Ord for Span {
	fn cmp (& self, other: & Self) -> Ordering {
		self.0.len ().cmp (& other.len ())
			.then (self.0.cmp (& other.0))
	}
}

impl Deref for Span {
	type Target = [u8];
	fn deref (& self) -> & [u8] { self.0.as_ref () }
}

#[ derive (Clone, Debug, Eq, Ord, PartialEq, PartialOrd) ]
enum Destiny {
	Unstable (Span),
	Atomic (Atomic),
	Stable (Rc <[Atomic]>),
}

#[ derive (Clone, Eq, Hash, Ord, PartialEq, PartialOrd) ]
struct Atomic (Rc <AtomicInner>);

impl Borrow <[u8]> for Atomic {
	fn borrow (& self) -> & [u8] { & self.deref ().key }
}

impl Deref for Atomic {
	type Target = AtomicInner;
	fn deref (& self) -> & AtomicInner { & self.0 }
}

impl From <AtomicInner> for Atomic {
	fn from (inner: AtomicInner) -> Self {
		Self (Rc::new (inner))
	}
}

#[ derive (Clone, Ord, PartialOrd) ]
struct AtomicInner {
	key: Span,
	next: Rc <[Atomic]>,
	first: [u8; 3],
	last: u8,
}

impl Debug for Atomic {
	fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
		write! (formatter, "Atomic ({} next=", self.key) ?;
		if self.next.is_empty () { write! (formatter, "TODO") ?; }
		for (next_idx, next) in self.next.iter ().enumerate () {
			if next_idx > 0 { write! (formatter, ",") ?; }
			write! (formatter, "{}", next.key) ?;
		}
		write! (formatter, " first=") ?;
		for item in self.first.iter ().copied () {
			write! (formatter, "{}", char::from_digit (item.pan_u32 (), 10).unwrap ()) ?;
		}
		write! (formatter, " last={})", self.last) ?;
		Ok (())
	}
}

impl PartialEq <[u8]> for AtomicInner {
	fn eq (& self, other: & [u8]) -> bool { self.key == * other }
}

impl PartialEq for AtomicInner {
	fn eq (& self, other: & Self) -> bool { self.key == other.key }
}

impl Eq for AtomicInner {}

impl Hash for AtomicInner {
	fn hash <Hr: Hasher> (& self, hasher: & mut Hr) {
		self.key.hash (hasher);
	}
}

impl PartialOrd <[u8]> for AtomicInner {
	fn partial_cmp (& self, other: & [u8]) -> Option <Ordering> {
		self.key.deref ().partial_cmp (other)
	}
}

const fn compatible (prev: u8, next: & [u8]) -> bool {
	prev != next [0] && prev != next [1] && prev != next [2]
}

#[ allow (clippy::needless_pass_by_value) ]
#[ allow (clippy::print_stdout) ]
#[ allow (clippy::todo) ]
#[ allow (clippy::too_many_lines) ]
pub fn run (args: Args) -> GenResult <()> {

	let stables = find_stables (args.max_length.unwrap_or (10), args.iterations.unwrap_or (20));
	println! ("NUM STABLES: {}", stables.len ());
	println! ("NUM SINGULAR: {}", stables.values ().filter (|stable|
		stable.as_ref ().map_or (false, |stable| stable.parts.len () == 1)).count ());

	let mut destinies: HashMap <Span, Destiny> = HashMap::new ();
	let mut atomics: HashSet <Atomic> = HashSet::new ();

	/*

	NOTE: commented out because of removal of itertools, wasn't working properly anyway...

	for length in (2 ..= args.max_length.unwrap_or (10)).step_by (2) {

		'STATE: for state in (0 .. length).map (|_| (1_u8 ..= 3_u8))
			.multi_cartesian_product ()
			.map (|nums| State::try_from (nums).unwrap ()) {

			let has_long_run = {
				let group_by_temp =
					state.iter ().copied ()
						.group_by (|& item| item);
				group_by_temp.into_iter ()
					.any (|(_, group)| group.count () > 3)
			};
			if has_long_run { continue }

			for prefix_len in (2 ..= length.pan_usize () - 2).step_by (2) {
				match (
					atomics.get (& state [ .. prefix_len]),
					destinies.get (& state [prefix_len .. ]).unwrap_or_else (||
						panic! ("Can't find destiny for {:?}", & state [prefix_len .. ])),
				) {
					(Some (_), & Destiny::Unstable (_)) | (None, _) => continue,
					(Some (prefix), & Destiny::Atomic (ref suffix)) => {
						if ! compatible (prefix.last, & suffix.first) { continue }
						destinies.insert (
							state.as_slice ().into (),
							Destiny::Stable ([ prefix.clone (), suffix.clone () ].as_slice ().into ()));
						continue 'STATE;
					},
					(Some (prefix), & Destiny::Stable (ref suffix)) => {
						if ! compatible (prefix.last, & suffix [0].first) { continue }
						destinies.insert (
							state.as_slice ().into (),
							Destiny::Stable (
								iter::once (prefix.clone ())
									.chain (suffix.iter ().cloned ())
									.collect::<Vec <_>> ()
									.into ()));
						continue 'STATE;
					},
				}
			}

			let mut seq_ord = Vec::new ();
			seq_ord.push (state.clone ());
			let mut seq_set = HashSet::new ();
			seq_set.insert (state.clone ());

			let mut prev_state = state.clone ();
			let mut left_1 = state [0] == 1;
			let mut left_2 = state [0] == 2;
			let mut left_3 = state [0] == 3;

			for _ in 0 .. args.iterations.unwrap_or (20) {
				let next_state = logic::one_round (& prev_state);

				if let Some (_whatever) = destinies.get (next_state.as_slice ()) {
					//todo! ("CHECK FOR EXISTING {} {}", state, next_state); TODO
				}

				seq_ord.push (next_state.clone ());
				seq_set.insert (next_state.clone ());

				if next_state [0] == 1 { left_1 = true; }
				if next_state [0] == 2 { left_2 = true; }
				if next_state [0] == 3 { left_3 = true; }

				if left_1 && left_2 && left_3 {
					destinies.insert (
						state.as_slice ().into (),
						Destiny::Unstable (seq_ord [1].as_slice ().into ()));
					continue 'STATE;
				}

				prev_state = next_state;
			}

			let mut next_rest = & seq_ord [1] [ .. ];
			let mut atomic_next = Vec::new ();
			'OUTER: while ! next_rest.is_empty () {
				for prefix_len in (2 .. next_rest.len ()).step_by (2) {
					if let Some (prefix_atomic) = atomics.get (& seq_ord [1] [0 .. prefix_len]) {
						atomic_next.push (prefix_atomic.clone ());
						next_rest = & next_rest [ prefix_len .. ];
						continue 'OUTER;
					}
				}
				//unreachable! ("Can't find atomics for: {}: {}", state, seq_ord [1]);
				atomic_next.clear ();
				break;
			}

			let key: Span = state.as_slice ().into ();
			let atomic: Atomic = AtomicInner {
				key: key.clone (),
				next: atomic_next.as_slice ().into (),
				first: [0, 1, 2].map (|idx| seq_ord [idx].first ().copied ().unwrap ()),
				last: seq_ord [0].last ().copied ().unwrap (),
			}.into ();
			atomics.insert (atomic.clone ());
			destinies.insert (key, Destiny::Atomic (atomic));

		}

	}

	println! ("ATOMICS: count={}", atomics.len ());
	for atomic in atomics.iter ().sorted_by_key (|atomic| atomic.key.clone ()) {
		println! (" - {:?}", atomic);
	}

	println! ("DESTINIES: count={}", destinies.len ());
	for (_key, _destiny) in destinies.iter ().sorted_by_key (|& (key, _)| key.clone ()) {
		//println! (" - {:?}", atomic);
	}
	*/

	todo! ();

}

struct Stable {
	parts: Vec <Span>,
	first: [u8; 3],
	last: u8,
}

fn find_stables (max_length: usize, iterations: usize) -> HashMap <Span, Option <Stable>> {

	let mut stables: HashMap <Span, Option <Stable>> = HashMap::new ();

	todo! ();

	/*

	NOTE: commented out because of removal of itertools

	for length in (2 ..= max_length).step_by (2) {

		'STATE: for state in (0 .. length).map (|_| (1_u8 ..= 3_u8))
			.multi_cartesian_product ()
			.map (|nums| State::try_from (nums).unwrap ()) {

			let has_long_run = {
				let group_by_temp =
					state.iter ().copied ()
						.group_by (|& item| item);
				group_by_temp.into_iter ()
					.any (|(_, group)| group.count () > 3)
			};
			if has_long_run { continue }

			for prefix_len in (2 ..= length.pan_usize () - 2).step_by (2) {
				match (
					stables [& state [ .. prefix_len]].as_ref (),
					stables [& state [prefix_len .. ]].as_ref (),
				) {
					(Some (prefix), Some (suffix))
							if compatible (prefix.last, & suffix.first) => {
						assert_eq! (prefix.parts.len (), 1);
						stables.insert (state.as_slice ().into (), Some (Stable {
							parts: prefix.parts.iter ().cloned ()
								.chain (suffix.parts.iter ().cloned ())
								.collect (),
							first: prefix.first,
							last: suffix.last,
						}));
						continue 'STATE;
					},
					_ => continue,
				}
			}

			let mut sequence = Vec::new ();
			sequence.push (state.clone ());

			let mut prev_state = state.clone ();
			let mut first_seq: TinyVec <u8, 3> = TinyVec::new ();
			first_seq.push (state.first ().copied ().unwrap ());

			let key = Span::from (state.as_slice ());

			for step_idx in 0 .. iterations {
				let next_state = logic::one_round (& prev_state);
				let next_first = next_state.first ().copied ().unwrap ();

				/*
				if let Some (whatever) = stables.get (next_state.as_slice ()) {
					todo! ("CHECK FOR EXISTING {} {}", state, next_state);
				}
				*/

				sequence.push (next_state.clone ());

				if ! first_seq.is_full () {
					first_seq.push (next_first);
					if [1, 2, 3].into_iter ()
						.all (|num_0| first_seq.iter ().copied ()
							.any (|num_1| num_0 == num_1)) {
						stables.insert (key.clone (), None);
						continue 'STATE;
					}
				} else if first_seq [(step_idx + 1) % 3] != next_first {
					stables.insert (key.clone (), None);
					continue 'STATE;
				}

				prev_state = next_state;
			}

			stables.insert (key.clone (), Some (Stable {
				parts: vec! [ key ],
				first: [ first_seq [0], first_seq [1], first_seq [2] ],
				last: state.last ().copied ().unwrap (),
			}));

		}

	}

	stables

	*/

}

pub trait Intern <Item> where Item: Eq + Hash {
	type Shared;
	fn intern (self, item: Item) -> Self::Shared;
	fn intern_get (self, item: & Item) -> Option <Self::Shared>;
}

impl <Item> Intern <Item> for & RefCell <HashSet <Rc <Item>>>
		where Item: Eq + Hash + Ord {
	type Shared = Rc <Item>;
	fn intern (self, item: Item) -> Rc <Item> {
		let mut lock = self.borrow_mut ();
		lock.deref_mut ().intern (item)
	}
	fn intern_get (self, item: & Item) -> Option <Rc <Item>> {
		let lock = self.borrow ();
		lock.deref ().get (item).cloned ()
	}
}

impl <Item> Intern <Item> for & mut HashSet <Rc <Item>>
		where Item: Eq + Hash + Ord {
	type Shared = Rc <Item>;
	fn intern (self, item: Item) -> Rc <Item> {
		if let Some (shared) = self.get (& item) {
			return Rc::clone (shared);
		}
		let shared = Rc::new (item);
		(* self).insert (Rc::clone (& shared));
		shared
	}
	fn intern_get (self, item: & Item) -> Option <Rc <Item>> {
		self.get (item).cloned ()
	}
}
