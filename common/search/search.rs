//! Iterative search algorithms for solutions in a problem space

use aoc_grid::GridPos;
use aoc_grid::prelude::*;
use aoc_misc::*;
use aoc_nums::NumResult;

/// Implements [Digkstra's algorithm](https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm)
///
/// This searches for the cheapest path from one point to another, given a starting point (or
/// points) and a way to iteratively find next points. While this can be used for path-finding, it
/// can also be used to solve many other problems, assuming their state can be modeled as points,
/// the next steps can be iterated, and each step can be scored with some kind of priority.
///
/// # Example
///
/// Here's an example taken from the
/// [wikipedia page](https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm). We use strings for the
/// nodes instead of integers to make things a bit clearer.
///
/// In this example, we supply a hash map directly when constructing the [`PrioritySearch`]. This
/// works because the trait [`PrioritySearchVisitor`] is implemented directly in this case. It is
/// also implemented for functions, so you can pass a closure, plus the trait can be implemented
/// directly, of course.
///
/// ```
/// # use aoc_misc::*;
/// # use aoc_search::*;
/// // set up a data structure with a map of connected nodes and the distance between them
/// let nodes: HashMap <& str, Vec <(& str, u64)>> = {
///
///    // use of Itertools::group_by requires a temporary...
///    let nodes_group_by_temp = [
///          // list of connected nodes and the distance between them
///          ("one", "two", 7), ("one", "three", 9), ("one", "six", 14),
///          ("two", "three", 10), ("two", "four", 15), ("three", "four", 11),
///          ("three", "six", 2), ("four", "five", 6), ("five", "six", 8),
///       ].into_iter ()
///          // double up the connections to include the reverse
///          .flat_map (|(node_1, node_2, dist)| [(node_1, node_2, dist), (node_2, node_1, dist)])
///          // group by node_1 (requires sort)
///          .sorted_by_key (|& (node_1, _, _)| node_1)
///         .group_by (|& (node_1, _, _)| node_1);
///
///    // collect into a hash map from node_1 to (node_2, dist)
///    nodes_group_by_temp.into_iter ()
///       .map (|(node_1, group)| (
///             node_1,
///             group.map (|(_, node_2, dist)| (node_2, dist)).collect::<Vec <_>> (),
///          ))
///       .collect ()
///
/// };
///
/// // create a PrioritySearch to traverse our nodes
/// let mut search = PrioritySearch::with_hash_map (nodes);
///
/// // add the starting point with a total distance of 0
/// search.push ("one", 0);
///
/// // verify the results
/// assert_eq! (search.next (), Some (("one", 0)));
/// assert_eq! (search.next (), Some (("two", 7)));
/// assert_eq! (search.next (), Some (("three", 9)));
/// assert_eq! (search.next (), Some (("six", 11)));
/// assert_eq! (search.next (), Some (("five", 19)));
/// assert_eq! (search.next (), Some (("four", 20)));
/// assert_eq! (search.next (), None);
/// ```

pub struct PrioritySearch <Node, Pri, Visitor, SeenImpl>
	where
		Node: Clone + Debug + Eq + Hash,
		Pri: Clone + Copy + Debug + Ord,
		SeenImpl: Seen <Node, Pri>,
		Visitor: PrioritySearchVisitor <Node, Pri, SeenImpl> {
	visitor: Visitor,
	inner: PrioritySearchInner <Node, Pri, SeenImpl>,
}

#[ allow (clippy::implicit_hasher) ]
impl <Node, Pri, Visitor>
	PrioritySearch <Node, Pri, Visitor, HashMap <Node, SeenState <Pri>>>
	where
		Node: Clone + Debug + Eq + Hash + Ord,
		Pri: Clone + Copy + Debug + Ord,
		Visitor: PrioritySearchVisitor <Node, Pri, HashMap <Node, SeenState <Pri>>> {

	#[ inline ]
	pub fn with_hash_map (visitor: Visitor) -> Self {
		Self {
			visitor,
			inner: PrioritySearchInner {
				seen: HashMap::new (),
				todo: BinaryHeap::new (),
			},
		}
	}

}

