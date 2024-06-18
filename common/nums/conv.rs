use super::*;

pub trait IntConv: Sized {

	#[ inline (always) ]
	fn pan_f32 (self) -> f32 { self.to_f32 ().unwrap () }

	#[ inline (always) ]
	fn pan_f64 (self) -> f64 { self.to_f64 ().unwrap () }

	#[ inline (always) ]
	fn pan_i8 (self) -> i8 { self.to_i8 ().unwrap () }

	#[ inline (always) ]
	fn pan_i16 (self) -> i16 { self.to_i16 ().unwrap () }

	#[ inline (always) ]
	fn pan_i32 (self) -> i32 { self.to_i32 ().unwrap () }

	#[ inline (always) ]
	fn pan_i64 (self) -> i64 { self.to_i64 ().unwrap () }

	#[ inline (always) ]
	fn pan_i128 (self) -> i128 { self.to_i128 ().unwrap () }

	#[ inline (always) ]
	fn pan_isize (self) -> isize { self.to_isize ().unwrap () }

	#[ inline (always) ]
	fn pan_u8 (self) -> u8 { self.to_u8 ().unwrap () }

	#[ inline (always) ]
	fn pan_u16 (self) -> u16 { self.to_u16 ().unwrap () }

	#[ inline (always) ]
	fn pan_u32 (self) -> u32 { self.to_u32 ().unwrap () }

	#[ inline (always) ]
	fn pan_u64 (self) -> u64 { self.to_u64 ().unwrap () }

	#[ inline (always) ]
	fn pan_u128 (self) -> u128 { self.to_u128 ().unwrap () }

	#[ inline (always) ]
	fn pan_usize (self) -> usize { self.to_usize ().unwrap () }

	#[ inline (always) ]
	fn pan_char (self) -> char { self.to_char ().unwrap () }

	fn qck_f32 (self) -> f32;
	fn qck_f64 (self) -> f64;
	fn qck_i8 (self) -> i8;
	fn qck_i16 (self) -> i16;
	fn qck_i32 (self) -> i32;
	fn qck_i64 (self) -> i64;
	fn qck_i128 (self) -> i128;
	fn qck_isize (self) -> isize;
	fn qck_u8 (self) -> u8;
	fn qck_u16 (self) -> u16;
	fn qck_u32 (self) -> u32;
	fn qck_u64 (self) -> u64;
	fn qck_u128 (self) -> u128;
	fn qck_usize (self) -> usize;

	fn from_char (val: char) -> NumResult <Self>;

	fn from_u8 (val: u8) -> NumResult <Self>;

	/// Safely convert from [`usize`]
	///
	/// # Errors
	///
	/// Returns `Err (Overflow)` if the result can't be represented
	///
	fn from_usize (val: usize) -> NumResult <Self>;

	fn from_isize (val: isize) -> NumResult <Self>;

	/// Safely convert to [`f32`]
	///
	/// # Errors
	///
	/// Returns `Err (Overflow)` if the result can't be represented
	///
	#[ inline (always) ]
	fn to_f32 (self) -> NumResult <f32> { self.to_u16 ().map (Into::into) }

	/// Safely convert to [`f64`]
	///
	/// # Errors
	///
	/// Returns `Err (Overflow)` if the result can't be represented
	///
	#[ inline (always) ]
	fn to_f64 (self) -> NumResult <f64> { self.to_u32 ().map (Into::into) }

	/// Safely convert to [`i8`]
	///
	/// # Errors
	///
	/// Returns `Err (Overflow)` if the result can't be represented
	///
	fn to_i8 (self) -> NumResult <i8>;

	/// Safely convert to [`i16`]
	///
	/// # Errors
	///
	/// Returns `Err (Overflow)` if the result can't be represented
	///
	fn to_i16 (self) -> NumResult <i16>;

	/// Safely convert to [`i32`]
	///
	/// # Errors
	///
	/// Returns `Err (Overflow)` if the result can't be represented
	///
	fn to_i32 (self) -> NumResult <i32>;

	/// Safely convert to [`i64`]
	///
	/// # Errors
	///
	/// Returns `Err (Overflow)` if the result can't be represented
	///
	fn to_i64 (self) -> NumResult <i64>;

	/// Safely convert to [`i128`]
	///
	/// # Errors
	///
	/// Returns `Err (Overflow)` if the result can't be represented
	///
	fn to_i128 (self) -> NumResult <i128>;

