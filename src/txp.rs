use std::ffi::*;
use std::marker::PhantomData;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

#[cfg(feature = "pyo3")]
#[cfg_attr(feature = "pyo3", pymodule(name = "txp"))]
pub(crate) mod txp_module {
	use pyo3::prelude::*;

	#[pymodule_export]
	use super::Format;
	#[pymodule_export]
	use super::Mipmap;
	#[pymodule_export]
	use super::PyTexture;
	#[pymodule_export]
	use super::Set;
	#[pymodule_export]
	use super::Texture;
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "pyo3", pyclass)]
pub enum Format {
	A8 = 0,
	RGB8 = 1,
	RGBA8 = 2,
	RGB5 = 3,
	RGB5A1 = 4,
	RGBA4 = 5,
	BC1 = 6,
	BC1a = 7,
	BC2 = 8,
	BC3 = 9,
	BC4 = 10,
	BC5 = 11,
	L8 = 12,
	L8A8 = 13,
	// MM+
	BC7 = 15,
	BC6H = 127,
}

#[bitfields::bitfield(u16)]
struct Rgb565 {
	#[bits(5)]
	r: u8,
	#[bits(6)]
	g: u8,
	#[bits(5)]
	b: u8,
}

#[bitfields::bitfield(u16)]
struct Rgba5551 {
	#[bits(5)]
	r: u8,
	#[bits(5)]
	g: u8,
	#[bits(5)]
	b: u8,
	#[bits(1)]
	a: u8,
}

#[bitfields::bitfield(u16)]
struct Rgba4444 {
	#[bits(4)]
	r: u8,
	#[bits(4)]
	g: u8,
	#[bits(4)]
	b: u8,
	#[bits(4)]
	a: u8,
}

pub(crate) mod ycbcr {
	// NOTE: These are the coeffs for sprites
	// The coeeffs for *sky* textures are 256.0001 / 255.0 and 128.5019 / 255.0
	pub const CBCR_MUL: f32 = 256.0 / 255.0;
	pub const CBCR_SUB: f32 = 128.0 / 255.0 * CBCR_MUL;
	pub const KB: f32 = 0.0722;
	pub const KR: f32 = 0.2126;
	pub const KG: f32 = 1.0 - KB - KR;
	pub const DECODE: [[f32; 3]; 3] = [
		[1.0, 0.0, 2.0 - 2.0 * KR],
		[
			1.0,
			-(KB / KG) * (2.0 - 2.0 * KB),
			-(KR / KG) * (2.0 - 2.0 * KR),
		],
		[1.0, 2.0 - 2.0 * KB, 0.0],
	];
	pub const ENCODE: [[f32; 3]; 3] = [
		[KR, KG, KB],
		[-KR / 1.8556, -KG / 1.8556, (1.0 - KB) / 1.8556],
		[(1.0 - KR) / 1.5748, -KG / 1.5748, -KB / 1.5748],
	];
}

#[cfg_attr(feature = "pyo3", pyclass)]
pub struct Set {
	pub(crate) ptr: *mut c_void,
}

unsafe impl Send for Set {}
unsafe impl Sync for Set {}

#[cfg_eval]
#[cfg_attr(feature = "pyo3", pymethods)]
impl Set {
	#[cfg_attr(feature = "pyo3", new)]
	pub fn new() -> Self {
		Self {
			ptr: unsafe { kkdlib_txp_set_new() },
		}
	}

	pub fn add_file(&mut self, texture: &Texture) {
		unsafe {
			kkdlib_txp_set_add_texture(self.ptr, texture.ptr);
		}
	}

	#[cfg_attr(feature = "pyo3", staticmethod)]
	pub fn from_buf(data: &[u8], big_endian: bool, modern: Option<u32>) -> Self {
		let set = Self::new();
		if let Some(signature) = modern {
			unsafe {
				kkdlib_txp_set_unpack_file_modern(
					set.ptr,
					data.as_ptr() as *const c_void,
					data.len(),
					signature,
				)
			};
		} else {
			unsafe {
				kkdlib_txp_set_unpack_file(set.ptr, data.as_ptr() as *const c_void, big_endian)
			};
		}
		set
	}

	pub fn to_buf(&self, big_endian: bool, modern: Option<u32>) -> Option<Vec<u8>> {
		let mut ptr = std::ptr::null_mut();
		let mut size = 0usize;
		if let Some(signature) = modern {
			unsafe {
				if !kkdlib_txp_set_pack_file_modern(
					self.ptr, &mut ptr, &mut size, big_endian, signature,
				) {
					return None;
				}
			};
		} else {
			unsafe {
				if !kkdlib_txp_set_pack_file(self.ptr, &mut ptr, &mut size, big_endian) {
					return None;
				}
			};
		};

		if ptr.is_null() || size == 0 {
			return None;
		}

		let slice = std::ptr::slice_from_raw_parts(ptr as *const u8, size);
		let slice = unsafe { slice.as_ref()? };

		let mut vec = Vec::with_capacity(size);
		vec.extend_from_slice(slice);

		unsafe { kkdlib_txp_set_delete_packed_file(ptr) };

		Some(vec)
	}

