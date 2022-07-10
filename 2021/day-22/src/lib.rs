use aoc_common::*;

puzzle! {
	name = "Reactor Reboot";
	year = 2021;
	day = 22;
	part_one = |lines| logic::calc_result_part_one (lines);
	part_two = |lines| logic::calc_result_part_two (lines);
}

mod logic {

	use super::*;
	use model::Cube;
	use model::Step;
	use rle::Rle;

	const BOUND_50: Cube = Cube { x0: -50, y0: -50, z0: -50, x1: 51, y1: 51, z1: 51 };
	const BOUND_NONE: Cube = Cube { x0: i32::MIN, y0: i32::MIN, z0: i32::MIN, x1: i32::MAX, y1: i32::MAX, z1: i32::MAX };

	pub fn calc_result_part_one (lines: & [& str]) -> GenResult <i64> {
		calc_result (lines, BOUND_50)
	}

	pub fn calc_result_part_two (lines: & [& str]) -> GenResult <i64> {
		calc_result (lines, BOUND_NONE)
	}

	pub fn calc_result (lines: & [& str], bound: Cube) -> GenResult <i64> {
		type Core = Rle <Rc <CoreNest0>, i32>;
		type CoreNest0 = Rle <Rc <CoreNest1>, i32>;
		type CoreNest1 = Rle <bool, i32>;
		let input = model::parse_input (lines) ?;
		let steps: Vec <Step> = input.iter ().cloned ().filter_map (
			|mut step| step.cube.intersect (bound).map (|cube| { step.cube = cube; step }),
		).collect ();
		let mut core: Core = Rle::new (Rc::new (Rle::new (Rc::new (Rle::new (false)))));
		for step in steps.iter () {
			core = core.with_update (step.cube.z0, step.cube.z1, |_, _, rle_0| {
				rle_0.with_update (step.cube.y0, step.cube.y1, |_, _, rle_1| {
					rle_1.with_update (step.cube.x0, step.cube.x1, |_, _, _| {
						step.state
					}).into ()
				}).into ()
			});
		}
		let num_active: usize = core.iter ().map (
			|(start_0, end_0, val_0)| val_0.iter ().map (
				|(start_1, end_1, val_1)| val_1.iter ().map (
					|& (start_2, end_2, val_2)| if val_2 { (end_2 - start_2) as usize } else { 0 },
				).sum::<usize> () * (end_1 - start_1) as usize,
			).sum::<usize> () * (end_0 - start_0) as usize,
		).sum::<usize> ();
		Ok (num_active as i64)
	}

}

mod model {

	use super::*;

	pub fn parse_input (lines: & [& str]) -> GenResult <Vec <Step>> {
		let err = || format! ("Invalid input");
		lines.iter ().map (|line| {
			let mut line_iter = line.split (" ");
			let state = match line_iter.next ().ok_or_else (err) ? {
				"on" => true,
				"off" => false,
				_ => Err (err ()) ?,
			};
			let mut coords_iter = line_iter.next ().ok_or_else (err) ?.split (",");
			let parse_coord = |prefix: & str, input: & str| -> GenResult <(i32, i32)> {
				if ! input.starts_with (prefix) { Err (err ()) ? }
				let mut input_iter = input [2 .. ].split ("..");
				let val_0 = input_iter.next ().ok_or_else (err) ?.parse ().map_err (|_| err ()) ?;
				let val_1 = input_iter.next ().ok_or_else (err) ?.parse ().map_err (|_| err ()) ?;
				Ok ((val_0, val_1))
			};
			let x_range = parse_coord ("x=", coords_iter.next ().ok_or_else (err) ?) ?;
			let y_range = parse_coord ("y=", coords_iter.next ().ok_or_else (err) ?) ?;
			let z_range = parse_coord ("z=", coords_iter.next ().ok_or_else (err) ?) ?;
			if coords_iter.next ().is_some () { Err (err ()) ? }
			if line_iter.next ().is_some () { Err (err ()) ? }
			Ok (Step {
				state,
				cube: Cube {
					x0: x_range.0, x1: x_range.1 + 1,
					y0: y_range.0, y1: y_range.1 + 1,
					z0: z_range.0, z1: z_range.1 + 1,
				},
			})
		}).collect ()
	}

