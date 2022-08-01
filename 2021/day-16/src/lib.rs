//! Advent of Code 2021: Day 15: Packet Decoder
//!
//! [https://adventofcode.com/2021/day/16](https://adventofcode.com/2021/day/16)

use aoc_common::*;

puzzle_info! {
	name = "Packet Decoder";
	year = 2021;
	day = 16;
	part_one = |lines| Ok::<_, Infallible> (logic::part_one (lines [0]));
	part_two = |lines| Ok::<_, Infallible> (logic::part_two (lines [0]));
}

mod logic {

	use super::*;
	use model::BitIter;
	use model::Packet;

	pub fn part_one (input: & str) -> u64 {
		let mut input_iter = BitIter::new (input);
		let packet = Packet::decode (& mut input_iter).unwrap ();
		packet.version_sum ()
	}

	pub fn part_two (input: & str) -> u64 {
		let mut input_iter = BitIter::new (input);
		let packet = Packet::decode (& mut input_iter).unwrap ();
		packet_eval (& packet)
	}

	fn packet_eval (packet: & Packet) -> u64 {
		let child_vals = || packet.children.iter ().map (packet_eval);
		let child_at = |pos| child_vals ().nth (pos).unwrap ();
		let from_bool = |val| if val { 1 } else { 0 };
		match packet.packet_type {
			0 => child_vals ().sum (),
			1 => child_vals ().product (),
			2 => child_vals ().min ().unwrap (),
			3 => child_vals ().max ().unwrap (),
			4 => packet.value,
			5 => from_bool (child_at (0) > child_at (1)),
			6 => from_bool (child_at (0) < child_at (1)),
			7 => from_bool (child_at (0) == child_at (1)),
			_ => panic! (),
		}
	}

}

mod model {

	use super::*;
	use nums::IntConv;

	#[ allow (dead_code) ]
	#[ derive (Debug) ]
	pub struct Packet {
		pub version: u64,
		pub packet_type: u64,
		pub value: u64,
		pub children: Vec <Packet>,
	}

	impl Packet {
		pub fn decode (iter: & mut BitIter <'_>) -> Option <Self> {
			if ! iter.has_next () { return None }
			let version = iter.next_uint (3).unwrap ();
			let packet_type = iter.next_uint (3).unwrap ();
			let mut value = 0;
			let mut children = Vec::new ();
			if packet_type == 4 {
				let mut bits = 0_i32;
				loop {
					let is_last = iter.next_bit ().unwrap () == 0;
					if bits + 4_i32 > 64_i32 { panic! (); }
					value = value << 4_i32 | iter.next_uint (4).unwrap ();
					bits += 4_i32;
					if is_last { break }
				}
			} else if iter.next_bit ().unwrap () == 0 {
				let child_bits = iter.next_uint (15).unwrap ();
				let end_position = iter.position + child_bits;
				while iter.position < end_position {
					children.push (Self::decode (iter).unwrap ());
				}
			} else {
				let num_children = iter.next_uint (11).unwrap ();
				for _ in 0 .. num_children {
					children.push (Self::decode (iter).unwrap ());
				}
			}
			Some (Self { version, packet_type, value, children })
		}
		pub fn version_sum (& self) -> u64 {
			self.version + self.children.iter ()
				.map (Self::version_sum)
				.sum::<u64> ()
		}
	}

	pub struct BitIter <'inr> {
		inner: Chars <'inr>,
		buffer: Vec <u64>,
		position: u64,
	}

	impl <'inr> BitIter <'inr> {
		pub fn new (inner_str: & 'inr str) -> BitIter <'inr> {
			BitIter {
				inner: inner_str.chars (),
				buffer: Vec::with_capacity (4),
				position: 0,
			}
		}
		fn next_uint (& mut self, bits: u8) -> Option <u64> {
			if bits > 64 { panic! (); }
			let mut val = 0;
			for _ in 0 .. bits {
				let next_bit = some_or! (self.next_bit (), return None);
				val = val << 1_i32 | next_bit;
			}
			Some (val)
		}
		fn next_bit (& mut self) -> Option <u64> {
			if ! self.has_next () { return None }
			self.position += 1;
			self.buffer.pop ()
		}
		fn has_next (& mut self) -> bool {
			if self.buffer.is_empty () {
				if let Some (next_char) = self.inner.next () {
					let mut next_nibble = next_char.to_digit (16).unwrap ().as_u64 ();
					for _ in 0_i32 .. 4_i32 {
						self.buffer.push (next_nibble & 1);
						next_nibble >>= 1_i32;
					}
				}
			}
			! self.buffer.is_empty ()
		}
	}

}

#[ cfg (test) ]
mod examples {

	use super::*;

	#[ test ]
	fn part_one () -> GenResult <()> {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("6", puzzle.part_one (& ["D2FE28"]));
		assert_eq_ok! ("9", puzzle.part_one (& ["38006F45291200"]));
		assert_eq_ok! ("14", puzzle.part_one (& ["EE00D40C823060"]));
		assert_eq_ok! ("16", puzzle.part_one (& ["8A004A801A8002F478"]));
		assert_eq_ok! ("12", puzzle.part_one (& ["620080001611562C8802118E34"]));
		assert_eq_ok! ("23", puzzle.part_one (& ["C0015000016115A2E0802F182340"]));
		assert_eq_ok! ("31", puzzle.part_one (& ["A0016C880162017C3686B18A3D4780"]));
		Ok (())
	}

	#[ test ]
	fn part_two () -> GenResult <()> {
		let puzzle = puzzle_metadata ();
		assert_eq_ok! ("3", puzzle.part_two (& ["C200B40A82"]));
		assert_eq_ok! ("54", puzzle.part_two (& ["04005AC33890"]));
		assert_eq_ok! ("7", puzzle.part_two (& ["880086C3E88112"]));
		assert_eq_ok! ("9", puzzle.part_two (& ["CE00C43D881120"]));
		assert_eq_ok! ("1", puzzle.part_two (& ["D8005AC2A8F0"]));
		assert_eq_ok! ("0", puzzle.part_two (& ["F600BC2D8F"]));
		assert_eq_ok! ("0", puzzle.part_two (& ["9C005AC2F8F0"]));
		assert_eq_ok! ("1", puzzle.part_two (& ["9C0141080250320F1802104A08"]));
		Ok (())
	}

}