	#[cfg(feature = "pyo3")]
	#[cfg_attr(feature = "pyo3", getter)]
	pub fn py_textures(&self) -> Vec<PyTexture> {
		self.textures()
			.map(|tex| {
				let mip = tex.get_mipmap(0, 0).unwrap();
				PyTexture {
					width: mip.width(),
					height: mip.height(),
					rgba: mip.rgba().unwrap_or_default(),
				}
			})
			.collect()
	}
}

#[cfg(feature = "pyo3")]
#[cfg_attr(feature = "pyo3", pyclass)]
pub struct PyTexture {
	pub width: i32,
	pub height: i32,
	pub rgba: Vec<u8>,
}

impl Set {
	pub fn textures<'a>(&'a self) -> TextureIterator<'a> {
		TextureIterator {
			ptr: self.ptr,
			index: 0,
			len: unsafe { kkdlib_txp_set_get_textures_size(self.ptr) },
			phantom: PhantomData,
		}
	}
}

impl Drop for Set {
	fn drop(&mut self) {
		unsafe { kkdlib_txp_set_delete(self.ptr) };
	}
}

#[cfg_attr(feature = "pyo3", pyclass)]
pub struct Texture {
	pub(crate) ptr: *mut c_void,
}

unsafe impl Send for Texture {}
unsafe impl Sync for Texture {}

#[cfg_eval]
#[cfg_attr(feature = "pyo3", pymethods)]
impl Texture {
	#[cfg_attr(feature = "pyo3", new)]
	pub fn new() -> Self {
		Self {
			ptr: unsafe { kkdlib_txp_new() },
		}
	}

	#[cfg_attr(feature = "pyo3", getter)]
	pub fn has_cube_map(&self) -> bool {
		unsafe { kkdlib_txp_get_has_cube_map(self.ptr) }
	}

	#[cfg_attr(feature = "pyo3", setter)]
	pub fn set_has_cube_map(&mut self, has_cube_map: bool) {
		unsafe { kkdlib_txp_set_has_cube_map(self.ptr, has_cube_map) };
	}

	#[cfg_attr(feature = "pyo3", getter)]
	pub fn array_size(&self) -> i32 {
		unsafe { kkdlib_txp_get_array_size(self.ptr) }
	}

	#[cfg_attr(feature = "pyo3", setter)]
	pub fn set_array_size(&mut self, array_size: i32) {
		unsafe { kkdlib_txp_set_array_size(self.ptr, array_size) };
	}

	#[cfg_attr(feature = "pyo3", getter)]
	pub fn mipmaps_count(&self) -> i32 {
		unsafe { kkdlib_txp_get_mipmaps_count(self.ptr) }
	}

	#[cfg_attr(feature = "pyo3", setter)]
	pub fn set_mipmaps_count(&mut self, mipmaps_count: i32) {
		unsafe { kkdlib_txp_set_mipmaps_count(self.ptr, mipmaps_count) };
	}

	pub fn add_mipmap(&mut self, mipmap: &Mipmap) {
		unsafe { kkdlib_txp_add_mipmap(self.ptr, mipmap.ptr) };
	}

	#[cfg_attr(feature = "pyo3", getter)]
	pub fn is_ycbcr(&self) -> bool {
		self.array_size() == 1
			&& self.mipmaps_count() == 2
			&& self.get_mipmap(0, 0).unwrap().format() == Format::BC5
			&& self.get_mipmap(0, 1).unwrap().format() == Format::BC5
	}

