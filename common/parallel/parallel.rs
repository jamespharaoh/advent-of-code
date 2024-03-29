use std::cmp;
use std::collections::VecDeque;
use std::fmt;
use std::fs::File;
use std::io::BufRead as _;
use std::io::BufReader;
use std::ops::Deref;
use std::process;
use std::sync::Arc;
use std::sync::Condvar;
use std::sync::Mutex;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::thread;
use std::thread::JoinHandle;

pub mod prelude {
	pub use super::IteratorThreadMap;
	pub use super::ThreadMap;
}

pub trait IteratorThreadMap: Iterator {

	#[ inline ]
	fn thread_map <MapFn, Out> (
		self,
		num_threads: usize,
		map_fn: MapFn,
	) -> ThreadMap <Self, Out>
		where
			MapFn: Fn (Self::Item) -> Out + Clone + Send + 'static,
			Out: Clone + Send + 'static,
			Self: Send + Sized + 'static {
		ThreadMap::start (self, map_fn, num_threads)
	}

}

impl <Iter> IteratorThreadMap for Iter where Iter: Iterator {
}

pub struct ThreadMap <Inner, Out> {
	shared: Arc <ThreadMapShared <Inner, Out>>,
	threads: Vec <JoinHandle <()>>,
}

struct ThreadMapShared <Inner, Out> {
	state: Mutex <ThreadMapState <Inner, Out>>,
	queue_push_cond: Condvar,
	queue_pop_cond: Condvar,
	queue_size: usize,
}

struct ThreadMapState <Inner, Out> {
	inner: Inner,
	queue: VecDeque <Arc <Mutex <Option <Out>>>>,
	finished: bool,
}

impl <Inner, Out> ThreadMap <Inner, Out>
	where Inner: Iterator + Send + 'static, Out: Clone + Send + 'static {

	#[ inline ]
	pub fn start <MapFn: Fn (Inner::Item) -> Out + Clone + Send + 'static> (
		inner: Inner,
		map_fn: MapFn,
		num_threads: usize,
	) -> Self {

		let state = Mutex::new (ThreadMapState {
			inner,
			queue: VecDeque::new (),
			finished: false,
		});

		let shared = Arc::new (ThreadMapShared {
			state,
			queue_push_cond: Condvar::new (),
			queue_pop_cond: Condvar::new (),
			queue_size: num_threads * 2,
		});

		let threads = (0 .. num_threads).map (|_| {
			let map_fn = map_fn.clone ();
			let shared = Arc::clone (& shared);
			thread::spawn (move || Self::worker (shared, map_fn))
		}).collect ();

		Self { shared, threads }

	}

	#[ allow (clippy::needless_pass_by_value) ]
	fn worker <MapFn> (
		shared: Arc <ThreadMapShared <Inner, Out>>,
		map_fn: MapFn,
	) where MapFn: Fn (Inner::Item) -> Out + Clone + Send + 'static {
		loop {

			let mut state = shared.state.lock ().unwrap ();

			let input_opt = loop {
				if state.finished { return }
				if state.queue.len () < shared.queue_size {
					break state.inner.next ();
				}
				state = shared.queue_pop_cond.wait (state).unwrap ();
			};

			let input = if let Some (input) = input_opt { input } else {
				state.finished = true;
				state.queue.push_back (Arc::new (Mutex::new (None)));
				return;
			};

			let output_mutex = Arc::new (Mutex::new (None));
			let mut output_lock = output_mutex.lock ().unwrap ();

			state.queue.push_back (Arc::clone (& output_mutex));
			shared.queue_push_cond.notify_one ();

			drop (state);

			* output_lock = Some (map_fn (input));

		}
	}

}

impl <Inner, Out> Drop for ThreadMap <Inner, Out> {

	#[ inline ]
	fn drop (& mut self) {

		let mut state = self.shared.state.lock ().unwrap ();
		state.finished = true;
		self.shared.queue_push_cond.notify_all ();
		self.shared.queue_pop_cond.notify_all ();
		drop (state);

		for handle in self.threads.drain ( .. ) {
			handle.join ().unwrap ();
		}

	}

}

impl <Inner, Out> Iterator for ThreadMap <Inner, Out>
	where Out: Clone {

	type Item = Out;

	#[ inline ]
	fn next (& mut self) -> Option <Out> {

		let mut state = self.shared.state.lock ().unwrap ();

		let output_mutex = loop {
			if let Some (output) = state.queue.pop_front () { break output }
			state = self.shared.queue_push_cond.wait (state).unwrap ();
		};

		self.shared.queue_pop_cond.notify_one ();

		drop (state);

		let output = output_mutex.lock ().unwrap ().deref ().clone ();
		output

	}

}

#[ inline ]
pub fn num_cpus_max <Val> (max: Val) -> Val
	where
		Val: Ord + TryFrom <usize> + TryInto <usize>,
		<Val as TryInto <usize>>::Error: fmt::Debug,
		<Val as TryFrom <usize>>::Error: fmt::Debug {
	let max: usize = max.try_into ().unwrap ();
	if max < 2 { return 1.try_into ().unwrap () }
	cmp::min (num_cpus ().unwrap_or (1), max).try_into ().unwrap ()
}

#[ inline ]
pub fn num_cpus () -> Option <usize> {

	static CACHE: AtomicUsize = AtomicUsize::new (0);
	let cached_value = CACHE.load (Ordering::Relaxed);
	if cached_value != 0 { return Some (cached_value) }

	let num_threads = num_cpus_real ().unwrap_or (1);
	CACHE.store (num_threads, Ordering::Relaxed);

	Some (num_threads)

}

fn num_cpus_real () -> Option <usize> {
	None
		.or_else (num_cpus_allowed)
		.or_else (num_cpus_cpuinfo)
}

fn num_cpus_allowed () -> Option <usize> {

	let path = format! ("/proc/{}/status", process::id ());
	let file = File::open (path).ok () ?;
	let reader = BufReader::new (file);

	for line in reader.lines () {
		let line = line.ok () ?;
		if let Some (mask_str) = line.strip_prefix ("Cpus_allowed:\t") {
			let mask = u128::from_str_radix (mask_str, 16).ok () ?;
			return Some (mask.count_ones ().try_into ().unwrap ());
		}
	}

	None

}

fn num_cpus_cpuinfo () -> Option <usize> {

	let file = File::open ("/proc/cpuinfo").ok () ?;
	let reader = BufReader::new (file);

	let mut num_threads = 0;
	for line in reader.lines () {
		let line = line.ok () ?;
		if ! line.starts_with ("processor\t: ") { continue }
		num_threads += 1;
	}

	Some (num_threads)

}
