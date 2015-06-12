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

pub struct Image<T: ?Sized>{
	pub header: ImageHeader,
	pub data: T,
}

impl<T: ?Sized> Image<T> {
	pub fn new<U>(width: usize, height: usize, format: ImageFormat, data: U) -> Image<U> {
		Image {
			header: ImageHeader {
				width: width,
				height: height,
				format: format,
			},
			data: data,
		}
	}

}