	pub fn decode_ycbcr(&self) -> Option<Vec<u8>> {
		if !self.is_ycbcr() {
			return None;
		}

		let ya_mip = self.get_mipmap(0, 0)?;
		let ya_data = ya_mip.data()?;
		let cbcr_mip = self.get_mipmap(0, 1)?;
		let cbcr_data = cbcr_mip.data()?;

		let mut ya_out = vec![0; ya_mip.width() as usize * ya_mip.height() as usize * 2];
		let mut cbcr_out = vec![0; cbcr_mip.width() as usize * cbcr_mip.height() as usize * 2];

		unsafe {
			let ptr = rdo_decode_rg(
				ya_data.as_ptr(),
				ya_mip.width() as u32,
				ya_mip.height() as u32,
				DXGI_FORMAT_BC5_UNORM,
			);
			if ptr.is_null() {
				return None;
			}
			ya_out.copy_from_slice(std::slice::from_raw_parts(
				ptr,
				ya_mip.width() as usize * ya_mip.height() as usize * 2,
			));
			rdo_free(ptr);

			let ptr = rdo_decode_rg(
				cbcr_data.as_ptr(),
				cbcr_mip.width() as u32,
				cbcr_mip.height() as u32,
				DXGI_FORMAT_BC5_UNORM,
			);
			if ptr.is_null() {
				return None;
			}
			cbcr_out.copy_from_slice(std::slice::from_raw_parts(
				ptr,
				cbcr_mip.width() as usize * cbcr_mip.height() as usize * 2,
			));
			rdo_free(ptr);
		}

		let cbcr_buffer = image::ImageBuffer::<image::LumaA<u8>, Vec<u8>>::from_raw(
			cbcr_mip.width() as u32,
			cbcr_mip.height() as u32,
			cbcr_out,
		)?;
		let cbcr_buffer = image::DynamicImage::ImageLumaA8(cbcr_buffer).resize(
			ya_mip.width() as u32,
			ya_mip.height() as u32,
			image::imageops::FilterType::Lanczos3,
		);
		let cbcr_buffer = cbcr_buffer.as_bytes();

		let mut out = vec![0; ya_mip.width() as usize * ya_mip.height() as usize * 4];
		for i in 0..(ya_mip.height() as usize * ya_mip.width() as usize) {
			let y = ya_out[i * 2 + 0] as f32 / 255.0;
			let a = ya_out[i * 2 + 1] as f32 / 255.0;
			let cb = cbcr_buffer[i * 2 + 0] as f32 / 255.0 * ycbcr::CBCR_MUL - ycbcr::CBCR_SUB;
			let cr = cbcr_buffer[i * 2 + 1] as f32 / 255.0 * ycbcr::CBCR_MUL - ycbcr::CBCR_SUB;

			let r = y * ycbcr::DECODE[0][0] + cb * ycbcr::DECODE[0][1] + cr * ycbcr::DECODE[0][2];
			let g = y * ycbcr::DECODE[1][0] + cb * ycbcr::DECODE[1][1] + cr * ycbcr::DECODE[1][2];
			let b = y * ycbcr::DECODE[2][0] + cb * ycbcr::DECODE[2][1] + cr * ycbcr::DECODE[2][2];

			out[i * 4 + 0] = (r * 255.0) as u8;
			out[i * 4 + 1] = (g * 255.0) as u8;
			out[i * 4 + 2] = (b * 255.0) as u8;
			out[i * 4 + 3] = (a * 255.0) as u8;
		}

		Some(out)
	}

	#[cfg_attr(feature = "pyo3", staticmethod)]
	pub fn encode_ycbcr(width: u32, height: u32, data: &[u8]) -> Option<Self> {
		let awidth = (width + 4 - 1) / 4 * 4;
		let aheight = (height + 4 - 1) / 4 * 4;
		let hwidth = (width / 2 + 4 - 1) / 4 * 4;
		let hheight = (height / 2 + 4 - 1) / 4 * 4;
		let mut ya_raw = vec![0; awidth as usize * aheight as usize * 2];
		let mut cbcr_raw = vec![128; hwidth as usize * 2 * hheight as usize * 2 * 2];

		for y in 0..(height.min(hheight * 2)) {
			for x in 0..(width.min(hwidth * 2)) {
				let i = (y * width + x) as usize;
				let r = data[i * 4 + 0] as f32 / 255.0;
				let g = data[i * 4 + 1] as f32 / 255.0;
				let b = data[i * 4 + 2] as f32 / 255.0;

				let luma =
					r * ycbcr::ENCODE[0][0] + g * ycbcr::ENCODE[0][1] + b * ycbcr::ENCODE[0][2];
				let cb = r * ycbcr::ENCODE[1][0]
					+ g * ycbcr::ENCODE[1][1]
					+ b * ycbcr::ENCODE[1][2]
					+ ycbcr::CBCR_SUB;
				let cr = r * ycbcr::ENCODE[2][0]
					+ g * ycbcr::ENCODE[2][1]
					+ b * ycbcr::ENCODE[2][2]
					+ ycbcr::CBCR_SUB;

				ya_raw[((y * awidth + x) * 2 + 0) as usize] = (luma * 255.0) as u8;
				ya_raw[((y * awidth + x) * 2 + 1) as usize] = data[i * 4 + 3];
				cbcr_raw[((y * hwidth * 2 + x) * 2 + 0) as usize] =
					(cb / ycbcr::CBCR_MUL * 255.0) as u8;
				cbcr_raw[((y * hwidth * 2 + x) * 2 + 1) as usize] =
					(cr / ycbcr::CBCR_MUL * 255.0) as u8;
			}
		}

		let cbcr_buffer = image::ImageBuffer::<image::LumaA<u8>, Vec<u8>>::from_raw(
			hwidth * 2,
			hheight * 2,
			cbcr_raw,
		)?;
		let cbcr_buffer = image::DynamicImage::ImageLumaA8(cbcr_buffer).resize(
			hwidth,
			hheight,
			image::imageops::FilterType::Lanczos3,
		);

		let mut texture = Self::new();
		texture.set_has_cube_map(false);
		texture.set_array_size(1);
		texture.set_mipmaps_count(2);

		let mut y_mip = Mipmap::new();
		y_mip.set_width(awidth as i32);
		y_mip.set_height(aheight as i32);
		y_mip.set_format(Format::BC5);

		let mut cbcr_mip = Mipmap::new();
		cbcr_mip.set_width(hwidth as i32);
		cbcr_mip.set_height(hheight as i32);
		cbcr_mip.set_format(Format::BC5);

		unsafe {
			let ptr = rdo_encode_rg(ya_raw.as_ptr(), awidth, aheight, DXGI_FORMAT_BC5_UNORM);
			if ptr.is_null() {
				return None;
			}
			kkdlib_txp_mipmap_set_data(y_mip.ptr, ptr as *const c_void);
			rdo_free(ptr);

			let ptr = rdo_encode_rg(
				cbcr_buffer.as_bytes().as_ptr(),
				hwidth,
				hheight,
				DXGI_FORMAT_BC5_UNORM,
			);
			if ptr.is_null() {
				return None;
			}
			kkdlib_txp_mipmap_set_data(cbcr_mip.ptr, ptr as *const c_void);
			rdo_free(ptr);
		}

		texture.add_mipmap(&y_mip);
		texture.add_mipmap(&cbcr_mip);

		Some(texture)
	}

