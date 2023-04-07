(function() {var implementors = {
"aoc_2019_day_21":[["impl&lt;const LEN: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"aoc_2019_day_21/model/struct.RegsIter.html\" title=\"struct aoc_2019_day_21::model::RegsIter\">RegsIter</a>&lt;LEN&gt;"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"aoc_2019_day_21/model/struct.FragIter.html\" title=\"struct aoc_2019_day_21::model::FragIter\">FragIter</a>"]],
"aoc_2021_day_20":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"aoc_2021_day_20/logic/struct.ImageIter.html\" title=\"struct aoc_2021_day_20::logic::ImageIter\">ImageIter</a>"]],
"aoc_2021_day_24":[["impl&lt;'stp, Nested: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a>&lt;Item = (<a class=\"struct\" href=\"aoc_stvec/struct.TinyVec.html\" title=\"struct aoc_stvec::TinyVec\">TinyVec</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>, 14&gt;, <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.i64.html\">i64</a>)&gt;&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"enum\" href=\"aoc_2021_day_24/quick/enum.NextNumIter.html\" title=\"enum aoc_2021_day_24::quick::NextNumIter\">NextNumIter</a>&lt;'stp, Nested&gt;"]],
"aoc_2021_day_25":[["impl&lt;Item, Left, Right&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"enum\" href=\"aoc_2021_day_25/model/enum.Either.html\" title=\"enum aoc_2021_day_25::model::Either\">Either</a>&lt;Left, Right&gt;<span class=\"where fmt-newline\">where\n    Left: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a>&lt;Item = Item&gt;,\n    Right: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a>&lt;Item = Item&gt;,</span>"]],
"aoc_bitvec":[["impl&lt;'dat, Item, Encoding&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"aoc_bitvec/struct.BitVecIter.html\" title=\"struct aoc_bitvec::BitVecIter\">BitVecIter</a>&lt;'dat, Item, Encoding&gt;<span class=\"where fmt-newline\">where\n    Encoding: <a class=\"trait\" href=\"aoc_bitvec/prelude/trait.BitVecEncoding.html\" title=\"trait aoc_bitvec::prelude::BitVecEncoding\">BitVecEncoding</a>&lt;Item&gt;,\n    Item: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>,</span>"]],
"aoc_grid":[["impl&lt;Inner, Pos, const DIMS: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"aoc_grid/struct.GridExtendIter.html\" title=\"struct aoc_grid::GridExtendIter\">GridExtendIter</a>&lt;Inner, Pos, DIMS&gt;<span class=\"where fmt-newline\">where\n    Inner: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a>,\n    Inner::<a class=\"associatedtype\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\" title=\"type core::iter::traits::iterator::Iterator::Item\">Item</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a>,\n    Pos: <a class=\"trait\" href=\"aoc_grid/prelude/trait.GridPos.html\" title=\"trait aoc_grid::prelude::GridPos\">GridPos</a>&lt;DIMS&gt;,</span>"],["impl&lt;'sto, Storage, Item&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"aoc_grid/struct.GridStorageClone.html\" title=\"struct aoc_grid::GridStorageClone\">GridStorageClone</a>&lt;Storage&gt;<span class=\"where fmt-newline\">where\n    Storage: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a>&lt;Item = <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\">&amp;'sto Item</a>&gt;,\n    Item: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + 'sto,</span>"],["impl&lt;Pos, const DIMS: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"aoc_grid/struct.GridCursorIter.html\" title=\"struct aoc_grid::GridCursorIter\">GridCursorIter</a>&lt;Pos, DIMS&gt;<span class=\"where fmt-newline\">where\n    Pos: <a class=\"trait\" href=\"aoc_grid/prelude/trait.GridPos.html\" title=\"trait aoc_grid::prelude::GridPos\">GridPos</a>&lt;DIMS&gt;,</span>"],["impl&lt;Pos, const DIMS: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"aoc_grid/struct.GridExtendCursors.html\" title=\"struct aoc_grid::GridExtendCursors\">GridExtendCursors</a>&lt;Pos, DIMS&gt;<span class=\"where fmt-newline\">where\n    Pos: <a class=\"trait\" href=\"aoc_grid/prelude/trait.GridPos.html\" title=\"trait aoc_grid::prelude::GridPos\">GridPos</a>&lt;DIMS&gt;,</span>"],["impl&lt;Pos, const DIMS: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"aoc_grid/struct.GridCursorWalk.html\" title=\"struct aoc_grid::GridCursorWalk\">GridCursorWalk</a>&lt;Pos, DIMS&gt;<span class=\"where fmt-newline\">where\n    Pos: <a class=\"trait\" href=\"aoc_grid/prelude/trait.GridPos.html\" title=\"trait aoc_grid::prelude::GridPos\">GridPos</a>&lt;DIMS&gt;,</span>"],["impl&lt;Pos: <a class=\"trait\" href=\"aoc_grid/prelude/trait.GridPos.html\" title=\"trait aoc_grid::prelude::GridPos\">GridPos</a>&lt;DIMS&gt;, const DIMS: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"aoc_grid/struct.GridKeysIter.html\" title=\"struct aoc_grid::GridKeysIter\">GridKeysIter</a>&lt;Pos, DIMS&gt;"],["impl&lt;Pos, const DIMS: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"aoc_grid/struct.GridTransformIter.html\" title=\"struct aoc_grid::GridTransformIter\">GridTransformIter</a>&lt;Pos, DIMS&gt;<span class=\"where fmt-newline\">where\n    Pos: <a class=\"trait\" href=\"aoc_grid/prelude/trait.GridPos.html\" title=\"trait aoc_grid::prelude::GridPos\">GridPos</a>&lt;DIMS&gt;,</span>"]],
"aoc_misc":[],
"aoc_parallel":[["impl&lt;Inner, Out&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"aoc_parallel/struct.ThreadMap.html\" title=\"struct aoc_parallel::ThreadMap\">ThreadMap</a>&lt;Inner, Out&gt;<span class=\"where fmt-newline\">where\n    Out: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>,</span>"]],
"aoc_parser":[["impl&lt;'par, 'inp, Delim, Output, ParseFn&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"aoc_parser/struct.ParserDelim.html\" title=\"struct aoc_parser::ParserDelim\">ParserDelim</a>&lt;'par, 'inp, Delim, Output, ParseFn&gt;<span class=\"where fmt-newline\">where\n    Delim: <a class=\"trait\" href=\"aoc_parser/trait.ParseDelimiter.html\" title=\"trait aoc_parser::ParseDelimiter\">ParseDelimiter</a>,\n    ParseFn: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html\" title=\"trait core::ops::function::FnMut\">FnMut</a>(&amp;mut <a class=\"struct\" href=\"aoc_parser/prelude/struct.Parser.html\" title=\"struct aoc_parser::prelude::Parser\">Parser</a>&lt;'inp&gt;) -&gt; <a class=\"type\" href=\"aoc_parser/type.ParseResult.html\" title=\"type aoc_parser::ParseResult\">ParseResult</a>&lt;Output&gt;,</span>"],["impl&lt;'par, 'inp, Output, ParseFn&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"aoc_parser/struct.ParserRepeat.html\" title=\"struct aoc_parser::ParserRepeat\">ParserRepeat</a>&lt;'par, 'inp, Output, ParseFn&gt;<span class=\"where fmt-newline\">where\n    ParseFn: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html\" title=\"trait core::ops::function::FnMut\">FnMut</a>(&amp;mut <a class=\"struct\" href=\"aoc_parser/prelude/struct.Parser.html\" title=\"struct aoc_parser::prelude::Parser\">Parser</a>&lt;'inp&gt;) -&gt; <a class=\"type\" href=\"aoc_parser/type.ParseResult.html\" title=\"type aoc_parser::ParseResult\">ParseResult</a>&lt;Output&gt;,</span>"]],
"aoc_search":[["impl&lt;Node, Pri, Visitor, SeenImpl&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"aoc_search/priority/struct.PrioritySearch.html\" title=\"struct aoc_search::priority::PrioritySearch\">PrioritySearch</a>&lt;Node, Pri, Visitor, SeenImpl&gt;<span class=\"where fmt-newline\">where\n    Node: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html\" title=\"trait core::cmp::Eq\">Eq</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html\" title=\"trait core::hash::Hash\">Hash</a>,\n    Pri: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html\" title=\"trait core::marker::Copy\">Copy</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html\" title=\"trait core::cmp::Ord\">Ord</a>,\n    SeenImpl: <a class=\"trait\" href=\"aoc_search/priority/trait.Seen.html\" title=\"trait aoc_search::priority::Seen\">Seen</a>&lt;Node, Pri&gt;,\n    Visitor: <a class=\"trait\" href=\"aoc_search/priority/trait.PrioritySearchVisitor.html\" title=\"trait aoc_search::priority::PrioritySearchVisitor\">PrioritySearchVisitor</a>&lt;Node, Pri, SeenImpl&gt;,</span>"]],
"aoc_stvec":[["impl&lt;Item, const LEN: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"aoc_stvec/struct.TinyIntoIter.html\" title=\"struct aoc_stvec::TinyIntoIter\">TinyIntoIter</a>&lt;Item, LEN&gt;"],["impl&lt;Item, const LEN: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"aoc_stvec/struct.MiniIntoIter.html\" title=\"struct aoc_stvec::MiniIntoIter\">MiniIntoIter</a>&lt;Item, LEN&gt;"]]
};if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()