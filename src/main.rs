extern crate rand;
use rand::Rng;

mod image;

fn set_random_data(img: &mut image::Image) {
	let mut rng = rand::thread_rng();
	let ref mut data = img.data;
	for i in 0..data.len() {
		data[i] = rng.gen::<u16>();
	}
}

fn average_filter(input: &image::Image, kernel_size: usize, output: &mut image::Image) {
	assert!(input.header == output.header);
	let hdr = input.header;
	let xl = hdr.width / kernel_size;
	let yl = hdr.height / kernel_size;
	for y in 0..yl {
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
			let top_idx = yidx*hdr.width + xidx;
			output.data[top_idx] = avg;
			output.data[top_idx + 1] = avg;
			let btm_idx = (yidx + 1)*hdr.width + xidx;
			output.data[btm_idx] = avg;
			output.data[btm_idx + 1] = avg;
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

	// img.write_to_file("before.bin").unwrap();
	let before = "before.pgm";
	img.write_as_pgm(before)
		.unwrap_or_else(|e| panic!("Error while writing to {}: {}", before, e));

	// apply average filter
	let mut tmp = image::Image::new(w, h, image::ImageFormat::GrayScale);
	average_filter(&img, 2, &mut tmp);

	// tmp.write_to_file("after.bin").unwrap();
	let after = "after.pgm";
	tmp.write_as_pgm(after)
		.unwrap_or_else(|e| panic!("Error while writing to {}: {}", after, e));
}
