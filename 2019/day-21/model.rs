use super::*;

pub use frag::Frag;
pub use frag::FragIter;
pub use regs::Regs;
pub use regs::RegsIter;
pub use rules::Rules;
pub use rules_set::RulesSet;
pub use sample::Sample;

pub type Cpu = intcode::Machine <Val>;
pub type Val = i32;

mod frag {

	use super::*;

	#[ derive (Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd) ]
	pub struct Frag <const LEN: usize> {
		mask: u16,
		regs: u16,
	}

	impl <const LEN: usize> Frag <LEN> {

		#[ must_use ]
		pub const fn get (self, idx: usize) -> Option <bool> {
			if LEN <= idx { return None }
			if self.mask << (16 - LEN + idx) & 0x8000 == 0 { return None }
			Some (self.regs << (16 - LEN + idx) & 0x8000 == 0)
		}

		#[ must_use ]
		pub const fn num_regs (self) -> u32 {
			self.mask.count_ones ()
		}

		#[ must_use ]
		pub const fn num_holes (self) -> u32 {
			self.regs.count_ones ()
		}

		#[ must_use ]
		pub fn first_reg (self) -> u32 {
			self.mask.leading_zeros () + LEN.pan_u32 () - 16
		}

		#[ must_use ]
		pub fn first_hole (self) -> u32 {
			self.regs.leading_zeros () + LEN.pan_u32 () - 16
		}

		#[ must_use ]
		pub fn last_reg (self) -> u32 {
			LEN.pan_u32 () - self.mask.trailing_zeros ()
		}

		#[ must_use ]
		pub fn last_hole (self) -> u32 {
			LEN.pan_u32 () - self.regs.trailing_zeros ()
		}

		#[ must_use ]
		pub fn iter (self) -> FragIter {
			FragIter {
				mask: self.mask << (16 - LEN),
				regs: self.regs << (16 - LEN),
				remain: LEN.pan_u16 (),
			}
		}

		#[ must_use ]
		pub const fn matches (self, regs: Regs <LEN>) -> bool {
			regs.holes () & self.mask == self.regs
		}

		pub fn iter_all () -> impl Iterator <Item = Self> {
			let mut mask = 0;
			let mut regs = 0;
			iter::from_fn (move || {
				if Regs::<LEN>::MASK < mask { return None }
				regs = (regs | (Regs::<LEN>::MASK & ! mask)) + 1;
				if Regs::<LEN>::MASK < regs {
					regs = 0;
					mask += 1;
					if Regs::<LEN>::MASK < mask { return None }
				}
				Some (Self { regs: regs & mask, mask })
			})
		}

	}

	impl <const LEN: usize> Debug for Frag <LEN> {
		fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
			let mut mask = self.mask << (16 - LEN);
			let mut regs = self.regs << (16 - LEN);
			for _ in 0 .. LEN {
				if mask & 0x8000 == 0 { formatter.write_char ('·') ?; }
				else if regs & 0x8000 == 0 { formatter.write_char ('#') ?; }
				else { formatter.write_char ('_') ?; }
				mask <<= 1_u32;
				regs <<= 1_u32;
			}
			Ok (())
		}
	}

	pub struct FragIter {
		mask: u16,
		regs: u16,
		remain: u16,
	}

	impl Iterator for FragIter {
		type Item = Option <bool>;
		fn next (& mut self) -> Option <Option <bool>> {
			if self.remain == 0 { return None }
			let val = if self.mask & 0x8000 == 0 { None } else { Some (self.regs & 0x8000 == 0) };
			self.mask <<= 1_u32;
			self.regs <<= 1_u32;
			self.remain -= 1_u16;
			Some (val)
		}
	}

}

mod regs {

	use super::*;

	#[ derive (Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd) ]
	pub struct Regs <const LEN: usize> { data: u16 }

	impl <const LEN: usize> Regs <LEN> {

