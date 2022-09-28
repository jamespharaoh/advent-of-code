use super::*;

#[ derive (Clone, Copy, Debug) ]
pub struct Operation {
	multiply: u64,
	add: u64,
	modulo: u64,
}

impl Operation {

	#[ inline ]
	#[ must_use ]
	pub const fn new (modulo: u64) -> Self {
		assert! (0 < modulo);
		Self {
			multiply: 1,
			add: 0,
			modulo,
		}
	}

	#[ inline ]
	#[ must_use ]
	pub fn then_add (self, arg: u64) -> Self {
		let add = self.add.pan_u128 ();
		let modulo = self.modulo.pan_u128 ();
		let arg = arg.pan_u128 ();
		Self {
			multiply: self.multiply,
			add: ((add + arg) % modulo).pan_u64 (),
			modulo: self.modulo,
		}
	}

	#[ inline ]
	#[ must_use ]
	pub fn then_multiply (self, arg: u64) -> Self {
		let multiply = self.multiply.pan_u128 ();
		let add = self.add.pan_u128 ();
		let modulo = self.modulo.pan_u128 ();
		let arg = arg.pan_u128 ();
		Self {
			multiply: (multiply * arg % modulo).pan_u64 (),
			add: (add * arg % modulo).pan_u64 (),
			modulo: self.modulo,
		}
	}

	#[ inline ]
	#[ must_use ]
	pub fn apply (self, arg: u64) -> u64 {
		let multiply = self.multiply.pan_u128 ();
		let add = self.add.pan_u128 ();
		let modulo = self.modulo.pan_u128 ();
		let arg = arg.pan_u128 ();
		((arg * multiply + add) % modulo).pan_u64 ()
	}

	#[ inline ]
	#[ must_use ]
	pub fn then (self, other: Self) -> Self {
		assert_eq! (self.modulo, other.modulo);
		let self_multiply = self.multiply.pan_u128 ();
		let self_add = self.add.pan_u128 ();
		let other_multiply = other.multiply.pan_u128 ();
		let other_add = other.add.pan_u128 ();
		let modulo = self.modulo.pan_u128 ();
		Self {
			multiply: (self_multiply * other_multiply % modulo).pan_u64 (),
			add: ((self_add * other_multiply + other_add) % modulo).pan_u64 (),
			modulo: self.modulo,
		}
	}

	#[ inline ]
	#[ must_use ]
	pub fn reverse (self) -> Option <Self> {
		Some (
			Self::new (self.modulo)
				.then_add (self.modulo - self.add)
				.then_multiply (modulo_inverse (self.multiply, self.modulo) ?)
		)
	}

	#[ inline ]
	#[ must_use ]
	pub fn repeat (self, arg: u64) -> Self {
		let mut next_op = self;
		let mut result = Self::new (self.modulo);
		let mut remain = arg;
		while remain != 0 {
			if remain & 0x1 != 0 { result = result.then (next_op); }
			next_op = next_op.then (next_op);
			remain >>= 1_u32;
		}
		result
	}

}

fn modulo_inverse (arg: u64, modulo: u64) -> Option <u64> {
	let (mut bzt, mut new_bzt) = (0_i128, 1_i128);
	let (mut rem, mut new_rem) = (modulo.pan_i128 (), arg.pan_i128 ());
	while new_rem != 0_i128 {
		let quot = rem / new_rem;
		(bzt, new_bzt) = (new_bzt, bzt - quot * new_bzt);
		(rem, new_rem) = (new_rem, rem - quot * new_rem);
	}
	if 1_i128 < rem { return None }
	if bzt < 0 { bzt += modulo.pan_i128 (); }
	Some (bzt.pan_u64 ())
}
