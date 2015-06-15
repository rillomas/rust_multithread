extern crate std;
use std::io::prelude::*;
use std::io::BufWriter;
use std::fs::File;
use std::mem;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ImageFormat {
	GrayScale,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ImageHeader {
	pub width: usize,
	pub height: usize,
	pub format: ImageFormat,
}

pub struct Image {
	pub header: ImageHeader,
	pub data: Vec<u16>,
}

impl Image {
	pub fn new(width: usize, height: usize, format: ImageFormat) -> Image {
		Image {
			header: ImageHeader {
				width: width,
				height: height,
				format: format,
			},
			data: vec![0; width*height],
		}
	}

	/// Write raw image content to the specified path
	pub fn write_to_file(&self, path: &str) -> std::io::Result<()> {
		let f = try!(File::create(path));
		let mut bw = BufWriter::new(f);
		// println!("before length: {}", self.data.len());
		let sl: &[u8];
		unsafe {
			// convert u16 slice to u8 slice without copying
			sl = std::slice::from_raw_parts(
				self.data.as_ptr() as *const u8,
				self.data.len() * mem::size_of::<u16>());
		}
		// println!("after length: {}", sl.len());
		try!(bw.write_all(sl));
		Ok(())
	}

	/// Write image content as PGM to the specified path
	pub fn write_as_pgm(&self, path: &str) -> std::io::Result<()> {
		let f = try!(File::create(path));
		let mut bw = BufWriter::new(f);
		// write format
		try!(write!(bw,"P2\n"));
		// write size
		try!(write!(bw, "{} {}\n", self.header.width, self.header.height));
		// write maximum brightness
		try!(write!(bw, "{}\n", 65535)); 
		// write all data
		for d in &self.data[..] {
			try!(write!(bw, "{} ", d));
		}
		Ok(())
	}

}