	#[cfg(all(feature = "pyo3"))]
	#[cfg_attr(feature = "pyo3", staticmethod)]
	pub fn py_from_rgba(width: i32, height: i32, data: &[u8], format: Format) -> Option<Self> {
		let mip = Mipmap::from_rgba(width, height, data, format)?;
		let mut tex = Self::new();
		tex.set_has_cube_map(false);
		tex.set_array_size(1);
		tex.set_mipmaps_count(1);
		tex.add_mipmap(&mip);
		Some(tex)
	}
}

impl Texture {
	pub fn get_mipmap<'a>(&'a self, array_index: i32, mipmap_index: i32) -> Option<MipmapRef<'a>> {
		let ptr = unsafe { kkdlib_txp_get_mipmap(self.ptr, array_index, mipmap_index) };
		if ptr.is_null() {
			return None;
		}
		Some(MipmapRef {
			_ptr: ptr,
			phantom: PhantomData,
		})
	}

	pub fn mipmaps<'a>(&'a self) -> MipmapIterator<'a> {
		MipmapIterator {
			ptr: self.ptr,
			index: 0,
			len: (self.array_size() * self.mipmaps_count()) as usize,
			phantom: PhantomData,
		}
	}
}

impl Drop for Texture {
	fn drop(&mut self) {
		unsafe { kkdlib_txp_delete(self.ptr) };
	}
}

pub struct TextureRef<'a> {
	pub(crate) _ptr: *mut c_void,
	pub(crate) phantom: PhantomData<&'a Texture>,
}

impl<'a> TextureRef<'a> {
	pub fn clone(&self) -> Texture {
		let mut new = Texture::new();
		new.set_has_cube_map(self.has_cube_map());
		new.set_array_size(self.array_size());
		new.set_mipmaps_count(self.mipmaps_count());
		for mipmap in self.mipmaps() {
			new.add_mipmap(&mipmap.clone());
		}
		new
	}

	pub fn has_cube_map(&self) -> bool {
		Texture::has_cube_map(unsafe { std::mem::transmute(self) })
	}

	pub fn array_size(&self) -> i32 {
		Texture::array_size(unsafe { std::mem::transmute(self) })
	}

	pub fn mipmaps_count(&self) -> i32 {
		Texture::mipmaps_count(unsafe { std::mem::transmute(self) })
	}

	pub fn is_ycbcr(&self) -> bool {
		Texture::is_ycbcr(unsafe { std::mem::transmute(self) })
	}

	pub fn get_mipmap(&'a self, array_index: i32, mipmap_index: i32) -> Option<MipmapRef<'a>> {
		Texture::get_mipmap(
			unsafe { std::mem::transmute(self) },
			array_index,
			mipmap_index,
		)
	}

	pub fn mipmaps(&'a self) -> MipmapIterator<'a> {
		Texture::mipmaps(unsafe { std::mem::transmute(self) })
	}
}

unsafe impl Send for TextureRef<'_> {}
unsafe impl Sync for TextureRef<'_> {}

pub struct TextureIterator<'a> {
	pub(crate) ptr: *mut c_void,
	index: usize,
	len: usize,
	phantom: PhantomData<&'a Set>,
}

impl<'a> Iterator for TextureIterator<'a> {
	type Item = TextureRef<'a>;