		#[ must_use ]
		pub fn from_slice (slice: & [bool]) -> Self {
			assert! (LEN <= slice.len ());
			let mut data = 0;
			for & val in slice.iter ().take (LEN) {
				data <<= 1_u32;
				if ! val { data |= 1; }
			}
			Self { data }
		}

		#[ must_use ]
		pub fn first_hole (self) -> u32 {
			if self.data == 0 { 0 } else { self.data.leading_zeros () + LEN.pan_u32 () - 16 }
		}

		#[ must_use ]
		pub fn last_hole (self) -> u32 {
			if self.data == 0 { 0 } else { LEN.pan_u32 () - self.data.trailing_zeros () }
		}

		#[ must_use ]
		pub fn get (self, idx: usize) -> Option <bool> {
			(idx.pan_usize () < LEN).then_some (
				self.data & (0x8000 >> (16 - LEN + idx)) == 0)
		}

		#[ must_use ]
		pub const fn holes (self) -> u16 {
			self.data
		}

		#[ must_use ]
		pub const fn not_holes (self) -> u16 {
			self.data ^ Self::MASK
		}

		#[ must_use ]
		pub fn iter (self) -> RegsIter <LEN> {
			RegsIter { data: self.data << (32 - LEN), remain: LEN.pan_u16 () }
		}

		pub const ALL: Self = Self { data: 0 };
		pub const NONE: Self = Self { data: Self::MASK };
		pub const MASK: u16 = ! (u16::MAX << LEN);

	}

	impl <const LEN: usize> TryFrom <u16> for Regs <LEN> {
		type Error = Overflow;
		fn try_from (data: u16) -> Result <Self, Overflow> {
			if data & ! Self::MASK != 0 { return Err (Overflow) }
			Ok (Self { data })
		}
	}

	impl <const LEN: usize> Display for Regs <LEN> {
		fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
			let mut data = self.data << (32 - LEN);
			for _ in 0 .. LEN {
				if data & 0x8000 == 0 {
					write! (formatter, "#") ?;
				} else {
					write! (formatter, "_") ?;
				}
				data <<= 1_u32;
			}
			Ok (())
		}
	}

	impl <const LEN: usize> IntoIterator for Regs <LEN> {
		type Item = bool;
		type IntoIter = RegsIter <LEN>;
		fn into_iter (self) -> RegsIter <LEN> {
			RegsIter { data: self.data << (32 - LEN), remain: LEN.pan_u16 () }
		}
	}

	pub struct RegsIter <const LEN: usize> {
		data: u16,
		remain: u16,
	}

	impl <const LEN: usize> Iterator for RegsIter <LEN> {
		type Item = bool;
		fn next (& mut self) -> Option <bool> {
			if self.remain == 0 { return None }
			let val = self.data & 0x8000 == 0;
			self.data <<= 1_u32;
			self.remain -= 1;
			Some (val)
		}
	}

}

mod rules {

	use super::*;

	#[ derive (Clone, Eq, Hash, Ord, PartialEq, PartialOrd) ]
	pub struct Rules <const LEN: usize> {
		rules: Vec <(Regs <LEN>, bool)>,
		stats: RulesStats <LEN>,
	}

	impl <const LEN: usize> Rules <LEN> {

		#[ must_use ]
		pub fn new () -> Self {
			Self {
				rules: Vec::new (),
				stats: RulesStats::new (),
			}
		}

		#[ must_use ]
		pub fn len (& self) -> usize {
			self.rules.len ()
		}

		#[ must_use ]
		pub fn is_empty (& self) -> bool {
			self.rules.is_empty ()
		}

		#[ must_use ]
		pub fn num_true (& self) -> usize {
			self.stats.num_true.pan_usize ()
		}

		#[ must_use ]
		pub fn num_false (& self) -> usize {
			self.stats.num_false.pan_usize ()
		}

