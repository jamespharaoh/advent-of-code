use super::*;
use machine::Instr;
use machine::MachineError;
use machine::Reg;
use machine::RegOrInt;

#[ derive (Clone) ]
pub struct Solver {
	inner: Rc <SolverInner>,
	inner_weak: RcWeak <SolverInner>,
}

struct SolverInner {
	state: RefCell <SolverState>,
}

struct SolverState {
	symbols: HashMap <Rc <str>, Symbol>,
	symbols_ordered: Vec <Symbol>,
}

#[ derive (Clone) ]
pub struct Symbol {
	inner: Rc <SymbolInner>,
}

struct SymbolInner {
	solver_inner: RcWeak <SolverInner>,
	name: Rc <str>,
	original_value: Option <SymVal>,
	original_depth: usize,
	original_len: usize,
	original_children: ArrayVec <Symbol, 2>,
	state: RefCell <SymbolState>,
}

#[ derive (Debug) ]
struct SymbolState {
	value: SymVal,
	depth: usize,
	len: usize,
}

#[ derive (Clone, Debug, Eq, PartialEq) ]
pub enum SymVal {
	Symbol (Symbol),
	Input (usize),
	Add (Symbol, Symbol),
	Multiply (Symbol, Symbol),
	Divide (Symbol, Symbol),
	Modulo (Symbol, Symbol),
	IsEqual (Symbol, Symbol),
	IsUnequal (Symbol, Symbol),
	Value (i64),
	Error (MachineError),
}

