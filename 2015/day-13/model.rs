use super::*;

use input::DiffSign;

wrapper_deref_mut! {
	pub struct HappinessTable <'inp> {
		inner: PairsMap <InpStr <'inp>, i32>,
	}
}

impl <'inp> HappinessTable <'inp> {
	pub fn build (
		input_pairs: & [(InpStr <'inp>, InpStr <'inp>, DiffSign, i32)],
	) -> GenResult <Self> {
		let inner: PairsMap <InpStr, i32> =
			input_pairs.iter ()
				.map (|& (ref person_0, ref person_1, sign, amnt)|
					(person_0.clone (), person_1.clone (), match sign {
						DiffSign::Gain => amnt,
						DiffSign::Lose => - amnt,
					}))
				.collect ();
		if 10 < inner.len () { return Err ("Max 10 people allowed".into ()) }
		Ok (Self { inner })
	}
}
