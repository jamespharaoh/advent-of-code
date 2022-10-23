use super::*;

use input::Input;
use model::Grid;
use model::GridInner;
use model::Either;
use model::Pos;
use model::Region;

pub fn part_one (input: & Input) -> GenResult <u32> {
	if 160 < input.grid.size ().y || 160 < input.grid.size ().x {
		return Err ("Maximum grid size is 160×160".into ());
	}
	let mut num_iters = 0;
	let mut prev_grid = input.grid.clone ();
	loop {
		if num_iters == input.params.max_iters {
			return Err ("Giving up after max iterations".into ());
		}
		let next_grid = move_both (& prev_grid);
		num_iters += 1;
		if prev_grid == next_grid { break }
		prev_grid = next_grid;
	}
	Ok (num_iters)
}

fn move_both (grid: & Grid) -> Grid {
	let size = grid.size ();
	let iter_row = |y| iter::empty ()
		.chain (iter::once (grid.get (Pos { y, x: size.x - 1 }).unwrap ()))
		.chain (grid.values ()
			.skip (size.x.pan_usize () * y.pan_usize ())
			.take (size.x.pan_usize ()))
		.chain (iter::once (grid.get (Pos { y, x: 0 }).unwrap ()))
		.collect::<Vec <Region>> ();
	let data =
		iter::once (iter_row (size.y - 1))
			.chain ((0 .. size.y).map (iter_row))
			.chain (iter::once (iter_row (0)))
			.scan ((Rc::new (Vec::new ()), Rc::new (Vec::new ())),
				move |rows, row| {
					let row_0 = Rc::clone (& rows.0);
					let row_1 = Rc::clone (& rows.1);
					let row_2 = Rc::new (row);
					* rows = (Rc::clone (& row_1), Rc::clone (& row_2));
					if row_0.len () == 0 || row_1.len () == 0 {
						return Some (Either::Left (iter::empty ()));
					}
					Some (Either::Right (
						(0 .. size.x.pan_usize ()).map (move |idx|
							calc_one_region (
								row_0 [idx .. idx + 3].try_into ().unwrap (),
								row_1 [idx .. idx + 3].try_into ().unwrap (),
								row_2 [idx .. idx + 3].try_into ().unwrap (),
							)
						)
					))
				})
			.flatten ()
			.collect::<GridInner> ();
	Grid::wrap_size (data, size)
}

const fn calc_one_region (
	above: [Region; 3],
	level: [Region; 3],
	below: [Region; 3],
) -> Region {
	use Region::{ Empty as X, East as E, South as S };
	#[ allow (clippy::unnested_or_patterns) ]
	match (above, level, below) {
		([_, _, _], [E, X, _], [_, _, _]) => E,
		([_, S, _], [_, E, X], [_, _, _])
			| ([_, S, _], [_, X, _], [_, _, _])
			| ([_, _, _], [_, S, _], [E, X, _]) => S,
		([_, _, _], [_, E, X], [_, _, _])
			| ([_, _, _], [_, S, _], [_, E, X])
			| ([_, _, _], [_, S, _], [_, X, _]) => X,
		([_, _, _], [_, h, _], [_, _, _]) => h,
	}
}

#[ cfg (test) ]
mod tests {

	use super::*;

	use model::Region::{ East, South };

	fn test_sequence <StepFn: Fn (& Grid) -> Grid> (
		step_fn: StepFn,
		lines: & [& [& str]],
	) -> GenResult <()> {
		let input = Input::parse_from_lines (lines [0]) ?;
		let mut grid = input.grid.clone ();
		let count = |grid: & Grid, region|
			grid.values ()
				.filter (|& other_region| region == other_region)
				.count ();
		let num_east = count (& grid, East);
		let num_south = count (& grid, South);
		for expect_lines in lines.iter ().skip (1) {
			let expect_input = Input::parse_from_lines (expect_lines) ?;
			grid = step_fn (& grid);
			assert_eq! (expect_input.grid, grid);
			assert_eq! (num_east, count (& grid, East));
			assert_eq! (num_south, count (& grid, South));
		}
		Ok (())
	}

	#[ test ]
	fn test_complex () -> GenResult <()> {
		test_sequence (move_both, & [
			& [ "v...>>.vv>", ".vv>>.vv..", ">>.>v>...v", ">>v>>.>.v.", "v>v.vv.v..",
				">.>>..v...", ".vv..>.>v.", "v.v..>>v.v", "....v..v.>" ],
			& [ "....>.>v.>", "v.v>.>v.v.", ">v>>..>v..", ">>v>v>.>.v", ".>v.v...v.",
				"v>>.>vvv..", "..v...>>..", "vv...>>vv.", ">.v.v..v.v" ],
		])
	}

	#[ test ]
	fn test_east () -> GenResult <()> {
		test_sequence (move_both, & [
			& ["...>>>>>..."], & ["...>>>>.>.."], & ["...>>>.>.>."], & ["...>>.>.>.>"],
		])
	}

	#[ test ]
	fn test_both () -> GenResult <()> {
		test_sequence (move_both, & [
			& ["..........", ".>v....v..", ".......>..", ".........."],
			& ["..........", ".>........", "..v....v>.", ".........."],
			& ["..........", "..>.......", ".........>", "..v....v.."],
		])
	}

}
