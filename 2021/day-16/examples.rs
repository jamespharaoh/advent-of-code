#![ cfg (test) ]

use super::*;

#[ test ]
fn part_one () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("6", puzzle.part_one (& ["D2FE28"]));
	assert_eq_ok! ("9", puzzle.part_one (& ["38006F45291200"]));
	assert_eq_ok! ("14", puzzle.part_one (& ["EE00D40C823060"]));
	assert_eq_ok! ("16", puzzle.part_one (& ["8A004A801A8002F478"]));
	assert_eq_ok! ("12", puzzle.part_one (& ["620080001611562C8802118E34"]));
	assert_eq_ok! ("23", puzzle.part_one (& ["C0015000016115A2E0802F182340"]));
	assert_eq_ok! ("31", puzzle.part_one (& ["A0016C880162017C3686B18A3D4780"]));
}

#[ test ]
fn part_two () {
	let puzzle = puzzle_metadata ();
	assert_eq_ok! ("3", puzzle.part_two (& ["C200B40A82"]));
	assert_eq_ok! ("54", puzzle.part_two (& ["04005AC33890"]));
	assert_eq_ok! ("7", puzzle.part_two (& ["880086C3E88112"]));
	assert_eq_ok! ("9", puzzle.part_two (& ["CE00C43D881120"]));
	assert_eq_ok! ("1", puzzle.part_two (& ["D8005AC2A8F0"]));
	assert_eq_ok! ("0", puzzle.part_two (& ["F600BC2D8F"]));
	assert_eq_ok! ("0", puzzle.part_two (& ["9C005AC2F8F0"]));
	assert_eq_ok! ("1", puzzle.part_two (& ["9C0141080250320F1802104A08"]));
}
