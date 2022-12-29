use super::*;

use std::time::Instant;

fn percentile (times: & [u64], num: u64, denom: u64) -> u64 {
	let size = times.len ().pan_u64 () - 1;
	let idx: u64 = num * size / denom;
	let rem = num * size % denom;
	if rem == 0 { return times [idx.pan_usize ()] }
	times [idx.pan_usize ()] * (denom - rem) / denom
		+ times [idx.pan_usize () + 1] * rem / denom
}

#[ allow (clippy::print_stdout) ]
pub (crate) fn runner (
	repeat: u64,
	mut inner_fn: impl FnMut (u64) -> GenResult <()>,
) -> GenResult <()> {
	let times = {
		let mut times: Vec <_> = (0 .. repeat)
			.map (|idx| { inner_fn (idx) ?; Ok (Instant::now ()) })
			.scan (Instant::now (), |state, cur|
				Some (cur.map (|cur| cur - mem::replace (state, cur))))
			.map_ok (|duration| duration.as_micros ().pan_u64 ())
			.collect::<GenResult <_>> () ?;
		times.sort_unstable ();
		times
	};
	if repeat == 1 { return Ok (()) }
	let total = times.iter ().map (|& val| val.pan_u128 ()).sum::<u128> ();
	let mean = (total / repeat.pan_u128 ()).pan_u64 ();
	let disp_float = |val, ref_val|
		if ref_val >= 2_000_000_f64 { format! ("{:.3}s", val / 1_000_000_f64) }
		else if ref_val >= 2_000_f64 { format! ("{:.3}ms", val / 1_000_f64) }
		else { format! ("{val:.0}µs") };
	let disp = |val: u128| disp_float (val.pan_f64 (), val.pan_f64 ());
	let disp_mean = |val: u64| disp_float (val.pan_f64 (), mean.pan_f64 ());
	let disp_pc = |pc| disp_float (percentile (& times, pc, 1000).pan_f64 (), mean.pan_f64 ());
	print! ("Statistics: total={} count={} mean={},", disp (total), repeat, disp_mean (mean));
	const PERCENTILE_OPTIONS: & [(u64, & [u64])] = & [
		(1000, & [0, 500, 900, 990, 999, 1000]),
		(100, & [0, 500, 900, 990, 1000]),
		(25, & [0, 500, 750, 900, 1000]),
		(10, & [0, 500, 900, 1000]),
		(0, & []),
	];
	for (min_repeat, percentiles) in PERCENTILE_OPTIONS.iter ().copied () {
		if repeat < min_repeat * 2 { continue }
		for percentile in percentiles.iter ().copied () {
			if percentile % 10 == 0 {
				print! (" p{}={}", percentile / 10, disp_pc (percentile));
			} else {
				print! (" p{}={}", percentile.pan_f64 () / 10.0_f64, disp_pc (percentile));
			}
		}
		if percentiles.is_empty () {
			print! (" min={} median={} max={}", disp_pc (0), disp_pc (500), disp_pc (1000));
		}
		break;
	}
	print! ("\n");
	Ok (())
}