impl Solver {
	pub fn from_prog (input: & [Instr]) -> (Solver, Vec <(Rc <str>, Symbol)>) {
		struct Context {
			namer: VerNamer,
			solver: Solver,
			literal_name: Rc <str>,
			reg_w_name: Rc <str>,
			reg_x_name: Rc <str>,
			reg_y_name: Rc <str>,
			reg_z_name: Rc <str>,
			reg_w: Symbol,
			reg_x: Symbol,
			reg_y: Symbol,
			reg_z: Symbol,
		}
		impl Context {
			fn get (& self, reg: Reg) -> Symbol {
				match reg {
					Reg::W => self.reg_w.clone (),
					Reg::X => self.reg_x.clone (),
					Reg::Y => self.reg_y.clone (),
					Reg::Z => self.reg_z.clone (),
				}
			}
			fn get_val (& mut self, reg_or_int: RegOrInt) -> Symbol {
				match reg_or_int {
					RegOrInt::W => self.reg_w.clone (),
					RegOrInt::X => self.reg_x.clone (),
					RegOrInt::Y => self.reg_y.clone (),
					RegOrInt::Z => self.reg_z.clone (),
					RegOrInt::Int (value) => self.lit (value),
				}
			}
			fn set (& mut self, reg: Reg, value: SymVal) {
				let (reg_name, reg) = match reg {
					Reg::W => (& self.reg_w_name, & mut self.reg_w),
					Reg::X => (& self.reg_x_name, & mut self.reg_x),
					Reg::Y => (& self.reg_y_name, & mut self.reg_y),
					Reg::Z => (& self.reg_z_name, & mut self.reg_z),
				};
				let symbol_name = self.namer.define (reg_name);
				let symbol = self.solver.define (symbol_name, value);
				* reg = symbol;
			}
			fn lit (& mut self, value: i64) -> Symbol {
				let symbol_name = self.namer.define (& self.literal_name);
				let symbol_value = SymVal::Value (value);
				let symbol = self.solver.define (symbol_name, symbol_value);
				symbol
			}
		}
		let mut ctx = {
			let mut namer = VerNamer::new ();
			let solver = Solver::new ();
			let literal_name = "lit".into ();
			let reg_w_name: Rc <str> = "w".into ();
			let reg_x_name: Rc <str> = "x".into ();
			let reg_y_name: Rc <str> = "y".into ();
			let reg_z_name: Rc <str> = "z".into ();
			let reg_w = solver.define (namer.define (& reg_w_name), SymVal::Value (0));
			let reg_x = solver.define (namer.define (& reg_x_name), SymVal::Value (0));
			let reg_y = solver.define (namer.define (& reg_y_name), SymVal::Value (0));
			let reg_z = solver.define (namer.define (& reg_z_name), SymVal::Value (0));
			Context {
				namer, solver,
				literal_name, reg_w_name, reg_x_name, reg_y_name, reg_z_name,
				reg_w, reg_x, reg_y, reg_z,
			}
		};
		let mut input_count = 0;
		for instr in input.iter ().copied () {
			match instr {
				Instr::Inp (dest) => {
					ctx.set (dest, SymVal::Input (input_count));
					input_count += 1;
				},
				Instr::Add (dest, src) => {
					let dest_val = ctx.get (dest);
					let src_val = ctx.get_val (src);
					ctx.set (dest, SymVal::Add (dest_val, src_val));
				},
				Instr::Mul (dest, src) => {
					let dest_val = ctx.get (dest);
					let src_val = ctx.get_val (src);
					ctx.set (dest, SymVal::Multiply (dest_val, src_val));
				},
				Instr::Div (dest, src) => {
					let dest_val = ctx.get (dest);
					let src_val = ctx.get_val (src);
					ctx.set (dest, SymVal::Divide (dest_val, src_val));
				},
				Instr::Mod (dest, src) => {
					let dest_val = ctx.get (dest);
					let src_val = ctx.get_val (src);
					ctx.set (dest, SymVal::Modulo (dest_val, src_val));
				},
				Instr::Eql (dest, src) => {
					let dest_val = ctx.get (dest);
					let src_val = ctx.get_val (src);
					ctx.set (dest, SymVal::IsEqual (dest_val, src_val));
				},
			}
		}
		(ctx.solver, vec! [
			(ctx.reg_w_name, ctx.reg_w),
			(ctx.reg_x_name, ctx.reg_x),
			(ctx.reg_y_name, ctx.reg_y),
			(ctx.reg_z_name, ctx.reg_z),
		])
	}
	pub fn eval (& self, input: & [& [i64]], symbol: & Symbol) -> Result <Vec <i64>, MachineError> {
		self.require_own_symbol ("eval", symbol);
		let inner = self.inner.as_ref ();
		let state = inner.state.borrow ();
		let mut seen: HashSet <Symbol> = HashSet::new ();
		seen.insert (symbol.clone ());
		let mut todo: Vec <Symbol> = vec! [ symbol.clone () ];
		while let Some (symbol) = todo.pop () {
			for child in symbol.children ().into_iter () {
				if seen.insert (child.clone ()) {
					todo.push (child);
				}
			}
		}
		let mut values: HashMap <Symbol, Rc <Vec <i64>>> = HashMap::new ();
		for symbol in state.symbols_ordered.iter () {
			if ! seen.contains (symbol) { continue }
			let value = symbol.eval (& |sym| values.get (sym).unwrap ().clone (), input) ?;
			let value = Rc::new (value);
			values.insert (symbol.clone (), value.clone ());
		}
		todo! ();
	}
	pub fn new () -> Solver {
		let inner = Rc::new (SolverInner {
			state: RefCell::new (SolverState {
				symbols: HashMap::new (),
				symbols_ordered: Vec::new (),
			}),
		});
		let inner_weak = Rc::downgrade (& inner);
		Solver { inner, inner_weak }
	}
	pub fn fork (& self, symbols: & mut [& mut Symbol], input: & [i64]) -> Solver {
		let inner = self.inner.as_ref ();
		let state = inner.state.borrow ();
		let mut seen: HashSet <Symbol> = symbols.iter ().map (|a| & ** a).cloned ().collect ();
		let mut todo: Vec <Symbol> = seen.iter ().cloned ().collect ();
		while let Some (symbol) = todo.pop () {
			for child in symbol.children ().into_iter () {
				if seen.insert (child.clone ()) {
					todo.push (child);
				}
			}
		}
		let mut new_solver = Solver::new ();
		for symbol in state.symbols_ordered.iter () {
			if ! seen.contains (symbol) { continue }
			let new_value = symbol.value ().migrate (& mut new_solver, & input);
			new_solver.define (symbol.name ().clone (), new_value);
		}
		for old_symbol in symbols.iter_mut () {
			let new_symbol = new_solver.get (old_symbol.name ()).unwrap ();
			** old_symbol = new_symbol;
		}
		new_solver
	}
	pub fn get (& self, name: & Rc <str>) -> Option <Symbol> {
		let state = self.inner.state.borrow ();
		state.symbols.get (name).cloned ()
	}
	pub fn define (& self, name: Rc <str>, value: SymVal) -> Symbol {
		value.children ().iter ().for_each (|child| self.require_own_symbol ("define", child));
		let mut state = self.inner.state.borrow_mut ();
		if state.symbols.contains_key (& name) { panic! () }
		let mut value = value;
		let mut depth = value.depth ();
		let mut len = value.len ();
		let mut original_value = None;
		let original_depth = value.original_depth ();
		let original_len = value.original_len ();
		let original_children = value.children ();
		if let Some (simplified_value) = value.simplify () {
			original_value = Some (value);
			value = simplified_value;
			depth = value.depth ();
			len = value.len ();
		}
		let symbol = Symbol {
			inner: Rc::new (SymbolInner {
				solver_inner: Rc::downgrade (& self.inner),
				name: name.clone (),
				original_value,
				original_depth,
				original_len,
				original_children,
				state: RefCell::new (SymbolState {
					value,
					depth,
					len,
				}),
			}),
		};
		state.symbols.insert (name, symbol.clone ());
		state.symbols_ordered.push (symbol.clone ());
		symbol
	}
	pub fn dump (& self, depth: usize, show_original: bool) {
		let options = FormatExpandOptions::Depth (depth);
		let state = self.inner.state.borrow ();
		for symbol in state.symbols_ordered.iter () {
			symbol.dump (options, show_original, None);
		}
	}
	fn require_own_symbol (& self, fn_name: & str, symbol: & Symbol) {
		if ! RcWeak::ptr_eq (& self.inner_weak, & symbol.inner.solver_inner) {
			panic! ("Tried to call Solver::{} on solver at {:p} with a symbol from solver at {:p}",
				fn_name,
				Rc::as_ptr (& self.inner),
				RcWeak::as_ptr (& symbol.inner.solver_inner));
		}
	}
	pub fn dump_symbol (& self, depth: usize, show_original: bool, symbol: & Symbol) {
		self.require_own_symbol ("dump_symbol", symbol);
		let state = self.inner.state.borrow ();
		let counts = state.symbol_use_counts (symbol);
		let options = FormatExpandOptions::Depth (depth);
		for symbol in state.symbols_ordered.iter () {
			let count = match counts.get (symbol) {
				Some (& count) => count,
				None => continue,
			};
			symbol.dump (options, show_original, Some (count));
		}
	}
	pub fn dump_symbol_auto (& self, symbol: & Symbol) {
		self.require_own_symbol ("dump_symbol_auto", symbol);
		let state = self.inner.state.borrow ();
		let counts = state.symbol_use_counts (symbol);
		let break_symbols: HashSet <Symbol> = counts.iter ().filter_map (
			|(symbol, & count)| if count > 1 { Some (symbol) } else { None },
		).cloned ().collect ();
let a: Vec <Symbol> = break_symbols.iter ().cloned ().collect ();
for b in a.iter () { println! ("{:?}", b); }
		let options = FormatExpandOptions::BreakSymbols (& break_symbols);
		for symbol in state.symbols_ordered.iter () {
			if counts.get (symbol).map (|count| * count == 1).unwrap_or (true) { continue }
			symbol.dump (options, true, None);
		}
	}
}

