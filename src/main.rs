extern crate rand;
use rand::Rng;

mod image;

fn set_random_data(img: &mut image::Image) {
	let size = img.header.width*img.header.height;
	let mut rng = rand::thread_rng();
	img.data.clear();
	img.data.reserve(size);
	for i in 0..size {
		img.data.push(rng.gen::<u16>());
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
}