impl <Node, Pri, Visitor, const DIMS: usize>
	PrioritySearch <Node, Pri, Visitor, GridBuf <Vec <SeenState <Pri>>, Node, DIMS>>
	where
		Node: Clone + Debug + Eq + Hash + GridPos <DIMS>,
		Pri: Clone + Copy + Debug + Ord,
		Visitor: PrioritySearchVisitor <Node, Pri, GridBuf <Vec <SeenState <Pri>>, Node, DIMS>> {

	#[ inline ]
	pub fn with_grid_range (start: Node, end: Node, visitor: Visitor) -> NumResult <Self> {
		Ok (Self {
			visitor,
			inner: PrioritySearchInner {
				seen: GridBuf::new_range (start, end) ?,
				todo: BinaryHeap::new (),
			},
		})
	}

	#[ inline ]
	pub fn with_grid_size (size: Node, visitor: Visitor) -> Self {
		Self {
			visitor,
			inner: PrioritySearchInner {
				seen: GridBuf::new_size (size),
				todo: BinaryHeap::new (),
			},
		}
	}

}

impl <Node, Pri, Visitor, SeenImpl> PrioritySearch <Node, Pri, Visitor, SeenImpl>
	where
		Node: Clone + Debug + Eq + Hash,
		Pri: Clone + Copy + Debug + Ord,
		SeenImpl: Seen <Node, Pri>,
		Visitor: PrioritySearchVisitor <Node, Pri, SeenImpl> {

	#[ inline ]
	pub fn len (& self) -> usize {
		self.inner.todo.len ()
	}

	#[ inline ]
	pub fn is_empty (& self) -> bool {
		self.len () == 0
	}

	#[ inline ]
	pub fn push (& mut self, node: Node, priority: Pri) -> & mut Self {
		self.inner.push (node, priority);
		self
	}

}

impl <Node, Pri, Visitor, SeenImpl> Iterator for PrioritySearch <Node, Pri, Visitor, SeenImpl>
	where
		Node: Clone + Debug + Eq + Hash,
		Pri: Clone + Copy + Debug + Ord,
		SeenImpl: Seen <Node, Pri>,
		Visitor: PrioritySearchVisitor <Node, Pri, SeenImpl> {

	type Item = Visitor::Item;

	#[ inline ]
	fn next (& mut self) -> Option <Self::Item> {
		if let Some (WithPriority { priority, value: node }) = self.inner.pop () {
			let adder = PrioritySearchAdder { inner: & mut self.inner };
			return Some (self.visitor.visit (node, priority, adder));
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
	fn partial_cmp (& self, other: & Self) -> Option <Ordering> {
		other.priority.partial_cmp (& self.priority)
	}
}

impl <Val, Pri> Ord for WithPriority <Val, Pri> where Pri: Ord {
	fn cmp (& self, other: & Self) -> Ordering {
		other.priority.cmp (& self.priority)
	}
}

struct PrioritySearchInner <Node, Pri, Seen> {
	seen: Seen,
	todo: BinaryHeap <WithPriority <Node, Pri>>,
}

impl <Node, Pri, SeenImpl> PrioritySearchInner <Node, Pri, SeenImpl>
	where
		Node: Clone + Eq + Hash,
		Pri: Clone + Ord,
		SeenImpl: Seen <Node, Pri> {
	fn push (& mut self, node: Node, priority: Pri) {
		if self.seen.seen_push (node.clone (), priority.clone ()) {
			self.todo.push (WithPriority { priority, value: node });
		}
	}
	fn pop (& mut self) -> Option <WithPriority <Node, Pri>> {
		while let Some (WithPriority { value: node, priority }) = self.todo.pop () {
			if self.seen.seen_visited (& node) { continue }
			return Some (WithPriority { value: node, priority });
		}
		None
	}
}

pub struct PrioritySearchAdder <'inr, Node, Pri, Seen> {
	inner: & 'inr mut PrioritySearchInner <Node, Pri, Seen>,
}

impl <'inr, Node, Pri, SeenImpl> PrioritySearchAdder <'inr, Node, Pri, SeenImpl>
	where
		Node: Clone + Debug + Eq + Hash,
		Pri: Clone + Debug + Ord,
		SeenImpl: Seen <Node, Pri> {

	#[ inline ]
	pub fn add (& mut self, node: Node, priority: Pri) {
		self.inner.push (node, priority);
	}

}

pub trait PrioritySearchVisitor <Node, Pri, Seen> {

	type Item;

	fn visit (
		& mut self,
		node: Node,
		priority: Pri,
		adder: PrioritySearchAdder <Node, Pri, Seen>,
	) -> Self::Item;

}

impl <VisitorFn, Node, Pri, Item, SeenImpl> PrioritySearchVisitor <Node, Pri, SeenImpl> for VisitorFn
	where
		Node: Clone,
		Pri: Clone + Ord,
		SeenImpl: Seen <Node, Pri>,
		VisitorFn: FnMut (Node, Pri, PrioritySearchAdder <Node, Pri, SeenImpl>) -> Item {

	type Item = Item;

	#[ inline ]
	fn visit (
		& mut self,
		node: Node,
		priority: Pri,
		adder: PrioritySearchAdder <Node, Pri, SeenImpl>,
	) -> Self::Item {
		self (node, priority, adder)
	}

}

impl <Node, Pri, SeenImpl, NextNodesIntoIter, Hshr> PrioritySearchVisitor <Node, Pri, SeenImpl>
	for HashMap <Node, NextNodesIntoIter, Hshr>
	where
		Hshr: BuildHasher,
		Node: Clone + Debug + Eq + Hash + Ord,
		Pri: Clone + Debug + Ord + Add <Output = Pri>,
		SeenImpl: Seen <Node, Pri>,
		for <'dat> & 'dat NextNodesIntoIter: IntoIterator <Item = & 'dat (Node, Pri)> {

	type Item = (Node, Pri);

	#[ inline ]
	fn visit (
		& mut self,
		node: Node,
		priority: Pri,
		mut adder: PrioritySearchAdder <Node, Pri, SeenImpl>,
	) -> Self::Item {
		if let Some (next_nodes) = self.get (& node) {
			for & (ref next_node, ref next_pri) in next_nodes {
				adder.add (next_node.clone (), priority.clone () + next_pri.clone ());
			}
		}
		(node, priority)
	}

}

