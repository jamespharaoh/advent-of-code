//! Advent of Code 2015: Day 19: Medicine for Rudolph
//!
//! [https://adventofcode.com/2015/day/19](https://adventofcode.com/2015/day/19)

use aoc_common::*;

puzzle_info! {
	name = "Medicine for Rudolph";
	year = 2015;
	day = 19;
	parse = |input| model::parse_input (input);
	part_one = |input| logic::part_one (input);
	part_two = |input| logic::part_two (input);
}

/// Logic for solving the puzzles.
///
pub mod logic {

	use super::*;
	use list::CharList;
	use model::Input;

	pub fn part_one (input: Input) -> GenResult <u32> {
		let mut results = HashSet::new ();
		for (from, to) in input.replacements.iter () {
			let mut last_pos = 0;
			while let Some (pos) = input.medicine [last_pos .. ].find (from) {
				let pos = last_pos + pos;
				let new_molecule = format! ("{}{}{}",
					& input.medicine [ .. pos],
					to,
					& input.medicine [pos + from.len () .. ],
				);
				results.insert (new_molecule);
				last_pos = pos + from.len ();
			}
		}
		Ok (results.len () as u32)
	}

	pub fn part_two (input: Input) -> GenResult <u32> {
		if ! input.replacements.iter ().any (|(from, _)| from == "e") {
			Err ("Must have at least one replacement from \"e\"") ?;
		}
		let mut todo: VecDeque <(u32, Vec <Rc <String>>, CharList)> = VecDeque::new ();
		todo.push_back ((0, Vec::new (), CharList::from (& input.medicine)));
		let mut seen: HashSet <(Vec <Rc <String>>, CharList)> = HashSet::new ();
		let mut min_match = None;
		'OUTER: while let Some ((todo_steps, todo_prefix, todo_suffix)) = todo.pop_back () {
			if ! seen.insert ((todo_prefix.clone (), todo_suffix.clone ())) { continue }
			const VERBOSE: bool = false;
			if VERBOSE {
				if todo_prefix.is_empty () {
					println! ("queue={} steps={} {}", todo.len (), todo_steps, todo_suffix);
				} else {
					println! ("queue={} steps={} {} | {}", todo.len (), todo_steps, todo_prefix.iter ().join (" | "), todo_suffix);
				}
			}
			for (from, to) in input.replacements.iter () {
				if from == "e" {
					if todo_prefix.is_empty () && to == todo_suffix {
						// TODO i am not convinced it is so simple, but i got the right answer...
						// i am guessing that this has something to do with this being an
						// implementation of a greedy matcher, always trying to match on the left
						// first then recusring. but i would like to think about this a bit more.
						min_match = Some (todo_steps + 1);
						break 'OUTER;
					}
				} else if let Some (suffix) = todo_suffix.strip_prefix (to) {
					if todo_prefix.is_empty () {
						todo.push_back ((
							todo_steps + 1,
							Vec::new (),
							suffix.prepend (from),
						));
					} else {
						todo.push_back ((
							todo_steps + 1,
							todo_prefix.iter ()
								.take (todo_prefix.len () - 1)
								.cloned ()
								.collect (),
							suffix.prepend (from)
								.prepend (todo_prefix.last ().unwrap ()),
						));
					}
				}
			}

			let mut prefix = String::new ();
			let mut suffix = todo_suffix.clone ();
			while let Some ((head, tail)) = suffix.cons () {
				prefix.push (head);
				suffix = tail;
				if ! input.replacements.iter ()
						.any (|(_, to)| to.starts_with (& prefix)) {
					break;
				}
				todo.push_back ((
					todo_steps,
					todo_prefix.iter ().cloned ()
						.chain (iter::once (Rc::new (prefix.clone ())))
						.collect (),
					suffix.clone (),
				));
			}
		}
		Ok (min_match.ok_or ("No solution found") ?)
	}

}

pub mod list {

	use aoc_common::*;

	pub use char_list::CharList;

	#[ derive (Clone) ]
	pub enum List <Item: Clone> {
		Present (Rc <(Item, List <Item>)>),
		Empty,
	}

