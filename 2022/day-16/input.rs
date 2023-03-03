use super::*;

#[ derive (Clone, Debug) ]
pub struct Input <'inp> {
	pub valves: Vec <InputValve <'inp>>,
	pub params: InputParams,
}

struct_parser_display! {
	input_lifetime = 'inp;
	Input <'inp> { valves, params } = [ params, @lines valves ]
}

#[ derive (Clone, Debug) ]
pub struct InputValve <'inp> {
	pub name: InpStr <'inp>,
	pub flow_rate: u8,
	pub tunnels: Vec <InpStr <'inp>>,
}

struct_parser_display! {
	input_lifetime = 'inp;
	InputValve <'inp> { name, flow_rate, tunnels } = [
		"Valve ", @str name = (|ch| { ch.is_ascii_uppercase () }, 1 ..= 2), " ",
		"has flow rate=", flow_rate, "; ",
		tunnels {
			type = Vec <InpStr <'inp>>;
			tunnels if (tunnels.len () == 1) = [
				"tunnel leads to valve ", @delim ", " tunnels {
					tunnel = [ @str tunnel = (|ch| { ch.is_ascii_uppercase () }, 1 ..= 2) ],
				},
			],
			tunnels = [
				"tunnels lead to valves ", @delim ", " tunnels {
					tunnel = [ @str tunnel = (|ch| { ch.is_ascii_uppercase () }, 1 ..= 2) ],
				},
			],
		},
	]
}

input_params! {
	#[ derive (Clone, Debug) ]
	pub struct InputParams {
		pub max_iters: u32 = ("MAX_ITERS=", 500_000, 1 .. ),
		pub max_plans: u32 = ("MAX_PLANS=", 20_000, 1 .. ),
	}
}
