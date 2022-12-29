use super::*;
use input::Input;
use input::Prog;

#[ derive (Debug) ]
pub struct ProgInfo <'inp> {
	pub name: InpStr <'inp>,
	pub holds: Vec <ProgInfo <'inp>>,
	pub prog_weight: u32,
	pub holds_weight: u32,
	pub total_weight: u32,
}

impl <'inp> ProgInfo <'inp> {

	#[ must_use ]
	pub fn nested_len (& self) -> usize {
		self.holds.iter ().fold (1, |sum, held| sum + held.nested_len ())
	}

	pub fn build (input: & Input <'inp>) -> GenResult <Self> {

		if input.progs.is_empty () { return Err ("Must have at least one program".into ()) }

		let mut progs = HashMap::new ();
		for prog in input.progs.iter () {
			if progs.contains_key (& prog.name) {
				return Err (format! ("Prog name duplicated: {}", prog.name).into ());
			}
			progs.insert (prog.name.clone (), prog);
		}

		let mut parents: HashMap <InpStr, InpStr> = HashMap::new ();
		for prog in input.progs.iter () {
			for held_name in prog.holds.iter () {
				if ! progs.contains_key (held_name) {
					return Err (format! ("Prog {} holds invalid prog {held_name}", prog.name).into ());
				}
				if parents.contains_key (held_name) {
					return Err (format! ("Prog is held multiple times: {held_name}").into ());
				}
				parents.insert (held_name.clone (), prog.name.clone ());
			}
		}

		let root = input.progs.iter ()
			.filter (|prog| ! parents.contains_key (& prog.name))
			.exactly_one ()
			.ok_or ("More than one prog which is not held by another") ?;

		Ok (Self::build_recursive (& progs, & root.name))

	}

	fn build_recursive (
		progs: & HashMap <InpStr <'inp>, & Prog <'inp>>,
		name: & InpStr <'inp>,
	) -> ProgInfo <'inp> {

		let prog = progs [name];

		let holds: Vec <Self> =
			prog.holds.iter ()
				.map (|held_name| Self::build_recursive (progs, held_name))
				.collect ();
		let holds_weight: u32 =
			holds.iter ()
				.map (|held| held.total_weight)
				.sum ();

		Self {
			name: name.clone (),
			holds,
			prog_weight: prog.weight,
			holds_weight,
			total_weight: prog.weight + holds_weight,
		}

	}

	pub fn write_input (& self, dest: & mut Vec <Prog <'inp>>) {

		dest.push (Prog {
			name: self.name.clone (),
			weight: self.prog_weight,
			holds: self.holds.iter ().map (|held| held.name.clone ()).collect (),
		});

		for held in self.holds.iter () {
			held.write_input (dest);
		}

	}

}
