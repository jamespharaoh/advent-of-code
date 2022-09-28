#![ no_main ]

use libfuzzer_sys::fuzz_target;
use rand::prelude::*;

use aoc_2019::day_14::*;
use aoc_common::*;
use aoc_fuzz::*;
use input::Input;
use model::ChemQty;
use model::Qty;
use model::Reaction;

fuzz_target! (|input_str: & str| {
	let input_vec: Vec <& str> = input_str.trim_end ().split ('\n').collect ();
	if let Ok (input) = Input::parse_from_lines (& input_vec) {
		let _ = logic::part_one (& input);
		let _ = logic::part_two (& input);
	}
});

aoc_fuzz_mutator! {

	transform_lifetimes = <'inp>;
	input_type = Input;

	transform add (100 * 1, 10 * 5, 1 * 25) = |input, rng| {
		let generator = Generator::new (input);
		let reaction = generator.reaction (rng);
		let idx = rng.gen_range (0 ..= input.reactions.len ());
		input.reactions.insert (idx, reaction);
	}

	pub transform remove (100 * 1, 10 * 5, 1 * 25) = |input, rng| {
		if input.reactions.is_empty () { return Some (()) }
		let idx = rng.gen_range (0 .. input.reactions.len ());
		input.reactions.remove (idx);
	}

	transform shuffle (1) = |input, rng| {
		input.reactions.shuffle (rng);
	}

	transform sort (1) = |input, _rng| {
		input.reactions.sort ();
	}

}

struct Generator <'inp> {
	chems: Vec <InpStr <'inp>>,
}

impl <'inp> Generator <'inp> {

	fn new <'inp0> (input: & 'inp0 Input <'inp>) -> Self {
		Generator {
			chems: input.reactions.iter ()
				.map (|reaction| reaction.output.chem.clone ())
				.filter (|chem| chem != & InpStr::borrow ("FUEL"))
				.chain (iter::once (InpStr::borrow ("ORE")))
				.collect (),
		}
	}

	pub fn reaction (& self, rng: & mut StdRng) -> Reaction <'inp> {
		let num_inputs = rng.gen_range (1 ..= 10);
		Reaction {
			inputs: iter::from_fn (|| Some (self.chem_qty (rng, true)))
				.take (num_inputs)
				.collect (),
			output: self.chem_qty (rng, false),
		}
	}

	pub fn chem_qty (& self, rng: & mut StdRng, input: bool) -> ChemQty <'inp> {
		ChemQty {
			chem: self.chem (rng, input),
			qty: self.qty (rng),
		}
	}

	pub fn chem (& self, rng: & mut StdRng, input: bool) -> InpStr <'inp> {
		if input && ! self.chems.is_empty () && rng.gen_bool (0.9) {
			return self.chems.choose (rng).unwrap ().clone ()
		}
		if ! input && ! self.chems.contains (& InpStr::borrow ("FUEL")) && rng.gen_bool (0.9) {
			return InpStr::borrow ("FUEL")
		}
		let len = Self::NAME_LENS.choose (rng).copied ().unwrap ();
		let mut name = String::new ();
		for _ in 0 .. len {
			name.push (Self::LETTERS.choose (rng).unwrap ().pan_char ());
		}
		InpStr::alloc (name)
	}

	pub fn qty (& self, rng: & mut StdRng) -> Qty {
		rng.gen::<Qty> () & (Qty::MAX >> rng.gen_range (0 .. Qty::BITS))
	}

	const NAME_LENS: & 'static [usize] = & [ 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 3, 3, 3, 4, 5 ];
	const LETTERS: & 'static [u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";

}
