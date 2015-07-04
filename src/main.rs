extern crate rand;
extern crate time;
use rand::Rng;

fn set_random_data(target: &mut Vec<u16>) {
	let mut rng = rand::thread_rng();
	for i in 0..target.len() {
		target[i] = rng.gen::<u16>();
	}
}

/// an example of a very simple filter
fn modular_filter_chunk(input: &[u16], slice_width: usize, slice_height: usize, mod_value: u16, output: &mut[u16]) {
	let size = slice_width*slice_height;
	for i in 0..size {
		output[i] = input[i] % mod_value;
	}
}

fn modular_filter_multi(input: &Vec<u16>, width: usize, height: usize, slice_num: usize, mod_value: u16, output: &mut Vec<u16>) {
	let st = time::get_time();
	// divide image vertically to slices
	let height_per_slice = height / slice_num;
	let size_per_chunk = height_per_slice * width;
	let in_itr = input.chunks(size_per_chunk);
	let out_itr = output.chunks_mut(size_per_chunk);
	for (input, output) in in_itr.zip(out_itr) {
		modular_filter_chunk(input, width, height_per_slice, mod_value, output);
	}
	// let mut handles = Vec::new();
	// for (input, output) in in_itr.zip(out_itr) {
	// 	let h = std::thread::spawn(move || {
	// 		modular_filter_chunk(input, width, height_per_slice, mod_value, output);
	// 	});
	// 	handles.push(h);
	// }
	// for handle in handles {
	// 	handle.join().unwrap();
	// }
	let et = time::get_time();
	let diff = et - st;
	println!("Time: {} msec", diff.num_milliseconds());
}

fn main() {
	let width: usize = 1024;
	let height: usize = 1024;
	let mut input = vec![0; width*height];
	set_random_data(&mut input);
	let mut output = vec![0; width*height];
	modular_filter_multi(&input, width, height, 4, 73, &mut output);
}
