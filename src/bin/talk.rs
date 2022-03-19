use std::{
	io::{BufRead, Read},
	time::Duration,
};

use rodio::{OutputStream, Sink};

use animalese::talk::Talker;

fn main() {
	let talker = Talker::new();
	let (_stream, stream_handle) = OutputStream::try_default().unwrap();
	let mut input = [0u8; 1];
	while matches!(std::io::stdin().lock().read(&mut input[..]), Ok(1)) {
		for c in input {
			talker.say(c as char, &stream_handle);
			std::thread::sleep(Duration::from_millis(75));
		}
	}
}