	/// Safely convert to [`isize`]
	///
	/// # Errors
	///
	/// Returns `Err (Overflow)` if the result can't be represented
	///
	fn to_isize (self) -> NumResult <isize>;

	/// Safely convert to [`u8`]
	///
	/// # Errors
	///
	/// Returns `Err (Overflow)` if the result can't be represented
	///
	fn to_u8 (self) -> NumResult <u8>;

	/// Safely convert to [`u16`]
	///
	/// # Errors
	///
	/// Returns `Err (Overflow)` if the result can't be represented
	///
	fn to_u16 (self) -> NumResult <u16>;

	/// Safely convert to [`u32`]
	///
	/// # Errors
	///
	/// Returns `Err (Overflow)` if the result can't be represented
	///
	fn to_u32 (self) -> NumResult <u32>;

	/// Safely convert to [`u64`]
	///
	/// # Errors
	///
	/// Returns `Err (Overflow)` if the result can't be represented
	///
	fn to_u64 (self) -> NumResult <u64>;

	/// Safely convert to [`u128`]
	///
	/// # Errors
	///
	/// Returns `Err (Overflow)` if the result can't be represented
	///
	fn to_u128 (self) -> NumResult <u128>;

	/// Safely convert to [`usize`]
	///
	/// # Errors
	///
	/// Returns `Err (Overflow)` if the result can't be represented
	///
	fn to_usize (self) -> NumResult <usize>;

	fn to_char (self) -> NumResult <char>;

}

impl IntConv for char {

	#[ inline (always) ]
	fn from_char (val: char) -> NumResult <Self> {
		Ok (val)
	}

	#[ inline (always) ]
	fn from_u8 (val: u8) -> NumResult <Self> {
		val.to_u32 () ?.try_into ().map_err (|_err| Overflow)
	}

	#[ inline (always) ]
	fn from_usize (val: usize) -> NumResult <Self> {
		val.to_u32 () ?.try_into ().map_err (|_err| Overflow)
	}

	#[ inline (always) ]
	fn from_isize (val: isize) -> NumResult <Self> {
		val.to_u32 () ?.try_into ().map_err (|_err| Overflow)
	}

	#[ inline (always) ]
	fn to_char (self) -> NumResult <char> {
		Ok (self)
	}

	#[ inline (always) ]
	fn to_i8 (self) -> NumResult <i8> {
		self.to_u32 () ?.try_into ().map_err (|_err| Overflow)
	}

	#[ inline (always) ]
	fn to_i16 (self) -> NumResult <i16> {
		self.to_u32 () ?.try_into ().map_err (|_err| Overflow)
	}

	#[ inline (always) ]
	fn to_i32 (self) -> NumResult <i32> {
		self.to_u32 () ?.try_into ().map_err (|_err| Overflow)
	}

	#[ inline (always) ]
	fn to_i64 (self) -> NumResult <i64> {
		Ok (self.to_u32 () ?.into ())
	}

	#[ inline (always) ]
	fn to_i128 (self) -> NumResult <i128> {
		Ok (self.to_u32 () ?.into ())
	}

	#[ inline (always) ]
	fn to_isize (self) -> NumResult <isize> {
		self.to_u32 () ?.try_into ().map_err (|_err| Overflow)
	}

	#[ inline (always) ]
	fn to_u8 (self) -> NumResult <u8> {
		self.to_u32 () ?.try_into ().map_err (|_err| Overflow)
	}

	#[ inline (always) ]
	fn to_u16 (self) -> NumResult <u16> {
		self.to_u32 () ?.try_into ().map_err (|_err| Overflow)
	}

	#[ inline (always) ]
	fn to_u32 (self) -> NumResult <u32> {
		Ok (self.into ())
	}

	#[ inline (always)]
	fn to_u64 (self) -> NumResult <u64> {
		Ok (self.to_u32 () ?.into ())
	}

	#[ inline (always) ]
	fn to_u128 (self) -> NumResult <u128> {
		Ok (self.to_u32 () ?.into ())
	}

	#[ inline (always) ]
	fn to_usize (self) -> NumResult <usize> {
		self.to_u32 () ?.try_into ().map_err (|_err| Overflow)
	}

	#[ inline (always) ]
	fn qck_f32 (self) -> f32 { self.qck_u32 ().qck_f32 () }

	#[ inline (always) ]
	fn qck_f64 (self) -> f64 { self.qck_u32 ().qck_f64 () }

	#[ inline (always) ]
	fn qck_i8 (self) -> i8 { self as i8 }

