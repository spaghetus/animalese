use std::io::BufRead;

use include_dir::{include_dir, Dir};
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink, Source};
const VOICES: Dir = include_dir!("voice");

pub struct Talker {
	clips: [Vec<u8>; 26],
}

impl Talker {
	pub fn new() -> Talker {
		let voice: usize = std::env::var("VOICE")
			.unwrap_or("425".to_string())
			.parse()
			.expect("invalid voice offset");
		eprintln!("Using voice: {}\nArranging voice sounds", voice);
		let clips: Vec<(usize, Vec<u8>)> = VOICES
			.files()
			.map(|file| {
				let name = file.path().file_stem().unwrap().to_string_lossy();
				let number = name.split(' ').next().unwrap().parse::<usize>().unwrap();
				(file, name, number)
			})
			.filter(|(_file, _name, number)| number >= &voice && number <= &(voice + 26))
			.map(|(file, _, number)| (number - voice, file.contents().to_vec()))
			.collect();
		// coerce this into a sized array; we know it's exactly 26 elements so this is safe
		let mut clips_: [Vec<u8>; 26] = Default::default();
		for i in 0..26 {
			clips_[i] = clips[i].1.clone();
		}
		Talker { clips: clips_ }
	}
	pub fn say(&self, c: char, stream_handle: &OutputStreamHandle) {
		if !c.is_alphabetic() {
			return;
		};
		let c = c.to_lowercase().next().unwrap();
		let file = self.clips[c as usize - 'a' as usize].clone();
		let source = Decoder::new(std::io::Cursor::new(file)).unwrap();
		let sink = Sink::try_new(&stream_handle).unwrap();
		sink.append(source);
		sink.detach()
	}
}
