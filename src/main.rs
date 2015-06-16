extern crate rand;
extern crate time;
extern crate rustc_serialize;
use rand::Rng;
use std::error::Error;

mod image;

fn set_random_data(img: &mut image::Image) {
	let mut rng = rand::thread_rng();
	let ref mut data = img.data;
	for i in 0..data.len() {
		data[i] = rng.gen::<u16>();
	}
}

struct FilterParameter {
	kernel_size : usize,
	start_y : usize, // start and end must be multiples of kernel_size
	end_y : usize, // start and end must be multiples of kernel_size
}

/// apply average filter in single thread
fn average_filter(input: &image::Image, fp: FilterParameter, output: &mut image::Image) {
	let hdr = input.header;
	assert!(hdr == output.header);
	assert!((hdr.width % fp.kernel_size) == 0);
	assert!((hdr.height % fp.kernel_size) == 0);
	assert!((fp.start_y % fp.kernel_size) == 0);
	assert!((fp.end_y % fp.kernel_size) == 0);
	let kernel_size = fp.kernel_size;
	let xl = hdr.width / kernel_size;
	// let yl = hdr.height / kernel_size;
	// divide image vertically to slices
	let sy = fp.start_y / kernel_size;
	let ey = fp.end_y / kernel_size;
	for y in sy..ey {
		let yidx = y *kernel_size;
		for x in 0..xl {
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

fn main() {
	let w: usize = 1024;
	let h: usize = 1024;
	let mut img = image::Image::new(w, h, image::ImageFormat::GrayScale);
	let hdr = img.header;
	let msg = format!("img: {:4}x{:4} {:?}", hdr.width, hdr.height, hdr.format);
	println!("{}", msg);
	set_random_data(&mut img);

	//img.write_to_file("before.bin").unwrap();
	let before = "before.pgm";
	img.write_as_pgm(before)
		.unwrap_or_else(|e| panic!("Error while writing to {}: {:?}", before, e));
	// let before = "before.rcbin";
	// img.encode(before)
	// 	.unwrap_or_else(|e| panic!("Error while writing to {}: {}", before, e));

	// apply average filter
	let mut tmp = image::Image::new(w, h, image::ImageFormat::GrayScale);
	let fp = FilterParameter {
		kernel_size: 2,
		start_y : 0,
		end_y : h,
	};
	let st = time::get_time();
	average_filter(&img, fp, &mut tmp);
	let et = time::get_time();
	let diff = et - st;
	println!("Time: {} msec", diff.num_milliseconds());

	//tmp.write_to_file("after.bin").unwrap();
	let after = "after.pgm";
	tmp.write_as_pgm(after)
		.unwrap_or_else(|e| panic!("Error while writing to {}: {:?}", after, e));
	// let after = "after.rcbin";
	// tmp.encode(after)
	// 	.unwrap_or_else(|e| panic!("Error while writing to {}: {}", after, e));
}