	#[ derive (Clone, Copy, Debug) ]
	pub struct Step {
		pub state: bool,
		pub cube: Cube,
	}

	#[ derive (Clone, Copy, Debug, Eq, Hash, PartialEq) ]
	pub struct Cube {
		pub x0: i32, pub x1: i32,
		pub y0: i32, pub y1: i32,
		pub z0: i32, pub z1: i32,
	}

	impl Cube {
		pub fn overlaps (self, other: Cube) -> bool {
			self.x0 < other.x1 && other.x0 < self.x1
				&& self.y0 < other.y1 && other.y0 < self.y1
				&& self.z0 < other.z1 && other.z0 < self.z1
		}
		pub fn intersect (self, other: Cube) -> Option <Cube> {
			if self.overlaps (other) {
				Some (Cube {
					x0: cmp::max (self.x0, other.x0),
					x1: cmp::min (self.x1, other.x1),
					y0: cmp::max (self.y0, other.y0),
					y1: cmp::min (self.y1, other.y1),
					z0: cmp::max (self.z0, other.z0),
					z1: cmp::min (self.z1, other.z1),
				})
			} else { None }
		}
	}

	#[ derive (Clone, Copy, Eq, Hash, PartialEq) ]
	pub struct Pos { pub x: i32, pub y: i32, pub z: i32 }

}

mod rle {

	use super::*;

	#[ derive (Clone, Debug, Eq, PartialEq) ]
	pub struct Rle <Val: Clone + fmt::Debug, Idx: Copy + fmt::Display + Ord> {
		data: Vec <(Idx, Idx, Val)>,
		rest: Val,
	}

	impl <Val: Clone + fmt::Debug + Eq, Idx: Copy + fmt::Display + Ord> Rle <Val, Idx> {

		pub fn new (rest: Val) -> Rle <Val, Idx> {
			let data = Vec::new ();
			Rle { data, rest }
		}

		pub fn iter <'a> (& 'a self) -> SliceIter <'a, (Idx, Idx, Val)> {
			self.data.iter ()
		}

		pub fn with_update <UpdateFn> (
			& self,
			new_start: Idx,
			new_end: Idx,
			update_fn: UpdateFn,
		) -> Self
				where UpdateFn: Fn (Idx, Idx, Val) -> Val {
			assert! (new_start < new_end);
			let mut new_start = new_start;
			let mut data_iter = self.data.iter ().cloned ();
			let mut data_item = None;
			let mut data = Vec::new ();
			let rest = self.rest.clone ();
			loop {
				if data_item.is_none () { data_item = data_iter.next (); }
				if data_item.is_none () && new_start == new_end { break; }
				let next;
				if let Some ((mut item_start, item_end, item_val)) = data_item.take () {
					if item_end <= new_start || new_start == new_end {
						next = (item_start, item_end, item_val.clone ());
						item_start = item_end;
					} else if item_start < new_start {
						let update_end = cmp::min (item_end, new_start);
						next = (item_start, update_end, item_val.clone ());
						item_start = update_end;
					} else if item_start == new_start && item_start < item_end {
						let update_end = cmp::min (item_end, new_end);
						let update_val = update_fn (item_start, update_end, item_val.clone ());
						next = (item_start, update_end, update_val);
						item_start = update_end;
						new_start = update_end;
					} else if new_start < item_start {
						let update_end = cmp::min (new_end, item_start);
						let update_val = update_fn (new_start, update_end, rest.clone ());
						next = (new_start, update_end, update_val);
						new_start = update_end;
					} else { unreachable! (); }
					if item_start != item_end {
						data_item = Some ((item_start, item_end, item_val));
					}
				} else if new_start < new_end {
					let update_val = update_fn (new_start, new_end, rest.clone ());
					next = (new_start, new_end, update_val);
					new_start = new_end;
				} else { unreachable! (); }
				data.push (next);
				if data.len () >= 1 {
					let (_, _, val) = & data [data.len () - 1];
					if val == & rest {
						data.pop ().unwrap ();
					}
				}
				if data.len () >= 2 {
					let data_len = data.len ();
					let (_, end_0, val_0) = & data [data_len - 2];
					let (start_1, end_1, val_1) = & data [data_len - 1];
					if end_0 == start_1 && val_0 == val_1 {
						let end_1 = * end_1;
						let (_, end_0, _) = & mut data [data_len - 2];
						* end_0 = end_1;
						data.pop ().unwrap ();
					}
				}
			}
			Rle { data, rest }
		}

	}

}

