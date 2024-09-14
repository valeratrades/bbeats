use std::{f32::consts::PI, thread, time::Duration};

use clap::Parser;
use rodio::{source::Source, OutputStream, Sink};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
	#[arg(long, short, default_value = "41.0")]
	left_freq: f32,
	#[arg(long, short, default_value = "39.0")]
	right_freq: f32,
}

struct StereoSineWave {
	pub left_freq: f32,
	pub right_freq: f32,
	pub sample_rate: u32,
	left_phase: f32,
	right_phase: f32,
	alt: bool, // Used to alternate between left and right channels
}

impl StereoSineWave {
	fn new(left_freq: f32, right_freq: f32, sample_rate: u32) -> Self {
		Self {
			left_phase: 0.0,
			right_phase: 0.0,
			left_freq,
			right_freq,
			sample_rate,
			alt: false,
		}
	}
}

impl Iterator for StereoSineWave {
	type Item = f32;

	fn next(&mut self) -> Option<Self::Item> {
		let sample = if !self.alt {
			// Left channel
			let left_sample = (2.0 * PI * self.left_phase).sin();
			self.left_phase += self.left_freq / self.sample_rate as f32;
			if self.left_phase >= 1.0 {
				self.left_phase -= 1.0;
			}
			left_sample
		} else {
			// Right channel
			let right_sample = (2.0 * PI * self.right_phase).sin();
			self.right_phase += self.right_freq / self.sample_rate as f32;
			if self.right_phase >= 1.0 {
				self.right_phase -= 1.0;
			}
			right_sample
		};

		self.alt = !self.alt;
		Some(sample)
	}
}

impl Source for StereoSineWave {
	fn current_frame_len(&self) -> Option<usize> {
		None
	}

	fn channels(&self) -> u16 {
		2
	}

	fn sample_rate(&self) -> u32 {
		self.sample_rate
	}

	fn total_duration(&self) -> Option<Duration> {
		None
	}
}

fn main() {
	let cli = Cli::parse();

	let (_stream, stream_handle) = OutputStream::try_default().unwrap();
	let sink = Sink::try_new(&stream_handle).unwrap();

	let sample_rate = 44100;
	let source = StereoSineWave::new(cli.left_freq, cli.right_freq, sample_rate);

	sink.append(source);

	thread::sleep(Duration::from_secs(u64::MAX));
}
