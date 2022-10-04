use super::*;

use input::Input;

pub type CaveId = u8;

pub struct Caves <'inp> {
	pub caves: Vec <Cave <'inp>>,
}

impl <'inp> Index <CaveId> for Caves <'inp> {
	type Output = Cave <'inp>;
	fn index (& self, idx: CaveId) -> & '_ Cave <'inp> {
		& self.caves [idx.pan_usize ()]
	}
}

impl <'inp> Caves <'inp> {

	#[ inline ]
	#[ must_use ]
	pub fn start (& self) -> & Cave <'inp> {
		& self.caves [0]
	}

	#[ inline ]
	#[ must_use ]
	pub fn end (& self) -> & Cave <'inp> {
		& self.caves [1]
	}

	pub fn build (input: & Input <'inp>) -> GenResult <Self> {
		if 100 < input.cnxns.len () {
			return Err ("Max 100 connections".into ());
		}
		fn cave_id_or_insert <'inp> (caves: & mut Vec <Cave <'inp>>, name: & InpStr <'inp>) -> CaveId {
			caves.iter ()
				.position (|cave| & cave.name == name)
				.unwrap_or_else (|| {
					let idx = caves.len ();
					caves.push (Cave::new (name.clone (), idx.pan_u8 ()));
					idx
				})
				.pan_u8 ()
		}
		let mut caves = vec! [
			Cave::new (InpStr::borrow ("start"), 0),
			Cave::new (InpStr::borrow ("end"), 1),
		];
		for cnxn in input.cnxns.iter () {
			let id_0 = cave_id_or_insert (& mut caves, & cnxn.cave_0);
			let id_1 = cave_id_or_insert (& mut caves, & cnxn.cave_1);
			if 25 < caves.len () {
				return Err ("Max 25 caves".into ());
			}
			caves [id_0.pan_usize ()].cnxns.push (id_1);
			caves [id_1.pan_usize ()].cnxns.push (id_0);
		}
		for cave in & caves {
			if cave.small { continue }
			for & cnxn in & cave.cnxns {
				if ! caves [cnxn.pan_usize ()].small {
					return Err ("Large caves can only be connected to small caves".into ());
				}
			}
		}
		Ok (Self { caves })
	}

}

pub struct Cave <'inp> {
	pub name: InpStr <'inp>,
	pub cnxns: Vec <CaveId>,
	pub small: bool,
	pub id: CaveId,
}

impl <'inp> Cave <'inp> {
	fn new (name: InpStr <'inp>, id: CaveId) -> Self {
		let cnxns = Vec::new ();
		let small = name.chars ().next ().unwrap ().is_ascii_lowercase ();
		Self { name, cnxns, small, id }
	}
}