	fn next(&mut self) -> Option<Self::Item> {
		if self.index == self.len {
			return None;
		}

		let ptr = unsafe { kkdlib_txp_set_get_texture_by_index(self.ptr, self.index) };
		if ptr.is_null() {
			return None;
		}

		self.index += 1;
		Some(Self::Item {
			_ptr: ptr,
			phantom: PhantomData,
		})
	}

	fn size_hint(&self) -> (usize, Option<usize>) {
		(self.len, Some(self.len))
	}
}

#[cfg_attr(feature = "pyo3", pyclass)]
pub struct Mipmap {
	pub(crate) ptr: *mut c_void,
}

unsafe impl Send for Mipmap {}
unsafe impl Sync for Mipmap {}

#[cfg_eval]
#[cfg_attr(feature = "pyo3", pymethods)]
impl Mipmap {
	#[cfg_attr(feature = "pyo3", new)]
	pub fn new() -> Self {
		Self {
			ptr: unsafe { kkdlib_txp_mipmap_new() },
		}
	}

	#[cfg_attr(feature = "pyo3", getter)]
	pub fn width(&self) -> i32 {
		unsafe { kkdlib_txp_mipmap_get_width(self.ptr) }
	}

	#[cfg_attr(feature = "pyo3", setter)]
	pub fn set_width(&mut self, width: i32) {
		unsafe { kkdlib_txp_mipmap_set_width(self.ptr, width) };
	}

	#[cfg_attr(feature = "pyo3", getter)]
	pub fn height(&self) -> i32 {
		unsafe { kkdlib_txp_mipmap_get_height(self.ptr) }
	}

	#[cfg_attr(feature = "pyo3", setter)]
	pub fn set_height(&mut self, height: i32) {
		unsafe { kkdlib_txp_mipmap_set_height(self.ptr, height) };
	}

	#[cfg_attr(feature = "pyo3", getter)]
	pub fn format(&self) -> Format {
		unsafe { kkdlib_txp_mipmap_get_format(self.ptr) }
	}

	#[cfg_attr(feature = "pyo3", setter)]
	pub fn set_format(&mut self, format: Format) {
		unsafe { kkdlib_txp_mipmap_set_format(self.ptr, format) };
	}

	#[cfg_attr(feature = "pyo3", getter)]
	pub fn size(&self) -> i32 {
		unsafe { kkdlib_txp_mipmap_get_size(self.ptr) }
	}

