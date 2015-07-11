// #![feature(scoped)]

extern crate rand;
extern crate time;
extern crate rustc_serialize;
// extern crate core;
use rand::Rng;
// use std::error::Error;
// use std::rc::Rc;
// use std::sync::{Arc, Mutex};
// use std::thread;
// use std::ops::Deref;
// use std::ops::DerefMut;
use std::sync::mpsc::{Sender, Receiver};
use std::thread::JoinHandle;

mod image;

#[derive(Debug)]
enum Message {
	Exit,
	AverageFilter,
}

struct Thread<T> {
	id : usize,
	sender: Sender<T>,
	handle: JoinHandle<()>,
}

struct ProcessMessage {
	message: Message,
	// input: &'a Vec<u16>,
	// outout: &'b [u16],
	slice_width: usize,
	slice_height: usize,
	slice_start_y: usize,
	kernel_size: usize,
}

impl ProcessMessage {
	fn new(msg_type: Message, input: &Vec<u16>, slice_width: usize, slice_height: usize, slice_start_y: usize, kernel_size: usize) -> ProcessMessage {
		ProcessMessage {
			message: msg_type,
			// input: input,
			slice_width: slice_width,
			slice_height: slice_height,
			slice_start_y : slice_start_y,
			kernel_size : kernel_size,
		}
	}

	fn exit() -> ProcessMessage {
		ProcessMessage {
			message: Message::Exit,
			// input: &Vec::new(),
			slice_width: 0,
			slice_height: 0,
			slice_start_y: 0,
			kernel_size : 0,
		}
	}
}

struct ImageProcessor {
	thread_num: usize,
	threads: Vec<Thread<ProcessMessage>>,
}

fn process_image_loop(thread_id: usize, r: Receiver<ProcessMessage>) {
	loop {
		let msg = r.recv().unwrap();
		println!("[{}] Received message: {:?}", thread_id, msg.message);
		match msg.message {
			Message::Exit => {
				println!("[{}] Exiting thread", thread_id);
				break;
			},
			Message::AverageFilter => {
				// process average filter
				println!("[{}] Executing average filter", thread_id);
			}
		}
	}
}

impl ImageProcessor {
	pub fn new(thread_num: usize) -> ImageProcessor {
		let mut threads = Vec::new();
		// spawn all threads
		for i in 0..thread_num {
			let (s, r) = std::sync::mpsc::channel();
			let h = std::thread::spawn(move || process_image_loop(i, r));
			let t = Thread {
				id : i,
				sender : s,
				handle : h,
			};
			threads.push(t);
		}
		ImageProcessor {
			thread_num : thread_num,
			threads : threads,
		}
	}

	pub fn average_filter(&self, input: &image::Image, kernel_size: usize, output: &mut image::Image) {
		let hdr = input.header;
		assert!(hdr == output.header);
		assert!((hdr.width % kernel_size) == 0);
		assert!((hdr.height % kernel_size) == 0);
		// divide image vertically to slices
		let height_per_slice = hdr.height / self.thread_num;
		let size_per_chunk = height_per_slice * hdr.width;
		let out_itr = output.data.chunks_mut(size_per_chunk);
		let mut i = 0;
		for (output, thread) in out_itr.zip(&self.threads) {
			let start_y = i*height_per_slice;
			let msg = ProcessMessage::new(Message::AverageFilter, &input.data, hdr.width, height_per_slice, start_y, kernel_size);
			thread.sender.send(msg);
			i += 1;
		}
		// for (input, output) in in_itr.zip(out_itr) {
		// 	average_filter_chunk(input, hdr.width, height_per_slice, kernel_size, output);
		// }
	}

	pub fn join(self) {
		// stop all threads and join
		for t in self.threads {
			t.sender.send(ProcessMessage::exit()).unwrap();
			t.handle.join().unwrap();
		}
		println!("Finished joining threads");
	}
}

fn set_random_data(img: &mut image::Image) {
	let mut rng = rand::thread_rng();
	let ref mut data = img.data;
	for i in 0..data.len() {
		data[i] = rng.gen::<u16>();
	}
}

