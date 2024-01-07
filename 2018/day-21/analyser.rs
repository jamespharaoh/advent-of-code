use super::*;

use std::fs::File;
use std::io::Write as _;
use std::path::PathBuf;

use input::Input;
use model::ArgType;
use model::CpuError;
use model::Instr;
use model::Op;
use model::Opcode;
use model::Val;

type MultiReg = MultiVal <8>;

args_decl! {
	pub struct Args {
		input: Option <PathBuf>,
	}
}

pub fn run (args: Args) -> GenResult <()> {

	let input_path = puzzle_metadata ().find_input_or_arg (& args.input);
	let input_string = fs::read_to_string (input_path) ?;
	let input_lines: Vec <& str> = input_string.trim_end ().split ('\n').collect ();
	let input = Input::parse_from_lines (& input_lines) ?;

	let reg_multis: [MultiReg; 6] =
		array::from_fn (|idx| match idx {
			0 => MultiReg::unlimited (),
			_ => MultiReg::single (0),
		});

	let instrs = get_instr_infos (input.ip.pan_u16 (), & input.instrs, & reg_multis);
	let blocks = get_blocks (& instrs);
	let super_blocks = get_super_blocks (& blocks);

	dot::write (& blocks, & super_blocks) ?;

	Ok (())

}

mod dot {

	use super::*;

	struct Styles {
		background_colour: & 'static str,
		block_fill: & 'static str,
		branch_fill: & 'static str,
		branch_text_colour: & 'static str,
		branch_text_font: & 'static str,
		code_fill: & 'static str,
		code_text_colour: & 'static str,
		code_text_font: & 'static str,
		edge_colour: & 'static str,
		end_fill: & 'static str,
		end_text_colour: & 'static str,
		end_text_font: & 'static str,
		start_fill: & 'static str,
		start_text_colour: & 'static str,
		start_text_font: & 'static str,
	}

	impl Default for Styles {
		fn default () -> Self {
			Self {
				background_colour: "black",
				block_fill: "dodgerblue4",
				branch_fill: "goldenrod1",
				branch_text_colour: "black",
				branch_text_font: "LiberationSansMono",
				code_fill: "white",
				code_text_colour: "black",
				code_text_font: "LiberationSansMono",
				edge_colour: "white",
				end_fill: "red2",
				end_text_colour: "white",
				end_text_font: "LiberationSans-Bold",
				start_fill: "forestgreen",
				start_text_font: "LiberationSans-Bold",
				start_text_colour: "white",
			}
		}
	}

	pub fn write (
		blocks: & HashMap <Val, Rc <Block>>,
		super_blocks: & HashMap <Val, Rc <SuperBlock>>,
	) -> GenResult <()> {

		let styles = Styles::default ();

		let mut file = File::create ("target/graph.dot") ?;
		write! (file, "digraph analysis {{\n") ?;
		write! (file, "  bgcolor = \"{}\"\n", styles.background_colour) ?;
		write! (file, "  edge [\n") ?;
		write! (file, "    color = \"{}\"\n", styles.edge_colour) ?;
		write! (file, "    len=2\n") ?;
		write! (file, "   ]\n") ?;
		write! (file, "  start [\n") ?;
		write! (file, "    fillcolor = \"{}\"\n", styles.start_fill) ?;
		write! (file, "    fontcolor = \"{}\"\n", styles.start_text_colour) ?;
		write! (file, "    fontname = \"{}\"\n", styles.start_text_font) ?;
		write! (file, "    style = \"filled\"\n") ?;
		write! (file, "  ]\n") ?;
		write! (file, "  end [\n") ?;
		write! (file, "    fillcolor = \"{}\"\n", styles.end_fill) ?;
		write! (file, "    fontcolor = \"{}\"\n", styles.end_text_colour) ?;
		write! (file, "    fontname = \"{}\"\n", styles.end_text_font) ?;
		write! (file, "    style = \"filled\"\n") ?;
		write! (file, "  ]\n") ?;
		write_super_blocks (& mut file, & styles, blocks, super_blocks) ?;
		for block in blocks.values () {
			if block.prev.values ().contains (& Val::MAX) {
				write! (file, "  start -> block_{}_code:start", block.idx) ?;
			}
		}
		for block in blocks.values () {
			let block_idx = block.idx;
			match block.block_next {
				BlockNext::Simple (next_idx) => {
					if next_idx == Val::MAX {
						write! (file, "    block_{block_idx}_end -> end\n") ?;
					} else {
						let next = & blocks [& next_idx];
						if next.super_idx == block.super_idx { continue }
						write! (file, "    block_{block_idx}_end -> block_{next_idx}_code:start\n") ?;
					}
				},
				BlockNext::Condition { ref options, .. } => {
					for & (val, next_idx) in options {
						if next_idx == Val::MAX {
							write! (file, "    block_{block_idx}_end_{val} -> end\n") ?;
						} else {
							let next = & blocks [& next_idx];
							if next.super_idx == block.super_idx { continue }
							write! (file, "    block_{block_idx}_end_{val} -> block_{next_idx}_code:start [\n") ?;
							write! (file, "    ]\n") ?;
						}
					}
				},
			}
		}
		write! (file, "}}\n") ?;

		Ok (())

	}

