use super::*;

use model::Coord;
use model::Pos;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub sensors: Vec <Sensor>,
	pub params: InputParams,
}

struct_parser_display! {
	Input { sensors, params } = [ params, @lines sensors ]
}

#[ derive (Clone, Debug) ]
pub struct Sensor {
	pub sensor: Pos,
	pub beacon: Pos,
}

struct_parser_display! {
	Sensor {
		sensor: Pos { y: sensor_y, x: sensor_x },
		beacon: Pos { y: beacon_y, x: beacon_x },
	} = [
		"Sensor at x=", sensor_x, ", y=", sensor_y, ": ",
		"closest beacon is at x=", beacon_x, ", y=", beacon_y,
	]
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
		pub check_row: Coord = ("CHECK_ROW=", 2_000_000, .. ),
		pub search_size: Coord = ("SEARCH_SIZE=", 4_000_000, .. ),
	}
}
