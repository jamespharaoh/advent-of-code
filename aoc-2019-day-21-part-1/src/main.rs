use intcode::Machine;
use intcode::RunResult;
use std::fs;
use std::rc::Rc;

mod intcode;

fn main () {
	let programme_source = fs::read_to_string ("input").unwrap ();
	let programme = intcode::from_str (& programme_source);

	let partials = gen_partial ();
	for partial_0 in partials.iter () {
		'NEXT_PROG: for partial_1 in partials.iter () {
			let mut prog: Vec <Rc <str>> = Vec::new ();
			prog.extend_from_slice (& partial_0);
			prog.extend_from_slice (& partial_1);
			let mut machine = Machine::new (programme.clone ());
			for line in prog.iter () {
				machine.input_str (& format! ("{}\n", line));
			}
			machine.input_str ("WALK\n");
			loop {
				match machine.run () {
					RunResult::Output (value) => {
						if value < 128 { continue }
						println! ("Got result: {}", value);
						for line in prog.iter () {
							println! ("  {}", line);
						}
					},
					RunResult::Halt => continue 'NEXT_PROG,
					_ => panic! (),
				}
			}
		}
	}

}

const REG_NAMES: [& str; 4] = ["A", "B", "C", "D"];

fn gen_partial () -> Vec <Vec <Rc <str>>> {
	let mut progs: Vec <Vec <Rc <str>>> = Vec::new ();
	for reg_name in REG_NAMES.iter () {
		let mut used_regs: Vec <& str> = Vec::new ();
		used_regs.push (reg_name);
		for invert_t in [false, true].iter () {
			let mut prog: Vec <Rc <str>> = Vec::new ();
			prog.push (format! ("NOT {} T", reg_name).into ());
			if ! invert_t {
				prog.push (format! ("NOT T T").into ());
			}
			gen_partial_regs (& mut progs, & used_regs, & prog);
		}
	}
	progs
}

fn gen_partial_regs (progs: & mut Vec <Vec <Rc <str>>>, used_regs: & [& str], prefix: & [Rc <str>]) {
	gen_partial_suffixes (progs, prefix);
	if used_regs.len () == 2 { return }
	for reg_name in REG_NAMES.iter () {
		if used_regs.contains (reg_name) { continue }
		let mut used_regs = used_regs.to_vec ();
		used_regs.push (reg_name);
		for op in ["OR", "AND"].iter () {
			let mut prog = prefix.to_vec ();
			prog.push (format! ("{} {} T", op, reg_name).into ());
			gen_partial_regs (progs, & used_regs, & prog);
		}
	}
}

fn gen_partial_suffixes (progs: & mut Vec <Vec <Rc <str>>>, prefix: & [Rc <str>]) {
	let mut prog_0 = prefix.to_vec ();
	prog_0.push (format! ("OR T J").into ());
	progs.push (prog_0);
	let mut prog_1 = prefix.to_vec ();
	prog_1.push (format! ("AND T J").into ());
	progs.push (prog_1);
}
