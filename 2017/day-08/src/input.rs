use super::*;
use cpu::Instr;

#[ derive (Clone, Debug) ]
pub struct Input <'inp> {
	pub instrs: Vec <Instr <'inp>>,
}

impl <'inp> Input <'inp> {

	pub fn parse (input: & [& 'inp str]) -> GenResult <Self> {
		let instrs: Vec <Instr <'inp>> = input.iter ()
			.enumerate ()
			.map (|(line_idx, line)| Parser::wrap (line, |parser| {
				parser
					.set_ignore_whitespace (true)
					.set_word_pred (|ch| ch.is_ascii_lowercase ());
				parser.item ()
			}).map_parse_err (|col_idx|
				format! ("Invalid input: line {}: col {}: {}",
					line_idx + 1, col_idx + 1, line)))
			.collect::<GenResult <_>> () ?;
		Ok (Self { instrs })
	}

	pub fn write_str (& self, writer: & mut dyn fmt::Write) -> fmt::Result {
		for instr in self.instrs.iter () {
			write! (writer, "{}\n", instr) ?;
		}
		Ok (())
	}

}
