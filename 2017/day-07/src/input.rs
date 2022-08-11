use super::*;
use parser::*;

#[ derive (Clone, Debug) ]
pub struct Input <'inp> {
	pub progs: Vec <Prog <'inp>>,
}

#[ derive (Clone, Debug, Eq, PartialEq) ]
pub struct Prog <'inp> {
	pub name: InpStr <'inp>,
	pub weight: u32,
	pub holds: ArrayVec <InpStr <'inp>, 7>,
}

impl <'inp> Input <'inp> {

	pub fn parse (input: & [& 'inp str]) -> GenResult <Self> {
		let progs: Vec <Prog <'inp>> = input.iter ()
			.enumerate ()
			.map (|(line_idx, line)| Parser::wrap (line, |parser| {
				parser.set_word_pred (|ch| ch.is_ascii_lowercase ());
				parser.item ()
			}).map_parse_err (|col_idx|
				format! ("Invalid input: line {}: col {}: {}",
					line_idx + 1, col_idx + 1, line)))
			.collect::<GenResult <_>> () ?;
		Ok (Self { progs })
	}

	pub fn write_str (& self, writer: & mut dyn fmt::Write) -> fmt::Result {
		for prog in self.progs.iter () {
			prog.write (writer) ?;
			write! (writer, "\n") ?;
		}
		Ok (())
	}

}

impl <'inp> Prog <'inp> {
	pub fn write (& self, writer: & mut dyn fmt::Write) -> fmt::Result {
		write! (writer, "{} ({})", self.name, self.weight) ?;
		if ! self.holds.is_empty () {
			write! (writer, " -> {}", self.holds [0]) ?;
			for hold in self.holds.iter ().skip (1) {
				write! (writer, ", {}", hold) ?;
			}
		}
		Ok (())
	}
}

impl <'inp> FromParser <'inp> for Prog <'inp> {
	fn from_parser (parser: & mut Parser <'inp>) -> ParseResult <Self> {
		let name = parser.word () ?;
		let weight = parser.expect (" (") ?.uint () ?;
		parser.expect (")") ?;
		let holds = if ! parser.is_empty () {
			let mut holds = ArrayVec::new ();
			holds.push (InpStr::borrow (parser.expect (" -> ") ?.word () ?));
			while parser.peek () == Some (',') {
				if holds.is_full () { return Err ("Prog can hold max seven others".into ()) }
				holds.push (InpStr::borrow (parser.expect (", ") ?.word () ?));
			}
			holds
		} else { ArrayVec::new () };
		parser.end () ?;
		Ok (Self {
			name: InpStr::borrow (name),
			weight,
			holds,
		})
	}
}
