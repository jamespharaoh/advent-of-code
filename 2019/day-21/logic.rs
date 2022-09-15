//! Logic for solving the puzzles

use super::*;

use emul::Instr;
use emul::Prog;
use emul::Reg;
use input::Input;
use model::Cpu;
use model::Frag;
use model::Val;
use model::Regs;
use model::Rules;
use model::RulesSet;
use model::Sample;

pub fn part_one (input: & Input) -> GenResult <Val> {
	calc_result::<4> (input, "WALK")
}

pub fn part_two (input: & Input) -> GenResult <Val> {
	calc_result::<9> (input, "RUN")
}

fn calc_result <const LEN: usize> (input: & Input, verb: & str) -> GenResult <Val> {
	let mut samples: Vec <Sample> = Vec::new ();
	let mut rules_set: RulesSet <LEN> = RulesSet::default ();
	loop {
		let prog = find_prog (& rules_set) ?;
		match run_prog::<LEN> (input, & prog, verb) ? {
			RunResult::Success (val) => return Ok (val),
			RunResult::Failure (sample) => {
				if 40 < sample.len () {
					return Err ("Refusing to handle very large sample".into ());
				}
				if 20 < samples.len () {
					return Err ("Refusing to handle too many failures".into ());
				}
				if let Some (prev) = samples.iter ().position (|other| other == & sample) {
					return Err (format! ("Sample repeated ({prev})").into ());
				}
				samples.push (sample.clone ());
				let new_rules_set = analyse::<LEN> (& sample);
				rules_set.append (& new_rules_set);
			},
		}
	}
}

fn find_prog <const LEN: usize> (rules_set: & RulesSet <LEN>) -> GenResult <Prog> {
	let mut all_rules: Vec <& Rules <LEN>> = rules_set.iter ().collect ();
	all_rules.sort_by_key (|rules| (rules.num_conflicts (), rules.num_true ()));
	for rules in all_rules {
		let frag_sort_key = |& (frag, num_matches): & (Frag <LEN>, _)|
			cmp::Reverse (num_matches * 0xffff_u32 / (frag.num_holes () + 4));
		let all_frags: Vec <(Frag <LEN>, u32)> = Frag::<LEN>::iter_all ()
			.filter_map (|frag| {
				let mut num_matches = 0;
				for & (regs, jump) in rules {
					if ! frag.matches (regs) { continue }
					if ! jump { return None }
					num_matches += 1;
				}
				if num_matches == 0 { return None }
				Some ((frag, num_matches))
			})
			.sorted_by_key (frag_sort_key)
			.collect ();
		'CHOOSE_FRAGS: for frag_iter in 0 .. 3 {
			if all_frags.len () < frag_iter { break }
			let mut frags: Vec <(Frag <LEN>, u32)> =
				all_frags.iter ().copied ()
					.skip (frag_iter)
					.collect ();
			let mut pending_rules: Vec <(Regs <LEN>, bool)> =
				rules.iter ().copied ()
					.filter (|& (_, jump)| jump)
					.filter (|& (regs, _)| regs != Regs::ALL)
					.collect ();
			let mut chosen_frags: Vec <Frag <LEN>> = Vec::new ();
			while ! pending_rules.is_empty () {
				let chosen_frag = some_or! (
					frags.iter ().copied ()
						.min_by_key (frag_sort_key)
						.map (|(frag, _)| frag),
					break 'CHOOSE_FRAGS);
				chosen_frags.push (chosen_frag);
				pending_rules.retain (|& (regs, _)| ! chosen_frag.matches (regs));
				for & mut (frag, ref mut num_matches) in frags.iter_mut () {
					* num_matches =
						pending_rules.iter ().copied ()
							.filter (|& (regs, _)| frag.matches (regs))
							.count ()
							.as_u32 ();
				}
				frags.retain (|& (_, num_matches)| num_matches != 0);
			}
			if let Some (frag_idx) =
				chosen_frags.iter ()
					.position (|& frag| frag.num_regs () == 1) {
				let frag = chosen_frags.remove (frag_idx);
				chosen_frags.insert (0, frag);
			}
			if let Some (prog) = assemble_prog (& chosen_frags) {
				return Ok (prog);
			}
		}
	}
	Err ("Can't find programme".into ())
}

