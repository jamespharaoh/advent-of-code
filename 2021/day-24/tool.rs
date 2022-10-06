use super::*;

use std::path::PathBuf;

use input::Input;
use machine::Machine;
use machine::MachineRegs;
use solver::Solver;

#[ derive (clap::Parser) ]
pub struct AllArgs {
	#[ clap (long, from_global, value_parser = PathBuf) ]
	input: PathBuf,
}

#[ allow (clippy::print_stdout) ]
pub fn all (args: AllArgs) -> GenResult <()> {
	let input_string = fs::read_to_string (args.input) ?;
	let input_lines: Vec <& str> = input_string.trim ().split ('\n').collect ();
	let input = Input::parse_from_lines (& input_lines) ?;
	let steps = quick::steps_for (& input.instrs) ?;
	quick::iterator (& steps, false).for_each (
		|input| println! ("{}", model::input_to_str (input)),
	);
	Ok (())
}

#[ derive (clap::Parser) ]
pub struct MachineArgs { inputs: Vec <String> }

#[ allow (clippy::needless_pass_by_value) ]
#[ allow (clippy::print_stdout) ]
pub fn machine (args: MachineArgs) -> GenResult <()> {
	fn dump_regs (regs: & MachineRegs) -> String {
		format! ("{:2}  {:10}  {:2}  {:10}", regs.w, regs.x, regs.y, regs.z)
	}
	let mut machines: Vec <(Machine, [i64; 14])> = args.inputs.iter ().map (
		|input_str| (
			Machine::new (),
			machine::machine_input (model::input_from_str (input_str)),
		),
	).collect ();
	let input = get_input () ?;
	let prog = & input.instrs;
	let mut done = false;
	while ! done {
		if machines [0].0.regs.pc % 18 == 0 {
			printer (& machines,
				|| print! ("+-- {:2} ---------+", machines [0].0.regs.pc / 18),
				|_| print! ("--------------------------------+"));
		}
		printer (& machines,
			|| print! ("| {:2}  {:9} |", machines [0].0.regs.pc % 18 + 1,
				prog.get (machines [0].0.regs.pc)
					.map_or_else (|| "(end)".to_owned (), |instr| format! ("{}", instr))),
			|machine| print! (" {:30} |", dump_regs (& machine.regs)));
		for & mut (ref mut machine, ref mut input) in machines.iter_mut () {
			if machine.step (prog, input).map_err (|err| format! ("{:?}", err)) ? {
				done = true;
			}
		}
	}
	printer (& machines,
		|| print! ("+---------------+"),
		|_| print! ("--------------------------------+"));
	printer (& machines,
		|| print! ("|               |"),
		|machine| print! (" {:30} |", format! ("Result: {}", machine.regs.z)));
	printer (& machines,
		|| print! ("+---------------+"),
		|_| print! ("--------------------------------+"));
	return Ok (());
	fn printer <
		BeforeFn: Fn (),
		EachFn: Fn (& Machine),
	> (machines: & [(Machine, [i64; 14])], before_fn: BeforeFn, each_fn: EachFn) {
		before_fn ();
		for & (ref machine, _) in machines.iter () {
			each_fn (machine);
		}
		print! ("\n");
	}
}

#[ derive (clap::Parser) ]
pub struct SolverArgs {
	args: Vec <String>,
}

#[ allow (clippy::needless_pass_by_value) ]
pub fn solver (args: SolverArgs) -> GenResult <()> {
	let input = get_input () ?;
	let (solver, regs) = Solver::from_prog (& input.instrs);
	let reg_z =
		regs.into_iter ()
			.filter (|& (ref name, _)| name.as_ref () == "z")
			.map (|(_, reg)| reg)
			.next ()
			.unwrap ();
	match args.args [0].as_str () {
		"solver-full" => {
			solver.dump (3, true);
		},
		"solver-z" => {
			solver.dump_symbol (3, true, & reg_z);
		},
		"solver-z-auto" => {
			solver.dump_symbol_auto (& reg_z);
		},
		_ => panic! (),
	}
	Ok (())
}

fn get_input () -> GenResult <Input> {
	let input_string = fs::read_to_string ("inputs/day-24") ?;
	let input_lines: Vec <& str> = input_string.trim ().split ('\n').collect ();
	let input = Input::parse_from_lines (& input_lines) ?;
	Ok (input)
}