impl SolverState {
	fn symbol_use_counts (& self, symbol: & Symbol) -> HashMap <Symbol, usize> {
		let mut counts: HashMap <Symbol, usize> = HashMap::new ();
		counts.insert (symbol.clone (), 0);
		let mut todo: Vec <Symbol> = Vec::new ();
		todo.push (symbol.clone ());
		while let Some (symbol) = todo.pop () {
			for child in symbol.original_children ().into_iter () {
				let child_count = counts.entry (child.clone ()).or_insert (0);
				if * child_count == 0 {
					todo.push (child.clone ());
				}
				* child_count += 1;
			}
		}
		counts
	}
}

impl Symbol {
	pub fn eval (
		& self,
		lookup: & dyn Fn (& Symbol) -> Rc <Vec <i64>>, input: & [& [i64]],
	) -> Result <Vec <i64>, MachineError> {
		let inner = self.inner.as_ref ();
		let state = inner.state.borrow ();
		state.value.eval (lookup, input)
	}
	pub fn name (& self) -> & Rc <str> {
		& self.inner.name
	}
	pub fn value (& self) -> SymVal {
		let inner = self.inner.as_ref ();
		let state = inner.state.borrow ();
		state.value.clone ()
	}
	pub fn children (& self) -> ArrayVec <Symbol, 2> {
		let inner = self.inner.as_ref ();
		let state = inner.state.borrow ();
		state.value.children ()
	}
	pub fn original_children (& self) -> ArrayVec <Symbol, 2> {
		let inner = self.inner.as_ref ();
		inner.original_children.clone ()
	}
	fn dump (
		& self,
		options: FormatExpandOptions,
		show_original: bool,
		use_count: Option <usize>,
	) {
		let inner = self.inner.as_ref ();
		let state = inner.state.borrow ();
		print! ("{}", inner.name);
		if let Some (use_count) = use_count {
			print! (" ({})", use_count);
		}
		match options {
			FormatExpandOptions::Depth (depth) => {
				if show_original {
					if let Some (original_value) = & inner.original_value {
						print! (" ← {}", original_value.expand (1));
					}
				}
				print! (" ← {}", state.value);
				print! (" ≡ {}", state.value.expand (depth));
			},
			FormatExpandOptions::BreakSymbols (_) =>
				print! (" ← {}", FormatExpand::SymVal (options, false, & state.value)),
		}
		print! ("\n");
	}
	pub fn depth (& self) -> usize {
		let inner = self.inner.as_ref ();
		let state = inner.state.borrow ();
		state.depth
	}
	pub fn len (& self) -> usize {
		let inner = self.inner.as_ref ();
		let state = inner.state.borrow ();
		state.len
	}
	pub fn original_depth (& self) -> usize {
		let inner = self.inner.as_ref ();
		inner.original_depth
	}
	pub fn original_len (& self) -> usize {
		let inner = self.inner.as_ref ();
		inner.original_len
	}
	fn fmt_expand (
		& self,
		formatter: & mut fmt::Formatter,
		options: FormatExpandOptions,
		wrap: bool,
	) -> fmt::Result {
		let inner = self.inner.as_ref ();
		let state = inner.state.borrow ();
		match options {
			FormatExpandOptions::Depth (depth) => {
				if depth > 0 {
					let options = FormatExpandOptions::Depth (depth - 1);
					write! (formatter, "C{}", FormatExpand::SymVal (options, wrap, & state.value)) ?;
				} else {
					write! (formatter, "D{}", inner.name) ?;
				}
			},
			FormatExpandOptions::BreakSymbols (break_symbols) => {
				if break_symbols.contains (& self) {
					write! (formatter, "A{}", inner.name) ?;
				} else {
					let value = inner.original_value.clone ().unwrap_or_else (|| state.value.clone ()).simplified ();
					write! (formatter, "B{}", FormatExpand::SymVal (options, wrap, & value)) ?;
				}
			},
		}
		Ok (())
	}
}

