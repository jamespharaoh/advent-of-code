use super::*;

use input::Input;
use input::Workflow;
use model::Field;
use model::Op;
use model::Val;

pub fn part_one (input: & Input) -> GenResult <u64> {
	let workflows: HashMap <& str, & Workflow> =
		input.workflows.iter ()
			.map (|workflow| (workflow.name.as_str (), workflow))
			.collect ();
	let mut sum = 0;
	for part in & input.parts {
		let mut workflow_name = "in";
		let mut num_iters = 0_u32;
		let accepted = 'OUTER: loop {
			if num_iters == 100 { return Err ("Max iterations exceeded".into ()); }
			num_iters += 1;
			if workflow_name == "A" { break true }
			if workflow_name == "R" { break false }
			let workflow =
				workflows.get (workflow_name)
					.ok_or_else (|| format! ("Workflow doesn't exist: {workflow_name}")) ?;
			for rule in & workflow.rules {
				let part_val = match rule.field {
					Field::X => part.x,
					Field::M => part.m,
					Field::A => part.a,
					Field::S => part.s,
				};
				let matched = match rule.op {
					Op::LessThan => part_val < rule.val,
					Op::GreaterThan => part_val > rule.val,
				};
				if matched {
					workflow_name = rule.target.as_str ();
					continue 'OUTER;
				}
			}
			workflow_name = workflow.default.as_str ();
		};
		if accepted {
			sum += part.x.pan_u64 ();
			sum += part.m.pan_u64 ();
			sum += part.a.pan_u64 ();
			sum += part.s.pan_u64 ();
		}
	}
	Ok (sum)
}

#[ allow (clippy::int_plus_one) ]
pub fn part_two (input: & Input) -> GenResult <u64> {
	let workflows: HashMap <& str, & Workflow> =
		input.workflows.iter ()
			.map (|workflow| (workflow.name.as_str (), workflow))
			.collect ();
	let range_all = Range { x: (1, 4001), m: (1, 4001), a: (1, 4001), s: (1, 4001) };
	let mut todo = vec! [ (range_all, "in") ];
	let mut accepts = Vec::new ();
	let mut num_iters = 0_u32;
	while let Some ((mut range, workflow_name)) = todo.pop () {
		if num_iters == 3_000 { return Err ("Max iterations exceeded".into ()); }
		num_iters += 1;
		if workflow_name == "A" { accepts.push (range); continue; }
		if workflow_name == "R" { continue; }
		let workflow =
			workflows.get (workflow_name)
				.ok_or_else (|| format! ("Workflow doesn't exist: {workflow_name}")) ?;
		for rule in & workflow.rules {
			let (range_start, range_end) = range.get (rule.field);
			let mut keep = None;
			let mut split = None;
			match rule.op {
				Op::LessThan => {
					if rule.val <= range_start {
						keep = Some ((range_start, range_end));
					} else if range_end <= rule.val {
						split = Some ((range_start, range_end));
					} else {
						split = Some ((range_start, rule.val));
						keep = Some ((rule.val, range_end));
					}
				},
				Op::GreaterThan => {
					if range_end <= rule.val + 1 {
						keep = Some ((range_start, range_end));
					} else if rule.val + 1 <= range_start {
						split = Some ((range_start, range_end));
					} else {
						keep = Some ((range_start, rule.val + 1));
						split = Some ((rule.val + 1, range_end));
					}
				},
			};
			if let Some (split) = split {
				let mut range = range;
				range.set (rule.field, split);
				todo.push ((range, rule.target.as_str ()));
			}
			let Some (keep) = keep else { break };
			range.set (rule.field, keep);
		}
		todo.push ((range, workflow.default.as_str ()));
	}
	Ok (accepts.iter ()
		.map (|range| {
			let x = range.x.1 - range.x.0;
			let m = range.m.1 - range.m.0;
			let a = range.a.1 - range.a.0;
			let s = range.s.1 - range.s.0;
			x.pan_u64 () * m.pan_u64 () * a.pan_u64 () * s.pan_u64 ()
		})
		.sum ())
}

#[ derive (Clone, Copy, Debug) ]
struct Range {
	x: (Val, Val),
	m: (Val, Val),
	a: (Val, Val),
	s: (Val, Val),
}

impl Range {
	fn get (& self, field: Field) -> (Val, Val) {
		match field {
			Field::X => self.x,
			Field::M => self.m,
			Field::A => self.a,
			Field::S => self.s,
		}
	}
	fn set (& mut self, field: Field, range: (Val, Val)) {
		match field {
			Field::X => self.x = range,
			Field::M => self.m = range,
			Field::A => self.a = range,
			Field::S => self.s = range,
		}
	}
}