	impl <Item: Clone + Debug> Debug for List <Item> {
		fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
			write! (formatter, "[") ?;
			let mut cur = self.clone ();
			let mut idx = 0;
			while let Some ((head, tail)) = cur.cons () {
				if idx > 0 { write! (formatter, ", ") ?; }
				Debug::fmt (& head, formatter) ?;
				cur = tail;
				idx += 1;
			}
			write! (formatter, "]") ?;
			Ok (())
		}
	}

	impl <Item: Clone + PartialEq> PartialEq for List <Item> {
		fn eq (& self, other: & Self) -> bool {
			let mut left = self.clone ();
			let mut right = other.clone ();
			loop {
				match (left.cons (), right.cons ()) {
					(None, None) => return true,
					(Some (_), None) | (None, Some (_)) => return false,
					(Some ((left_head, left_tail)), Some ((right_head, right_tail))) => {
						if ! Item::eq (& left_head, & right_head) { return false }
						(left, right) = (left_tail, right_tail)
					},
				}
			}
		}
	}

	impl <Item: Clone + Eq> Eq for List <Item> { }

	impl <Item: Clone + Hash> Hash for List <Item> {
		fn hash <Hshr: Hasher> (& self, hasher: & mut Hshr) {
			let mut cur = self.clone ();
			while let Some ((head, tail)) = cur.cons () {
				head.hash (hasher);
				cur = tail;
			}
		}
	}

	impl Display for List <char> {
		fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
			let mut cur = self.clone ();
			while let List::Present (inner) = cur {
				let & (head, ref tail) = inner.deref ();
				write! (formatter, "{}", head) ?;
				cur = tail.clone ();
			}
			Ok (())
		}
	}

	impl <Item: Clone> List <Item> {
		#[ inline ]
		pub fn cons (& self) -> Option <(Item, List <Item>)> {
			match self {
				List::Present (inner) => {
					let (head, tail) = inner.deref ();
					Some ((head.clone (), tail.clone ()))
				},
				List::Empty => None,
			}
		}
		#[ inline ]
		pub fn head (& self) -> Option <Item> { self.cons ().map (|(head, _)| head) }
		#[ inline ]
		pub fn tail (& self) -> Option <List <Item>> { self.cons ().map (|(_, tail)| tail) }
		#[ inline ]
		pub fn is_empty (& self) -> bool { self.cons ().is_none () }
		#[ inline ]
		pub fn len (& self) -> usize {
			let mut cur = self.clone ();
			let mut len = 0;
			while let List::Present (inner) = cur {
				let (_, new) = inner.deref ();
				cur = new.clone ();
				len += 1
			}
			len
		}
		#[ inline ]
		pub fn push_front (& self, head: Item) -> List <Item> {
			List::Present (Rc::new ((head, self.clone ())))
		}
	}

	mod char_list {

		use super::*;

		pub type CharList = List <char>;

		impl CharList {
			#[ inline ]
			pub fn starts_with (& self, pat: & str) -> bool {
				self.strip_prefix (pat).is_some ()
			}
			#[ inline ]
			pub fn strip_prefix (& self, pat: & str) -> Option <CharList> {
				let mut cur = self.clone ();
				let mut pat_chars = pat.chars ();
				loop {
					match (cur.cons (), pat_chars.next ()) {
						(_, None) => return Some (cur),
						(None, _) => return None,
						(Some ((head, tail)), Some (pat_ch)) => {
							if head != pat_ch { return None }
							cur = tail.clone ();
						},
					}
				}
			}
			#[ inline ]
			pub fn prepend (& self, prefix: & str) -> CharList {
				let mut cur = self.clone ();
				for prefix_ch in prefix.chars ().rev () {
					cur = cur.push_front (prefix_ch);
				}
				cur
			}
		}

		impl From <& str> for CharList {
			#[ inline ]
			fn from (src: & str) -> CharList {
				CharList::Empty.prepend (src)
			}
		}

		impl From <& String> for CharList {
			#[ inline ]
			fn from (src: & String) -> CharList {
				CharList::Empty.prepend (src)
			}
		}

		impl PartialEq <str> for CharList {
			#[ inline ]
			fn eq (& self, other: & str) -> bool {
				let mut cur = self.clone ();
				let mut other_iter = other.chars ();
				loop {
					match (cur.cons (), other_iter.next ()) {
						(Some (_), None) | (None, Some (_)) => return false,
						(None, None) => return true,
						(Some ((cur_head, cur_tail)), Some (other_ch)) => {
							if cur_head != other_ch { return false }
							cur = cur_tail;
						},
					}
				}
			}
		}

		impl PartialEq <String> for CharList {
			#[ inline ]
			fn eq (& self, other: & String) -> bool {
				PartialEq::eq (self, other.as_str ())
			}
		}

		impl PartialEq <CharList> for & String {
			#[ inline ]
			fn eq (& self, other: & CharList) -> bool {
				PartialEq::eq (self.as_str (), other)
			}
		}

		impl PartialEq <CharList> for str {
			#[ inline ]
			fn eq (& self, other: & CharList) -> bool {
				PartialEq::eq (other, self)
			}
		}

		#[ cfg (test) ]
		mod tests {

			use super::*;
			use CharList as CL;

			fn cl (src: & str) -> CL { CL::from (src) }

			const SAMPLES: & [& str] = & [ "", "a", "ab", "abc", "b", "bc", "c" ];

			#[ test ]
			fn eq () {
				for left in SAMPLES.iter ().cloned () {
					for right in SAMPLES.iter ().cloned () {
						if left == right {
							assert! (cl (left) == cl (right),
								"{:?} == {:?} but CharList::from ({:?}) != CharList::from ({:?})",
								left, right, left, right);
						} else {
							assert! (cl (left) != cl (right),
								"{:?} != {:?} but CharList::from ({:?}) == CharList::from ({:?})",
								left, right, left, right);
						}
					}
				}
			}

			#[ test ]
			fn strip_prefix () {
				for left in SAMPLES.iter ().cloned () {
					for right in SAMPLES.iter ().cloned () {
						let expected = left.strip_prefix (right);
						let actual = cl (left).strip_prefix (right);
						assert_eq! (expected.map (cl), actual,
							"{:?}.strip_prefix ({:?}) == {:?} but {:?}.strip_prefix ({:?}) == {:?}",
							left, right, expected, cl (left), right, actual);
					}
				}
			}

		}

	}

}

