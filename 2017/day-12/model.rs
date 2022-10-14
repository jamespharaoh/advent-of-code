use super::*;
use input::Input;

pub type Village = u16;

#[ derive (Clone, Debug, Default) ]
pub struct Grouper {
	data: BTreeSet <(Village, Village)>,
	reverse: HashMap <Village, Village>,
}

impl Grouper {

	#[ must_use ]
	pub fn build (input: & Input) -> Self {

		#[ derive (Default) ]
		struct State {
			reverse: HashMap <Village, Village>,
			villages: Vec <Village>,
		}

		impl State {
			fn resolve (& mut self, village: Village) -> Village {
				let mut cur = some_or! (self.reverse.get (& village).copied (), {
					self.reverse.insert (village, village);
					self.villages.push (village);
					return village;
				});
				let group = loop {
					let next = self.reverse.get (& cur).copied ().unwrap ();
					if next == cur { break cur }
					cur = next;
				};
				let mut cur = village;
				while cur != group {
					cur = self.reverse.insert (cur, group).unwrap ();
				}
				group
			}
		}

		let mut state = State::default ();

		for (left, right) in input.pipes.iter ()
			.flat_map (|pipe| pipe.right.iter ()
				.map (|& right| (pipe.left, right))) {
			let left_group = state.resolve (left);
			let right_group = state.resolve (right);
			let new_group = cmp::min (left_group, right_group);
			let old_group = cmp::max (left_group, right_group);
			state.reverse.insert (old_group, new_group);
		}

		let villages = mem::take (& mut state.villages);
		let data: BTreeSet <(Village, Village)> =
			villages.iter ()
				.map (|& village| (state.resolve (village), village))
				.collect ();

		Self { data, reverse: state.reverse }

	}

	#[ must_use ]
	pub fn group_size (& self, group: Village) -> usize {
		let & group = some_or! (self.reverse.get (& group), return 0);
		self.data
			.range ((group, Village::MIN) ..= (group, Village::MAX))
			.count ()
	}

	pub fn groups (& self) -> impl Iterator <Item = Village> + '_ {
		let mut bound = Bound::Included ((Village::MIN, Village::ZERO));
		iter::from_fn (move || {
			let & (next, _) = self.data.range ((bound, Bound::Unbounded)).next () ?;
			bound = Bound::Excluded ((next, Village::MAX));
			Some (next)
		})
	}

}
