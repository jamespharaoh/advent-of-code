use super::*;

pub trait Constraint <Val> {
	fn check (& self, vals: & [Val]) -> bool;
	fn clone_box (& self) -> Box <dyn Constraint <Val>>;
}

pub type BoxConstraint <Val> = Box <dyn Constraint <Val>>;

#[ derive (Clone) ]
pub struct FnConstraint <CheckFn> {
	check_fn: CheckFn,
}

impl <CheckFn> FnConstraint <CheckFn> {
	pub const fn new (check_fn: CheckFn) -> Self {
		Self { check_fn }
	}
}

impl <CheckFn, Val> Constraint <Val> for FnConstraint <CheckFn>
	where CheckFn: Fn (& [Val]) -> bool + Clone + 'static {

	fn check (& self, vals: & [Val]) -> bool {
		(self.check_fn) (vals)
	}

	fn clone_box (& self) -> Box <dyn Constraint <Val>> {
		Box::new (self.clone ())
	}

}

pub struct Solver <Val> {
	variables: Vec <(Rc <str>, Vec <Val>)>,
	constraints: Vec <(Vec <usize>, Box <dyn Constraint <Val>>)>,
}

impl <Val: Clone> Clone for Solver <Val> {
	fn clone (& self) -> Self {
		Self {
			variables: self.variables.clone (),
			constraints: self.constraints.iter ()
				.map (|& (ref vars, ref cnstr)| (vars.clone (), cnstr.clone_box ()))
				.collect (),
		}
	}
}

impl <Val: Clone + Debug> Solver <Val> {

	pub fn new () -> Self {
		Self::new_from_parts (None)
	}

	fn new_from_parts (parts: Option <SolverParts <Val>>) -> Self {
		let parts = parts.unwrap_or_else (|| SolverParts {
			variables: Vec::new (),
			constraints: Vec::new (),
		});
		Self {
			variables: parts.variables,
			constraints: parts.constraints,
		}
	}

	fn into_parts (mut self) -> SolverParts <Val> {
		self.variables.clear ();
		self.constraints.clear ();
		SolverParts {
			variables: self.variables,
			constraints: self.constraints,
		}
	}

	pub fn declare (
		& mut self,
		name: Rc <str>,
		values: impl IntoIterator <Item = Val>,
	) -> SolverVar {
		assert! (! self.variables.iter ().any (|& (ref var_name, _)| var_name == & name),
			"Variable declared twice: {name}");
		let idx = self.variables.len ();
		self.variables.push ((name, values.into_iter ().collect ()));
		SolverVar { idx }
	}

	pub fn get_var (& self, name: & str) -> SolverVar {
		SolverVar {
			idx: self.variables.iter ()
				.position (|& (ref var_name, _)| var_name.as_ref () == name)
				.unwrap (),
		}
	}

	pub fn add_constraint (& mut self, vars: & [SolverVar], cnstr: BoxConstraint <Val>) {
		self.constraints.push ((
			vars.iter ().map (|var| var.idx).collect (),
			cnstr,
		));
	}