#[ cfg (test) ]
mod examples {

	use super::*;

	const EXAMPLE_0: & [& str] = & [
		"on x=10..12,y=10..12,z=10..12",
		"on x=11..13,y=11..13,z=11..13",
		"off x=9..11,y=9..11,z=9..11",
		"on x=10..10,y=10..10,z=10..10",
	];

	const EXAMPLE_1: & [& str] = & [
		"on x=-20..26,y=-36..17,z=-47..7",
		"on x=-20..33,y=-21..23,z=-26..28",
		"on x=-22..28,y=-29..23,z=-38..16",
		"on x=-46..7,y=-6..46,z=-50..-1",
		"on x=-49..1,y=-3..46,z=-24..28",
		"on x=2..47,y=-22..22,z=-23..27",
		"on x=-27..23,y=-28..26,z=-21..29",
		"on x=-39..5,y=-6..47,z=-3..44",
		"on x=-30..21,y=-8..43,z=-13..34",
		"on x=-22..26,y=-27..20,z=-29..19",
		"off x=-48..-32,y=26..41,z=-47..-37",
		"on x=-12..35,y=6..50,z=-50..-2",
		"off x=-48..-32,y=-32..-16,z=-15..-5",
		"on x=-18..26,y=-33..15,z=-7..46",
		"off x=-40..-22,y=-38..-28,z=23..41",
		"on x=-16..35,y=-41..10,z=-47..6",
		"off x=-32..-23,y=11..30,z=-14..3",
		"on x=-49..-5,y=-3..45,z=-29..18",
		"off x=18..30,y=-20..-8,z=-3..13",
		"on x=-41..9,y=-7..43,z=-33..15",
		"on x=-54112..-39298,y=-85059..-49293,z=-27449..7877",
		"on x=967..23432,y=45373..81175,z=27513..53682",
	];

