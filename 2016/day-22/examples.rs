#![ cfg (test) ]

use super::*;

const EXAMPLE: & [& str] = & [
	"root@ebhq-gridcenter# df -h",
	"Filesystem              Size  Used  Avail  Use%",
	"/dev/grid/node-x0-y0     93T   68T    25T   73%",
	"/dev/grid/node-x0-y1     91T   69T    22T   75%",
	"/dev/grid/node-x0-y2     92T   68T    24T   73%",
	"/dev/grid/node-x0-y3     92T   73T    19T   79%",
	"/dev/grid/node-x0-y4     89T   69T    20T   77%",
	"/dev/grid/node-x1-y0     89T   65T    24T   73%",
	"/dev/grid/node-x1-y1     89T   71T    18T   79%",
	"/dev/grid/node-x1-y2     88T   73T    15T   82%",
	"/dev/grid/node-x1-y3     93T   68T    25T   73%",
	"/dev/grid/node-x1-y4     91T   68T    23T   74%",
	"/dev/grid/node-x2-y0     94T   73T    21T   77%",
	"/dev/grid/node-x2-y1     93T   67T    26T   72%",
	"/dev/grid/node-x2-y2     87T   69T    18T   79%",
	"/dev/grid/node-x2-y3     86T   66T    20T   76%",
	"/dev/grid/node-x2-y4     94T   68T    26T   72%",
	"/dev/grid/node-x3-y0     94T   65T    29T   69%",
	"/dev/grid/node-x3-y1     87T   71T    16T   81%",
	"/dev/grid/node-x3-y2     91T   0T     91T   0%",
	"/dev/grid/node-x3-y3     94T   69T    25T   73%",
	"/dev/grid/node-x3-y4     90T   73T    17T   81%",
	"/dev/grid/node-x4-y0     86T   69T    17T   80%",
	"/dev/grid/node-x4-y1     92T   72T    20T   78%",
	"/dev/grid/node-x4-y2     94T   64T    30T   68%",
	"/dev/grid/node-x4-y3     85T   64T    21T   75%",
	"/dev/grid/node-x4-y4     92T   68T    24T   73%",
];

#[ test ]
fn part_one () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("24", puzzle.part_one (EXAMPLE));
}

#[ test ]
fn part_two () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("18", puzzle.part_two (EXAMPLE));
}
