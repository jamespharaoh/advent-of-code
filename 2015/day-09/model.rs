use super::*;

use input::Input;

pub type Dist = u32;

pub struct DistTable <'inp> {
	places: MapToIndex <InpStr <'inp>>,
	dists: Vec <Dist>,
}

impl <'inp> DistTable <'inp> {

	#[ must_use ]
	pub fn build (input: & Input <'inp>) -> Self {
		let places: MapToIndex <InpStr> =
			input.dists.iter ()
				.flat_map (|& (ref place_0, ref place_1, _)| [ place_0, place_1 ])
				.cloned ()
				.collect ();
		let mut dists = vec! [0; places.len () * places.len ()];
		for & (ref place_0, ref place_1, dist) in & input.dists {
			let idx_0 = places [place_0];
			let idx_1 = places [place_1];
			dists [idx_0 * places.len () + idx_1] = dist;
			dists [idx_1 * places.len () + idx_0] = dist;
		}
		Self { places, dists }
	}

	#[ inline ]
	#[ must_use ]
	pub fn len (& self) -> usize {
		self.places.len ()
	}

	#[ inline ]
	#[ must_use ]
	pub fn is_empty (& self) -> bool {
		self.places.is_empty ()
	}

}

impl <'inp> Index <(usize, usize)> for DistTable <'inp> {
	type Output = Dist;
	fn index (& self, (idx_0, idx_1): (usize, usize)) -> & Dist {
		& self.dists [idx_0 * self.places.len () + idx_1]
	}
}