/// apply average filter in single thread
fn average_filter(input: &image::Image, kernel_size: usize, output: &mut image::Image) {
	let hdr = input.header;
	assert!(hdr == output.header);
	assert!((hdr.width % kernel_size) == 0);
	assert!((hdr.height % kernel_size) == 0);
	let ex = hdr.width / kernel_size;
	let ey = hdr.height / kernel_size;
	for y in 0..ey {
		let yidx = y *kernel_size;
		for x in 0..ex {
			let xidx = x * kernel_size;
			let mut sum : u32 = 0;
			for ky in 0..kernel_size {
				let row_start = (yidx+ky) * hdr.width;
				for kx in 0..kernel_size {
					let idx = row_start + (xidx + kx);
					sum += input.data[idx] as u32;
				}
			}
			let avg = (sum / ((kernel_size*kernel_size) as u32)) as u16;
			for ky in 0..kernel_size {
				let row_start = (yidx+ky) * hdr.width;
				for kx in 0..kernel_size {
					let idx = row_start + (xidx + kx);
					output.data[idx] = avg;
				}
			}
		}
	}
}

/// Perfrom an average filter using the given input and kernel size
/// example when kernel_size is two :
///   input        output
///  1 2 3 4  ->  3 3 5 5 
///  5 6 7 8  ->  3 3 5 5
fn average_filter_chunk(input: &[u16], slice_width: usize, slice_height: usize, kernel_size: usize, output: &mut[u16]) {
	let ex = slice_width / kernel_size;
	let ey = slice_height / kernel_size;
	for y in 0..ey {
		let yidx = y * kernel_size;
		for x in 0..ex {
			let xidx = x * kernel_size;
			let mut sum : u32 = 0;
			for ky in 0..kernel_size {
				let row_start = (yidx+ky) * slice_width;
				for kx in 0..kernel_size {
					let idx = row_start + (xidx + kx);
					sum += input[idx] as u32;
				}
			}
			let avg = (sum / ((kernel_size*kernel_size) as u32)) as u16;
			for ky in 0..kernel_size {
				let row_start = (yidx+ky) * slice_width;
				for kx in 0..kernel_size {
					let idx = row_start + (xidx + kx);
					output[idx] = avg;
				}
			}
		}
	}
}

// fn average_filter_multi(input: &image::Image, kernel_size: usize, slice_num: usize, output: &mut image::Image) {
// 	let hdr = input.header;
// 	assert!(hdr == output.header);
// 	assert!((hdr.width % kernel_size) == 0);
// 	assert!((hdr.height % kernel_size) == 0);
// 	let st = time::get_time();
// 	// divide image vertically to slices
// 	let height_per_slice = hdr.height / slice_num;
// 	let size_per_chunk = height_per_slice * hdr.width;
// 	let in_itr = input.data.chunks(size_per_chunk);
// 	let out_itr = output.data.chunks_mut(size_per_chunk);
// 	// for (input, output) in in_itr.zip(out_itr) {
// 	// 	average_filter_chunk(input, hdr.width, height_per_slice, kernel_size, output);
// 	// }
// 	let mut handles = Vec::new();
// 	for (input, output) in in_itr.zip(out_itr) {
// 		let h = std::thread::scoped(move || {
// 			average_filter_chunk(input, hdr.width, height_per_slice, kernel_size, output);
// 		});
// 		handles.push(h);
// 	}
// 	for handle in handles {
// 		handle.join();
// 	}
// 	let et = time::get_time();
// 	let diff = et - st;
// 	println!("Time: {} msec", diff.num_milliseconds());
// }

fn main() {
	let w: usize = 1024;
	let h: usize = 1024;
	let mut img = image::Image::new(w, h, image::ImageFormat::GrayScale);
	let hdr = img.header;
	let msg = format!("img: {:4}x{:4} {:?}", hdr.width, hdr.height, hdr.format);
	println!("{}", msg);
	set_random_data(&mut img);

	let ip = ImageProcessor::new(4);

	// let before = "before.pgm";
	// img.write_as_pgm(before)
	// 	.unwrap_or_else(|e| panic!("Error while writing to {}: {:?}", before, e));

	// apply average filter
	let mut tmp = image::Image::new(w, h, image::ImageFormat::GrayScale);
	ip.average_filter(&img, 4, &mut tmp);
	// average_filter_multi(&img, 4, 4, &mut tmp);
	// average_filter(&img, 2, &mut tmp);

	// let after = "after.pgm";
	// tmp.write_as_pgm(after)
	// 	.unwrap_or_else(|e| panic!("Error while writing to {}: {:?}", after, e));
	ip.join();
}