pub trait Seen <Node, Pri> where Node: Clone, Pri: Clone + Ord {

	fn seen_get_mut (& mut self, node: Node) -> & mut SeenState <Pri>;

	#[ inline ]
	fn seen_push (& mut self, node: Node, priority: Pri) -> bool {
		let seen_state = self.seen_get_mut (node);
		match seen_state.clone () {
			SeenState::New => {
				* seen_state = SeenState::Unvisited (priority);
				true
			},
			SeenState::Unvisited (seen_priority) if priority < seen_priority => {
				* seen_state = SeenState::Unvisited (priority);
				true
			},
			SeenState::Unvisited (_) | SeenState::Visited => false,
		}
	}

	#[ inline ]
	fn seen_visited (& mut self, node: & Node) -> bool {
		let seen_state = self.seen_get_mut (node.clone ());
		if let SeenState::Visited = seen_state.clone () {
			true
		} else {
			* seen_state = SeenState::Visited;
			false
		}
	}

}

impl <Node, Pri, Hshr> Seen <Node, Pri>
for HashMap <Node, SeenState <Pri>, Hshr>
	where
		Hshr: BuildHasher,
		Node: Clone + Eq + Hash + Ord,
		Pri: Clone + Ord {

	#[ inline ]
	fn seen_get_mut (& mut self, node: Node) -> & mut SeenState <Pri> {
		self.entry (node).or_insert (SeenState::New)
	}

}

impl <Node, Pri, const DIMS: usize> Seen <Node, Pri>
	for GridBuf <Vec <SeenState <Pri>>, Node, DIMS>
	where
		Node: GridPos <DIMS> + Clone + Eq + Hash,
		Pri: Clone + Ord {

	#[ inline ]
	fn seen_get_mut (& mut self, node: Node) -> & mut SeenState <Pri> {
		Self::get_mut (self, node).unwrap_or_else (
			|| panic! ("Position is not in grid: {node:?}"))
	}

}

#[ derive (Clone) ]
pub enum SeenState <Pri: Clone> {
	New,
	Unvisited (Pri),
	Visited,
}

impl <Pri: Clone> Default for SeenState <Pri> {

	#[ inline ]
	fn default () -> Self {
		Self::New
	}

}