	const EXAMPLE_2: & [& str] = & [
		"on x=-5..47,y=-31..22,z=-19..33",
		"on x=-44..5,y=-27..21,z=-14..35",
		"on x=-49..-1,y=-11..42,z=-10..38",
		"on x=-20..34,y=-40..6,z=-44..1",
		"off x=26..39,y=40..50,z=-2..11",
		"on x=-41..5,y=-41..6,z=-36..8",
		"off x=-43..-33,y=-45..-28,z=7..25",
		"on x=-33..15,y=-32..19,z=-34..11",
		"off x=35..47,y=-46..-34,z=-11..5",
		"on x=-14..36,y=-6..44,z=-16..29",
		"on x=-57795..-6158,y=29564..72030,z=20435..90618",
		"on x=36731..105352,y=-21140..28532,z=16094..90401",
		"on x=30999..107136,y=-53464..15513,z=8553..71215",
		"on x=13528..83982,y=-99403..-27377,z=-24141..23996",
		"on x=-72682..-12347,y=18159..111354,z=7391..80950",
		"on x=-1060..80757,y=-65301..-20884,z=-103788..-16709",
		"on x=-83015..-9461,y=-72160..-8347,z=-81239..-26856",
		"on x=-52752..22273,y=-49450..9096,z=54442..119054",
		"on x=-29982..40483,y=-108474..-28371,z=-24328..38471",
		"on x=-4958..62750,y=40422..118853,z=-7672..65583",
		"on x=55694..108686,y=-43367..46958,z=-26781..48729",
		"on x=-98497..-18186,y=-63569..3412,z=1232..88485",
		"on x=-726..56291,y=-62629..13224,z=18033..85226",
		"on x=-110886..-34664,y=-81338..-8658,z=8914..63723",
		"on x=-55829..24974,y=-16897..54165,z=-121762..-28058",
		"on x=-65152..-11147,y=22489..91432,z=-58782..1780",
		"on x=-120100..-32970,y=-46592..27473,z=-11695..61039",
		"on x=-18631..37533,y=-124565..-50804,z=-35667..28308",
		"on x=-57817..18248,y=49321..117703,z=5745..55881",
		"on x=14781..98692,y=-1341..70827,z=15753..70151",
		"on x=-34419..55919,y=-19626..40991,z=39015..114138",
		"on x=-60785..11593,y=-56135..2999,z=-95368..-26915",
		"on x=-32178..58085,y=17647..101866,z=-91405..-8878",
		"on x=-53655..12091,y=50097..105568,z=-75335..-4862",
		"on x=-111166..-40997,y=-71714..2688,z=5609..50954",
		"on x=-16602..70118,y=-98693..-44401,z=5197..76897",
		"on x=16383..101554,y=4615..83635,z=-44907..18747",
		"off x=-95822..-15171,y=-19987..48940,z=10804..104439",
		"on x=-89813..-14614,y=16069..88491,z=-3297..45228",
		"on x=41075..99376,y=-20427..49978,z=-52012..13762",
		"on x=-21330..50085,y=-17944..62733,z=-112280..-30197",
		"on x=-16478..35915,y=36008..118594,z=-7885..47086",
		"off x=-98156..-27851,y=-49952..43171,z=-99005..-8456",
		"off x=2032..69770,y=-71013..4824,z=7471..94418",
		"on x=43670..120875,y=-42068..12382,z=-24787..38892",
		"off x=37514..111226,y=-45862..25743,z=-16714..54663",
		"off x=25699..97951,y=-30668..59918,z=-15349..69697",
		"off x=-44271..17935,y=-9516..60759,z=49131..112598",
		"on x=-61695..-5813,y=40978..94975,z=8655..80240",
		"off x=-101086..-9439,y=-7088..67543,z=33935..83858",
		"off x=18020..114017,y=-48931..32606,z=21474..89843",
		"off x=-77139..10506,y=-89994..-18797,z=-80..59318",
		"off x=8476..79288,y=-75520..11602,z=-96624..-24783",
		"on x=-47488..-1262,y=24338..100707,z=16292..72967",
		"off x=-84341..13987,y=2429..92914,z=-90671..-1318",
		"off x=-37810..49457,y=-71013..-7894,z=-105357..-13188",
		"off x=-27365..46395,y=31009..98017,z=15428..76570",
		"off x=-70369..-16548,y=22648..78696,z=-1892..86821",
		"on x=-53470..21291,y=-120233..-33476,z=-44150..38147",
		"off x=-93533..-4276,y=-16170..68771,z=-104985..-24507",
	];

	#[ test ]
	fn part_one_0 () {
		assert_eq! (39, logic::calc_result_part_one (EXAMPLE_0).unwrap ());
	}

	#[ test ]
	fn part_one_1 () {
		assert_eq! (590784, logic::calc_result_part_one (EXAMPLE_1).unwrap ());
	}

	#[ test ]
	fn part_two () {
		assert_eq! (2758514936282235, logic::calc_result_part_two (EXAMPLE_2).unwrap ());
	}

}