	pub fn data<'a>(&'a self) -> Option<&'a [u8]> {
		let ptr = unsafe { kkdlib_txp_mipmap_get_data(self.ptr) };
		if ptr.is_null() {
			return None;
		}
		let slice = std::ptr::slice_from_raw_parts(ptr as *const u8, self.size() as usize);
		unsafe { slice.as_ref() }
	}

	pub fn set_data(&mut self, data: &[u8]) {
		assert_eq!(data.len(), self.size() as usize);
		unsafe { kkdlib_txp_mipmap_set_data(self.ptr, data.as_ptr() as *const c_void) };
	}

	pub fn rgba(&self) -> Option<Vec<u8>> {
		let size = self.width() * self.height() * 4;
		let mut out = vec![0; size as usize];

		let data = self.data()?;
		match self.format() {
			Format::A8 => {
				for (i, px) in data.iter().enumerate() {
					out[i * 4 + 0] = 255;
					out[i * 4 + 1] = 255;
					out[i * 4 + 2] = 255;
					out[i * 4 + 3] = *px;
				}
			}
			Format::RGB8 => {
				for i in 0..(self.width() * self.height()) {
					let offset = i as usize * 3;
					let out_offset = i as usize * 4;
					out[out_offset + 0] = data[offset + 0];
					out[out_offset + 1] = data[offset + 1];
					out[out_offset + 2] = data[offset + 2];
					out[out_offset + 3] = 255;
				}
			}
			Format::RGBA8 => {
				out.copy_from_slice(data);
			}
			Format::RGB5 => {
				let data = data.as_ptr() as *const Rgb565;
				let data = std::ptr::slice_from_raw_parts(
					data,
					self.width() as usize * self.height() as usize * 2,
				);
				let data = unsafe { data.as_ref()? };
				for (i, px) in data.iter().enumerate() {
					out[i * 4 + 0] = px.r() << 3;
					out[i * 4 + 1] = px.g() << 2;
					out[i * 4 + 2] = px.b() << 3;
					out[i * 4 + 3] = 255;
				}
			}
			Format::RGB5A1 => {
				let data = data.as_ptr() as *const Rgba5551;
				let data = std::ptr::slice_from_raw_parts(
					data,
					self.width() as usize * self.height() as usize * 2,
				);
				let data = unsafe { data.as_ref()? };
				for (i, px) in data.iter().enumerate() {
					out[i * 4 + 0] = px.r() << 3;
					out[i * 4 + 1] = px.g() << 2;
					out[i * 4 + 2] = px.b() << 3;
					out[i * 4 + 3] = px.a() * 255;
				}
			}
			Format::RGBA4 => {
				let data = data.as_ptr() as *const Rgba4444;
				let data = std::ptr::slice_from_raw_parts(
					data,
					self.width() as usize * self.height() as usize * 2,
				);
				let data = unsafe { data.as_ref()? };
				for (i, px) in data.iter().enumerate() {
					out[i * 4 + 0] = px.r() << 4;
					out[i * 4 + 1] = px.g() << 4;
					out[i * 4 + 2] = px.b() << 4;
					out[i * 4 + 3] = px.a() << 4;
				}
			}
			Format::BC1 | Format::BC1a | Format::BC3 | Format::BC4 | Format::BC5 | Format::BC7 => {
				let fmt = match self.format() {
					Format::BC1 | Format::BC1a => DXGI_FORMAT_BC1_UNORM,
					Format::BC3 => DXGI_FORMAT_BC3_UNORM,
					Format::BC4 => DXGI_FORMAT_BC4_UNORM,
					Format::BC5 => DXGI_FORMAT_BC5_UNORM,
					Format::BC7 => DXGI_FORMAT_BC7_UNORM,
					_ => unreachable!(),
				};

				unsafe {
					let ptr = rdo_decode_rgba(
						data.as_ptr(),
						self.width() as u32,
						self.height() as u32,
						fmt,
					);
					if ptr.is_null() {
						return None;
					}
					out.copy_from_slice(std::slice::from_raw_parts(
						ptr,
						self.width() as usize * self.height() as usize * 4,
					));
					rdo_free(ptr);
				}

				if self.format() == Format::BC5 {
					for i in 0..(size as usize / 4) {
						out[i * 4 + 2] = 0xFF;
						out[i * 4 + 3] = 0xFF;
					}
				}
			}
			Format::BC2 | Format::BC6H => {
				return None;
			}
			Format::L8 => {
				for (i, px) in data.iter().enumerate() {
					out[i * 4 + 0] = *px;
					out[i * 4 + 1] = *px;
					out[i * 4 + 2] = *px;
					out[i * 4 + 3] = 255;
				}
			}
			Format::L8A8 => {
				for i in 0..(self.width() * self.height()) {
					let offset = i as usize * 2;
					let out_offset = i as usize * 4;
					out[out_offset + 0] = data[offset + 0];
					out[out_offset + 1] = data[offset + 0];
					out[out_offset + 2] = data[offset + 0];
					out[out_offset + 3] = data[offset + 1];
				}
			}
		}

		Some(out)
	}

	#[cfg_attr(feature = "pyo3", staticmethod)]
	pub fn from_rgba(width: i32, height: i32, data: &[u8], format: Format) -> Option<Self> {
		let mut mip = Mipmap::new();
		mip.set_width(width);
		mip.set_height(height);
		mip.set_format(format);

		let mut mip_data = vec![0; mip.size() as usize];

		match format {
			Format::A8 => {
				for i in 0..(width as usize * height as usize) {
					mip_data[i] = data[i * 4 + 3];
				}
			}
			Format::RGB8 => {
				for i in 0..(width as usize * height as usize) {
					let offset = i * 4;
					let mip_offset = i * 3;
					mip_data[mip_offset + 0] = data[offset + 0];
					mip_data[mip_offset + 1] = data[offset + 1];
					mip_data[mip_offset + 2] = data[offset + 2];
				}
			}
			Format::RGBA8 => {
				mip_data.copy_from_slice(data);
			}
			Format::RGB5 => {
				for i in 0..(width as usize * height as usize) {
					let offset = i * 4;
					let mip_offset = i * 2;
					let rgb = Rgb565Builder::new()
						.with_r(data[offset + 0] >> 3)
						.with_g(data[offset + 1] >> 2)
						.with_b(data[offset + 2] >> 3)
						.build()
						.into_bits();
					mip_data[mip_offset + 0] = (rgb >> 0) as u8;
					mip_data[mip_offset + 1] = (rgb >> 8) as u8;
				}
			}
			Format::RGB5A1 => {
				for i in 0..(width as usize * height as usize) {
					let offset = i * 4;
					let mip_offset = i * 2;
					let rgba = Rgba5551Builder::new()
						.with_r(data[offset + 0] >> 3)
						.with_g(data[offset + 1] >> 3)
						.with_b(data[offset + 2] >> 3)
						.with_a(data[offset + 3] / 255)
						.build()
						.into_bits();
					mip_data[mip_offset + 0] = (rgba >> 0) as u8;
					mip_data[mip_offset + 1] = (rgba >> 8) as u8;
				}
			}
			Format::RGBA4 => {
				for i in 0..(width as usize * height as usize) {
					let offset = i * 4;
					let mip_offset = i * 2;
					let rgba = Rgba4444Builder::new()
						.with_r(data[offset + 0] >> 4)
						.with_g(data[offset + 1] >> 4)
						.with_b(data[offset + 2] >> 4)
						.with_a(data[offset + 3] >> 4)
						.build()
						.into_bits();
					mip_data[mip_offset + 0] = (rgba >> 0) as u8;
					mip_data[mip_offset + 1] = (rgba >> 8) as u8;
				}
			}
			Format::BC1 | Format::BC1a | Format::BC3 | Format::BC4 | Format::BC5 | Format::BC7 => {
				let fmt = match format {
					Format::BC1 | Format::BC1a => DXGI_FORMAT_BC1_UNORM,
					Format::BC3 => DXGI_FORMAT_BC3_UNORM,
					Format::BC4 => DXGI_FORMAT_BC4_UNORM,
					Format::BC5 => DXGI_FORMAT_BC5_UNORM,
					Format::BC7 => DXGI_FORMAT_BC7_UNORM,
					_ => unreachable!(),
				};

				unsafe {
					let ptr = rdo_encode_rgba(data.as_ptr(), width as u32, height as u32, fmt);
					if ptr.is_null() {
						return None;
					}
					mip_data.copy_from_slice(std::slice::from_raw_parts(ptr, mip.size() as usize));
					rdo_free(ptr);
				}
			}
			Format::BC2 | Format::BC6H => {
				return None;
			}
			Format::L8 => {
				for i in 0..(width as usize * height as usize) {
					let offset = i * 4;
					let r = data[offset + 0] as f32;
					let g = data[offset + 1] as f32;
					let b = data[offset + 2] as f32;

					let y =
						r * ycbcr::ENCODE[0][0] + g * ycbcr::ENCODE[0][1] + b * ycbcr::ENCODE[0][2];
					mip_data[i] = y as u8;
				}
			}
			Format::L8A8 => {
				for i in 0..(width as usize * height as usize) {
					let offset = i * 4;
					let mip_offset = i * 2;
					let r = data[offset + 0] as f32;
					let g = data[offset + 1] as f32;
					let b = data[offset + 2] as f32;
					let a = data[offset + 3];

					let y =
						r * ycbcr::ENCODE[0][0] + g * ycbcr::ENCODE[0][1] + b * ycbcr::ENCODE[0][2];
					mip_data[mip_offset + 0] = y as u8;
					mip_data[mip_offset + 1] = a;
				}
			}
		}

		mip.set_data(&mip_data);
		Some(mip)
	}
}

