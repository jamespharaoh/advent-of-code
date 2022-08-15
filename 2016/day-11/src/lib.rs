//! Advent of Code 2016: Day 11: Radioisotope Thermoelectric Generators
//!
//! [https://adventofcode.com/2016/day/11](https://adventofcode.com/2016/day/11)

#![ allow (clippy::missing_inline_in_public_items) ]

use aoc_common::*;

puzzle_info! {
	name = "Radioisotope Thermoelectric Generators";
	year = 2016;
	day = 11;
	parse = |input| model::Input::parse (input);
	part_one = |input| logic::part_one (& input);
	part_two = |input| logic::part_two (& input);
	commands = [
		( name = "corpus-gen"; method = tools::corpus_gen; ),
	];
}

pub mod logic {

	use super::*;
	use model::Component;
	use model::Input;

	pub fn part_one (input: & Input) -> GenResult <usize> {
		calc_result (input)
	}

	pub fn part_two (input: & Input) -> GenResult <usize> {
		let mut input = input.clone ();
		let elerium_str = Rc::from ("elerium");
		let dilithium_str = Rc::from ("dilithium");
		input.floors [0].push (Component::Generator (Rc::clone (& elerium_str)));
		input.floors [0].push (Component::Microchip (elerium_str));
		input.floors [0].push (Component::Generator (Rc::clone (& dilithium_str)));
		input.floors [0].push (Component::Microchip (dilithium_str));
		calc_result (& input)
	}

	fn calc_result (input: & Input) -> GenResult <usize> {
		let (mut state, _names) = State::from_input (input) ?;
		state.comps [ .. state.comps_len.as_usize ()].sort ();
		let mut seen = HashSet::new ();
		seen.insert (state.compact ());
		let mut todo = VecDeque::new ();
		todo.push_back ((state.compact (), 0));
		let mut next_states = Vec::new ();
		while let Some ((state_compact, steps)) = todo.pop_front () {
			let state = state_compact.expand ();
			if state.is_done () { return Ok (steps) }
			next_states.clear ();
			state.next_states (& mut next_states);
			for next_state in next_states.iter_vals () {
				if ! seen.insert (next_state) { continue }
				todo.push_back ((next_state, steps + 1));
			}
		}
		Err ("No solution found".into ())
	}

	#[ derive (Clone, Copy, Debug) ]
	struct State {
		comps: [[u8; 2]; 7],
		comps_len: u8,
		elevator: u8,
	}

	#[ derive (Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd) ]
	struct StateCompact {
		data: u64,
	}

	impl State {

		fn from_input (input: & Input) -> GenResult <(Self, Vec <Rc <str>>)> {
			let names: Vec <Rc <str>> =
				input.floors.iter ()
					.flat_map (|floor| floor.iter ()
						.map (|comp| match * comp {
							Component::Generator (ref name) => Rc::clone (name),
							Component::Microchip (ref name) => Rc::clone (name),
						}))
					.sorted ()
					.dedup ()
					.collect ();
			if names.is_empty () { return Err ("Must have at least one component type".into ()) }
			if names.len () > 7 { return Err ("Only support seven types of component".into ()) }
			let comp_floor = |comp|
				input.floors.iter ()
					.enumerate ()
					.flat_map (|(floor_idx, floor)| floor.iter ()
						.map (move |comp| ((floor_idx + 1).as_u8 (), comp)))
					.filter (|& (_, some_comp)| * some_comp == comp)
					.map (|(floor, _)| floor)
					.exactly_one ()
					.map_err (|_err| "Must have exactly one generator of each type");
			Ok ((Self {
				comps: names.iter ()
					.map (|name| Ok::<_, GenError> ([
						comp_floor (Component::Generator (Rc::clone (name))) ?,
						comp_floor (Component::Microchip (Rc::clone (name))) ?,
					]))
					.chain (iter::from_fn (|| Some (Ok ([1, 1]))))
					.take (7)
					.collect_array_ok () ?
					.unwrap (),
				comps_len: names.len ().as_u8 (),
				elevator: 1,
			}, names))
		}

		fn next_states (& self, results: & mut Vec <StateCompact>) {
			let mut next_floors = ArrayVec::<u8, 2>::new ();
			if self.elevator > 1 { next_floors.push (self.elevator - 1); }
			if self.elevator < 4 { next_floors.push (self.elevator + 1); }
			for name_idx_0 in 0 .. self.comps_len.as_usize () {
				for comp_idx_0 in 0 .. 2 {
					if self.comps [name_idx_0] [comp_idx_0] != self.elevator { continue }
					for next_floor in next_floors.iter_vals () {
						let mut next_state = * self;
						next_state.elevator = next_floor;
						next_state.comps [name_idx_0] [comp_idx_0] = next_floor;
						if next_state.is_valid () {
							next_state.comps [ .. next_state.comps_len.as_usize ()].sort ();
							results.push (next_state.compact ());
						}
					}
					for name_idx_1 in name_idx_0 .. self.comps_len.as_usize () {
						for comp_idx_1 in 0 .. 2 {
							if self.comps [name_idx_1] [comp_idx_1] != self.elevator { continue }
							if (name_idx_0, comp_idx_0) >= (name_idx_1, comp_idx_1) { continue }
							for next_floor in next_floors.iter_vals () {
								let mut next_state = * self;
								next_state.elevator = next_floor;
								next_state.comps [name_idx_0] [comp_idx_0] = next_floor;
								next_state.comps [name_idx_1] [comp_idx_1] = next_floor;
								if next_state.is_valid () {
									next_state.comps [ .. next_state.comps_len.as_usize ()].sort ();
									results.push (next_state.compact ());
								}
							}
						}
					}
				}
			}
		}