	fn write_super_blocks (
		file: & mut File,
		styles: & Styles,
		blocks: & HashMap <Val, Rc <Block>>,
		super_blocks: & HashMap <Val, Rc <SuperBlock>>,
	) -> GenResult <()> {
		for super_block in super_blocks.values () {
			let super_block_idx = super_block.idx;
			write! (file, "  subgraph cluster_super_{super_block_idx} {{\n") ?;
			write! (file, "    style = \"filled\"\n") ?;
			write! (file, "    fillcolor = \"{}\"\n", styles.block_fill) ?;
			for block in & super_block.blocks {
				let block_idx = block.idx;
				let mut label = String::new ();
				for instr in & block.instrs {
					if label.is_empty () {
						write! (label, "<start>") ?;
					} else {
						write! (label, "|") ?;
					}
					write! (label, "{}", InstrDisplay { instr, idx: true, orig: false }) ?;
				}
				write! (file, "    block_{block_idx}_code [\n") ?;
				write! (file, "      fillcolor = \"{}\"\n", styles.code_fill) ?;
				write! (file, "      fontcolor = \"{}\"\n", styles.code_text_colour) ?;
				write! (file, "      fontname = \"{}\"\n", styles.code_text_font) ?;
				write! (file, "      label = \"{{{label}}}\"\n") ?;
				write! (file, "      shape = \"record\"\n") ?;
				write! (file, "      style = \"filled\"\n") ?;
				write! (file, "    ]\n") ?;
				match block.block_next {
					BlockNext::Simple (next_idx) => {
						if next_idx == Val::MAX {
							write! (file, "    block_{block_idx}_code:s -> block_{block_idx}_end:n\n") ?;
							write! (file, "    block_{block_idx}_end [\n") ?;
							write! (file, "      fillcolor = \"{}\"\n", styles.branch_fill) ?;
							write! (file, "      label = \"else\"\n") ?;
							write! (file, "      style = \"filled\"\n") ?;
							write! (file, "    ]\n") ?;
						} else {
							let next = & blocks [& next_idx];
							if block.super_idx == next.super_idx {
								write! (file, "    block_{block_idx}_code:s -> block_{next_idx}_code:start\n") ?;
							} else {
								write! (file, "    block_{block_idx}_code:s -> block_{block_idx}_end\n") ?;
								write! (file, "    block_{block_idx}_end [\n") ?;
								write! (file, "      label=\"goto {next_idx}\"\n") ?;
								write! (file, "    ]\n") ?;
							}
						}
					},
					BlockNext::Condition { arg, ref options } => {
						for & (val, next_idx) in options {
							if next_idx == Val::MAX {
								write! (file, "    block_{block_idx}_end_{val} [\n") ?;
								write! (file, "      fillcolor = \"{}\"\n", styles.branch_fill) ?;
								write! (file, "      label = \"halt\"\n") ?;
								write! (file, "      style = \"filled\"\n") ?;
								write! (file, "    ]\n") ?;
								write! (file, "    block_{block_idx}_code:s -> block_{block_idx}_end_{val} [\n") ?;
								write! (file, "      len=0.1\n") ?;
								write! (file, "    ]\n") ?;
							} else {
								let next = & blocks [& next_idx];
								write! (file, "    block_{block_idx}_end_{val} [\n") ?;
								write! (file, "      fillcolor = \"{}\"\n", styles.branch_fill) ?;
								write! (file, "      fontcolor = \"{}\"\n", styles.branch_text_colour) ?;
								write! (file, "      fontname = \"{}\"\n", styles.branch_text_font) ?;
								write! (file, "      label = \"{arg} = {val}\"\n") ?;
								write! (file, "      style = \"filled\"\n") ?;
								write! (file, "    ]\n") ?;
								write! (file, "    block_{block_idx}_code:s -> block_{block_idx}_end_{val}\n") ?;
								if block.super_idx == next.super_idx {
									write! (file, "    block_{block_idx}_end_{val} -> block_{next_idx}_code:start [\n") ?;
									write! (file, "      constraint=false\n") ?;
									write! (file, "    ]\n") ?;
								}
							}
						}
					},
				}
			}
			write! (file, "  }}\n") ?;
		}
		Ok (())
	}

}

