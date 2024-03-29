use super::*;

use model::Dir;
use model::Grid;
use model::Pos;
use model::SeenGrid;
use model::Tile;

pub struct State {
	grid: Grid,
	goblin_attack: u16,
	elf_attack: u16,
	units: Vec <(Pos, u16)>,
	num_rounds: u32,
	frozen: bool,
	walk_queue: VecDeque <(u32, Pos, Pos)>,
	seen: SeenGrid,
}

impl State {

	pub fn build (grid: Grid, goblin_attack: u16, elf_attack: u16) -> GenResult <Self> {
		let units: Vec <(Pos, u16)> = grid.iter ()
			.filter_map (|(pos, tile)| matches! (tile, Tile::Goblin | Tile::Elf)
				.then_some ((pos, 200)))
			.collect ();
		if units.is_empty () { return Err ("No combatants found".into ()) }
		if units.len () > 50 { return Err ("More than 50 combatants".into ()) }
		let seen = SeenGrid::new_range (grid.start (), grid.end ()) ?;
		Ok (Self {
			grid,
			goblin_attack,
			elf_attack,
			units,
			num_rounds: 0,
			frozen: false,
			walk_queue: VecDeque::new (),
			seen,
		})
	}

	pub fn tick (& mut self) -> bool {
		let mut unit_idx = 0;
		let mut moved = false;
		let mut attacked = false;
		let num_before = self.units.len ();
		while unit_idx < self.units.len () {
			if self.units.iter ()
					.map (|& (unit_pos, _)| self.grid.get (unit_pos).unwrap ())
					.all_equal () {
				return true;
			}
			if ! self.frozen && self.unit_move (unit_idx) { moved = true; }
			if self.unit_attack (& mut unit_idx) { attacked = true; }
			if num_before != self.units.len () { self.frozen = false; }
			unit_idx += 1;
		}
		if ! self.frozen && ! moved && num_before == self.units.len () { self.frozen = true; }
		self.units.sort_by_key (|& (unit_pos, _)| unit_pos);
		self.num_rounds += 1;
		! (moved || attacked)
	}

	fn unit_move (& mut self, unit_idx: usize) -> bool {

		let (unit_pos, unit_hp) = self.units [unit_idx];
		let unit_tile = self.grid.get (unit_pos).unwrap ();

		// don't move if we are already next to an enemy

		if unit_pos.adjacent_4 ().iter ().copied ()
				.any (|adj_pos| self.grid.get (adj_pos) == Some (unit_tile.enemy ())) {
			return false;
		}

		// start path finding, breadth first

		self.walk_queue.clear ();
		for step_dir in [ Dir::Up, Dir::Left, Dir::Right, Dir::Down ].iter ().copied () {
			let step_pos = ok_or! (unit_pos.try_add ((step_dir, 1)), continue);
			if self.grid.get (step_pos) != Some (Tile::Open) { continue }
			self.walk_queue.push_back ((1, step_pos, step_pos));
		}
		self.seen.reset ();
		let mut found: Option <(u32, Pos, Pos)> = None;
		while let Some ((dist, walk_pos, step_pos)) = self.walk_queue.pop_front () {

			// abort if we found a route with a shorter distance already

			if found.map_or (false, |(found_dist, _, _)| found_dist < dist) { break }

			// abort if this path crossing another

			if self.seen.get (walk_pos).unwrap_or (true) { continue }
			self.seen.set (walk_pos, true);

			// iterate next steps

			for adj_pos in walk_pos.adjacent_4 ().iter ().copied () {
				let adj_tile = some_or! (self.grid.get (adj_pos), continue);

				// if there is an enemy and this route is better then record it

				if adj_tile == unit_tile.enemy ()
						&& found.map_or (true, |found| (dist, walk_pos, step_pos) < found) {
					found = Some ((dist, walk_pos, step_pos));
				}

				// if we can step then iterate paths with a step in that direction

				if adj_tile == Tile::Open {
					self.walk_queue.push_back ((dist + 1, adj_pos, step_pos));
				}

			}

		}

		let (_, _, step_pos) = some_or! (found, return false);

		// move the unit

		self.grid.set (unit_pos, Tile::Open);
		self.grid.set (step_pos, unit_tile);
		self.units [unit_idx] = (step_pos, unit_hp);

		true

	}

	fn unit_attack (& mut self, unit_idx: & mut usize) -> bool{
		let (unit_pos, _) = self.units [* unit_idx];
		let unit_tile = self.grid.get (unit_pos).unwrap ();
		let mut found: Option <(Pos, u16)> = None;
		for enemy_dir in [ Dir::Up, Dir::Left, Dir::Right, Dir::Down ].iter ().copied () {
			let enemy_pos = ok_or! (unit_pos.try_add ((enemy_dir, 1)), continue);
			let enemy_tile = some_or! (self.grid.get (enemy_pos), continue);
			if enemy_tile != unit_tile.enemy () { continue }
			let enemy_hp = self.units.iter ().copied ()
				.find (|& (other_pos, _)| other_pos == enemy_pos)
				.map (|(_, other_hp)| other_hp)
				.unwrap ();
			if found.map_or (true, |(_, found_hp)| enemy_hp < found_hp) {
				found = Some ((enemy_pos, enemy_hp));
			}
		}
		let (enemy_pos, _) = some_or! (found, return false);
		let enemy_idx =
			self.units.iter ()
				.position (|& (other_pos, _)| other_pos == enemy_pos)
				.unwrap ();
		let & mut (_, ref mut enemy_hp) = & mut self.units [enemy_idx];
		let attack = match unit_tile {
			Tile::Goblin => self.goblin_attack,
			Tile::Elf => self.elf_attack,
			Tile::Open | Tile::Cavern => unreachable! (),
		};
		* enemy_hp = cmp::max (* enemy_hp, attack) - attack;
		if * enemy_hp == 0 {
			self.grid.set (enemy_pos, Tile::Open);
			self.units.remove (enemy_idx);
			if enemy_idx < * unit_idx { * unit_idx -= 1; }
		}
		true
	}

	#[ must_use ]
	pub fn score (& self) -> u32 {
		let remain_hp = self.units.iter ()
			.map (|& (_, unit_hp)| unit_hp.pan_u32 ())
			.sum::<u32> ();
		self.num_rounds * remain_hp
	}

	#[ must_use ]
	pub fn winner (& self) -> Option <Tile> {
		let unit_tile = self.grid.get (self.units [0].0).unwrap ();
		self.units.iter ().copied ()
			.map (|(unit_pos, _)| self.grid.get (unit_pos).unwrap ())
			.all (|other_tile| other_tile == unit_tile)
			.then_some (unit_tile)
	}

	#[ inline ]
	#[ must_use ]
	pub fn units (& self) -> & [(Pos, u16)] {
		& self.units
	}

	#[ inline ]
	#[ must_use ]
	pub fn num_rounds (& self) -> u32 {
		self.num_rounds
	}

}