	pub fn constrain <CheckFn> (& mut self, vars: & [SolverVar], check_fn: CheckFn)
			where CheckFn: Fn (& [Val]) -> bool + Clone + 'static {
		self.add_constraint (vars, Box::new (FnConstraint::new (check_fn)));
	}

	pub fn reduce (& mut self) {
		let mut solver_parts = None;
		let mut cnstrs_todo: VecDeque <_> =
			self.constraints.iter ()
				.sorted_by_cached_key (|&& (ref cnstr_vars, _)| (cnstr_vars.len (), cnstr_vars.iter ()
					.map (|& var_idx| self.variables [var_idx].1.len ())
					.product::<usize> ()))
				.collect ();
		let mut cnstrs_done = Vec::with_capacity (self.constraints.len ());
		let mut inner_vars = Vec::with_capacity (self.variables.len ());
		while let Some (cnstr_ref) = cnstrs_todo.pop_front () {
			let & (ref cnstr_vars, ref cnstr) = cnstr_ref;
			let mut solver = Self::new_from_parts (solver_parts);
			for & var_idx in cnstr_vars {
				let & (ref var_name, ref var_values) = & self.variables [var_idx];
				solver.declare (Rc::clone (var_name), var_values.clone ());
			}
			inner_vars.clear ();
			inner_vars.extend ((0 .. cnstr_vars.len ()).map (|idx| SolverVar { idx }));
			solver.add_constraint (& inner_vars, cnstr.clone_box ());
			solver.reduce_brute ();
			for (solver_var_idx, & var_idx) in cnstr_vars.iter ().enumerate () {
				if self.variables [var_idx].1.len () == solver.variables [solver_var_idx].1.len () {
					continue;
				}
				self.variables [var_idx].1 = mem::take (& mut solver.variables [solver_var_idx].1);
				cnstrs_done.retain (|& other_ref: && (Vec <usize>, _)| {
					let & (ref other_vars, _) = other_ref;
					if ! other_vars.iter ().any (|& other_var_idx| other_var_idx == var_idx) { return true }
					cnstrs_todo.push_back (other_ref);
					false
				});
			}
			cnstrs_done.push (cnstr_ref);
			solver_parts = Some (solver.into_parts ());
		}
	}

	fn reduce_brute (& mut self) {
		let mut one_value = Vec::with_capacity (1);
		let mut solver_iter_parts: Option <SolverIterParts <_>> = None;
		for var_idx in 0 .. self.variables.len () {
			let mut values = mem::take (& mut self.variables [var_idx].1);
			values.retain (|val| {
				one_value.clear ();
				one_value.push (val.clone ());
				self.variables [var_idx].1 = mem::take (& mut one_value);
				let mut solver_iter = SolverIter::new_with_parts (self, mem::take (& mut solver_iter_parts));
				let result = solver_iter.find_next ();
				solver_iter_parts = Some (solver_iter.into_parts ());
				one_value = mem::take (& mut self.variables [var_idx].1);
				result
			});
			self.variables [var_idx].1 = values;
		}
	}

	pub fn iter (& self) -> SolverIter <Val> {
		SolverIter::new (self)
	}

}

struct SolverParts <Val> {
	variables: Vec <(Rc <str>, Vec <Val>)>,
	constraints: Vec <(Vec <usize>, Box <dyn Constraint <Val>>)>,
}

#[ derive (Clone, Copy, Debug) ]
pub struct SolverVar {
	idx: usize,
}

struct SolverIterParts <Val> {
	values: Vec <Option <Val>>,
	todo: Vec <(usize, usize)>,
	cnstr_vals: Vec <Val>,
}

#[ derive (Clone) ]
pub struct SolverIter <'slvr, Val> {
	solver: & 'slvr Solver <Val>,
	values: Vec <Option <Val>>,
	todo: Vec <(usize, usize)>,
	cnstr_vals: Vec <Val>,
	found: bool,
}

impl <'slvr, Val: Clone + Debug> SolverIter <'slvr, Val> {
	fn new (solver: & 'slvr Solver <Val>) -> Self {
		Self::new_with_parts (solver, None)
	}
	fn new_with_parts (solver: & 'slvr Solver <Val>, parts: Option <SolverIterParts <Val>>) -> Self {
		let mut result = if let Some (parts) = parts {
			Self {
				solver,
				values: parts.values,
				todo: parts.todo,
				cnstr_vals: parts.cnstr_vals,
				found: false,
			}
		} else {
			Self {
				solver,
				values: Vec::new (),
				todo: Vec::new (),
				cnstr_vals: Vec::new (),
				found: false,
			}
		};
		result.init ();
		result
	}
	fn into_parts (mut self) -> SolverIterParts <Val> {
		self.values.clear ();
		self.todo.clear ();
		self.cnstr_vals.clear ();
		SolverIterParts {
			values: self.values,
			todo: self.todo,
			cnstr_vals: self.cnstr_vals,
		}
	}
	fn init (& mut self) {
		self.values.extend (iter::repeat (None).take (self.solver.variables.len ()));
		self.todo.push ((0, 0));
	}
	pub fn find_next (& mut self) -> bool {
		'OUTER: while let Some ((mut var_idx, mut val_idx)) = self.todo.pop () {
			loop {
				for val in & mut self.values [var_idx .. ] { * val = None; }
				if var_idx == self.values.len () { self.found = true; return true }
				let & (_, ref values) = & self.solver.variables [var_idx];
				if val_idx + 1 < values.len () {
					self.todo.push ((var_idx, val_idx + 1));
				}
				self.values [var_idx] = Some (values [val_idx].clone ());
				for & (ref cnstr_var_idxes, ref cnstr) in & self.solver.constraints {
					if cnstr_var_idxes.iter ().any (|& cnstr_var_idx| var_idx < cnstr_var_idx) { continue }
					if ! cnstr_var_idxes.iter ().any (|& cnstr_var_idx| cnstr_var_idx == var_idx) { continue }
					self.cnstr_vals.extend (
						cnstr_var_idxes.iter ()
							.map (|& cnstr_var_idx| self.values [cnstr_var_idx].as_ref ().unwrap ().clone ()));
					let found = cnstr.check (& self.cnstr_vals);
					self.cnstr_vals.clear ();
					if ! found { continue 'OUTER }
				}
				var_idx += 1;
				val_idx = 0;
			}
		}
		false
	}