impl Drop for Mipmap {
	fn drop(&mut self) {
		unsafe { kkdlib_txp_mipmap_delete(self.ptr) };
	}
}

pub struct MipmapRef<'a> {
	pub(crate) _ptr: *mut c_void,
	phantom: PhantomData<&'a Mipmap>,
}

unsafe impl Send for MipmapRef<'_> {}
unsafe impl Sync for MipmapRef<'_> {}

impl<'a> MipmapRef<'a> {
	pub fn clone(&self) -> Mipmap {
		let mut new = Mipmap::new();
		new.set_width(self.width());
		new.set_height(self.height());
		new.set_format(self.format());
		if let Some(data) = self.data() {
			new.set_data(data);
		}
		new
	}

	pub fn width(&self) -> i32 {
		Mipmap::width(unsafe { std::mem::transmute(self) })
	}

	pub fn height(&self) -> i32 {
		Mipmap::height(unsafe { std::mem::transmute(self) })
	}

	pub fn format(&self) -> Format {
		Mipmap::format(unsafe { std::mem::transmute(self) })
	}

	pub fn size(&self) -> i32 {
		Mipmap::size(unsafe { std::mem::transmute(self) })
	}

	pub fn data(&'a self) -> Option<&'a [u8]> {
		Mipmap::data(unsafe { std::mem::transmute(self) })
	}

	pub fn rgba(&self) -> Option<Vec<u8>> {
		Mipmap::rgba(unsafe { std::mem::transmute(self) })
	}
}

pub struct MipmapIterator<'a> {
	pub(crate) ptr: *mut c_void,
	index: usize,
	len: usize,
	phantom: PhantomData<&'a Texture>,
}