/// Representation of the puzzle input, etc.
///
pub mod model {

	use super::*;
	use parser::*;

	#[ derive (Clone) ]
	pub struct Input {
		pub replacements: Replacements,
		pub medicine: String,
	}

	pub type Replacements = Vec <Replacement>;
	pub type Replacement = (String, String);

	pub fn parse_input (input: & [& str]) -> GenResult <Input> {
		if input.len () < 2 { Err ("Invalid input") ?; }
		if ! input [input.len () - 2].is_empty () { Err ("Invalid input") ?; }
		let is_chem = |input: & str| input.chars ().all (|ch| ch.is_ascii_alphanumeric ());
		let replacements = input [0 .. input.len () - 2].iter ().enumerate ()
			.map (|(line_idx, line)|
				Parser::wrap (line, |parser| {
					parser.set_ignore_whitespace (true);
					let from = parser.word_if (is_chem) ?;
					let to = parser.expect_word ("=>") ?.word_if (is_chem) ?;
					parser.end () ?;
					if to.len () < from.len () { Err (parser.err ()) ?; }
					Ok ((from.to_string (), to.to_string ()))
				}).map_parse_err (|col_idx| format! (
					"Invalid input: line {}: col {}: {}", line_idx + 1, col_idx + 1, line))
			).collect::<GenResult <Replacements>> () ?;
		Ok (Input {
			replacements,
			medicine: input [input.len () - 1].to_string (),
		})
	}

}

#[ cfg (test) ]
mod examples {

	use super::*;

	const EXAMPLE_ONE: & [& str] = & [
		"H => HO",
		"H => OH",
		"O => HH",
		"",
		"HOH",
	];

	const EXAMPLE_TWO: & [& str] = & [
		"e => H",
		"e => O",
		"H => HO",
		"H => OH",
		"O => HH",
		"",
		"HOH",
	];

	#[ test ]
	fn part_one () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("4", puzzle.part_one (EXAMPLE_ONE));
	}

	#[ test ]
	fn part_two () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("3", puzzle.part_two (EXAMPLE_TWO));
	}

}
