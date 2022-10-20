use super::*;

use input::Input;
use model::Room;

pub fn part_one (input: & Input) -> GenResult <u32> {
	Ok (
		input.rooms.iter ()
			.filter (|room| room_is_valid (room))
			.map (|room| room.sector)
			.sum ()
	)
}

pub fn part_two (input: & Input) -> GenResult <u32> {
	Ok (
		input.rooms.iter ()
			.filter (|room| room_is_valid (room))
			.filter (|room| room_name_decode (room).eq ("northpole object storage".chars ()))
			.map (|room| room.sector)
			.next ()
			.ok_or ("No solution found") ?
	)
}

/// Decode a room name according to the sector number.
///
/// This returns an [`Iterator <Item = char>`] to prevent allocations.
///
fn room_name_decode <'dat> (room: & 'dat Room) -> impl Iterator <Item = char> + 'dat {
	let rotate = room.sector % 26;
	room.name.chars ()
		.map (move |ch| if ch == '-' { ' ' } else {
			let mut ord = ch.pan_u32 () - 'a'.pan_u32 () + rotate;
			if 26 <= ord { ord -= 26; }
			('a'.pan_u32 () + ord).pan_char ()
		})
}

/// Check if a room's checksum is valid for its name.
///
/// The checksum should be the five most common alphabetic characters in the encoded name, in
/// order.
///
/// We use an efficient algorithm which does not allocate. An array is collected with the 26
/// letters in order and a count, which is then used to count each letter. This is then sorted,
/// and the first five characters are compared to the checksum.
///
fn room_is_valid (room: & Room) -> bool {
	let mut char_counts: [(char, u32); 26] = array::from_fn (|idx|
		(('a'.pan_u32 () + idx.pan_u32 ()).pan_char (), 0_u32));
	for ch in room.name.chars () {
		if ! ch.is_ascii_lowercase () { continue }
		char_counts [(ch.pan_u32 () - 'a'.pan_u32 ()).pan_usize ()].1 += 1;
	}
	char_counts.sort_by_key (|& (_, num)| cmp::Reverse (num));
	char_counts.into_iter ().take (5).map (|(ch, _)| ch).eq (room.checksum.chars ())
}