		#[ must_use ]
		pub fn first_hole (& self) -> u32 {
			self.stats.first_hole.pan_u32 ()
		}

		#[ must_use ]
		pub fn last_hole (& self) -> u32 {
			self.stats.last_hole.pan_u32 ()
		}

		#[ must_use ]
		pub fn num_jumps (& self) -> u32 {
			self.stats.num_jumps.pan_u32 ()
		}

		#[ must_use ]
		pub const fn num_jump_holes (& self) -> u32 {
			self.stats.jump_holes.count_ones ()
		}

		#[ must_use ]
		pub const fn num_conflicts (& self) -> u32 {
			(self.stats.holes & self.stats.not_holes).count_ones ()
		}

		#[ must_use ]
		pub const fn order (& self) -> u16 {
			self.stats.hash
		}

		pub fn iter (& self) -> SliceIter <(Regs <LEN>, bool)> {
			self.rules.iter ()
		}

		#[ must_use ]
		pub fn push (& mut self, push_regs: Regs <LEN>, push_jump: bool) -> Option <()> {
			for & (rule_regs, rule_jump) in & self.rules {
				if rule_regs != push_regs { continue }
				if rule_jump != push_jump { return None }
				return Some (());
			}
			self.rules.push ((push_regs, push_jump));
			self.rules.sort ();
			self.stats.update (push_regs, push_jump);
			Some (())
		}

		#[ must_use ]
		pub fn with_rule (& self, with_regs: Regs <LEN>, with_jump: bool) -> Option <Self> {
			for & (rule_regs, rule_jump) in & self.rules {
				if rule_regs != with_regs { continue }
				if rule_jump != with_jump { return None }
				return Some (self.clone ());
			}
			let mut rules = self.rules.clone ();
			rules.push ((with_regs, with_jump));
			rules.sort ();
			let mut stats = self.stats;
			stats.update (with_regs, with_jump);
			Some (Self { rules, stats })
		}

		#[ must_use ]
		pub fn with_rules (& self, other: & Self) -> Option <Self> {
			let mut result = self.clone ();
			for & (regs, jump) in & other.rules {
				result = result.with_rule (regs, jump) ?;
			}
			Some (result)
		}

	}

	impl <const LEN: usize> Debug for Rules <LEN> {
		fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
			if self.rules.is_empty () {
				return formatter.write_str ("Rules []");
			}
			formatter.write_str ("Rules [\n") ?;
			for & (regs, jump) in & self.rules {
				write! (formatter, "    {regs} => {jump}\n") ?;
			}
			formatter.write_str ("]") ?;
			Ok (())
		}
	}

	impl <const LEN: usize> Default for Rules <LEN> {
		fn default () -> Self {
			Self::new ().with_rule (Regs::ALL, false).unwrap ()
		}
	}

	impl <'rules, const LEN: usize> IntoIterator for & 'rules Rules <LEN> {
		type Item = & 'rules (Regs <LEN>, bool);
		type IntoIter = SliceIter <'rules, (Regs <LEN>, bool)>;
		fn into_iter (self) -> SliceIter <'rules, (Regs <LEN>, bool)> {
			self.rules.iter ()
		}
	}

	#[ derive (Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd) ]
	struct RulesStats <const LEN: usize> {
		num_false: u8,
		num_true: u8,
		first_hole: u8,
		last_hole: u8,
		num_jumps: u8,
		jump_holes: u16,
		holes: u16,
		not_holes: u16,
		hash: u16,
	}

	impl <const LEN: usize> RulesStats <LEN> {
		fn new () -> Self {
			Self {
				num_false: 0,
				num_true: 0,
				first_hole: LEN.pan_u8 (),
				last_hole: 0,
				num_jumps: 0,
				jump_holes: 0,
				holes: 0,
				not_holes: 0,
				hash: 0,
			}
		}
		fn update (& mut self, regs: Regs <LEN>, jump: bool) {
			if jump { self.num_true += 1; } else { self.num_false += 1; }
			self.first_hole = cmp::min (self.first_hole, regs.first_hole ().pan_u8 ());
			self.last_hole = cmp::max (self.last_hole, regs.last_hole ().pan_u8 ());
			if jump { self.num_jumps += 1; }
			if jump { self.jump_holes |= regs.holes (); }
			let hash_val = if jump { 0xf000 } else { 0 } | regs.holes ();
			self.holes |= regs.holes ();
			self.not_holes |= regs.holes ();
			self.hash = self.hash.rotate_left (1) ^ hash_val;
			self.hash = (self.hash << 8_u32) | (self.hash >> 8_u32);
		}
	}

}

