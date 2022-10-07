use super::*;

use input::Input;

pub type Dist = u32;

wrapper_deref_mut! {
	pub struct DistTable <'inp> {
		inner: PairsMap <InpStr <'inp>, Dist>,
	}
}

impl <'inp> DistTable <'inp> {

	#[ must_use ]
	pub fn build (input: & Input <'inp>) -> Self {
		Self {
			inner: input.dists.iter ()
				.flat_map (|& (ref place_0, ref place_1, dist)| [
					(place_0.clone (), place_1.clone (), dist),
					(place_1.clone (), place_0.clone (), dist) ])
				.collect ()
		}
	}

}
