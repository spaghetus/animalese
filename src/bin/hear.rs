use std::{
	io::{BufRead, Read, Write},
	thread,
	time::{Duration, Instant},
};

use animalese::hear::Hearer;

const NEWLINE_TIME: Duration = Duration::from_secs(1);

fn main() {
	let mut so_far: String = String::new();
	let mut last_new: Instant = Instant::now();
	let hearer = Hearer::new();
	let mut useless = [0u8; 1];
	loop {
		let new_inference = hearer.current_inference.lock().unwrap().clone();
		let same = so_far
			.chars()
			.zip(new_inference.chars())
			.take_while(|(a, b)| a == b)
			.count();
		let backtrack = so_far.len() - same;
		so_far = new_inference;
		let mut out = std::io::stdout();
		out.write([27u8].repeat(backtrack).as_slice()).unwrap();
		out.write(so_far[(same)..].as_bytes()).unwrap();
		out.flush().unwrap();
		thread::sleep(Duration::from_millis(100));
	}
	hearer.thread.join().unwrap();
}
