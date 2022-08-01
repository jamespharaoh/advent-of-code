//! Shared build script for years and days

use std::error::Error;

/// Entry point, calles [`aoc_codegen::invoke`]
///
fn main () -> Result <(), Box <dyn Error>> {
	aoc_codegen::invoke ()
}
