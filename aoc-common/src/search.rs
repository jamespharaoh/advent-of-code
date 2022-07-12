use super::*;

pub struct PrioritySearchAdder <'a, Node, Pri> {
	todo: & 'a mut BinaryHeap <WithPriority <Node, Pri>>,
}

impl <'a, Node, Pri> PrioritySearchAdder <'a, Node, Pri>
	where Node: Clone + Eq + hash::Hash, Pri: Clone + Ord {
	pub fn add (& mut self, node: Node, priority: Pri) {
		self.todo.push (WithPriority { priority, value: node });
	}
}

pub struct PrioritySearch <Node, Pri, NextFn> {
	next_fn: NextFn,
	seen: HashSet <Node>,
	todo: BinaryHeap <WithPriority <Node, Pri>>,
}

impl <Node, Pri, NextFn> PrioritySearch <Node, Pri, NextFn>
	where
		Node: Clone + Eq + hash::Hash,
		Pri: Clone + Ord,
		NextFn: Fn (& Node, & Pri, PrioritySearchAdder <Node, Pri>) {

	pub fn new (next_fn: NextFn) -> PrioritySearch <Node, Pri, NextFn> {
		PrioritySearch {
			next_fn,
			seen: HashSet::new (),
			todo: BinaryHeap::new (),
		}
	}

	pub fn push (& mut self, node: Node, priority: Pri) {
		self.todo.push (WithPriority { priority, value: node });
	}

}

impl <Node, Pri, NextFn> Iterator for PrioritySearch <Node, Pri, NextFn>
	where
		Node: Clone + Eq + hash::Hash,
		Pri: Clone + Ord,
		NextFn: Fn (& Node, & Pri, PrioritySearchAdder <Node, Pri>) {

	type Item = (Node, Pri);

	fn next (& mut self) -> Option <(Node, Pri)> {
		while let Some (WithPriority { priority, value: node }) = self.todo.pop () {
			if self.seen.contains (& node) { continue }
			self.seen.insert (node.clone ());
			let adder = PrioritySearchAdder { todo: & mut self.todo };
			(self.next_fn) (& node, & priority, adder);
			return Some ((node, priority));
		}
		None
	}

}

struct WithPriority <Val, Pri> {
	priority: Pri,
	value: Val,
}

impl <Val, Pri> PartialEq for WithPriority <Val, Pri> where Pri: PartialEq {
	fn eq (& self, other: & Self) -> bool {
		self.priority.eq (& other.priority)
	}
}

impl <Val, Pri> Eq for WithPriority <Val, Pri> where Pri: Eq {
}

impl <Val, Pri> PartialOrd for WithPriority <Val, Pri> where Pri: PartialOrd {
	fn partial_cmp (& self, other: & Self) -> Option <cmp::Ordering> {
		other.priority.partial_cmp (& self.priority)
	}
}

impl <Val, Pri> Ord for WithPriority <Val, Pri> where Pri: Ord {
	fn cmp (& self, other: & Self) -> cmp::Ordering {
		other.priority.cmp (& self.priority)
	}
}