		fn is_valid (& self) -> bool {
			for name_idx_0 in 0 .. self.comps_len.as_usize () {
				if self.comps [name_idx_0] [1] == self.comps [name_idx_0] [0] { continue }
				for name_idx_1 in 0 .. self.comps_len.as_usize () {
					if name_idx_1 == name_idx_0 { continue }
					if self.comps [name_idx_0] [1] == self.comps [name_idx_1] [0] { return false }
				}
			}
			true
		}

		fn is_done (& self) -> bool {
			self.comps.iter ()
				.take (self.comps_len.as_usize ())
				.all (|& [gen, chip]| gen == 4 && chip == 4)
		}

		fn compact (& self) -> StateCompact {
			let mut data = 0_u64;
			for [gen, chip] in self.comps.iter_vals ().rev () {
				debug_assert! ((1 ..= 4).contains (& gen));
				data <<= 2_u32;
				data |= chip.as_u64 () - 1;
				debug_assert! ((1 ..= 4).contains (& chip));
				data <<= 2_u32;
				data |= gen.as_u64 () - 1;
			}
			debug_assert! ((1 ..= 7).contains (& self.comps_len));
			data <<= 3_u32;
			data |= self.comps_len.as_u64 ();
			debug_assert! ((1 ..= 4).contains (& self.elevator));
			data <<= 2_u32;
			data |= self.elevator.as_u64 () - 1;
			StateCompact { data }
		}

	}

	impl StateCompact {
		fn expand (self) -> State {
			let mut data = self.data;
			let elevator = (data & 0x3).as_u8 () + 1;
			data >>= 2_u32;
			let comps_len = (data & 0x7).as_u8 ();
			data >>= 3_u32;
			let comps = (0_u32 .. 7_u32).map (|_| {
				let gen = (data & 0x3).as_u8 () + 1;
				data >>= 2_u32;
				let chip = (data & 0x3).as_u8 () + 1;
				data >>= 2_u32;
				[gen, chip]
			}).collect_array ().unwrap ();
			debug_assert! (data == 0);
			State { comps, comps_len, elevator }
		}
	}

}

pub mod model {

	use super::*;

	pub type Val = u16;

	#[ derive (Clone, Debug, Default, Eq, PartialEq) ]
	pub struct Input {
		pub floors: Vec <Vec <Component>>,
	}

	#[ derive (Clone, Debug, Eq, PartialEq) ]
	pub enum Component {
		Generator (Rc <str>),
		Microchip (Rc <str>),
	}

	impl Input {
		pub fn parse (input: & [& str]) -> GenResult <Self> {
			let floors = input.iter ()
				.enumerate ()
				.map (|(line_idx, line)|
					Parser::wrap (line, |parser| {
						parser.set_word_pred (char::is_alphabetic);
						parser.expect (match line_idx {
							0 => "The first floor contains ",
							1 => "The second floor contains ",
							2 => "The third floor contains ",
							3 => "The fourth floor contains ",
							_ => return Err (parser.err ()),
						}) ?;
						parser.any ()
							.of (|parser| {
								parser.expect ("nothing relevant.") ?.confirm ().end () ?;
								Ok (vec! [])
							})
							.of (|parser| {
								let comp: Component = parser.expect ("a ") ?.item () ?;
								parser.expect (".") ?.end () ?;
								Ok (vec! [ comp ])
							})
							.of (|parser| {
								let comp_0 = parser.expect ("a ") ?.item () ?;
								let comp_1 = parser.expect (" and a ") ?.item () ?;
								parser.expect (".") ?.end () ?;
								Ok (vec! [ comp_0, comp_1 ])
							})
							.of (|parser| {
								let comp = parser.expect ("a ") ?.item () ?;
								let mut comps = vec! [ comp ];
								loop {
									if parser.rest ().starts_with (", and a ") {
										let comp: Component =
											parser.expect (", and a ") ?.item () ?;
										parser.expect (".") ?;
										comps.push (comp);
										break;
									}
									let comp = parser.expect (", a ") ?.item () ?;
									comps.push (comp);
								}
								Ok (comps)
							})
							.done ()
					}).map_parse_err (|_, col_idx| format! (
						"Invalid input: line {}: col {}: {}", line_idx + 1, col_idx + 1, line)))
				.collect::<GenResult <_>> () ?;
			Ok (Self { floors })
		}
	}