	#[ inline (always) ]
	fn qck_i16 (self) -> i16 { self as i16 }

	#[ inline (always) ]
	fn qck_i32 (self) -> i32 { self as i32 }

	#[ inline (always) ]
	fn qck_i64 (self) -> i64 { self as i64 }

	#[ inline (always) ]
	fn qck_i128 (self) -> i128 { self as i128 }

	#[ inline (always) ]
	fn qck_isize (self) -> isize { self as isize }

	#[ inline (always) ]
	fn qck_u8 (self) -> u8 { self as u8 }

	#[ inline (always) ]
	fn qck_u16 (self) -> u16 { self as u16 }

	#[ inline (always) ]
	fn qck_u32 (self) -> u32 { self as u32 }

	#[ inline (always) ]
	fn qck_u64 (self) -> u64 { self as u64 }

	#[ inline (always) ]
	fn qck_u128 (self) -> u128 { self as u128 }

	#[ inline (always) ]
	fn qck_usize (self) -> usize { self as usize }

}

pub trait QuickFrom <Other> {
	fn quick_from (other: Other) -> Self;
}

pub trait QuickInto <Other> {
	fn quick_into (self) -> Other;
}

impl <From, To> QuickInto <To> for From where To: QuickFrom <From> {
	#[ inline (always) ]
	fn quick_into (self) -> To {
		To::quick_from (self)
	}
}

