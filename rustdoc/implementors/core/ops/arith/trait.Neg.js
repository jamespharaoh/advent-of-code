(function() {var implementors = {
"aoc_grid":[["impl&lt;Pos, const DIMS: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html\" title=\"trait core::ops::arith::Neg\">Neg</a> for <a class=\"struct\" href=\"aoc_grid/struct.GridOffset.html\" title=\"struct aoc_grid::GridOffset\">GridOffset</a>&lt;Pos, DIMS&gt;<span class=\"where fmt-newline\">where\n    Pos: <a class=\"trait\" href=\"aoc_grid/trait.GridPos.html\" title=\"trait aoc_grid::GridPos\">GridPos</a>&lt;DIMS&gt;,\n    Pos::<a class=\"associatedtype\" href=\"aoc_grid/trait.GridPos.html#associatedtype.Coord\" title=\"type aoc_grid::GridPos::Coord\">Coord</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html\" title=\"trait core::ops::arith::Neg\">Neg</a>&lt;Output = Pos::<a class=\"associatedtype\" href=\"aoc_grid/trait.GridPos.html#associatedtype.Coord\" title=\"type aoc_grid::GridPos::Coord\">Coord</a>&gt;,</span>"]],
"aoc_misc":[],
"aoc_pos":[["impl&lt;Val: <a class=\"trait\" href=\"aoc_nums/int/trait.Int.html\" title=\"trait aoc_nums::int::Int\">Int</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html\" title=\"trait core::ops::arith::Neg\">Neg</a>&lt;Output = Val&gt;&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html\" title=\"trait core::ops::arith::Neg\">Neg</a> for <a class=\"struct\" href=\"aoc_pos/struct.PosRowCol.html\" title=\"struct aoc_pos::PosRowCol\">PosRowCol</a>&lt;Val&gt;"],["impl&lt;Val: <a class=\"trait\" href=\"aoc_nums/int/trait.Int.html\" title=\"trait aoc_nums::int::Int\">Int</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html\" title=\"trait core::ops::arith::Neg\">Neg</a>&lt;Output = Val&gt;&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html\" title=\"trait core::ops::arith::Neg\">Neg</a> for <a class=\"struct\" href=\"aoc_pos/struct.PosXYZ.html\" title=\"struct aoc_pos::PosXYZ\">PosXYZ</a>&lt;Val&gt;"],["impl&lt;Val: <a class=\"trait\" href=\"aoc_nums/int/trait.Int.html\" title=\"trait aoc_nums::int::Int\">Int</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html\" title=\"trait core::ops::arith::Neg\">Neg</a>&lt;Output = Val&gt;&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html\" title=\"trait core::ops::arith::Neg\">Neg</a> for <a class=\"struct\" href=\"aoc_pos/struct.PosWXYZ.html\" title=\"struct aoc_pos::PosWXYZ\">PosWXYZ</a>&lt;Val&gt;"],["impl&lt;Val: <a class=\"trait\" href=\"aoc_nums/int/trait.Int.html\" title=\"trait aoc_nums::int::Int\">Int</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html\" title=\"trait core::ops::arith::Neg\">Neg</a>&lt;Output = Val&gt;&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html\" title=\"trait core::ops::arith::Neg\">Neg</a> for <a class=\"struct\" href=\"aoc_pos/struct.PosYX.html\" title=\"struct aoc_pos::PosYX\">PosYX</a>&lt;Val&gt;"],["impl&lt;Val: <a class=\"trait\" href=\"aoc_nums/int/trait.Int.html\" title=\"trait aoc_nums::int::Int\">Int</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html\" title=\"trait core::ops::arith::Neg\">Neg</a>&lt;Output = Val&gt;&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html\" title=\"trait core::ops::arith::Neg\">Neg</a> for <a class=\"struct\" href=\"aoc_pos/struct.PosXY.html\" title=\"struct aoc_pos::PosXY\">PosXY</a>&lt;Val&gt;"],["impl&lt;Val: <a class=\"trait\" href=\"aoc_nums/int/trait.Int.html\" title=\"trait aoc_nums::int::Int\">Int</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html\" title=\"trait core::ops::arith::Neg\">Neg</a>&lt;Output = Val&gt;&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html\" title=\"trait core::ops::arith::Neg\">Neg</a> for <a class=\"struct\" href=\"aoc_pos/struct.PosXYZW.html\" title=\"struct aoc_pos::PosXYZW\">PosXYZW</a>&lt;Val&gt;"],["impl&lt;Val: <a class=\"trait\" href=\"aoc_nums/int/trait.Int.html\" title=\"trait aoc_nums::int::Int\">Int</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html\" title=\"trait core::ops::arith::Neg\">Neg</a>&lt;Output = Val&gt;&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html\" title=\"trait core::ops::arith::Neg\">Neg</a> for <a class=\"struct\" href=\"aoc_pos/struct.PosXYZT.html\" title=\"struct aoc_pos::PosXYZT\">PosXYZT</a>&lt;Val&gt;"],["impl&lt;Val: <a class=\"trait\" href=\"aoc_nums/int/trait.Int.html\" title=\"trait aoc_nums::int::Int\">Int</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html\" title=\"trait core::ops::arith::Neg\">Neg</a>&lt;Output = Val&gt;&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html\" title=\"trait core::ops::arith::Neg\">Neg</a> for <a class=\"struct\" href=\"aoc_pos/struct.PosGeo.html\" title=\"struct aoc_pos::PosGeo\">PosGeo</a>&lt;Val&gt;"],["impl&lt;Val: <a class=\"trait\" href=\"aoc_nums/int/trait.Int.html\" title=\"trait aoc_nums::int::Int\">Int</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html\" title=\"trait core::ops::arith::Neg\">Neg</a>&lt;Output = Val&gt;&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html\" title=\"trait core::ops::arith::Neg\">Neg</a> for <a class=\"struct\" href=\"aoc_pos/struct.PosGeoHexLat.html\" title=\"struct aoc_pos::PosGeoHexLat\">PosGeoHexLat</a>&lt;Val&gt;"]]
};if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()