use std::{
	collections::VecDeque,
	io::Read,
	sync::{Arc, Mutex},
	time::{Duration, Instant},
};

use coqui_stt::{Model, Stream};
use rodio::{
	cpal::{
		traits::{HostTrait, StreamTrait},
		SampleFormat, SampleRate,
	},
	DeviceTrait,
};

const INFER_INTERVAL: Duration = Duration::from_millis(500);

pub struct Hearer {
	pub thread: std::thread::JoinHandle<()>,
	pub current_inference: Arc<Mutex<String>>,
}

impl Hearer {
	pub fn new() -> Hearer {
		let current_inference = Arc::new(Mutex::new(String::new()));
		let hearer = Hearer {
			thread: {
				let current_inference = current_inference.clone();
				std::thread::spawn(|| Hearer::infer_thread(current_inference))
			},
			current_inference: current_inference.clone(),
		};
		let current_inference = hearer.current_inference.clone();
		hearer
	}
	pub fn go(&mut self) {}
	pub fn infer_thread(inference: Arc<Mutex<String>>) {
		fn build_stream() -> Stream {
			let mut model = Model::new("./model.tflite").expect("couldn't build model");
			// let dict: Vec<&str> = include_str!("/usr/share/dict/words").lines().collect();
			// for word in dict {
			// 	model.add_hot_word(word, 1.0).unwrap();
			// }
			let stream = model
				.into_streaming()
				.expect("couldn't set up streaming inference");
			stream
		}
		let mut stream = build_stream();
		let mut last_inference = Instant::now();
		let mut last_new = Instant::now();
		// set up the audio stream
		let queue: Arc<Mutex<VecDeque<i16>>> = Arc::new(Mutex::new(VecDeque::new()));
		let audio_in = {
			let host = rodio::cpal::default_host();
			let device = host.default_input_device().expect("no input device found");
			let config = device
				.supported_input_configs()
				.unwrap()
				.filter(|c| {
					c.channels() == 1
						&& c.sample_format() == SampleFormat::I16
						&& c.min_sample_rate().0 <= 16000
						&& c.max_sample_rate().0 >= 16000
				})
				.map(|c| c.with_sample_rate(SampleRate(16000)))
				.next()
				.expect("no supported config found");
			let queue = queue.clone();
			let stream = device
				.build_input_stream(
					&config.into(),
					{
						let queue = queue.clone();
						move |d, i| queue.lock().unwrap().extend(d)
					},
					|e| eprintln!("{:#?}", e),
				)
				.expect("couldn't build input stream");
			stream.play().expect("couldn't start stream");
			stream
		};
		loop {
			// feed audio to the model
			while last_inference.elapsed() < INFER_INTERVAL {
				let slice = {
					let mut queue = queue.lock().unwrap();
					let slices = queue.as_slices();
					let out = slices
						.0
						.iter()
						.chain(slices.1.iter())
						.map(|&v| v)
						.collect::<Vec<i16>>();
					queue.clear();
					out
				};
				stream.feed_audio(&slice);
			}
			// run an inference
			last_inference = Instant::now();
			let mut new_inference = stream.intermediate_decode().expect("couldn't decode");
			// if inference.lock().unwrap().len() != new_inference.len() {
			// 	last_new = Instant::now();
			// }
			// if last_new.elapsed() > INFER_INTERVAL * 2 {
			// 	new_inference = stream
			// 		.finish_stream()
			// 		.expect("couldn't finish stream")
			// 		.to_string();
			// 	stream = build_stream()
			// }
			*inference.lock().unwrap() = new_inference.trim().to_string();
		}
	}
}