macro_rules! int_conv_impl {
	( $signed:ident , $unsigned:ident, $bits:literal ) => {
		impl IntConv for $signed {

			#[ inline (always) ]
			fn from_char (val: char) -> NumResult <Self> {
				val.qck_u32 ().try_into ().ok ().ok_or (Overflow)
			}

			#[ inline (always) ]
			fn from_u8 (val: u8) -> NumResult <Self> {
				val.try_into ().ok ().ok_or (Overflow)
			}

			#[ inline (always) ]
			fn from_usize (val: usize) -> NumResult <Self> {
				val.try_into ().ok ().ok_or (Overflow)
			}

			#[ inline (always) ]
			fn from_isize (val: isize) -> NumResult <Self> {
				val.try_into ().ok ().ok_or (Overflow)
			}

			#[ inline (always) ]
			fn to_char (self) -> NumResult <char> {
				self.to_u32 () ?.try_into ().ok ().ok_or (Overflow)
			}

			#[ inline (always) ]
			fn to_i8 (self) -> NumResult <i8> {
				self.try_into ().ok ().ok_or (Overflow)
			}

			#[ inline (always) ]
			fn to_i16 (self) -> NumResult <i16> {
				self.try_into ().ok ().ok_or (Overflow)
			}

			#[ inline (always) ]
			fn to_i32 (self) -> NumResult <i32> {
				self.try_into ().ok ().ok_or (Overflow)
			}

			#[ inline (always) ]
			fn to_i64 (self) -> NumResult <i64> {
				self.try_into ().ok ().ok_or (Overflow)
			}

			#[ inline (always) ]
			fn to_i128 (self) -> NumResult <i128> {
				self.try_into ().ok ().ok_or (Overflow)
			}

			#[ inline (always) ]
			fn to_isize (self) -> NumResult <isize> {
				self.try_into ().ok ().ok_or (Overflow)
			}

			#[ inline (always) ]
			fn to_u8 (self) -> NumResult <u8> {
				self.try_into ().ok ().ok_or (Overflow)
			}

			#[ inline (always) ]
			fn to_u16 (self) -> NumResult <u16> {
				self.try_into ().ok ().ok_or (Overflow)
			}

			#[ inline (always) ]
			fn to_u32 (self) -> NumResult <u32> {
				self.try_into ().ok ().ok_or (Overflow)
			}

			#[ inline (always) ]
			fn to_u64 (self) -> NumResult <u64> {
				self.try_into ().ok ().ok_or (Overflow)
			}

			#[ inline (always) ]
			fn to_u128 (self) -> NumResult <u128> {
				self.try_into ().ok ().ok_or (Overflow)
			}

			#[ inline (always) ]
			fn to_usize (self) -> NumResult <usize> {
				self.try_into ().ok ().ok_or (Overflow)
			}

			#[ allow (clippy::cast_lossless) ]
			#[ allow (clippy::cast_precision_loss) ]
			#[ inline (always) ]
			fn qck_f32 (self) -> f32 { self as f32 }

			#[ allow (clippy::cast_lossless) ]
			#[ allow (clippy::cast_precision_loss) ]
			#[ inline (always) ]
			fn qck_f64 (self) -> f64 { self as f64 }

			#[ allow (clippy::cast_possible_truncation) ]
			#[ inline (always) ]
			fn qck_i8 (self) -> i8 { self as i8 }

			#[ allow (clippy::cast_lossless) ]
			#[ allow (clippy::cast_possible_truncation) ]
			#[ inline (always) ]
			fn qck_i16 (self) -> i16 { self as i16 }

			#[ allow (clippy::cast_lossless) ]
			#[ allow (clippy::cast_possible_truncation) ]
			#[ inline (always) ]
			fn qck_i32 (self) -> i32 { self as i32 }

			#[ allow (clippy::cast_lossless) ]
			#[ allow (clippy::cast_possible_truncation) ]
			#[ inline (always) ]
			fn qck_i64 (self) -> i64 { self as i64 }

			#[ allow (clippy::cast_lossless) ]
			#[ inline (always) ]
			fn qck_i128 (self) -> i128 { self as i128 }

			#[ allow (clippy::cast_possible_truncation) ]
			#[ inline (always) ]
			fn qck_isize (self) -> isize { self as isize }

			#[ allow (clippy::cast_possible_truncation) ]
			#[ allow (clippy::cast_sign_loss) ]
			#[ inline (always) ]
			fn qck_u8 (self) -> u8 { self as u8 }

			#[ allow (clippy::cast_possible_truncation) ]
			#[ allow (clippy::cast_sign_loss) ]
			#[ inline (always) ]
			fn qck_u16 (self) -> u16 { self as u16 }

			#[ allow (clippy::cast_possible_truncation) ]
			#[ allow (clippy::cast_sign_loss) ]
			#[ inline (always) ]
			fn qck_u32 (self) -> u32 { self as u32 }

			#[ allow (clippy::cast_possible_truncation) ]
			#[ allow (clippy::cast_sign_loss) ]
			#[ inline (always) ]
			fn qck_u64 (self) -> u64 { self as u64 }

			#[ allow (clippy::cast_sign_loss) ]
			#[ inline (always) ]
			fn qck_u128 (self) -> u128 { self as u128 }

			#[ allow (clippy::cast_possible_truncation) ]
			#[ allow (clippy::cast_sign_loss) ]
			#[ inline (always) ]
			fn qck_usize (self) -> usize { self as usize }

		}

		impl IntConv for $unsigned {

			#[ inline (always) ]
			fn from_char (val: char) -> NumResult <Self> {
				val.qck_u32 ().try_into ().ok ().ok_or (Overflow)
			}

			#[ inline (always) ]
			fn from_u8 (val: u8) -> NumResult <Self> {
				val.try_into ().ok ().ok_or (Overflow)
			}

			#[ inline (always) ]
			fn from_usize (val: usize) -> NumResult <Self> {
				val.try_into ().ok ().ok_or (Overflow)
			}

			#[ inline (always) ]
			fn from_isize (val: isize) -> NumResult <Self> {
				val.try_into ().ok ().ok_or (Overflow)
			}

			#[ inline (always) ]
			fn to_char (self) -> NumResult <char> {
				self.to_u32 () ?.try_into ().ok ().ok_or (Overflow)
			}

			#[ inline (always) ]
			fn to_i8 (self) -> NumResult <i8> {
				self.try_into ().ok ().ok_or (Overflow)
			}

			#[ inline (always) ]
			fn to_i16 (self) -> NumResult <i16> {
				self.try_into ().ok ().ok_or (Overflow)
			}

			#[ inline (always) ]
			fn to_i32 (self) -> NumResult <i32> {
				self.try_into ().ok ().ok_or (Overflow)
			}

			#[ inline (always) ]
			fn to_i64 (self) -> NumResult <i64> {
				self.try_into ().ok ().ok_or (Overflow)
			}

			#[ inline (always) ]
			fn to_i128 (self) -> NumResult <i128> {
				self.try_into ().ok ().ok_or (Overflow)
			}

			#[ inline (always) ]
			fn to_isize (self) -> NumResult <isize> {
				self.try_into ().ok ().ok_or (Overflow)
			}

			#[ inline (always) ]
			fn to_u8 (self) -> NumResult <u8> {
				self.try_into ().ok ().ok_or (Overflow)
			}

			#[ inline (always) ]
			fn to_u16 (self) -> NumResult <u16> {
				self.try_into ().ok ().ok_or (Overflow)
			}

			#[ inline (always) ]
			fn to_u32 (self) -> NumResult <u32> {
				self.try_into ().ok ().ok_or (Overflow)
			}

			#[ inline (always) ]
			fn to_u64 (self) -> NumResult <u64> {
				self.try_into ().ok ().ok_or (Overflow)
			}

			#[ inline (always) ]
			fn to_u128 (self) -> NumResult <u128> {
				self.try_into ().ok ().ok_or (Overflow)
			}

			#[ inline (always) ]
			fn to_usize (self) -> NumResult <usize> {
				self.try_into ().ok ().ok_or (Overflow)
			}

			#[ allow (clippy::cast_lossless) ]
			#[ allow (clippy::cast_precision_loss) ]
			#[ inline (always) ]
			fn qck_f32 (self) -> f32 { self as f32 }

			#[ allow (clippy::cast_lossless) ]
			#[ allow (clippy::cast_possible_truncation) ]
			#[ allow (clippy::cast_precision_loss) ]
			#[ inline (always) ]
			fn qck_f64 (self) -> f64 { self as f64 }

			#[ allow (clippy::cast_possible_truncation) ]
			#[ allow (clippy::cast_possible_wrap) ]
			#[ inline (always) ]
			fn qck_i8 (self) -> i8 { self as i8 }

			#[ allow (clippy::cast_lossless) ]
			#[ allow (clippy::cast_possible_truncation) ]
			#[ allow (clippy::cast_possible_wrap) ]
			#[ inline (always) ]
			fn qck_i16 (self) -> i16 { self as i16 }

			#[ allow (clippy::cast_lossless) ]
			#[ allow (clippy::cast_possible_truncation) ]
			#[ allow (clippy::cast_possible_wrap) ]
			#[ inline (always) ]
			fn qck_i32 (self) -> i32 { self as i32 }

			#[ allow (clippy::cast_lossless) ]
			#[ allow (clippy::cast_possible_truncation) ]
			#[ allow (clippy::cast_possible_wrap) ]
			#[ inline (always) ]
			fn qck_i64 (self) -> i64 { self as i64 }

			#[ allow (clippy::cast_lossless) ]
			#[ allow (clippy::cast_possible_wrap) ]
			#[ inline (always) ]
			fn qck_i128 (self) -> i128 { self as i128 }

			#[ allow (clippy::cast_possible_truncation) ]
			#[ allow (clippy::cast_possible_wrap) ]
			#[ inline (always) ]
			fn qck_isize (self) -> isize { self as isize }

			#[ allow (clippy::cast_possible_truncation) ]
			#[ inline (always) ]
			fn qck_u8 (self) -> u8 { self as u8 }

			#[ allow (clippy::cast_lossless) ]
			#[ allow (clippy::cast_possible_truncation) ]
			#[ inline (always) ]
			fn qck_u16 (self) -> u16 { self as u16 }

			#[ allow (clippy::cast_lossless) ]
			#[ allow (clippy::cast_possible_truncation) ]
			#[ inline (always) ]
			fn qck_u32 (self) -> u32 { self as u32 }

			#[ allow (clippy::cast_lossless) ]
			#[ allow (clippy::cast_possible_truncation) ]
			#[ inline (always) ]
			fn qck_u64 (self) -> u64 { self as u64 }

			#[ inline (always) ]
			fn qck_u128 (self) -> u128 { self as u128 }

			#[ allow (clippy::cast_possible_truncation) ]
			#[ inline (always) ]
			fn qck_usize (self) -> usize { self as usize }

		}

		quick_from_impl! ($signed, i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);
		quick_from_impl! ($unsigned, i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);

	};
}

macro_rules! quick_from_impl {
	( $target:ident, $( $source:ident ),* ) => {
		$(

			impl QuickFrom <$source> for $target {

				#[ allow (clippy::cast_lossless) ]
				#[ allow (clippy::cast_possible_truncation) ]
				#[ allow (clippy::cast_possible_wrap) ]
				#[ allow (clippy::cast_sign_loss) ]
				#[ inline (always) ]
				fn quick_from (arg: $source) -> Self {
					arg as $target
				}

			}

		)*
	};
}

int_conv_impl! (i8, u8, 8);
int_conv_impl! (i16, u16, 16);
int_conv_impl! (i32, u32, 32);
int_conv_impl! (i64, u64, 64);
int_conv_impl! (i128, u128, 128);
int_conv_impl! (isize, usize, 128);
