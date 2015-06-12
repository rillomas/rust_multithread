extern crate rand;
use rand::Rng;

mod image;

fn set_random_data(img: &mut image::Image<[u16]>) {
	let mut rng = rand::thread_rng();
	let ref mut data = img.data;
	for i in 0..data.len() {
		data[i] = rng.gen::<u16>();
	}
}

fn main() {
	let w: usize = 1024;
	let h: usize = 1024;
	//let size = w * h;
	//let data = [0; size];
	let data: [u16; 4] = [1,2,3,4];
	//let data = vec![0; w*h];
	let mut img = image::Image::<[u16]>::new(w, h, image::ImageFormat::GrayScale, data);
	let hdr = img.header;
	let msg = format!("img: {:4}x{:4} {:?}", hdr.width, hdr.height, hdr.format);
	println!("{}", msg);
	set_random_data(&mut img);
}
