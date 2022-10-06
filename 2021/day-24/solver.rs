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

	#[ allow (clippy::too_many_lines) ]
	#[ must_use ]
	pub fn from_prog (input: & [Instr]) -> (Self, Vec <(Rc <str>, Symbol)>) {
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
				self.solver.define (symbol_name, symbol_value)
			}
		}
		let mut ctx = {
			let mut namer = VerNamer::new ();
			let solver = Self::new ();
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

	#[ allow (clippy::todo) ]
	pub fn eval (& self, input: & [& [i64]], symbol: & Symbol) -> Result <Vec <i64>, MachineError> {
		self.require_own_symbol ("eval", symbol);
		let inner = self.inner.as_ref ();
		let state = inner.state.borrow ();
		let mut seen: HashSet <Symbol> = HashSet::new ();
		seen.insert (symbol.clone ());
		let mut todo: Vec <Symbol> = vec! [ symbol.clone () ];
		while let Some (symbol) = todo.pop () {
			for child in symbol.children () {
				if seen.insert (child.clone ()) {
					todo.push (child);
				}
			}
		}
		let mut values: HashMap <Symbol, Rc <[i64]>> = HashMap::new ();
		for symbol in state.symbols_ordered.iter () {
			if ! seen.contains (symbol) { continue }
			let value = symbol.eval (& |sym| Rc::clone (& values [sym]), input) ?;
			let value = Rc::from (value.as_slice ());
			values.insert (symbol.clone (), Rc::clone (& value));
		}
		todo! ();
	}

	#[ inline ]
	#[ must_use ]
	pub fn new () -> Self {
		let inner = Rc::new (SolverInner {
			state: RefCell::new (SolverState {
				symbols: HashMap::new (),
				symbols_ordered: Vec::new (),
			}),
		});
		let inner_weak = Rc::downgrade (& inner);
		Self { inner, inner_weak }
	}

	#[ must_use ]
	pub fn fork (& self, symbols: & mut [& mut Symbol], input: & [i64]) -> Self {
		let inner = self.inner.as_ref ();
		let state = inner.state.borrow ();
		let mut seen: HashSet <Symbol> = symbols.iter ().map (|a| & ** a).cloned ().collect ();
		let mut todo: Vec <Symbol> = seen.iter ().cloned ().collect ();
		while let Some (symbol) = todo.pop () {
			for child in symbol.children () {
				if seen.insert (child.clone ()) {
					todo.push (child);
				}
			}
		}
		let mut new_solver = Self::new ();
		for symbol in state.symbols_ordered.iter () {
			if ! seen.contains (symbol) { continue }
			let new_value = symbol.value ().migrate (& mut new_solver, input);
			let _symbol = new_solver.define (Rc::clone (symbol.name ()), new_value);
		}
		for old_symbol in symbols.iter_mut () {
			let new_symbol = new_solver.get (old_symbol.name ()).unwrap ();
			** old_symbol = new_symbol;
		}
		new_solver
	}

	#[ inline ]
	#[ must_use ]
	pub fn get (& self, name: & Rc <str>) -> Option <Symbol> {
		let state = self.inner.state.borrow ();
		state.symbols.get (name).cloned ()
	}

	#[ must_use ]
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
				name: Rc::clone (& name),
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
		assert! (
			RcWeak::ptr_eq (& self.inner_weak, & symbol.inner.solver_inner),
			"Tried to call Solver::{} on solver at {:p} with a symbol from solver at {:p}",
			fn_name,
			Rc::as_ptr (& self.inner),
			RcWeak::as_ptr (& symbol.inner.solver_inner));
	}

	pub fn dump_symbol (& self, depth: usize, show_original: bool, symbol: & Symbol) {
		self.require_own_symbol ("dump_symbol", symbol);
		let state = self.inner.state.borrow ();
		let counts = symbol.use_counts ();
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
		let counts = symbol.use_counts ();
		let break_symbols: HashSet <Symbol> =
			counts.iter ()
				.filter_map (|(symbol, & count)| (count > 1).then_some (symbol))
				.cloned ()
				.collect ();
		let options = FormatExpandOptions::BreakSymbols (& break_symbols);
		for symbol in state.symbols_ordered.iter () {
			if counts.get (symbol).map_or (true, |count| * count == 1) { continue }
			symbol.dump (options, true, None);
		}
	}

}