fn get_super_blocks (
	blocks: & HashMap <Val, Rc <Block>>,
) -> HashMap <Val, Rc <SuperBlock>> {

	let mut super_blocks: HashMap <Val, SuperBlock> = HashMap::new ();

	for block in blocks.values () {
		let mut final_block = block;
		while let BlockNext::Simple (next_idx) = final_block.block_next {
			if next_idx == Val::MAX { break }
			final_block = & blocks [& next_idx];
		}
		let super_block =
			super_blocks.entry (final_block.idx)
				.or_insert (SuperBlock { idx: final_block.idx, blocks: Vec::new () });
		super_block.blocks.push (Rc::clone (block));
	}

	// TODO this fails with nightly for some reason, should be .into_iter ()
	super_blocks.iter ()
		.map (|(key, val)| (* key, Rc::new (val.clone ())))
		.collect ()

}

#[ derive (Clone, Debug) ]
pub struct SuperBlock {
	idx: Val,
	blocks: Vec <Rc <Block>>,
}

fn get_instr_infos (
	ip: u16,
	instrs: & [Instr],
	reg_multis: & [MultiReg; 6],
) -> Vec <Rc <InstrInfo>> {

	let mut instrs: Vec <InstrInfo> =
		instrs.iter ().copied ()
			.enumerate ()
			.map (|(instr_idx, instr)| {
				let instr_idx = Val::from_usize (instr_idx).unwrap ();
				InstrInfo::new (ip, instr_idx, instr)
			})
			.collect ();
	let instrs_len = instrs.len ();

	let mut todo: Vec <(Val, Val, [MultiReg; 6])> = Vec::new ();
	let mut seen: HashSet <(Val, [MultiReg; 6])> = HashSet::new ();
	match reg_multis [ip.pan_usize ()] {
		MultiVal::Limited (ref vals) => {
			for val in vals.iter ().copied () {
				let mut reg_multis = reg_multis.clone ();
				reg_multis [ip.pan_usize ()] = MultiReg::single (val);
				if seen.insert ((val, reg_multis.clone ())) {
					todo.push ((Val::MAX, val, reg_multis));
				}
			}
		},
		MultiReg::Unlimited => panic! (),
	}

	while let Some ((prev_idx, instr_idx, reg_multis)) = todo.pop () {
		let instr = & mut instrs [instr_idx.pan_usize ()];
		instr.prev.push (prev_idx);
		for (reg_vals, reg_vals_new) in instr.before.iter_mut ().zip (reg_multis.iter ()) {
			reg_vals.update (reg_vals_new);
		}
		let arg_a = match instr.opcode.arg_a () {
			ArgType::Reg => Some (instr.before [instr.arg_a.as_reg_idx ()].clone ()),
			ArgType::Imm => Some (MultiReg::single (instr.arg_a.as_imm_val ())),
			ArgType::Ignore => None,
		};
		let arg_b = match instr.opcode.arg_b () {
			ArgType::Reg => Some (instr.before [instr.arg_b.as_reg_idx ()].clone ()),
			ArgType::Imm => Some (MultiReg::single (instr.arg_b.as_imm_val ())),
			ArgType::Ignore => None,
		};
		let args: Option <Vec <(Option <Val>, Option <Val>)>> = match (arg_a, arg_b) {
			(Some (arg_a), Some (arg_b)) =>
				(arg_a.is_limited () && arg_b.is_limited ())
					.then (|| arg_a.values ().iter ()
						.flat_map (|& val_a| arg_b.values ().iter ()
							.map (move |& val_b| (Some (val_a), Some (val_b))))
						.collect ()),
			(Some (arg_a), None) =>
				arg_a.is_limited ()
					.then (|| arg_a.values ().iter ()
						.map (|& val_a| (Some (val_a), None))
						.collect ()),
			(None, Some (arg_b)) =>
				arg_b.is_limited ()
					.then (|| arg_b.values ().iter ()
						.map (|& val_b| (None, Some (val_b)))
						.collect ()),
			(None, None) => unreachable! (),
		};
		if let Some (args) = args {
			for (arg_a, arg_b) in args {
				let arg_c = instr.opcode.op ().apply (
						arg_a.ok_or (CpuError::Internal),
						arg_b.ok_or (CpuError::Internal))
					.unwrap ();
				let next_idx = if instr.arg_c.is_ip () { arg_c + 1 } else { instr_idx + 1 };
				let mut reg_multis = instr.before.clone ();
				reg_multis [instr.arg_c.as_reg_idx ()] = MultiReg::single (arg_c);
				reg_multis [ip.pan_usize ()] = MultiReg::single (next_idx);
				if next_idx.pan_usize () < instrs_len {
					instr.next.push (next_idx);
					if seen.insert ((next_idx, reg_multis.clone ())) {
						todo.push ((instr_idx, next_idx, reg_multis));
					}
				} else {
					instr.next.push (Val::MAX);
					instr.halts.push (reg_multis);
				}
			}
		} else if instr.arg_c.is_ip () {
			panic! ("{instr_idx} {args:?} {:?}", instr.before);
		} else {
			let mut reg_multis = instr.before.clone ();
			reg_multis [instr.arg_c.as_reg_idx ()] =
				match instr.opcode.op () {
					Op::Add | Op::Mul | Op::Ban | Op::Bor | Op::Set => MultiVal::unlimited (),
					Op::Gt | Op::Eq => MultiReg::double (0, 1),
				};
			reg_multis [ip.pan_usize ()] = MultiReg::single (instr_idx + 1);
			instr.next.push (instr_idx + 1);
			if seen.insert ((instr_idx + 1, reg_multis.clone ())) {
				todo.push ((instr_idx, instr_idx + 1, reg_multis));
			}
		}
	}

	fill_instr_prov (ip, & mut instrs);

	instrs.into_iter ()
		.filter (|instr| ! instr.prev.is_empty ())
		.map (Rc::new)
		.collect ()

}

