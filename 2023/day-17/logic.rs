use super::*;

use input::Input;
use model::Dir;
use model::Pos;

pub fn part_one (input: & Input) -> GenResult <u32> {
	calc_result (input, 0, 3)
}

pub fn part_two (input: & Input) -> GenResult <u32> {
	calc_result (input, 4, 10)
}

fn calc_result (input: & Input, min_steps: u8, max_steps: u8) -> GenResult <u32> {
	let grid = & input.grid;
	if input.params.max_size < grid.size ().y || input.params.max_size < grid.size ().x {
		return Err ("Grid must not be bigger than 150×150".into ());
	}
	let mut search = PrioritySearch::with_hash_map (
			|(old_pos, old_dir, old_steps), pri, mut adder: PrioritySearchAdder <_, _, _>| {
		for new_dir in [ Dir::Up, Dir::Down, Dir::Left, Dir::Right ] {
			if let Some (old_dir) = old_dir {
				if new_dir.around () == old_dir { continue }
				if new_dir == old_dir && old_steps == max_steps { continue }
				if new_dir != old_dir && old_steps < min_steps { continue }
			}
			let new_pos = old_pos + Pos::from (new_dir);
			let Some (cost) = grid.get (new_pos) else { continue };
			let new_steps = if Some (new_dir) == old_dir { old_steps + 1 } else { 1 };
			adder.add ((new_pos, Some (new_dir), new_steps), pri + cost.pan_u16 ());
		}
		(old_pos, old_steps, pri)
	});
	search.push ((Pos::ZERO, None, 0), 0_u16);
	let end = Pos::new (grid.end ().y - 1, grid.end ().x - 1);
	let mut num_iters = 0;
	for (pos, steps, heat_loss) in search {
		if pos == end && min_steps <= steps {
			return Ok (heat_loss.pan_u32 ());
		}
		num_iters += 1;
		if num_iters == input.params.max_iters {
			return Err ("Max iterations exceeded".into ());
		}
	}
	Err ("No solution found".into ())
}
