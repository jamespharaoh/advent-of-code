#[ macro_export ]
macro_rules! aoc_fuzz_mutator {
	(
		transform_lifetimes = <$trans_life:lifetime>;
		input_type = $input_type:ty;
		$(
			$trans_vis:vis transform $trans_name:ident ($($trans_weights:tt)*) =
			|$trans_input:ident, $trans_rng:ident| { $($trans_body:tt)* }
		)*
	) => {

		::libfuzzer_sys::fuzz_mutator! (|
			data: & mut [u8],
			size: usize,
			max_size: usize,
			seed: u32,
		| {
			use ::rand::prelude::*;
			let mut rng = StdRng::seed_from_u64 (seed as u64);
			let mut size = size;
			let mut num_mutations = 0;
			if rng.gen_bool (0.7) {
				if let Some (new_size) = mutator::main (data, size, max_size, & mut rng) {
					size = new_size;
					num_mutations += 1;
				}
			}
			while num_mutations == 0 || rng.gen_bool (0.1) {
				size = ::libfuzzer_sys::fuzzer_mutate (data, size, max_size);
				num_mutations += 1;
			}
			size
		});

		mod mutator {

			use super::*;
			use ::rand::prelude::*;

			pub fn main (
				data: & mut [u8],
				size: usize,
				max_size: usize,
				rng: & mut StdRng,
			) -> Option <usize> {

				// parse input

				let input_str = str::from_utf8 (& data [0 .. size]).ok () ?;
				let input_vec: Vec <& str> = input_str.trim ().split ('\n').collect ();
				let mut input = Input::parse_from_lines (& input_vec).ok () ?;

				// apply a random transform

				transforms::random (& mut input, rng);

				// convert to string, removing random lines until it is the right length

				let output_str = loop {
					let output_str = input.to_string ();
					if output_str.as_bytes ().len () <= max_size { break output_str }
					transforms::remove (& mut input, rng);
				};

				// update data, and return

				(& mut data [ .. output_str.len ()]).copy_from_slice (output_str.as_bytes ());

				Some (output_str.len ())

			}

			mod transforms {

				use super::*;

				type TransFn = for <'inp0, 'inp> fn (& 'inp0 mut $input_type, & mut StdRng) -> Option <()>;

				aoc_fuzz_mutator! (@transforms $(( $trans_name $($trans_weights)* ))*);

				pub fn random <'inp0, 'inp> (input: & 'inp0 mut $input_type, rng: & mut StdRng) {
					let & (_, reps, ref trans_fn) =
						TRANSFORMS.choose_weighted (rng, |& (weight, _, _)| weight).unwrap ();
					let mut num_failure = 0;
					for _ in 0 .. reps {
						loop {
							let success = trans_fn (input, rng).is_some ();
							if success { break }
							num_failure += 1;
							if num_failure >= 10 { break }
						}
					}
				}

				$(
					$trans_vis fn $trans_name <'inp0, $trans_life> (
						$trans_input: & 'inp0 mut $input_type,
						$trans_rng: & mut StdRng,
					) -> Option <()> {
						$($trans_body)*
						Some (())
					}
				)*

			}

		}

	};

	(@transforms $($body:tt)*) => {
		aoc_fuzz_mutator! (@transforms_impl [] $($body)*);
	};
	(@transforms_impl [$($data:tt)*] ($name:ident $weight:literal) $($rest:tt)*) => {
		aoc_fuzz_mutator! (@transforms_impl [$($data)* ($weight, 1, $name)] $($rest)*);
	};
	(@transforms_impl [$($data:tt)*] ($name:ident $($weight:literal * $reps:literal),*) $($rest:tt)*) => {
		aoc_fuzz_mutator! (@transforms_impl [$($data)* $(($weight, $reps, $name))*] $($rest)*);
	};
	(@transforms_impl [$(( $($data:tt)* ))*]) => {
		const TRANSFORMS: & [(u32, u32, TransFn)] = & [
			$(($($data)*)),*
		];
	};

}