impl<'a> Iterator for MipmapIterator<'a> {
	type Item = MipmapRef<'a>;

	fn next(&mut self) -> Option<Self::Item> {
		if self.index == self.len {
			return None;
		}

		let ptr = unsafe { kkdlib_txp_get_mipmap(self.ptr, 0, self.index as i32) };
		if ptr.is_null() {
			return None;
		}

		self.index += 1;
		Some(Self::Item {
			_ptr: ptr,
			phantom: PhantomData,
		})
	}

	fn size_hint(&self) -> (usize, Option<usize>) {
		(self.len, Some(self.len))
	}
}

unsafe extern "C" {
	fn kkdlib_txp_mipmap_new() -> *mut c_void;
	fn kkdlib_txp_mipmap_get_width(mipmap: *mut c_void) -> i32;
	fn kkdlib_txp_mipmap_set_width(mipmap: *mut c_void, width: i32);
	fn kkdlib_txp_mipmap_get_height(mipmap: *mut c_void) -> i32;
	fn kkdlib_txp_mipmap_set_height(mipmap: *mut c_void, height: i32);
	fn kkdlib_txp_mipmap_get_format(mipmap: *mut c_void) -> Format;
	fn kkdlib_txp_mipmap_set_format(mipmap: *mut c_void, format: Format);
	fn kkdlib_txp_mipmap_get_size(mipmap: *mut c_void) -> i32;
	fn kkdlib_txp_mipmap_get_data(mipmap: *mut c_void) -> *const c_void;
	fn kkdlib_txp_mipmap_set_data(mipmap: *mut c_void, data: *const c_void);
	fn kkdlib_txp_mipmap_delete(mipmap: *mut c_void);

	fn kkdlib_txp_new() -> *mut c_void;
	fn kkdlib_txp_get_has_cube_map(txp: *mut c_void) -> bool;
	fn kkdlib_txp_set_has_cube_map(txp: *mut c_void, has_cube_map: bool);
	fn kkdlib_txp_get_array_size(txp: *mut c_void) -> i32;
	fn kkdlib_txp_set_array_size(txp: *mut c_void, array_size: i32);
	fn kkdlib_txp_get_mipmaps_count(txp: *mut c_void) -> i32;
	fn kkdlib_txp_set_mipmaps_count(txp: *mut c_void, mipmaps_count: i32);
	fn kkdlib_txp_get_mipmap(txp: *mut c_void, array_index: i32, mipmap_index: i32) -> *mut c_void;
	fn kkdlib_txp_add_mipmap(txp: *mut c_void, mipmap: *mut c_void);
	fn kkdlib_txp_delete(txp: *mut c_void);

	fn kkdlib_txp_set_new() -> *mut c_void;
	fn kkdlib_txp_set_get_textures_size(set: *mut c_void) -> usize;
	pub(crate) fn kkdlib_txp_set_get_texture_by_index(
		set: *mut c_void,
		index: usize,
	) -> *mut c_void;
	fn kkdlib_txp_set_add_texture(set: *mut c_void, txp: *mut c_void);
	fn kkdlib_txp_set_pack_file(
		set: *mut c_void,
		data: *mut *mut c_void,
		size: *mut usize,
		big_endian: bool,
	) -> bool;
	fn kkdlib_txp_set_pack_file_modern(
		set: *mut c_void,
		data: *mut *mut c_void,
		size: *mut usize,
		big_endian: bool,
		signature: u32,
	) -> bool;
	fn kkdlib_txp_set_delete_packed_file(data: *mut c_void);
	fn kkdlib_txp_set_unpack_file(set: *mut c_void, data: *const c_void, big_endian: bool) -> bool;
	fn kkdlib_txp_set_unpack_file_modern(
		set: *mut c_void,
		data: *const c_void,
		size: usize,
		signature: u32,
	) -> bool;
	fn kkdlib_txp_set_delete(set: *mut c_void);
}

const DXGI_FORMAT_BC1_UNORM: u32 = 71;
const DXGI_FORMAT_BC3_UNORM: u32 = 77;
const DXGI_FORMAT_BC4_UNORM: u32 = 80;
const DXGI_FORMAT_BC5_UNORM: u32 = 83;
const DXGI_FORMAT_BC7_UNORM: u32 = 98;
unsafe extern "C" {
	fn rdo_encode_rgba(rgba: *const u8, width: u32, height: u32, format: u32) -> *mut u8;
	fn rdo_encode_rg(rg: *const u8, width: u32, height: u32, format: u32) -> *mut u8;
	fn rdo_decode_rgba(blocks: *const u8, width: u32, height: u32, format: u32) -> *mut u8;
	fn rdo_decode_rg(blocks: *const u8, width: u32, height: u32, format: u32) -> *mut u8;
	fn rdo_free(ptr: *mut u8);
}