	pub fn get (& self) -> Solution <Val> {
		assert! (self.found);
		Solution {
			values: self.values.iter ().flatten ().cloned ().collect (),
		}
	}

}

impl <'slvr, Val: Clone + Debug> Iterator for SolverIter <'slvr, Val> {

	type Item = Solution <Val>;

	fn next (& mut self) -> Option <Solution <Val>> {
		self.find_next ().then (|| self.get ())
	}

}

pub struct Solution <Val> {
	values: Vec <Val>,
}

impl <Val> Index <SolverVar> for Solution <Val> {
	type Output = Val;
	#[ inline ]
	fn index (& self, var: SolverVar) -> & Val {
		& self.values [var.idx]
	}
}

#[ macro_export ]
macro_rules! solver_constrain {
	(
		solver = $solver:expr;
		value_type = $val_type:ty;
		$( |$($arg:ident),*| $check:expr; )*
	) => { $( {
		#[ derive (Clone) ]
		struct MyConstraint;
		impl Constraint <$val_type> for MyConstraint {
			fn check (& self, vals: & [$val_type]) -> bool {
				let mut vals_iter = vals.iter ().copied ();
				$(
					let $arg = vals_iter.next ().unwrap ().clone ();
				)*
				$check
			}
			fn clone_box (& self) -> Box <dyn Constraint <$val_type>> {
				Box::new (self.clone ())
			}
		}
		let vars = & [ $($solver.get_var (stringify! ($arg))),* ];
		let cnstr = Box::new (MyConstraint);
		$solver.add_constraint (vars, cnstr);
	} )* };
}

#[ macro_export ]
macro_rules! make_solver {
	(
		value_type = $val_type:ty;
		variables { $( $($modif:ident)* ($($var:ident),*) = $vals:expr; )* }
		constraints { $($rest:tt)* }
	) => {
		{
			let mut solver = Solver::new ();
			#[ allow (dead_code) ]
			struct Vars {
				$($( $var: SolverVar, )*)*
			}
			$(
				$( let $var = solver.declare (Rc::from (stringify! ($var)), $vals); )*
				make_solver! (@unique solver ($($modif)*) ($($var),*));
			)*
			solver_constrain! {
				solver = solver;
				value_type = $val_type;
				$($rest)*
			}
			(solver, Vars { $($($var,)*)* })
		}
	};
	( @unique $solver:ident () ($($var:ident),*) ) => {
	};
	( @unique $solver:ident (unique) ($($var:ident),*) ) => {
		{
			let mut vars = Vec::new ();
			for var in [ $($var),* ] {
				vars.push (var);
				if 1 < vars.len () {
					$solver.constrain (& vars, |vals| {
						let last = vals [vals.len () - 1];
						for & other in & vals [ .. vals.len () - 1] {
							if last == other { return false }
						}
						true
					});
				}
			}
		}
	};
}