fn fill_instr_prov (ip: u16, instrs: & mut [InstrInfo]) {

	// track each start register from each starting point

	for reg in 0 .. 6 {
		if reg == ip { continue }
		for instr_idx in 0 .. Val::from_usize (instrs.len ()).unwrap () {
			let instr = & instrs [instr_idx.pan_usize ()];
			if ! instr.prev.contains (Val::MAX) { continue }
			fill_instr_prov_from (instrs, instr_idx, reg, ValProv::InReg (reg));
		}
	}

	// track each output register from each instruction

	for instr_idx in 0 .. Val::from_usize (instrs.len ()).unwrap () {
		let instr = & instrs [instr_idx.pan_usize ()];
		if instr.arg_c.is_ip () { continue }
		let reg_num = instr.arg_c.as_reg_num ();
		for instr_next_idx in 0 .. instr.next.len () {
			let next_idx = instrs [instr_idx.pan_usize ()].next.values () [instr_next_idx.pan_usize ()];
			if next_idx == Val::MAX {
				unimplemented! ();
			} else {
				fill_instr_prov_from (instrs, next_idx, reg_num, ValProv::Instr (instr_idx));
			}
		}
	}

	// assign temporary ids where possible

	// TODO match start regs
	let mut temps: HashMap <ValProv, u16> = HashMap::new ();
	for orig_instr_idx in 0 .. Val::from_usize (instrs.len ()).unwrap () {
		let orig_instr = & instrs [orig_instr_idx.pan_usize ()];
		if orig_instr.arg_c.is_ip () { continue }
		let mut aliases: HashSet <ValProv> = HashSet::new ();
		if temps.contains_key (& ValProv::Instr (orig_instr_idx)) { continue }
		aliases.insert (ValProv::Instr (orig_instr_idx));
		loop {
			let mut progress = false;
			for instr in & * instrs {
				if instr.arg_a_prov.iter ().any (|& arg_prov| aliases.contains (& arg_prov)) {
					for & arg_prov in & instr.arg_a_prov {
						if aliases.insert (arg_prov) {
							progress = true;
						}
					}
				}
				if instr.arg_b_prov.iter ().any (|& arg_prov| aliases.contains (& arg_prov)) {
					for & arg_prov in & instr.arg_b_prov {
						if aliases.insert (arg_prov) {
							progress = true;
						}
					}
				}
			}
			if ! progress { break }
		}
		let temp_idx = temps.len ().pan_u16 ();
		for & alias in aliases.iter () { temps.insert (alias, temp_idx); }
	}

	// replace references to registers with temporaries

	for instr in & mut * instrs {
		for & arg_a_prov in & instr.arg_a_prov {
			if let Some (& temp_idx) = temps.get (& arg_a_prov) {
				instr.arg_a.temp_assign (temp_idx);
			}
		}
		for & arg_b_prov in & instr.arg_b_prov {
			if let Some (& temp_idx) = temps.get (& arg_b_prov) {
				instr.arg_b.temp_assign (temp_idx);
			}
		}
		if let Some (& temp_idx) = temps.get (& ValProv::Instr (instr.idx)) {
			instr.arg_c.temp_assign (temp_idx);
		}
	}

}

