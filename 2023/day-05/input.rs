use super::*;

#[ derive (Clone, Debug) ]
pub struct Input {
	pub seeds: Vec <u32>,
	pub seed_to_soil: Vec <InputMap>,
	pub soil_to_fertilizer: Vec <InputMap>,
	pub fertilizer_to_water: Vec <InputMap>,
	pub water_to_light: Vec <InputMap>,
	pub light_to_temperature: Vec <InputMap>,
	pub temperature_to_humidity: Vec <InputMap>,
	pub humidity_to_location: Vec <InputMap>,
	pub params: InputParams,
}

struct_parser_display! {
	Input {
		seeds,
		seed_to_soil,
		soil_to_fertilizer,
		fertilizer_to_water,
		water_to_light,
		light_to_temperature,
		temperature_to_humidity,
		humidity_to_location,
		params,
	} = [
		params,
		"seeds: ", @delim " " seeds, "\n",
		"\n",
		"seed-to-soil map:\n",
		@lines seed_to_soil, "\n",
		"\n",
		"soil-to-fertilizer map:\n",
		@lines soil_to_fertilizer, "\n",
		"\n",
		"fertilizer-to-water map:\n",
		@lines fertilizer_to_water, "\n",
		"\n",
		"water-to-light map:\n",
		@lines water_to_light, "\n",
		"\n",
		"light-to-temperature map:\n",
		@lines light_to_temperature, "\n",
		"\n",
		"temperature-to-humidity map:\n",
		@lines temperature_to_humidity, "\n",
		"\n",
		"humidity-to-location map:\n",
		@lines humidity_to_location,
	]
}

#[ derive (Clone, Debug) ]
pub struct InputMap {
	pub dest: u32,
	pub src: u32,
	pub len: u32,
}

struct_parser_display! {
	InputMap { dest, src, len } = [ dest, " ", src, " ", len ]
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
	}
}
