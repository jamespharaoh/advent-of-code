//! Advent of Code 2021: Day 24: Arithmetic Logic Unit
//!
//! [https://adventofcode.com/2021/day/24](https://adventofcode.com/2021/day/24)

#![ allow (clippy::missing_inline_in_public_items) ]
#![ allow (dead_code) ]

use aoc_common::*;

puzzle_info! {
	name = "Arithmetic Logic Unit";
	year = 2021;
	day = 24;
	part_one = |lines| logic::part_one (lines);
	part_two = |lines| logic::part_two (lines);
	commands = [
		( name = "all"; method = tool::all; ),
		( name = "machine"; method = tool::machine; ),
		( name = "solver"; method = tool::solver; ),
	];
}

pub mod machine;
pub mod quick;
pub mod solver;

pub mod logic {

	use super::*;
	use machine::Instr;

	pub fn part_one (lines: & [& str]) -> GenResult <String> {
		let prog = machine::parse_prog (lines) ?;
		calc_result (& prog, true)
	}

	pub fn part_two (lines: & [& str]) -> GenResult <String> {
		let prog = machine::parse_prog (lines) ?;
		calc_result (& prog, false)
	}

	pub fn calc_result (prog: & [Instr], reverse: bool) -> GenResult <String> {
		let steps = quick::steps_for (prog) ?;
		let result = quick::iterator (& steps, reverse).next ()
			.ok_or ("Not found") ?;
		Ok (model::input_to_str (result))
	}

}

pub mod model {

	use super::*;

	pub type Input = [u8; 14];

	#[ inline ]
	#[ must_use ]
	pub fn input_from_str (input_str: & str) -> Input {
		input_str.chars ().map (
			|letter| letter.to_digit (10).unwrap ().pan_u8 (),
		).collect::<Vec <u8>> ().try_into ().unwrap ()
	}

	#[ inline ]
	#[ must_use ]
	pub fn input_to_str (input: Input) -> String {
		input.into_iter ().map (
			|val| char::from_digit (val.pan_u32 (), 10).unwrap (),
		).collect::<String> ()
	}

}

pub mod tool {

	use super::*;
	use machine::Instr;
	use machine::Machine;
	use machine::MachineRegs;
	use solver::Solver;

	#[ derive (clap::Parser) ]
	pub struct AllArgs {
		input: String,
	}

	#[ allow (clippy::print_stdout) ]
	pub fn all (args: AllArgs) -> GenResult <()> {
		let input_string = fs::read_to_string (args.input) ?;
		let input_lines: Vec <& str> = input_string.trim ().split ('\n').collect ();
		let prog = machine::parse_prog (& input_lines) ?;
		let steps = quick::steps_for (& prog) ?;
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
		let prog = load_prog () ?;
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
				if machine.step (& prog, input).map_err (|err| format! ("{:?}", err)) ? {
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
		let prog = load_prog () ?;
		let (solver, regs) = Solver::from_prog (& prog);
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

	fn load_prog () -> GenResult <Vec <Instr>> {
		let input_string = fs::read_to_string ("inputs/day-24") ?;
		let input_lines: Vec <& str> = input_string.trim ().split ('\n').collect ();
		machine::parse_prog (& input_lines)
	}

}
