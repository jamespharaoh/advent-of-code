pub const fn decode (encoded: u128) -> Option <char> {
	if encoded.trailing_zeros () >= 60 {
		decode_six (encoded)
	} else if encoded.trailing_zeros () >= 40 {
		decode_eight (encoded)
	} else if encoded.trailing_zeros () >= 20 {
		decode_ten (encoded)
	} else {
		None
	}
}

pub const fn decode_ten (encoded: u128) -> Option <char> {
	Some (match encoded >> 24_u32 {
		0x_3012_0842_1084_3f08_4210_8421 => 'A',
		0x_f821_0842_10f8_2108_4210_843e => 'B',
		0x_7821_0802_0080_2008_0200_841e => 'C',
		// TODO => 'D',
		0x_fc20_0802_00f8_2008_0200_803f => 'E',
		0x_fc20_0802_00f8_2008_0200_8020 => 'F',
		0x_7821_0802_0080_2708_4210_8c1d => 'G',
		0x_8421_0842_10fc_2108_4210_8421 => 'H',
		0x_1c02_0080_2008_0200_8220_881c => 'J',
		0x_8422_0902_80c0_300a_0240_8821 => 'K',
		0x_8020_0802_0080_2008_0200_803f => 'L',
		// TODO => 'M',
		// TODO => 'N',
		// TODO => 'O',
		// TODO => 'P',
		// TODO => 'Q',
		0x_f821_0842_10f8_2408_8220_8421 => 'R',
		// TODO => 'S',
		// TODO => 'T',
		// TODO => 'U',
		// TODO => 'V',
		// TODO => 'W',
		0x_8421_0481_2030_0c04_8120_8421 => 'X',
		// TODO => 'Y',
		0x_fc01_0040_2010_0804_0200_803f => 'Z',
		_ => return None,
	})
}

pub const fn decode_eight (encoded: u128) -> Option <char> {
	Some (match encoded >> 40_u32 {
		0x_8822_0883_e088_2208_8220 => 'H',
		0x_e010_0401_0040_1004_0380 => 'I',
		_ => return None,
	})
}

pub const fn decode_six (encoded: u128) -> Option <char> {
	Some (match encoded >> 64_u32 {
		0x_0060_2409_03c0_9024 => 'A',
		0x_00e0_240e_0240_9038 => 'B',
		0x_0060_2408_0200_9018 => 'C',
		// TODO => 'D',
		0x_00f0_200e_0200_803c => 'E',
		0x_00f0_200e_0200_8020 => 'F',
		0x_0060_2408_02c0_901c => 'G',
		0x_0090_240f_0240_9024 => 'H',
		0x_00e0_1004_0100_4038 => 'I',
		0x_0030_0401_0040_9018 => 'J',
		0x_0090_280c_0280_a024 => 'K',
		0x_0080_2008_0200_803c => 'L',
		// TODO => 'M',
		// TODO => 'N',
		0x_0060_2409_0240_9018 => 'O',
		0x_00e0_2409_0380_8020 => 'P',
		// TODO => 'Q',
		0x_00e0_2409_0380_a024 => 'R',
		0x_0070_2008_0180_1038 => 'S',
		0x_0090_2409_0240_9018 => 'U',
		// TODO => 'V',
		// TODO => 'W',
		// TODO => 'X',
		0x_0088_2205_0080_2008 => 'Y',
		0x_00f0_0402_0100_803c => 'Z',
		_ => return None,
	})
}