mod rules_set {

	use super::*;

	#[ derive (Clone, Eq, Hash, Ord, PartialEq, PartialOrd) ]
	pub struct RulesSet <const LEN: usize> {
		rules: Vec <Rules <LEN>>,
	}

	impl <const LEN: usize> RulesSet <LEN> {

		#[ must_use ]
		pub const fn new () -> Self {
			Self { rules: Vec::new () }
		}

		#[ must_use ]
		pub fn is_empty (& self) -> bool {
			self.rules.is_empty ()
		}

		#[ must_use ]
		pub fn len (& self) -> usize {
			self.rules.len ()
		}

		pub fn iter (& self) -> SliceIter <Rules <LEN>> {
			self.rules.iter ()
		}

		pub fn push (& mut self, rules: Rules <LEN>) {
			if self.rules.contains (& rules) { return }
			let insert_pos = self.rules.binary_search_by_key (
					& (rules.num_conflicts (), rules.num_true ()),
					|rules| (rules.num_conflicts (), rules.num_true ()))
				.either ();
			self.rules.insert (insert_pos, rules);
		}

		pub fn append (& mut self, other: & Self) {
			let mut new_rules = Vec::new ();
			for self_rules in & self.rules {
				for other_rules in & other.rules {
					if let Some (combined_rules) = self_rules.with_rules (other_rules) {
						new_rules.push (combined_rules);
					}
				}
			}
			new_rules.sort_by_key (|rules| (rules.num_conflicts (), rules.num_true ()));
			self.rules = new_rules;
		}

	}

	impl <const LEN: usize> Default for RulesSet <LEN> {
		fn default () -> Self {
			let mut result = Self::new ();
			result.push (Rules::default ());
			result
		}
	}

	impl <const LEN: usize> IntoIterator for RulesSet <LEN> {
		type IntoIter = VecIntoIter <Rules <LEN>>;
		type Item = Rules <LEN>;
		fn into_iter (self) -> VecIntoIter <Rules <LEN>> {
			self.rules.into_iter ()
		}
	}

	impl <'set, const LEN: usize> IntoIterator for & 'set RulesSet <LEN> {
		type IntoIter = SliceIter <'set, Rules <LEN>>;
		type Item = & 'set Rules <LEN>;
		fn into_iter (self) -> SliceIter <'set, Rules <LEN>> {
			self.rules.iter ()
		}
	}

}

mod sample {

	use super::*;

	#[ derive (Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd) ]
	pub struct Sample {
		data: Vec <bool>,
	}

	impl Sample {

		#[ must_use ]
		pub fn is_empty (& self) -> bool {
			self.data.is_empty ()
		}

		#[ must_use ]
		pub fn len (& self) -> usize {
			self.data.len ()
		}

	}

	impl Deref for Sample {
		type Target = [bool];
		fn deref (& self) -> & [bool] {
			& self.data
		}
	}

	impl Display for Sample {
		fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
			for & item in & self.data {
				if item {
					formatter.write_char ('#') ?;
				} else {
					formatter.write_char ('_') ?;
				}
			}
			Ok (())
		}
	}

	impl From <Vec <bool>> for Sample {
		fn from (data: Vec <bool>) -> Self {
			Self { data }
		}
	}

}
