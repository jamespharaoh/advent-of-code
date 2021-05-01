use intcode::Machine;
use intcode::Mem;
use intcode::RunResult;
use rand::Rng as _;
use rand::seq::SliceRandom as _;
use rayon::prelude::*;
use std::cmp::Reverse;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::fs;
use std::str::FromStr;

mod intcode;

type Prog = Vec <Instr>;
type ProgRef = [Instr];

fn main () {
	let input_string = fs::read_to_string ("input").unwrap ();
	let input_programme = intcode::from_str (& input_string);
	let springbot = Springbot::new (input_programme);
	let mut progs: Vec <Prog> = Vec::new ();
	progs.push (Vec::new ());
	//progs.push (vec! ["NOT A J".parse ().unwrap ()]);
	let mut samples: Vec <Sample> = Vec::new ();
	loop {
		progs = evolve_progs (progs, & samples);
		for prog in progs.iter () {
			match springbot.execute (prog) {
				Ok (value) => {
					println! ("Amount of dust collected: {}", value);
					return;
				},
				Err (SpringbotError::Died (sample)) => {
					if ! samples.contains (& sample) {
						println! ("New sample: {}", sample);
						samples.push (sample);
					}
					continue;
				},
				Err (SpringbotError::Other (error)) => {
					println! ("Error: {}", error);
					for instr in prog.iter () {
						println! ("  {}", instr);
					}
					return;
				},
			}
		}
	}
}

fn evolve_progs (mut progs: Vec <Prog>, samples: & [Sample]) -> Vec <Prog> {
	let mut num_samples_matched: usize = 0;
	let mut last_samples_matched: usize = 0;
	while num_samples_matched < samples.len () {
		if num_samples_matched != last_samples_matched {
			println! ("Evolution {}/{}", num_samples_matched, samples.len ());
			last_samples_matched = num_samples_matched;
		}
		let mut progs_by_fitness: Vec <(usize, Prog)> = progs.into_par_iter ().map (
			|prog| (programme_fitness (& prog, & samples), prog),
		).collect ();
		progs_by_fitness.sort_by_key (|(fitness, prog)| (Reverse (* fitness), prog.len ()));
		num_samples_matched = progs_by_fitness [0].0;
		progs = progs_by_fitness.into_iter ().take (100).map (|(_, prog)| prog).collect ();
		let new_progs: Vec <Prog> = progs.par_iter ().map (|prog| {
			let prog = prog.clone ();
			(0 .. 5).into_par_iter ().map (move |num_changes| {
				let mut rng = rand::thread_rng ();
				let mut new_prog = prog.clone ();
				for _ in 0 .. num_changes {
					match rng.gen_range (0 .. 3) {
						0 => { // insert
							if new_prog.len () == 15 { continue }
							let new_index = rng.gen_range (0 ..= new_prog.len ());
							let new_op = [ Op::And, Op::Or, Op::Not ].choose (& mut rng).unwrap ();
							let new_src = [
								Src::A, Src::B, Src::C, Src::D, Src::E, Src::F, Src::G, Src::H,
								Src::I, Src::T, Src::J,
							].choose (& mut rng).unwrap ();
							let new_dst = [ Dst::T, Dst::J ].choose (& mut rng).unwrap ();
							new_prog.insert (new_index, Instr { op: * new_op, src: * new_src, dst: * new_dst });
						},
						1 => { // remove
							if new_prog.len () == 0 { continue }
							let old_index = rng.gen_range (0 .. new_prog.len ());
							new_prog.remove (old_index);
						},
						2 => { // move one
							if new_prog.len () < 2 { continue }
							let old_index = rng.gen_range (0 .. new_prog.len ());
							let instr = new_prog.remove (old_index);
							let new_index = rng.gen_range (0 ..= new_prog.len ());
							new_prog.insert (new_index, instr);
						},
						_ => unreachable! (),
					}
				}
				new_prog
			})
		}).flatten ().collect ();
		progs.extend_from_slice (& new_progs);
	}
	println! ("Evolution successful for {} samples", samples.len ());
	progs
}

