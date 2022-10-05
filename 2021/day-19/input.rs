use super::*;

use model::Pos;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub scanners: Vec <InputScanner>,
	pub params: InputParams,
}

struct_parser_display! {
	Input { scanners, params } = [ params, @delim "\n\n" scanners ]
}

#[ derive (Clone, Debug) ]
pub struct InputScanner {
	pub id: u16,
	pub beacons: Vec <Pos>,
}

struct_parser_display! {
	InputScanner { id, beacons } = [
		"--- scanner ", id, " ---\n",
		@lines beacons { Pos { x, y, z } = [ x, ",", y, ",", z ] },
	]
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
	}
}