/// Build a [`Prog`] from a list of [`Frag`]s
///
/// Each [`Frag`] represents a set of registers to be matched to specific values. The resulting
/// [`Prog`] should leave `true` in the jump register `J` if any of the fragments matches, but not
/// otherwise.
///
/// If the first [`Frag`] only matches a single register then this execute more efficiently, using
/// only a single instruction. Otherwise there will be one instruction per register, plus an extra
/// two, or only one if there is only a single negative register match.
///
#[ must_use ]
fn assemble_prog <const LEN: usize> (frags: & [Frag <LEN>]) -> Option <Prog> {
	let mut prog = Prog::new ();
	for frag in frags.iter () {
		if prog.is_empty () && frag.num_regs () == 1 {
			let (idx, val) =
				frag.iter ().enumerate ()
					.find (|& (_, val)| val.is_some ())
					.map (|(idx, val)| (idx, val.unwrap ()))
					.unwrap ();
			assert! (! val);
			prog.push (Instr::Not (idx.try_into ().ok () ?, Reg::J)) ?;
			continue;
		}
		if frag.num_holes () == 1 {
			let idx = frag.iter ()
				.position (|val| val == Some (false))
				.unwrap ();
			prog.push (Instr::Not (idx.try_into ().ok () ?, Reg::T)) ?;
		} else {
			for (idx, _) in frag.iter ().enumerate ()
					.filter (|& (_, val)| val == Some (false)) {
				prog.push (Instr::Or (idx.try_into ().ok () ?, Reg::T)) ?;
			}
			prog.push (Instr::Not (Reg::T, Reg::T)) ?;
		}
		for (idx, _) in frag.iter ().enumerate ()
				.filter (|& (_, val)| val == Some (true)) {
			prog.push (Instr::And (idx.try_into ().ok () ?, Reg::T)) ?;
		}
		prog.push (Instr::Or (Reg::T, Reg::J)) ?;
	}
	Some (prog)
}

/// Analyse a [`Sample`] to produce a [`RulesSet`]
///
/// This performs a branching iteration over paths from the start to the end of the sample,
/// recording the decision made at each point along with the register values. The result is a list
/// of rules, where a program that implements any one of them will be able to pass this specific
/// sample.
///
fn analyse <const LEN: usize> (sample: & Sample) -> RulesSet <LEN> {
	let mut todo: Vec <(Rules <LEN>, usize)> = Vec::new ();
	let start_rules = Rules::default ();
	todo.push ((start_rules, 0));
	let mut seen: HashSet <(Rules <LEN>, usize)> = HashSet::new ();
	let mut rules_set = RulesSet::new ();
	while let Some ((rules, idx)) = todo.pop () {
		if ! seen.insert ((rules.clone (), idx)) { continue }
		if sample [idx .. ].iter ().all (|& val| val) {
			rules_set.push (rules);
			continue;
		}
		let regs = Regs::from_slice (& sample [idx + 1 .. ]);
		if sample.get (idx + 1).copied ().unwrap_or (true) {
			if let Some (new_rules) = rules.with_rule (regs, false) {
				todo.push ((new_rules, idx + 1));
			}
		}
		if sample.get (idx + 4).copied ().unwrap_or (true) {
			if let Some (new_rules) = rules.with_rule (regs, true) {
				todo.push ((new_rules, idx + 4));
			}
		}
	}
	rules_set
}

fn run_prog <const LEN: usize> (input: & Input, prog: & Prog, verb: & str) -> GenResult <RunResult> {
	let mut cpu = Cpu::new (input.data.clone ());
	cpu.set_max_ops (input.params.max_ops);
	cpu.input_str (& format! ("{prog}{verb}\n"));
	let mut output_str = String::new ();
	let mut output_val = None;
	while let Some (val) = cpu.run ().output () ? {
		if (0x00_i32 .. 0x80_i32).contains (& val) { output_str.push (val.as_char ()); }
		else if 0x80_i32 <= val { output_val = Some (val); }
		else { return Err (format! ("Invalid output: {val}").into ()) }
	}
	if let Some (val) = output_val { return Ok (RunResult::Success (val)) }
	let sample = extract_sample::<LEN> (& output_str) ?;
	Ok (RunResult::Failure (sample))
}

fn extract_sample <const LEN: usize> (output: & str) -> GenResult <Sample> {
	let sample_line =
		output.split ('\n')
			.find (|line| line.starts_with ('#'))
			.ok_or ("Couldn't find sample in output") ?;
	let mut sample_data: Vec <bool> =
		sample_line.chars ()
			.map (|ch| Ok::<_, GenError> (match ch {
				'#' => true,
				'.' => false,
				other => return Err (format! ("Invalid character in sample: {other}").into ()),
			}))
			.try_collect () ?;
	sample_data.extend (iter::repeat (true).take (LEN));
	Ok (Sample::from (sample_data))
}

enum RunResult {
	Failure (Sample),
	Success (Val),
}