fn programme_fitness (instrs: & ProgRef, samples: & [Sample]) -> usize {
	samples.iter ().filter (
		|sample| simulate (instrs, & sample.data).is_ok (),
	).count ()
}

#[ derive (Clone, Copy, Debug, Eq, PartialEq) ]
struct Sample {
	data: [bool; 25],
}

impl Display for Sample {
	fn fmt (& self, formatter: & mut Formatter) -> FmtResult {
		for datum in self.data.iter () {
			write! (formatter, "{}", if * datum { '#' } else { '.' }) ?;
		}
		Ok (())
	}
}

impl FromStr for Sample {
	type Err = String;
	fn from_str (source: & str) -> Result <Sample, String> {
		let mut chars = source.chars ();
		let mut data = [false; 25];
		for index in 0 .. data.len () {
			let ch = chars.next ().unwrap_or ('#');
			data [index] = match ch {
				'#' => true,
				'.' => false,
				_ => return Err (format! ("Sample contains invalid chars: {}", source)),
			};
		}
		if chars.next ().is_some () {
			return Err (format! ("Sample too long: {}", source));
		}
		Ok (Sample { data })
	}
}

struct Springbot {
	intcode: Mem,
}

#[ derive (Debug) ]
enum SpringbotError {
	Died (Sample),
	Other (String),
}

impl Springbot {

	fn new (intcode: Mem) -> Springbot {
		Springbot { intcode }
	}

	fn execute (& self, instrs: & ProgRef) -> Result <i64, SpringbotError>{
		let mut machine = Machine::new (self.intcode.clone ());
		for instr in instrs.iter () {
			machine.input_str (& format! ("{}\n", instr));
		}
		machine.input_str ("RUN\n");
		let mut output = String::new ();
		loop {
			match machine.run () {
				RunResult::Output (value) if (0 .. 128).contains (& value) =>
					output.push (value as u8 as char),
				RunResult::Output (value) if value >= 128 => return Ok (value),
				RunResult::Halt => break,
				_ => panic! (),
			}
		}
		let output_lines: Vec <& str> = output.split ('\n').collect ();
		if output_lines.len () > 5 && output_lines [5] == "Didn't make it across:" {
			Err (SpringbotError::Died (output_lines [10].parse ().unwrap ()))
		} else {
			Err (SpringbotError::Other (output))
		}
	}

}

fn emulate (instrs: & [Instr], sensors: & [bool]) -> bool {
	if sensors.len () != 9 { panic! (); }
	let mut temp: bool = false;
	let mut jump: bool = false;
	for (instr_idx, instr) in instrs.iter ().enumerate () {
		let left = match instr.src {
			Src::A => sensors [0],
			Src::B => sensors [1],
			Src::C => sensors [2],
			Src::D => sensors [3],
			Src::E => sensors [4],
			Src::F => sensors [5],
			Src::G => sensors [6],
			Src::H => sensors [7],
			Src::I => sensors [8],
			Src::T => temp,
			Src::J => jump,
		};
		let right = match instr.dst {
			Dst::T => temp,
			Dst::J => jump,
		};
		let result = match instr.op {
			Op::And => left && right,
			Op::Or => left || right,
			Op::Not => ! left,
		};
		match instr.dst {
			Dst::T => temp = result,
			Dst::J => jump = result,
		};
		#[ cfg (debug_assertions) ]
		println! (
			"A={} B={} C={} D={} E={} F={} G={} H={} I={} T={} J={} {:02} {:3} {} {} {}",
			if sensors [0] { '1' } else { '0' },
			if sensors [1] { '1' } else { '0' },
			if sensors [2] { '1' } else { '0' },
			if sensors [3] { '1' } else { '0' },
			if sensors [4] { '1' } else { '0' },
			if sensors [5] { '1' } else { '0' },
			if sensors [6] { '1' } else { '0' },
			if sensors [7] { '1' } else { '0' },
			if sensors [8] { '1' } else { '0' },
			if temp { '1' } else { '0' },
			if jump { '1' } else { '0' },
			instr_idx,
			instr.op,
			instr.src,
			instr.dst,
			sensors.iter ().take (9).map (|v| if * v { '#' } else { '.' }).collect::<String> (),
		);
	}
	jump
}

