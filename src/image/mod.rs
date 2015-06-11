#[derive(Debug, Copy, Clone)]
pub enum ImageFormat {
	GrayScale,
	Rgb32,
}

#[derive(Debug, Copy, Clone)]
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
			data: Vec::new(),
		}
	}

}