fn fill_instr_prov_from (instrs: & mut [InstrInfo], start_idx: Val, reg: u16, prov: ValProv) {
	let mut todo: Vec <Val> = Vec::new ();
	todo.push (start_idx);
	let mut seen: HashSet <Val> = HashSet::new ();
	while let Some (instr_idx) = todo.pop () {
		let instr = & mut instrs [instr_idx.pan_usize ()];
		if instr.arg_a.is_reg () && instr.arg_a.as_reg_num () == reg
				&& ! instr.arg_a_prov.contains (& prov) {
			instr.arg_a_prov.push (prov);
			instr.arg_a_prov.sort ();
		}
		if instr.arg_b.is_reg () && instr.arg_b.as_reg_num () == reg
				&& ! instr.arg_b_prov.contains (& prov) {
			instr.arg_b_prov.push (prov);
			instr.arg_b_prov.sort ();
		}
		if instr.arg_c.is_reg () && instr.arg_c.as_reg_num () == reg { continue }
		if ! seen.insert (instr_idx) { continue }
		for & next_idx in instr.next.values () {
			if next_idx == Val::MAX {
				// TODO
			} else {
				todo.push (next_idx);
			}
		}
	}
}

fn get_blocks (instrs: & [Rc <InstrInfo>]) -> HashMap <Val, Rc <Block>> {
	let all_instrs: HashMap <Val, Rc <InstrInfo>> =
		instrs.iter ()
			.map (|instr| (instr.idx, Rc::clone (instr)))
			.collect ();
	let mut blocks_temp: HashMap <Val, Block> = HashMap::new ();
	let mut instr_blocks: HashMap <Val, Val> = HashMap::new ();
	for mut instr in instrs {
		let first = instr;
		let mut block_instrs = Vec::new ();
		if instr.prev.len () == 1 && ! instr.prev.contains (Val::MAX) {
			let prev = & all_instrs [& instr.prev.values () [0]];
			if prev.next.len () == 1 { continue }
		}
		loop {
			instr_blocks.insert (instr.idx, first.idx);
			if ! instr.arg_c.is_ip () {
				block_instrs.push (Rc::clone (instr));
			}
			if instr.next.len () > 1 { break }
			let next = & all_instrs [& instr.next.values () [0]];
			if next.prev.len () > 1 { break }
			instr = next;
		}
		let last = instr;
		let block_next = if last.next.len () == 1 {
			BlockNext::Simple (last.next.values () [0])
		} else {
			let arg = match (last.arg_a.is_ip (), last.arg_b.is_ip ()) {
				(false, true) => last.arg_a,
				(true, false) => last.arg_b,
				_ => panic! (),
			};
			let mut options = Vec::new ();
			for & next_idx in last.next.values () {
				let val = next_idx - last.idx - 1;
				options.push ((val, next_idx));
			}
			BlockNext::Condition { arg, options }
		};
		blocks_temp.insert (first.idx, Block {
			idx: first.idx,
			super_idx: Val::MAX,
			instrs: block_instrs,
			halts: last.halts.clone (),
			prev: first.prev.clone (),
			next: last.next.clone (),
			block_next,
		});
	}
	blocks_temp.values ()
		.map (|block| {
			let mut final_block = block;
			while let BlockNext::Simple (next_idx) = final_block.block_next {
				if next_idx == Val::MAX { break }
				final_block = & blocks_temp [& next_idx];
			}
			( block.idx, Rc::new (Block {
				super_idx: final_block.idx,
				prev: {
					let mut prev = MultiVal::empty ();
					for & prev_idx in block.prev.values () {
						if prev_idx == Val::MAX {
							prev.push (Val::MAX);
						} else {
							prev.push (instr_blocks [& prev_idx]);
						}
					}
					prev
				},
				.. block.clone ()
			}))
		})
		.collect ()
}

