//! Iterative search algorithms for solutions in a problem space

use super::*;

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
/// # use aoc_common::*;
/// # use aoc_common::search::*;
/// // set up a data structure with a map of connected nodes and the distance between them
/// let nodes: HashMap <& str, Vec <(& str, u64)>> = {
///
///    // use of Itertools::group_by requires a temporary...
///    let nodes_group_by_temp = [
///          // list of connected nodes and the distance between them
///          ("one", "two", 7), ("one", "three", 9), ("one", "six", 14),
///          ("two", "three", 10), ("two", "four", 15), ("three", "four", 11),
///          ("three", "six", 2), ("four", "five", 6), ("five", "six", 9),
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
/// let mut search = PrioritySearch::new (nodes);
///
/// // add the starting point with a total distance of 0
/// search.push ("one", 0);
///
/// // verify the results
/// assert_eq! (search.next (), Some (("one", 0)));
/// assert_eq! (search.next (), Some (("two", 7)));
/// assert_eq! (search.next (), Some (("three", 9)));
/// assert_eq! (search.next (), Some (("six", 11)));
/// assert_eq! (search.next (), Some (("five", 20)));
/// assert_eq! (search.next (), Some (("four", 20)));
/// assert_eq! (search.next (), None);
/// ```

pub struct PrioritySearch <Node, Pri, Visitor> {
	visitor: Visitor,
	seen: HashSet <Node>,
	todo: BinaryHeap <WithPriority <Node, Pri>>,
}

impl <Node, Pri, Visitor> PrioritySearch <Node, Pri, Visitor>
	where
		Node: Clone + Eq + Hash,
		Pri: Clone + Ord,
		Visitor: PrioritySearchVisitor <Node, Pri> {

	pub fn new (visitor: Visitor) -> PrioritySearch <Node, Pri, Visitor> {
		PrioritySearch {
			visitor,
			seen: HashSet::new (),
			todo: BinaryHeap::new (),
		}
	}

	pub fn len (& self) -> usize { self.todo.len ()	}

	pub fn push (& mut self, node: Node, priority: Pri) -> & mut Self {
		self.todo.push (WithPriority { priority, value: node });
		self
	}

}

impl <Node, Pri, Visitor> Iterator for PrioritySearch <Node, Pri, Visitor>
	where
		Node: Clone + Eq + Hash,
		Pri: Clone + Ord,
		Visitor: PrioritySearchVisitor <Node, Pri> {

	type Item = Visitor::Item;

	fn next (& mut self) -> Option <Self::Item> {
		while let Some (WithPriority { priority, value: node }) = self.todo.pop () {
			if self.seen.contains (& node) { continue }
			self.seen.insert (node.clone ());
			let adder = PrioritySearchAdder { todo: & mut self.todo };
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

pub struct PrioritySearchAdder <'a, Node, Pri> {
	todo: & 'a mut BinaryHeap <WithPriority <Node, Pri>>,
}

impl <'a, Node, Pri> PrioritySearchAdder <'a, Node, Pri>
	where Node: Clone + Eq + Hash, Pri: Clone + Ord {
	pub fn add (& mut self, node: Node, priority: Pri) {
		self.todo.push (WithPriority { priority, value: node });
	}
}

pub trait PrioritySearchVisitor <Node, Pri> {
	type Item;
	fn visit (
		& mut self,
		node: Node,
		priority: Pri,
		adder: PrioritySearchAdder <Node, Pri>,
	) -> Self::Item;
}

impl <VisitorFn, Node, Pri, Item> PrioritySearchVisitor <Node, Pri> for VisitorFn
	where VisitorFn: FnMut (Node, Pri, PrioritySearchAdder <Node, Pri>) -> Item {
	type Item = Item;
	fn visit (
		& mut self,
		node: Node,
		priority: Pri,
		adder: PrioritySearchAdder <Node, Pri>,
	) -> Self::Item {
		self (node, priority, adder)
	}
}

impl <Node, Pri, NextNodesIntoIter>
	PrioritySearchVisitor <Node, Pri> for HashMap <Node, NextNodesIntoIter>
		where Node: Clone + Eq + Hash, Pri: Clone + Ord + Add <Output = Pri>,
			for <'a> & 'a NextNodesIntoIter: IntoIterator <Item = & 'a (Node, Pri)> {
	type Item = (Node, Pri);
	fn visit (
		& mut self,
		node: Node,
		priority: Pri,
		mut adder: PrioritySearchAdder <Node, Pri>,
	) -> Self::Item {
		if let Some (next_nodes) = self.get (& node) {
			for (next_node, next_pri) in next_nodes.into_iter () {
				adder.add (next_node.clone (), priority.clone () + next_pri.clone ());
			}
		}
		(node, priority)
	}
}
