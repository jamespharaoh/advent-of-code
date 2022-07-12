use aoc_common::*;

puzzle_info! {
	name = "Sea Cucumber";
	year = 2021;
	day = 25;
	part_one = |lines| logic::calc_result (lines);
}

mod logic {

	use super::*;
	use model::Region;
	use model::Seafloor;

	pub fn calc_result (lines: & [& str]) -> GenResult <i64> {
		let seafloor = Seafloor::parse (lines) ?;
		let mut count = 0;
		let mut prev_seafloor = seafloor;
		loop {
			let next_seafloor = move_both (& prev_seafloor);
			count += 1;
			if prev_seafloor == next_seafloor { break }
			prev_seafloor = next_seafloor;
		}
		Ok (count)
	}

	pub fn move_both (seafloor: & Seafloor) -> Seafloor {
		move_south (& move_east (seafloor))
	}

	pub fn move_east (seafloor: & Seafloor) -> Seafloor {
		let (rows, cols) = seafloor.size ();
		Seafloor::new ((rows, cols), seafloor.iter ().map (|((row, col), here)| {
			let west = seafloor.get ((row, (col + cols - 1) % cols));
			let east = seafloor.get ((row, (col + 1) % cols));
			match (west, here, east) {
				(Region::East, Region::Empty, _) => Region::East,
				(_, Region::East, Region::Empty) => Region::Empty,
				(_, _, _) => here,
			}
		}))
	}

	pub fn move_south (seafloor: & Seafloor) -> Seafloor {
		let (rows, cols) = seafloor.size ();
		Seafloor::new ((rows, cols), seafloor.iter ().map (|((row, col), here)| {
			let north = seafloor.get (((row + rows - 1) % rows, col));
			let south = seafloor.get (((row + 1) % rows, col));
			match (north, here, south) {
				(Region::South, Region::Empty, _) => Region::South,
				(_, Region::South, Region::Empty) => Region::Empty,
				(_, _, _) => here,
			}
		}))
	}

	#[ cfg (test) ]
	fn test_sequence <StepFn: Fn (& Seafloor) -> Seafloor> (
		step_fn: StepFn,
		seafloor_strs: & [& [& str]],
	) -> GenResult <()> {
		let mut seafloor = Seafloor::parse (seafloor_strs [0]) ?;
		for expect_str in seafloor_strs.iter ().skip (1) {
			let expect = Seafloor::parse (expect_str) ?;
			seafloor = step_fn (& seafloor);
			assert_eq! (expect, seafloor);
		}
		Ok (())
	}

	#[ test ]
	fn test_move_east () -> GenResult <()> {
		test_sequence (move_east, & [
			& ["...>>>>>..."],
			& ["...>>>>.>.."],
			& ["...>>>.>.>."],
			& ["...>>.>.>.>"],
		])
	}

	#[ test ]
	fn test_move_both () -> GenResult <()> {
		test_sequence (move_both, & [
			& ["..........", ".>v....v..", ".......>..", ".........."],
			& ["..........", ".>........", "..v....v>.", ".........."],
			& ["..........", "..>.......", ".........>", "..v....v.."],
		])
	}

}

mod model {

	use super::*;

	#[ derive (Clone, Eq, PartialEq) ]
	pub struct Seafloor {
		size: (usize, usize),
		data: Vec <Region>,
	}

	impl Seafloor {
		pub fn new <IntoIter> (size: (usize, usize), iter: IntoIter) -> Seafloor
				where IntoIter: IntoIterator <Item = Region> {
			let mut iter = iter.into_iter ();
			let data: Vec <_> = (& mut iter).take (size.0 * size.1).collect ();
			if data.len () < size.0 * size.1 || iter.next ().is_some () { panic! () }
			Seafloor { size, data }
		}
		pub fn size (& self) -> (usize, usize) { self.size }
		pub fn parse (lines: & [& str]) -> GenResult <Seafloor> {
			if lines.is_empty () { Err (format! ("Invalid input")) ? }
			let size = (lines.len (), lines [0].chars ().count ());
			let data = lines.iter ().enumerate ().map (|(line_idx, line)| {
				let line_err = move || format! ("Invalid input: {}: {}", line_idx, line);
				if line.chars ().count () != size.1 { Err (line_err ()) ? }
				Ok (line.chars ().map (move |letter| Ok (match letter {
					'.' => Region::Empty,
					'>' => Region::East,
					'v' => Region::South,
					_ => Err (line_err ()) ?,
				})))
			}).flatten_ok ().collect::<GenResult <GenResult <_>>> () ? ?;
			Ok (Seafloor { size, data })
		}
		pub fn get (& self, pos: (usize, usize)) -> Region {
			if ! (0 .. self.size.0).contains (& pos.0) { panic! () }
			if ! (0 .. self.size.1).contains (& pos.1) { panic! () }
			self.data [pos.0 * self.size.1 + pos.1]
		}
		pub fn iter <'a> (& 'a self) -> impl Iterator <Item = ((usize, usize), Region)> + 'a {
			self.data.iter ().scan ((0, 0), |pos, val| {
				let ret = (* pos, * val);
				pos.1 += 1;
				if pos.1 == self.size.1 {
					pos.1 = 0;
					pos.0 += 1;
				}
				Some (ret)
			})
		}
	}

	impl fmt::Debug for Seafloor {
		fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
			write! (formatter, "Seafloor {{ {:?}, ", self.size) ?;
			for (pos, region) in self.iter () {
				if pos.1 == 0 && pos.0 > 0 { write! (formatter, ", ") ?; }
				if pos.1 == 0 { write! (formatter, "\"") ?; }
				write! (formatter, "{}", region) ?;
				if pos.1 + 1 == self.size.1 { write! (formatter, "\"") ?; }
			}
			write! (formatter, " }}") ?;
			Ok (())
		}
	}

	#[ derive (Clone, Copy, Debug, Eq, PartialEq) ]
	pub enum Region {
		Empty,
		East,
		South,
	}

	impl fmt::Display for Region {
		fn fmt (& self, formatter: & mut fmt::Formatter) -> fmt::Result {
			write! (formatter, "{}", match * self {
				Region::Empty => '.',
				Region::East => '>',
				Region::South => 'v',
			})
		}
	}

}

#[ cfg (test) ]
mod examples {

	use super::*;

	const EXAMPLE_0: & 'static [& 'static str] = & [
		"v...>>.vv>",
		".vv>>.vv..",
		">>.>v>...v",
		">>v>>.>.v.",
		"v>v.vv.v..",
		">.>>..v...",
		".vv..>.>v.",
		"v.v..>>v.v",
		"....v..v.>",
	];

	#[ test ]
	fn test_example_0 () -> GenResult <()> {
		assert_eq! (58, logic::calc_result (EXAMPLE_0) ?);
		Ok (())
	}

}