#[ derive (Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd) ]
pub struct Block {
	idx: Val,
	super_idx: Val,
	instrs: Vec <Rc <InstrInfo>>,
	halts: Vec <[MultiReg; 6]>,
	prev: MultiVal <{ usize::MAX }>,
	next: MultiVal <{ usize::MAX }>,
	block_next: BlockNext,
}

#[ derive (Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd) ]
enum BlockNext {
	Simple (Val),
	Condition { arg: ArgInfo, options: Vec <(Val, Val)> },
}

#[ derive (Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd) ]
struct InstrInfo {
	idx: Val,
	before: [MultiReg; 6],
	after: [MultiReg; 6],
	halts: Vec <[MultiReg; 6]>,
	prev: MultiVal <{ usize::MAX }>,
	next: MultiVal <{ usize::MAX }>,
	opcode: Opcode,
	arg_a: ArgInfo,
	arg_b: ArgInfo,
	arg_c: ArgInfo,
	arg_a_prov: Vec <ValProv>,
	arg_b_prov: Vec <ValProv>,
}

impl InstrInfo {
	fn new (ip: u16, instr_idx: Val, instr: Instr) -> Self {
		Self {
			idx: instr_idx,
			before: array::from_fn (|_| MultiReg::empty ()),
			after: array::from_fn (|_| MultiReg::empty ()),
			halts: Vec::new (),
			prev: MultiVal::empty (),
			next: MultiVal::empty (),
			opcode: instr.opcode,
			arg_a: ArgInfo::auto (ip, instr.opcode.arg_a (), instr.arg_a),
			arg_b: ArgInfo::auto (ip, instr.opcode.arg_b (), instr.arg_b),
			arg_c: ArgInfo::auto (ip, ArgType::Reg, instr.arg_c),
			arg_a_prov: Vec::new (),
			arg_b_prov: Vec::new (),
		}
	}
}