impl cmp::PartialOrd for Symbol {
	fn partial_cmp (& self, other: & Symbol) -> Option <cmp::Ordering> {
		self.inner.name.partial_cmp (& other.inner.name)
	}
}

impl fmt::Debug for Symbol {
	fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
		let inner = self.inner.as_ref ();
		write! (formatter, "Symbol {:?}", inner.name) ?;
		Ok (())
	}
}

impl fmt::Display for Symbol {
	fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
		write! (formatter, "{}",
			FormatExpand::Symbol (
				FormatExpandOptions::Depth (formatter.precision ().unwrap_or (0)),
				false,
				self)) ?;
		Ok (())
	}
}

impl hash::Hash for Symbol {
	fn hash <Hasher: hash::Hasher> (& self, state: & mut Hasher) {
		self.inner.name.hash (state);
	}
}

impl PartialEq for Symbol {
	fn eq (& self, other: & Symbol) -> bool { self.inner.name == other.inner.name }
}

impl Eq for Symbol {}

impl SymVal {
	fn eval (& self, lookup: & dyn Fn (& Symbol) -> Rc <Vec <i64>>, input: & [& [i64]]) -> Result <Vec <i64>, MachineError> {
		fn combine <Combine: Fn (i64, i64) -> i64> (left: Rc <Vec <i64>>, right: Rc <Vec <i64>>, combine: Combine) -> Vec <i64> {
			let mut results = HashSet::new ();
			for left in left.iter ().copied () { for right in right.iter ().copied () {
				results.insert (combine (left, right));
			} }
			let mut results: Vec <i64> = results.into_iter ().collect ();
			results.sort ();
			results
		}
		Ok (match & * self {
			SymVal::Symbol (arg) => lookup (arg).to_vec (),
			SymVal::Input (arg) => input.get (* arg).copied ().ok_or (MachineError::NoMoreInput) ?.to_vec (),
			SymVal::Add (left, right) => combine (lookup (left), lookup (right), |a, b| a + b),
			SymVal::Multiply (left, right) => combine (lookup (left), lookup (right), |a, b| a * b),
			SymVal::Divide (left, right) => combine (lookup (left), lookup (right), |a, b| a / b),
			SymVal::Modulo (left, right) => combine (lookup (left), lookup (right), |a, b| a % b),
			SymVal::IsEqual (left, right) => combine (lookup (left), lookup (right), |a, b| if a == b { 1 } else { 0 }),
			SymVal::IsUnequal (left, right) => combine (lookup (left), lookup (right), |a, b| if a != b { 1 } else { 0 }),
			SymVal::Value (arg) => vec! [ * arg ],
			SymVal::Error (arg) => Err (* arg) ?,
		})
	}
	pub fn depth (& self) -> usize {
		self.children ().iter ().map (|child| child.depth ()).max ().unwrap_or (0) + 1
	}
	pub fn len (& self) -> usize {
		self.children ().iter ().fold (1, |len, child| len + child.len ())
	}
	pub fn original_depth (& self) -> usize {
		self.children ().iter ().map (|child| child.original_depth ()).max ().unwrap_or (0) + 1
	}
	pub fn original_len (& self) -> usize {
		self.children ().iter ().fold (1, |len, child| len + child.original_len ())
	}
	pub fn children (& self) -> ArrayVec <Symbol, 2> {
		fn make <'solver, const CAP: usize> (arg: [& Symbol; CAP]) -> ArrayVec <Symbol, 2> {
			arg.into_iter ().cloned ().collect ()
		}
		match & * self {
			SymVal::Symbol (arg) => make ([ arg ]),
			SymVal::Input (_) => make ([ ]),
			SymVal::Add (left, right) => make ([ left, right ]),
			SymVal::Multiply (left, right) => make ([ left, right ]),
			SymVal::Divide (left, right) => make ([ left, right ]),
			SymVal::Modulo (left, right) => make ([ left, right ]),
			SymVal::IsEqual (left, right) => make ([ left, right ]),
			SymVal::IsUnequal (left, right) => make ([ left, right ]),
			SymVal::Value (_) => make ([ ]),
			SymVal::Error (_) => make ([ ]),
		}
	}
	fn fmt_expand (
		& self,
		formatter: & mut fmt::Formatter,
		options: FormatExpandOptions,
		wrap: bool,
	) -> fmt::Result {
		let expand = |symbol, wrap| FormatExpand::Symbol (options, wrap, symbol);
		let (open, close) = if wrap { ("(", ")") } else { ("", "") };
		match self {
			SymVal::Symbol (other_symbol) =>
				write! (formatter, "{}", expand (other_symbol, wrap)) ?,
			SymVal::Input (index) =>
				write! (formatter, "input [{}]", index) ?,
			SymVal::Add (left, right) =>
				write! (formatter, "{}{} + {}{}", open, expand (left, true), expand (right, true), close) ?,
			SymVal::Multiply (left, right) =>
				write! (formatter, "{}{} × {}{}", open, expand (left, true), expand (right, true), close) ?,
			SymVal::Divide (left, right) =>
				write! (formatter, "{}{} ÷ {}{}", open, expand (left, true), expand (right, true), close) ?,
			SymVal::Modulo (left, right) =>
				write! (formatter, "{}{} mod {}{}", open, expand (left, true), expand (right, true), close) ?,
			SymVal::IsEqual (left, right) =>
				write! (formatter, "{}{} = {}{}", open, expand (left, true), expand (right, true), close) ?,
			SymVal::IsUnequal (left, right) =>
				write! (formatter, "{}{} ≠ {}{}", open, expand (left, true), expand (right, true), close) ?,
			SymVal::Value (value) =>
				write! (formatter, "{}", value) ?,
			SymVal::Error (arg) =>
				write! (formatter, "{:?}", arg) ?,
		}
		Ok (())
	}
	fn expand (& self, depth: usize) -> FormatExpand {
		let options = FormatExpandOptions::Depth (depth);
		FormatExpand::SymVal (options, false, self)
	}
	fn simplified (& self) -> SymVal {
		match self.simplify () {
			Some (val) => val,
			None => self.clone (),
		}
	}
	fn simplify (& self) -> Option <SymVal> {
		let mut value = self;
		let mut result = None;
		loop {
			if let Some (temp) = value.simplify_real () {
				result = Some (temp);
				value = result.as_ref ().unwrap ();
			} else { break }
		}
		result
	}
	fn simplify_real (& self) -> Option <SymVal> {
		match self {
			SymVal::Input (_) => None,
			SymVal::Add (left_sym, right_sym) => {
				let left_symval = left_sym.value ();
				let right_symval = right_sym.value ();
				match (& left_symval, & right_symval) {
					(SymVal::Error (_), _) => Some (left_symval),
					(_, SymVal::Error (_)) => Some (right_symval),
					(SymVal::Value (left_val), SymVal::Value (right_val)) =>
						Some (SymVal::Value (left_val + right_val)),
					(_, SymVal::Value (0)) => Some (left_symval),
					(SymVal::Value (0), _) => Some (right_symval),
					(_, _) if left_symval == right_symval => todo! (),
					_ => None,
				}
			},
			SymVal::Multiply (left_sym, right_sym) => {
				let left_symval = left_sym.value ();
				let right_symval = right_sym.value ();
				match (& left_symval, & right_symval) {
					(SymVal::Error (_), _) => Some (left_symval),
					(_, SymVal::Error (_)) => Some (right_symval),
					(SymVal::Value (left_val), SymVal::Value (right_val)) =>
						Some (SymVal::Value (left_val * right_val)),
					(_, SymVal::Value (0)) => Some (SymVal::Value (0)),
					(SymVal::Value (0), _) => Some (SymVal::Value (0)),
					_ => None,
				}
			},
			SymVal::Divide (left_sym, right_sym) => {
				let left_symval = left_sym.value ();
				let right_symval = right_sym.value ();
				match (& left_symval, & right_symval) {
					(SymVal::Error (_), _) => Some (left_symval),
					(_, SymVal::Error (_)) => Some (right_symval),
					(SymVal::Value (left_val), SymVal::Value (right_val)) =>
						Some (SymVal::Value (left_val / right_val)),
					(_, SymVal::Value (1)) => Some (left_symval),
					(_, SymVal::Value (0)) =>
						Some (SymVal::Error (MachineError::DivideByZero)),
					(SymVal::Value (0), _) => Some (SymVal::Value (0)),
					(_, _) if left_symval == right_symval => todo! (),
					_ => None,
				}
			}
			SymVal::Modulo (left_sym, right_sym) => {
				let left_symval = left_sym.value ();
				let right_symval = right_sym.value ();
				match (& left_symval, & right_symval) {
					(SymVal::Error (_), _) => Some (left_symval),
					(_, SymVal::Error (_)) => Some (right_symval),
					(& SymVal::Value (left_val), & SymVal::Value (right_val)) => {
						if left_val >= 0 && right_val > 0 {
							Some (SymVal::Value (left_val % right_val))
						} else if right_val == 0 {
							Some (SymVal::Error (MachineError::DivideByZero))
						} else {
							Some (SymVal::Error (MachineError::NegativeModulo))
						}
					},
					(_, SymVal::Value (0)) =>
						Some (SymVal::Error (MachineError::DivideByZero)),
					(_, & SymVal::Value (right_val)) if right_val < 0 =>
						Some (SymVal::Error (MachineError::NegativeModulo)),
					(& SymVal::Value (left_val), _) if left_val < 0 =>
						Some (SymVal::Error (MachineError::NegativeModulo)),
					_ => None,
				}
			},
			SymVal::IsEqual (left_sym, right_sym) => {
				let left_symval = left_sym.value ();
				let right_symval = right_sym.value ();
				match (& left_symval, & right_symval) {
					(SymVal::Error (_), _) => Some (left_symval),
					(_, SymVal::Error (_)) => Some (right_symval),
					(SymVal::IsEqual (inner_left_sym, inner_right_sym), SymVal::Value (0)) =>
						Some (SymVal::IsUnequal (inner_left_sym.clone (), inner_right_sym.clone ())),
					(SymVal::Value (0), SymVal::IsEqual (inner_left_sym, inner_right_sym)) =>
						Some (SymVal::IsUnequal (inner_left_sym.clone (), inner_right_sym.clone ())),
					(SymVal::IsEqual (_, _), SymVal::Value (1)) => Some (left_symval),
					(SymVal::Value (1), SymVal::IsEqual (_, _)) => Some (right_symval),
					(SymVal::Value (left_val), SymVal::Value (right_val)) =>
						Some (SymVal::Value (if left_val == right_val { 1 } else { 0 })),
					_ => None,
				}
			},
			SymVal::IsUnequal (left_sym, right_sym) => {
				let left_symval = left_sym.value ();
				let right_symval = right_sym.value ();
				match (& left_symval, & right_symval) {
					(SymVal::Error (_), _) => Some (left_symval),
					(_, SymVal::Error (_)) => Some (right_symval),
					(SymVal::Value (left_val), SymVal::Value (right_val)) =>
						Some (SymVal::Value (if left_val != right_val { 1 } else { 0 })),
					_ => None,
				}
			},
			SymVal::Value (_) => None,
			_ => todo! ("SymVal::{:?}", self),
		}
	}
	fn migrate (& self, solver: & mut Solver, input: & [i64]) -> SymVal {
		let dup = |arg: Symbol| solver.get (arg.name ()).unwrap ().clone ();
		match self.clone () {
			SymVal::Symbol (arg) => SymVal::Symbol (dup (arg)),
			SymVal::Input (arg) =>
				if let Some (& val) = input.get (arg) {
					SymVal::Value (val)
				} else {
					SymVal::Input (arg)
				},
			SymVal::Add (left, right) => SymVal::Add (dup (left), dup (right)),
			SymVal::Multiply (left, right) => SymVal::Multiply (dup (left), dup (right)),
			SymVal::Divide (left, right) => SymVal::Divide (dup (left), dup (right)),
			SymVal::Modulo (left, right) => SymVal::Modulo (dup (left), dup (right)),
			SymVal::IsEqual (left, right) => SymVal::IsEqual (dup (left), dup (right)),
			SymVal::IsUnequal (left, right) => SymVal::IsUnequal (dup (left), dup (right)),
			SymVal::Value (arg) => SymVal::Value (arg),
			SymVal::Error (arg) => SymVal::Error (arg),
		}
	}
}