impl Symbol {

	pub fn eval (
		& self,
		lookup: & dyn Fn (& Self) -> Rc <[i64]>, input: & [& [i64]],
	) -> Result <Vec <i64>, MachineError> {
		let inner = self.inner.as_ref ();
		let state = inner.state.borrow ();
		state.value.eval (lookup, input)
	}

	#[ inline ]
	#[ must_use ]
	pub fn name (& self) -> & Rc <str> {
		& self.inner.name
	}

	#[ inline ]
	#[ must_use ]
	pub fn value (& self) -> SymVal {
		let inner = self.inner.as_ref ();
		let state = inner.state.borrow ();
		state.value.clone ()
	}

	#[ inline ]
	#[ must_use ]
	pub fn children (& self) -> ArrayVec <Self, 2> {
		let inner = self.inner.as_ref ();
		let state = inner.state.borrow ();
		state.value.children ()
	}

	#[ inline ]
	#[ must_use ]
	pub fn original_children (& self) -> ArrayVec <Self, 2> {
		let inner = self.inner.as_ref ();
		inner.original_children.clone ()
	}

	#[ allow (clippy::print_stdout) ]
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
					if let Some (original_value) = inner.original_value.as_ref () {
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

	#[ inline ]
	#[ must_use ]
	pub fn depth (& self) -> usize {
		let inner = self.inner.as_ref ();
		let state = inner.state.borrow ();
		state.depth
	}

	#[ inline ]
	#[ must_use ]
	pub fn len (& self) -> usize {
		let inner = self.inner.as_ref ();
		let state = inner.state.borrow ();
		state.len
	}

	#[ inline ]
	#[ must_use ]
	pub fn is_empty (& self) -> bool {
		self.len () == 0
	}

	#[ inline ]
	#[ must_use ]
	pub fn original_depth (& self) -> usize {
		let inner = self.inner.as_ref ();
		inner.original_depth
	}

	#[ inline ]
	#[ must_use ]
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
				if break_symbols.contains (self) {
					write! (formatter, "A{}", inner.name) ?;
				} else {
					let value = inner.original_value.clone ().unwrap_or_else (|| state.value.clone ()).simplified ();
					write! (formatter, "B{}", FormatExpand::SymVal (options, wrap, & value)) ?;
				}
			},
		}
		Ok (())
	}

	fn use_counts (& self) -> HashMap <Self, usize> {
		let mut counts: HashMap <Self, usize> = HashMap::new ();
		counts.insert (self.clone (), 0);
		let mut todo: Vec <Self> = Vec::new ();
		todo.push (self.clone ());
		while let Some (symbol) = todo.pop () {
			for child in symbol.original_children () {
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

impl Eq for Symbol {}

impl Hash for Symbol {
	fn hash <Hasher: hash::Hasher> (& self, state: & mut Hasher) {
		self.inner.name.hash (state);
	}
}

impl Ord for Symbol {
	fn cmp (& self, other: & Self) -> Ordering {
		self.inner.name.cmp (& other.inner.name)
	}
}

impl PartialEq for Symbol {
	fn eq (& self, other: & Self) -> bool { self.inner.name == other.inner.name }
}

impl cmp::PartialOrd for Symbol {
	fn partial_cmp (& self, other: & Self) -> Option <Ordering> {
		self.inner.name.partial_cmp (& other.inner.name)
	}
}

impl SymVal {

	fn eval (
		& self,
		lookup: & dyn Fn (& Symbol) -> Rc <[i64]>,
		input: & [& [i64]],
	) -> Result <Vec <i64>, MachineError> {

		fn combine <Combine: Fn (i64, i64) -> i64> (
			left: & Rc <[i64]>,
			right: & Rc <[i64]>,
			combine: Combine,
		) -> Vec <i64> {
			let mut results = HashSet::new ();
			for left in left.iter ().copied () { for right in right.iter ().copied () {
				results.insert (combine (left, right));
			} }
			let mut results: Vec <i64> = results.into_iter ().collect ();
			results.sort_unstable ();
			results
		}

		Ok (match * self {
			Self::Symbol (ref arg) =>
				lookup (arg).to_vec (),
			Self::Input (ref arg) =>
				input.get (* arg).copied ().ok_or (MachineError::NoMoreInput) ?.to_vec (),
			Self::Add (ref left, ref right) =>
				combine (& lookup (left), & lookup (right), |a, b| a + b),
			Self::Multiply (ref left, ref right) =>
				combine (& lookup (left), & lookup (right), |a, b| a * b),
			Self::Divide (ref left, ref right) =>
				combine (& lookup (left), & lookup (right), |a, b| a / b),
			Self::Modulo (ref left, ref right) =>
				combine (& lookup (left), & lookup (right), |a, b| a % b),
			Self::IsEqual (ref left, ref right) =>
				combine (& lookup (left), & lookup (right), |a, b| i64::from (a == b)),
			Self::IsUnequal (ref left, ref right) =>
				combine (& lookup (left), & lookup (right), |a, b| i64::from (a != b)),
			Self::Value (ref arg) =>
				vec! [ * arg ],
			Self::Error (ref arg) =>
				Err (* arg) ?,
		})

	}

	#[ inline ]
	#[ must_use ]
	pub fn depth (& self) -> usize {
		self.children ().iter ()
			.map (Symbol::depth)
			.max ()
			.unwrap_or (0) + 1
	}

	#[ inline ]
	#[ must_use ]
	pub fn len (& self) -> usize {
		self.children ().iter ().fold (1, |len, child| len + child.len ())
	}

	#[ inline ]
	#[ must_use ]
	pub fn is_empty (& self) -> bool {
		self.children ().iter ().next ().is_some ()
	}

	#[ inline ]
	#[ must_use ]
	pub fn original_depth (& self) -> usize {
		self.children ().iter ()
			.map (Symbol::original_depth)
			.max ()
			.unwrap_or (0) + 1
	}

	#[ inline ]
	#[ must_use ]
	pub fn original_len (& self) -> usize {
		self.children ().iter ().fold (1, |len, child| len + child.original_len ())
	}

	#[ must_use ]
	pub fn children (& self) -> ArrayVec <Symbol, 2> {
		fn make <const CAP: usize> (arg: [& Symbol; CAP]) -> ArrayVec <Symbol, 2> {
			arg.into_iter ().cloned ().collect ()
		}
		match * self {
			Self::Symbol (ref arg) => make ([ arg ]),
			Self::Input (_) => make ([ ]),
			Self::Add (ref left, ref right) => make ([ left, right ]),
			Self::Multiply (ref left, ref right) => make ([ left, right ]),
			Self::Divide (ref left, ref right) => make ([ left, right ]),
			Self::Modulo (ref left, ref right) => make ([ left, right ]),
			Self::IsEqual (ref left, ref right) => make ([ left, right ]),
			Self::IsUnequal (ref left, ref right) => make ([ left, right ]),
			Self::Value (_) => make ([ ]),
			Self::Error (_) => make ([ ]),
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
		match * self {
			Self::Symbol (ref other_symbol) =>
				write! (formatter, "{}", expand (other_symbol, wrap)) ?,
			Self::Input (ref index) =>
				write! (formatter, "input [{}]", index) ?,
			Self::Add (ref left, ref right) =>
				write! (formatter, "{}{} + {}{}", open, expand (left, true), expand (right, true), close) ?,
			Self::Multiply (ref left, ref right) =>
				write! (formatter, "{}{} × {}{}", open, expand (left, true), expand (right, true), close) ?,
			Self::Divide (ref left, ref right) =>
				write! (formatter, "{}{} ÷ {}{}", open, expand (left, true), expand (right, true), close) ?,
			Self::Modulo (ref left, ref right) =>
				write! (formatter, "{}{} mod {}{}", open, expand (left, true), expand (right, true), close) ?,
			Self::IsEqual (ref left, ref right) =>
				write! (formatter, "{}{} = {}{}", open, expand (left, true), expand (right, true), close) ?,
			Self::IsUnequal (ref left, ref right) =>
				write! (formatter, "{}{} ≠ {}{}", open, expand (left, true), expand (right, true), close) ?,
			Self::Value (ref value) =>
				write! (formatter, "{}", value) ?,
			Self::Error (ref arg) =>
				write! (formatter, "{:?}", arg) ?,
		}
		Ok (())
	}
	const fn expand (& self, depth: usize) -> FormatExpand {
		let options = FormatExpandOptions::Depth (depth);
		FormatExpand::SymVal (options, false, self)
	}
	fn simplified (& self) -> Self {
		self.simplify ().unwrap_or_else (|| self.clone ())
	}
	fn simplify (& self) -> Option <Self> {
		let mut value = self;
		let mut result = None;
		while let Some (temp) = value.simplify_real () {
			result = Some (temp);
			value = result.as_ref ().unwrap ();
		}
		result
	}
	#[ allow (clippy::todo) ]
	fn simplify_real (& self) -> Option <Self> {
		match * self {
			Self::Input (_) => None,
			Self::Add (ref left_sym, ref right_sym) => {
				let left_symval = left_sym.value ();
				let right_symval = right_sym.value ();
				match (& left_symval, & right_symval) {
					(& Self::Error (_), _) => Some (left_symval),
					(_, & Self::Error (_)) => Some (right_symval),
					(& Self::Value (ref left_val), & Self::Value (ref right_val)) =>
						Some (Self::Value (left_val + right_val)),
					(_, & Self::Value (0)) => Some (left_symval),
					(& Self::Value (0), _) => Some (right_symval),
					(_, _) if left_symval == right_symval => todo! (),
					_ => None,
				}
			},
			Self::Multiply (ref left_sym, ref right_sym) => {
				let left_symval = left_sym.value ();
				let right_symval = right_sym.value ();
				match (& left_symval, & right_symval) {
					(& Self::Error (_), _) => Some (left_symval),
					(_, & Self::Error (_)) => Some (right_symval),
					(& Self::Value (left_val), & Self::Value (right_val)) =>
						Some (Self::Value (left_val * right_val)),
					(_, & Self::Value (0)) => Some (Self::Value (0)),
					(& Self::Value (0), _) => Some (Self::Value (0)),
					_ => None,
				}
			},
			Self::Divide (ref left_sym, ref right_sym) => {
				let left_symval = left_sym.value ();
				let right_symval = right_sym.value ();
				match (& left_symval, & right_symval) {
					(& Self::Error (_), _) => Some (left_symval),
					(_, & Self::Error (_)) => Some (right_symval),
					(& Self::Value (left_val), & Self::Value (right_val)) =>
						Some (Self::Value (left_val / right_val)),
					(_, & Self::Value (1)) => Some (left_symval),
					(_, & Self::Value (0)) =>
						Some (Self::Error (MachineError::DivideByZero)),
					(& Self::Value (0), _) => Some (Self::Value (0)),
					(_, _) if left_symval == right_symval => todo! (),
					_ => None,
				}
			}
			Self::Modulo (ref left_sym, ref right_sym) => {
				let left_symval = left_sym.value ();
				let right_symval = right_sym.value ();
				match (& left_symval, & right_symval) {
					(& Self::Error (_), _) => Some (left_symval),
					(_, & Self::Error (_)) => Some (right_symval),
					(& Self::Value (left_val), & Self::Value (right_val)) => {
						if left_val >= 0 && right_val > 0 {
							Some (Self::Value (left_val % right_val))
						} else if right_val == 0 {
							Some (Self::Error (MachineError::DivideByZero))
						} else {
							Some (Self::Error (MachineError::NegativeModulo))
						}
					},
					(_, & Self::Value (0)) =>
						Some (Self::Error (MachineError::DivideByZero)),
					(_, & Self::Value (right_val)) if right_val < 0 =>
						Some (Self::Error (MachineError::NegativeModulo)),
					(& Self::Value (left_val), _) if left_val < 0 =>
						Some (Self::Error (MachineError::NegativeModulo)),
					_ => None,
				}
			},
			Self::IsEqual (ref left_sym, ref right_sym) => {
				let left_symval = left_sym.value ();
				let right_symval = right_sym.value ();
				match (& left_symval, & right_symval) {
					(& Self::Error (_), _) => Some (left_symval),
					(_, & Self::Error (_)) => Some (right_symval),
					(& Self::IsEqual (ref inner_left_sym, ref inner_right_sym), & Self::Value (0)) =>
						Some (Self::IsUnequal (inner_left_sym.clone (), inner_right_sym.clone ())),
					(& Self::Value (0), & Self::IsEqual (ref inner_left_sym, ref inner_right_sym)) =>
						Some (Self::IsUnequal (inner_left_sym.clone (), inner_right_sym.clone ())),
					(& Self::IsEqual (_, _), & Self::Value (1)) => Some (left_symval),
					(& Self::Value (1), & Self::IsEqual (_, _)) => Some (right_symval),
					(& Self::Value (left_val), & Self::Value (right_val)) =>
						Some (Self::Value (i64::from (left_val == right_val))),
					_ => None,
				}
			},
			Self::IsUnequal (ref left_sym, ref right_sym) => {
				let left_symval = left_sym.value ();
				let right_symval = right_sym.value ();
				match (& left_symval, & right_symval) {
					(& Self::Error (_), _) => Some (left_symval),
					(_, & Self::Error (_)) => Some (right_symval),
					(& Self::Value (ref left_val), & Self::Value (ref right_val)) =>
						Some (Self::Value (i64::from (left_val != right_val))),
					_ => None,
				}
			},
			Self::Value (_) => None,
			Self::Symbol (_) | Self::Error (_) => todo! ("SymVal::{:?}", self),
		}
	}
	fn migrate (& self, solver: & mut Solver, input: & [i64]) -> Self {
		let dup = |arg: Symbol| solver.get (arg.name ()).unwrap ();
		match self.clone () {
			Self::Symbol (arg) => Self::Symbol (dup (arg)),
			Self::Input (arg) =>
				if let Some (& val) = input.get (arg) {
					Self::Value (val)
				} else {
					Self::Input (arg)
				},
			Self::Add (left, right) => Self::Add (dup (left), dup (right)),
			Self::Multiply (left, right) => Self::Multiply (dup (left), dup (right)),
			Self::Divide (left, right) => Self::Divide (dup (left), dup (right)),
			Self::Modulo (left, right) => Self::Modulo (dup (left), dup (right)),
			Self::IsEqual (left, right) => Self::IsEqual (dup (left), dup (right)),
			Self::IsUnequal (left, right) => Self::IsUnequal (dup (left), dup (right)),
			Self::Value (arg) => Self::Value (arg),
			Self::Error (arg) => Self::Error (arg),
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

enum FormatExpand <'dat> {
	Symbol (FormatExpandOptions <'dat>, bool, & 'dat Symbol),
	SymVal (FormatExpandOptions <'dat>, bool, & 'dat SymVal),
}

impl <'dat> fmt::Display for FormatExpand <'dat> {
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
enum FormatExpandOptions <'dat> {
	Depth (usize),
	BreakSymbols (& 'dat HashSet <Symbol>),
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

	fn new () -> Self {
		Self {
			entries: HashMap::new (),
		}
	}

	fn define (& mut self, base_name: & Rc <str>) -> Rc <str> {
		let entry =
			self.entries.entry (Rc::clone (base_name))
				.and_modify (|entry| entry.latest_rev += 1)
				.or_insert (VerNamerEntry {
					base_name: Rc::clone (base_name),
					latest_name: Rc::clone (base_name),
					latest_rev: 0,
				});
		entry.latest_name = format! ("{}{}", entry.base_name, entry.latest_rev).into ();
		Rc::clone (& entry.latest_name)
	}

}