struct InstrDisplay <'ins> {
	instr: & 'ins InstrInfo,
	idx: bool,
	orig: bool,
}

impl <'ins> Display for InstrDisplay <'ins> {
	fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
		if self.idx { write! (formatter, "[{}]: ", self.instr.idx) ?; }
		write! (formatter, "{}", self.instr.arg_a.display (self.orig)) ?;
		if self.instr.opcode.arg_b () != ArgType::Ignore {
			match self.instr.opcode.op () {
				Op::Add => write! (formatter, " + ") ?,
				Op::Mul => write! (formatter, " × ") ?,
				Op::Ban => write! (formatter, " and ") ?,
				Op::Bor => write! (formatter, " or ") ?,
				Op::Set => unreachable! (),
				Op::Gt => write! (formatter, " &gt; ") ?,
				Op::Eq => write! (formatter, " = ") ?,
			}
			write! (formatter, "{}", self.instr.arg_b.display (self.orig)) ?;
		}
		write! (formatter, " ⇒ {}", self.instr.arg_c.display (self.orig)) ?;
		Ok (())
	}
}

#[ derive (Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd) ]
enum ArgInfo {
	Immediate (Val),
	#[ allow (unused) ] InReg (u16),
	#[ allow (unused) ] OutReg (u16),
	InOutReg (u16),
	Temp (u16, u16),
	InstrPtr (u16),
	Unused (u16),
}

impl ArgInfo {
	fn auto (ip: u16, arg_type: ArgType, arg: Val) -> Self {
		let ip_val = Val::from (ip);
		match arg_type {
			ArgType::Reg if arg == ip_val => Self::InstrPtr (ip),
			ArgType::Reg => Self::InOutReg (arg.pan_u16 ()),
			ArgType::Imm => Self::Immediate (arg),
			ArgType::Ignore => Self::Unused (arg.pan_u16 ()),
		}
	}
	const fn is_ip (self) -> bool {
		matches! (self, Self::InstrPtr (_))
	}
	const fn is_reg (self) -> bool {
		matches! (self, Self::InReg (_) | Self::OutReg (_) | Self::InOutReg (_) |
			Self::Temp (_, _) | Self::InstrPtr (_))
	}
	fn as_imm_val (self) -> Val {
		match self {
			Self::Immediate (val) => val,
			Self::InReg (_) | Self::OutReg (_) | Self::InOutReg (_) | Self::Temp (_, _)
				| Self::InstrPtr (_) | Self::Unused (_) => panic! (),
		}
	}
	fn as_reg_num (self) -> u16 {
		match self {
			Self::InReg (reg) => reg,
			Self::OutReg (reg) => reg,
			Self::InOutReg (reg) => reg,
			Self::Temp (reg, _) => reg,
			Self::InstrPtr (reg) => reg,
			Self::Immediate (_) | Self::Unused (_) => panic! (),
		}
	}
	fn as_reg_idx (self) -> usize {
		self.as_reg_num ().pan_usize ()
	}
	fn temp_assign (& mut self, temp_idx: u16) {
		* self = Self::Temp (self.as_reg_num (), temp_idx);
	}
	const fn display (& self, orig: bool) -> ArgInfoDisplay {
		ArgInfoDisplay { arg: self, orig }
	}
}