#[ derive (Clone, Copy, Eq, Ord, PartialEq, PartialOrd) ]
struct Instr {
	op: Op,
	src: Src,
	dst: Dst,
}

impl Display for Instr {
	fn fmt (& self, formatter: & mut Formatter) -> FmtResult {
		write! (formatter, "{} {} {}", self.op, self.src, self.dst)
	}
}

impl FromStr for Instr {
	type Err = String;
	fn from_str (source: & str) -> Result <Instr, String> {
		let parts: Vec <& str> = source.split (' ').collect ();
		if parts.len () != 3 {
			return Err (format! ("Invalid instruction: {}", source));
		}
		Ok (Instr {
			op: parts [0].parse () ?,
			src: parts [1].parse () ?,
			dst: parts [2].parse () ?,
		})
	}
}

#[ derive (Clone, Copy, Eq, Ord, PartialEq, PartialOrd) ]
enum Op { And, Or, Not }

impl Display for Op {
	fn fmt (& self, formatter: & mut Formatter) -> FmtResult {
		match self {
			Op::And => formatter.pad ("AND"),
			Op::Or => formatter.pad ("OR"),
			Op::Not => formatter.pad ("NOT"),
		}
	}
}

impl FromStr for Op {
	type Err = String;
	fn from_str (source: & str) -> Result <Op, String> {
		Ok (match source {
			"AND" => Op::And,
			"OR" => Op::Or,
			"NOT" => Op::Not,
			_ => return Err (format! ("Op not recognised: {}", source)),
		})
	}
}

#[ derive (Clone, Copy, Eq, Ord, PartialEq, PartialOrd) ]
enum Src { A, B, C, D, E, F, G, H, I, T, J }

impl Display for Src {
	fn fmt (& self, formatter: & mut Formatter) -> FmtResult {
		match self {
			Src::A => write! (formatter, "A"),
			Src::B => write! (formatter, "B"),
			Src::C => write! (formatter, "C"),
			Src::D => write! (formatter, "D"),
			Src::E => write! (formatter, "E"),
			Src::F => write! (formatter, "F"),
			Src::G => write! (formatter, "G"),
			Src::H => write! (formatter, "H"),
			Src::I => write! (formatter, "I"),
			Src::T => write! (formatter, "T"),
			Src::J => write! (formatter, "J"),
		}
	}
}

impl FromStr for Src {
	type Err = String;
	fn from_str (source: & str) -> Result <Src, String> {
		Ok (match source {
			"A" => Src::A,
			"B" => Src::B,
			"C" => Src::C,
			"D" => Src::D,
			"E" => Src::E,
			"F" => Src::F,
			"G" => Src::G,
			"H" => Src::H,
			"I" => Src::I,
			"T" => Src::T,
			"J" => Src::J,
			_ => return Err (format! ("Source register not recognised: {}", source)),
		})
	}
}

#[ derive (Clone, Copy, Eq, Ord, PartialEq, PartialOrd) ]
enum Dst { J, T }

impl Display for Dst {
	fn fmt (& self, formatter: & mut Formatter) -> FmtResult {
		match self {
			Dst::T => write! (formatter, "T"),
			Dst::J => write! (formatter, "J"),
		}
	}
}

impl FromStr for Dst {
	type Err = String;
	fn from_str (source: & str) -> Result <Dst, String> {
		Ok (match source {
			"T" => Dst::T,
			"J" => Dst::J,
			_ => return Err (format! ("Dest register not recognised: {}", source)),
		})
	}
}

fn simulate (instrs: & [Instr], ground: & [bool]) -> Result <(), usize> {
	let mut pos: usize = 0;
	while pos + 9 < ground.len () {
		if ! ground [pos] {
			return Err (pos);
		}
		if emulate (& instrs, & ground [pos  + 1 .. pos + 10]) {
			pos += 4;
		} else {
			pos += 1;
		}
	}
	Ok (())
}
