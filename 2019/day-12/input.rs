use super::*;
use model::Pos;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub moons: Vec <Pos>,
	pub params: InputParams,
}

struct_display! {
	Input { moons, params } = [ params, @lines moons = display_moon ]
}

fn display_moon (moon: & Pos, formatter: & mut fmt::Formatter) -> fmt::Result {
	write! (formatter, "<x={x}, y={y}, z={z}>", x = moon.x, y = moon.y, z = moon.z)
}

struct_parser! {
	Input { moons, params } = [ params, @lines moons = parse_moon ]
}

fn parse_moon (parser: & mut Parser) -> ParseResult <Pos> {
	parse! (parser, "<x=", x, ", y=", y, ", z=", z, ">");
	Ok (Pos { x, y, z })
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
		pub num_steps_one: u32 = ("NUM_STEPS_ONE=", 1000, 1_u32 .. ),
		pub num_steps_two: u32 = ("NUM_STEPS_TWO=", 500_000, 1_u32 .. ),
	}
}