impl Debug for ArgInfo {
	fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
		match * self {
			Self::InReg (reg) => write! (formatter, "InReg ({reg})"),
			Self::OutReg (reg) => write! (formatter, "OutReg ({reg})"),
			Self::InOutReg (reg) => write! (formatter, "InOutReg ({reg})"),
			Self::Temp (reg, temp) => write! (formatter, "Temp ({reg}, {temp})"),
			Self::InstrPtr (reg) => write! (formatter, "InstrPtr ({reg})"),
			Self::Immediate (val) => write! (formatter, "Immediate ({val})"),
			Self::Unused (val) => write! (formatter, "Unused ({val})"),
		}
	}
}

impl Display for ArgInfo {
	fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
		Display::fmt (& ArgInfoDisplay { arg: self, orig: false }, formatter)
	}
}

struct ArgInfoDisplay <'arg> {
	arg: & 'arg ArgInfo,
	orig: bool,
}

impl <'arg> Display for ArgInfoDisplay <'arg> {
	fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
		match (* self.arg, self.orig) {
			(ArgInfo::Immediate (val), _) => write! (formatter, "{val}") ?,
			(ArgInfo::InReg (_), _) => unimplemented! (),
			(ArgInfo::OutReg (_), _) => unimplemented! (),
			(ArgInfo::InOutReg (reg), _) => write! (formatter, "r{reg}") ?,
			(ArgInfo::Temp (_, temp), false) => write! (formatter, "v{temp}") ?,
			(ArgInfo::Temp (reg, _), true) => write! (formatter, "r{reg}") ?,
			(ArgInfo::InstrPtr (_), false) => write! (formatter, "ip") ?,
			(ArgInfo::InstrPtr (reg), true) => write! (formatter, "r{reg}") ?,
			(ArgInfo::Unused (_), _) => unreachable! (),
		}
		Ok (())
	}
}

#[ derive (Clone, Eq, Hash, Ord, PartialEq, PartialOrd) ]
enum MultiVal <const MAX: usize> {
	Limited (Vec <Val>),
	Unlimited,
}

impl <const MAX: usize> MultiVal <MAX> {
	const fn empty () -> Self {
		Self::Limited (Vec::new ())
	}
	const fn unlimited () -> Self {
		Self::Unlimited
	}
	fn single (val: Val) -> Self {
		Self::Limited (vec! [ val ])
	}
	fn double (val_0: Val, val_1: Val) -> Self {
		Self::Limited (vec! [ val_0, val_1 ])
	}
	const fn is_limited (& self) -> bool {
		matches! (* self, Self::Limited (_))
	}
	fn contains (& self, val: Val) -> bool {
		self.values ().contains (& val)
	}
	fn push (& mut self, val: Val) {
		if let Self::Limited (ref mut values) = * self {
			if values.len () == MAX {
				* self = Self::Unlimited;
			} else if ! values.contains (& val) {
				values.push (val);
				values.sort ();
			}
		}
	}
	fn is_empty (& self) -> bool {
		self.values ().is_empty ()
	}
	fn len (& self) -> usize {
		self.values ().len ()
	}
	fn values (& self) -> & [Val] {
		match * self {
			Self::Limited (ref vals) => vals,
			Self::Unlimited => panic! (),
		}
	}
	fn update (& mut self, other: & Self) {
		match * other {
			Self::Limited (ref vals) => {
				for val in vals.iter ().copied () {
					self.push (val);
				}
			},
			Self::Unlimited => * self = Self::Unlimited,
		}
	}
}

impl <const MAX: usize> Debug for MultiVal <MAX> {
	fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
		match * self {
			Self::Limited (ref vals) =>
				write! (formatter, "MultiVal [{}]", vals.display_delim (", ")),
			Self::Unlimited =>
				write! (formatter, "MultiVal [ .. ]"),
		}
	}
}

#[ derive (Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd) ]
enum ValProv {
	InReg (u16),
	Instr (Val),
}

impl Debug for ValProv {
	fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
		match * self {
			Self::InReg (reg) => write! (formatter, "InReg ({reg})"),
			Self::Instr (instr_idx) => write! (formatter, "Instr ({instr_idx})"),
		}
	}
}
