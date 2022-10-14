use super::*;

#[ derive (Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd) ]
pub struct Ingredient <'inp> {
	pub name: InpStr <'inp>,
	pub stats: [i32; 5],
}

struct_parser_display! {
	input_lifetime = 'inp;
	Ingredient <'inp> { name, stats: [ capacity, durability, flavour, texture, calories ] } = [
		@str name = (|ch| { ch.is_ascii_alphanumeric () }, 1 ..= 20), ": ",
		"capacity ", capacity, ", ",
		"durability ", durability, ", ",
		"flavor ", flavour, ", ",
		"texture ", texture, ", ",
		"calories ", calories = 1_i32 ..= 500_i32,
	]
}

#[ derive (Clone, Copy, Debug, Default) ]
pub struct Recipe {
	pub num_ingrs: i8,
	pub ingrs: [i8; 6],
	pub stats: [i32; 5],
}

impl Recipe {

	#[ inline ]
	#[ must_use ]
	pub const fn calories (& self) -> i32 {
		self.stats [4]
	}

	#[ inline ]
	pub fn score (& self) -> NumResult <u64> {
		self.stats [0 .. 4].iter ()
			.map (|& item| cmp::max (item, 0_i32).pan_u64 ())
			.try_fold (1, |prod, item| chk! (prod * item))
	}

	#[ inline ]
	pub fn add_ingrs (& mut self, ingrs: & [Ingredient], ingr_idx: usize, num: i8) -> GenResult <()> {
		self.num_ingrs += num;
		self.ingrs [ingr_idx] += num;
		let num = num.pan_i32 ();
		for stat_idx in 0 .. 5 {
			chk! (self.stats [stat_idx] += ingrs [ingr_idx].stats [stat_idx] * num) ?;
		}
		Ok (())
	}

}