impl fmt::Display for SymVal {
	fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
		let options = FormatExpandOptions::Depth (formatter.precision ().unwrap_or (0));
		write! (formatter, "{}",
			FormatExpand::SymVal (options, false, self)) ?;
		Ok (())
	}
}

enum FormatExpand <'a> {
	Symbol (FormatExpandOptions <'a>, bool, & 'a Symbol),
	SymVal (FormatExpandOptions <'a>, bool, & 'a SymVal),
}

impl <'a> fmt::Display for FormatExpand <'a> {
	fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
		match * self {
			FormatExpand::Symbol (options, wrap, symbol) =>
				symbol.fmt_expand (formatter, options, wrap),
			FormatExpand::SymVal (options, wrap, symbol_value) =>
				symbol_value.fmt_expand (formatter, options, wrap),
		}
	}
}

#[ derive (Clone, Copy) ]
enum FormatExpandOptions <'a> {
	Depth (usize),
	BreakSymbols (& 'a HashSet <Symbol>),
}

struct VerNamer {
	entries: HashMap <Rc <str>, VerNamerEntry>,
}

struct VerNamerEntry {
	base_name: Rc <str>,
	latest_name: Rc <str>,
	latest_rev: usize,
}

impl VerNamer {

	fn new () -> VerNamer {
		VerNamer {
			entries: HashMap::new (),
		}
	}

	fn define (& mut self, base_name: & Rc <str>) -> Rc <str> {
		let entry = self.entries.entry (base_name.clone ()).and_modify (|entry| {
			entry.latest_rev += 1;
		}).or_insert (VerNamerEntry {
			base_name: base_name.clone (),
			latest_name: base_name.clone (),
			latest_rev: 0,
		});
		entry.latest_name = format! ("{}{}", entry.base_name, entry.latest_rev).into ();
		entry.latest_name.clone ()
	}

}