	impl <'inp> FromParser <'inp> for Component {
		fn from_parser (parser: & mut Parser <'inp>) -> ParseResult <Self> {
			parser.any ()
				.of (|parser| {
					let name = parser.word () ?;
					parser.expect ("-compatible microchip") ?;
					Ok (Self::Microchip (Rc::from (name)))
				})
				.of (|parser| {
					let name = parser.word () ?;
					parser.expect (" generator") ?;
					Ok (Self::Generator (Rc::from (name)))
				})
				.done ()
		}
	}

}

mod tools {

	use super::*;
	use std::fs::File;
	use std::io::Read as _;
	use std::io::Write as _;
	use std::path::PathBuf;
	use std::process::Command;
	use std::process::Stdio;

	#[ derive (clap::Parser) ]
	pub struct CorpusGenArgs {

		#[ clap (long, default_value = "fuzz/corpus/2016-day-11") ]
		output_dir: PathBuf,

		#[ clap (long, default_value = "10") ]
		num_files: usize,

		#[ clap (long, default_value = "5") ]
		num_comps: usize,

		#[ clap (long, default_value = "4") ]
		name_len: usize,

	}

	#[ allow (clippy::needless_pass_by_value) ]
	pub fn corpus_gen (args: CorpusGenArgs) -> GenResult <()> {
		let mut rand = File::open ("/dev/urandom") ?;
		for _ in 0 .. args.num_files {
			let names =
				iter::from_fn (
						|| -> Option <String> {
							let mut name = String::new ();
							while name.len () < args.name_len {
								let mut buf = [0];
								assert_eq! (rand.read (& mut buf).unwrap (), 1);
								let ch = ok_or! (buf [0].to_char (), continue);
								if ! ch.is_ascii_lowercase () { continue }
								name.push (ch);
							}
							Some (name)
						})
					.take (args.num_comps)
					.collect::<Vec <String>> ();
			let comps =
				iter::from_fn (
						|| -> Option <[u8; 2]> {
							let mut buf = [0];
							assert_eq! (rand.read (& mut buf).unwrap (), 1);
							Some ([(buf [0] & 0x3) + 1, ((buf [0] >> 2_u32) & 0x3) + 1])
						})
					.take (7)
					.collect::<Vec <[u8; 2]>> ();
			let mut output = String::new ();
			for floor in 1 ..= 4 {
				let mut items = Vec::new ();
				for (name, [gen, chip]) in
					names.iter ()
						.zip (comps.iter_vals ())
						.take (args.num_comps) {
					if gen == floor {
						items.push (format! ("{} generator", name));
					}
					if chip == floor {
						items.push (format! ("{}-compatible microchip", name));
					}
				}
				let floor_name = match floor {
					1 => "first", 2 => "second", 3 => "third", 4 => "fourth",
					_ => unreachable! (),
				};
				if items.is_empty () {
					writeln! (& mut output, "The {} floor contains nothing relevant.",
						floor_name).unwrap ();
				} else if items.len () == 1 {
					writeln! (& mut output, "The {} floor contains a {}.", floor_name,
						items [0]).unwrap ();
				} else if items.len () == 2 {
					writeln! (& mut output, "The {} floor contains a {} and a {}.", floor_name,
						items [0], items [1]).unwrap ();
				} else {
					write! (& mut output, "The {} floor contains a {}", floor_name, items [0]).unwrap ();
					for item in & items [1 .. items.len () - 1] {
						write! (& mut output, ", a {}", item).unwrap ();
					}
					writeln! (& mut output, ", and a {}.", items [items.len () - 1]).unwrap ();
				}
			}
			let mut sum_command =
				Command::new ("sha1sum")
					.stdin (Stdio::piped ())
					.stdout (Stdio::piped ())
					.spawn ()
					.unwrap ();
			let mut sum_stdin = sum_command.stdin.take ().unwrap ();
			sum_stdin.write_all (output.as_bytes ()).unwrap ();
			drop (sum_stdin);
			let sum_output = sum_command.wait_with_output ().unwrap ();
			let sum_output_vec = sum_output.stdout [0 .. 40].to_vec ();
			let sum_output_str = String::from_utf8 (sum_output_vec).unwrap ();
			let mut output_path = args.output_dir.clone ();
			output_path.push (sum_output_str);
			let mut output_file = File::create (output_path).unwrap ();
			output_file.write_all (output.as_bytes ()).unwrap ();
		}
		Ok (())
	}

}

#[ cfg (test) ]
mod examples {

	use super::*;

	const EXAMPLE_ONE: & [& str] = & [
		"The first floor contains a hydrogen-compatible microchip and a lithium-compatible microchip.",
		"The second floor contains a hydrogen generator.",
		"The third floor contains a lithium generator.",
		"The fourth floor contains nothing relevant.",
	];

	const EXAMPLE_TWO: & [& str] = & [
		"The first floor contains a hydrogen generator.",
		"The second floor contains a lithium generator.",
		"The third floor contains a hydrogen-compatible microchip and a lithium-compatible microchip.",
		"The fourth floor contains nothing relevant.",
	];

	#[ test ]
	fn part_one () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("11", puzzle.part_one (EXAMPLE_ONE));
	}

	#[ test ]
	fn part_two () {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("29", puzzle.part_two (EXAMPLE_TWO));
	}

}
