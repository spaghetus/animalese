use std::time::Duration;

use rodio::{OutputStream, Sink};

use crate::talk::Talker;

pub mod talk;

fn main() {
	eprintln!("Load model...");
	let model = coqui_stt::Model::new("./model.pbmm").unwrap();
	let talker = Talker::new();
	let (_stream, stream_handle) = OutputStream::try_default().unwrap();
	std::thread::sleep(Duration::from_millis(1000));
	let test = "This is a test string, with SpEcIaL *$^#(*^$#* characters.";
	for c in test.chars() {
		talker.say(c, &stream_handle);
		std::thread::sleep(Duration::from_millis(75));
	}
	std::thread::sleep(Duration::from_millis(500));
}
