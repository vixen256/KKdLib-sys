use std::ffi::*;
use std::marker::PhantomData;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;
#[cfg(feature = "pyo3")]
use std::collections::HashMap;

#[cfg(feature = "pyo3")]
#[cfg_attr(feature = "pyo3", pymodule(name = "spr"))]
pub(crate) mod spr_module {
	#[pymodule_export]
	use super::Info;
	#[pymodule_export]
	use super::ResolutionMode;
	#[pymodule_export]
	use super::Set;
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "pyo3", pyclass)]
pub enum ResolutionMode {
	QVGA = 0x00,
	VGA = 0x01,
	SVGA = 0x02,
	XGA = 0x03,
	SXGA = 0x04,
	SXGAPlus = 0x05,
	UXGA = 0x06,
	WVGA = 0x07,
	WSVGA = 0x08,
	WXGA = 0x09,
	FWXGA = 0x0A,
	WUXGA = 0x0B,
	WQXGA = 0x0C,
	HD = 0x0D,
	FHD = 0x0E,
	UHD = 0x0F,
	Res3KatUHD = 0x10,
	Res3K = 0x11,
	QuadHD = 0x12,
	WQVGA = 0x13,
	QuarterHD = 0x14,
	XGAPlus = 0x15,
	Res1176x664 = 0x16,
	Res1200x960 = 0x17,
	WXGA1280x900 = 0x18,
	SXGAMinus = 0x19,
	FWXGA1366x768 = 0x1A,
	WXGAPlus = 0x1B,
	HDPlus = 0x1C,
	WSXGA = 0x1D,
	WSXGAPlus = 0x1E,
	Res1920x1440 = 0x1F,
	QWXGA = 0x20,
}

#[cfg_eval]
#[cfg_attr(feature = "pyo3", pymethods)]
impl ResolutionMode {
	pub fn resolution(&self) -> (u32, u32) {
		match *self {
			Self::QVGA => (320, 240),
			Self::VGA => (640, 480),
			Self::SVGA => (800, 600),
			Self::XGA => (1024, 768),
			Self::SXGA => (1280, 1024),
			Self::SXGAPlus => (1400, 1050),
			Self::UXGA => (1600, 1200),
			Self::WVGA => (800, 480),
			Self::WSVGA => (1024, 600),
			Self::WXGA => (1280, 768),
			Self::FWXGA => (1360, 768),
			Self::WUXGA => (1920, 1200),
			Self::WQXGA => (2560, 1536),
			Self::HD => (1280, 720),
			Self::FHD => (1920, 1080),
			Self::UHD => (3840, 2160),
			Self::Res3KatUHD => (3840, 2160),
			Self::Res3K => (2880, 1620),
			Self::QuadHD => (2560, 1440),
			Self::WQVGA => (480, 272),
			Self::QuarterHD => (960, 544),
			Self::XGAPlus => (1152, 864),
			Self::Res1176x664 => (1176, 664),
			Self::Res1200x960 => (1200, 960),
			Self::WXGA1280x900 => (1280, 900),
			Self::SXGAMinus => (1280, 960),
			Self::FWXGA1366x768 => (1366, 768),
			Self::WXGAPlus => (1440, 900),
			Self::HDPlus => (1600, 900),
			Self::WSXGA => (1600, 1024),
			Self::WSXGAPlus => (1680, 1050),
			Self::Res1920x1440 => (1920, 1440),
			Self::QWXGA => (2048, 1152),
		}
	}
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
			ptr: unsafe { kkdlib_spr_set_new() },
		}
	}

	#[cfg_attr(feature = "pyo3", getter)]
	pub fn ready(&self) -> bool {
		unsafe { kkdlib_spr_set_get_ready(self.ptr) }
	}

	#[cfg_attr(feature = "pyo3", setter)]
	pub fn set_ready(&mut self, ready: bool) {
		unsafe { kkdlib_spr_set_set_ready(self.ptr, ready) };
	}

	#[cfg_attr(feature = "pyo3", getter)]
	pub fn modern(&self) -> bool {
		unsafe { kkdlib_spr_set_get_modern(self.ptr) }
	}

	#[cfg_attr(feature = "pyo3", setter)]
	pub fn set_modern(&mut self, modern: bool) {
		unsafe { kkdlib_spr_set_set_modern(self.ptr, modern) };
	}

	#[cfg_attr(feature = "pyo3", getter)]
	pub fn big_endian(&self) -> bool {
		unsafe { kkdlib_spr_set_get_big_endian(self.ptr) }
	}

	#[cfg_attr(feature = "pyo3", setter)]
	pub fn set_big_endian(&mut self, big_endian: bool) {
		unsafe { kkdlib_spr_set_set_big_endian(self.ptr, big_endian) };
	}

	#[cfg_attr(feature = "pyo3", getter)]
	pub fn is_x(&self) -> bool {
		unsafe { kkdlib_spr_set_get_is_x(self.ptr) }
	}

	#[cfg_attr(feature = "pyo3", setter)]
	pub fn set_is_x(&mut self, is_x: bool) {
		unsafe { kkdlib_spr_set_set_is_x(self.ptr, is_x) };
	}

	#[cfg_attr(feature = "pyo3", getter)]
	pub fn flag(&self) -> u32 {
		unsafe { kkdlib_spr_set_get_flag(self.ptr) }
	}

	#[cfg_attr(feature = "pyo3", setter)]
	pub fn set_flag(&mut self, flag: u32) {
		unsafe { kkdlib_spr_set_set_flag(self.ptr, flag) };
	}

	pub fn set_txp(&mut self, txp: &crate::txp::Set, names: Vec<String>) {
		if names.len() != txp.textures().count() {
			return;
		}
		let mut vec = Vec::new();
		for name in names {
			let Ok(c) = CString::new(name) else {
				return;
			};
			vec.push(c);
		}
		let ptrs = vec.iter().map(|name| name.as_ptr()).collect::<Vec<_>>();
		unsafe { kkdlib_spr_set_set_txp(self.ptr, txp.ptr, ptrs.as_ptr()) }
	}

	pub fn add_spr(&mut self, info: &Info, name: &str) {
		let Ok(c) = CString::new(name) else {
			return;
		};
		unsafe { kkdlib_spr_set_add_spr(self.ptr, info.spr_info, c.as_ptr(), info.sprite_data) };
	}

	#[cfg_attr(feature = "pyo3", staticmethod)]
	pub fn from_buf(data: &[u8], modern: bool) -> Self {
		let set = Self::new();
		unsafe {
			kkdlib_spr_set_unpack_file(set.ptr, data.as_ptr() as *const c_void, data.len(), modern)
		};
		set
	}

	pub fn to_buf(&self) -> Option<Vec<u8>> {
		let mut ptr = std::ptr::null_mut();
		let mut size = 0usize;
		unsafe {
			kkdlib_spr_set_pack_file(self.ptr, &mut ptr, &mut size);
		}

		if ptr == std::ptr::null_mut() || size == 0 {
			return None;
		}

		let slice = std::ptr::slice_from_raw_parts(ptr as *const u8, size);
		let slice = unsafe { slice.as_ref()? };

		let mut vec = Vec::with_capacity(size);
		vec.extend_from_slice(slice);

		unsafe { kkdlib_spr_set_delete_packed_file(ptr) };

		Some(vec)
	}

	#[cfg(feature = "pyo3")]
	#[cfg_attr(feature = "pyo3", getter)]
	pub fn py_sprites(&self) -> HashMap<String, Info> {
		self.sprites()
			.map(|(name, info)| (name, info.clone()))
			.collect()
	}

	#[cfg(feature = "pyo3")]
	#[cfg_attr(feature = "pyo3", getter)]
	pub fn py_textures(&self) -> HashMap<String, crate::txp::PyTexture> {
		self.textures()
			.map(|(name, texture)| {
				let mip = texture.get_mipmap(0, 0).unwrap();
				(
					name,
					crate::txp::PyTexture {
						width: mip.width(),
						height: mip.height(),
						rgba: mip.rgba().unwrap_or_default(),
					},
				)
			})
			.collect()
	}
}

impl Set {
	pub fn sprites<'a>(&'a self) -> InfoIterator<'a> {
		InfoIterator {
			ptr: self.ptr,
			index: 0,
			len: unsafe { kkdlib_spr_set_get_num_of_sprite(self.ptr) },
			phantom: PhantomData,
		}
	}

	pub fn textures<'a>(&'a self) -> SprTexIterator<'a> {
		SprTexIterator {
			ptr: self.ptr,
			index: 0,
			len: unsafe { kkdlib_spr_set_get_num_of_texture(self.ptr) },
			phantom: PhantomData,
		}
	}
}

impl Drop for Set {
	fn drop(&mut self) {
		unsafe { kkdlib_spr_set_delete(self.ptr) };
	}
}

#[cfg_attr(feature = "pyo3", pyclass)]
pub struct Info {
	pub(crate) spr_info: *mut c_void,
	pub(crate) sprite_data: *mut c_void,
}

unsafe impl Send for Info {}
unsafe impl Sync for Info {}

#[cfg_eval]
#[cfg_attr(feature = "pyo3", pymethods)]
impl Info {
	#[cfg_attr(feature = "pyo3", new)]
	pub fn new() -> Self {
		Self {
			spr_info: unsafe { kkdlib_spr_info_new() },
			sprite_data: unsafe { kkdlib_sprite_data_new() },
		}
	}

	#[cfg_attr(feature = "pyo3", getter)]
	pub fn texid(&self) -> u32 {
		unsafe { kkdlib_spr_info_get_texid(self.spr_info) }
	}

	#[cfg_attr(feature = "pyo3", setter)]
	pub fn set_texid(&mut self, texid: u32) {
		unsafe { kkdlib_spr_info_set_texid(self.spr_info, texid) }
	}

	#[cfg_attr(feature = "pyo3", getter)]
	pub fn rotate(&self) -> i32 {
		unsafe { kkdlib_spr_info_get_rotate(self.spr_info) }
	}

	#[cfg_attr(feature = "pyo3", setter)]
	pub fn set_rotate(&mut self, rotate: i32) {
		unsafe { kkdlib_spr_info_set_rotate(self.spr_info, rotate) }
	}

	#[cfg_attr(feature = "pyo3", getter)]
	pub fn px(&self) -> f32 {
		unsafe { kkdlib_spr_info_get_px(self.spr_info) }
	}

	#[cfg_attr(feature = "pyo3", setter)]
	pub fn set_px(&mut self, px: f32) {
		unsafe { kkdlib_spr_info_set_px(self.spr_info, px) }
	}

	#[cfg_attr(feature = "pyo3", getter)]
	pub fn py(&self) -> f32 {
		unsafe { kkdlib_spr_info_get_py(self.spr_info) }
	}

	#[cfg_attr(feature = "pyo3", setter)]
	pub fn set_py(&mut self, py: f32) {
		unsafe { kkdlib_spr_info_set_py(self.spr_info, py) }
	}

	#[cfg_attr(feature = "pyo3", getter)]
	pub fn width(&self) -> f32 {
		unsafe { kkdlib_spr_info_get_width(self.spr_info) }
	}

	#[cfg_attr(feature = "pyo3", setter)]
	pub fn set_width(&mut self, width: f32) {
		unsafe { kkdlib_spr_info_set_width(self.spr_info, width) }
	}

	#[cfg_attr(feature = "pyo3", getter)]
	pub fn height(&self) -> f32 {
		unsafe { kkdlib_spr_info_get_height(self.spr_info) }
	}

	#[cfg_attr(feature = "pyo3", setter)]
	pub fn set_height(&mut self, height: f32) {
		unsafe { kkdlib_spr_info_set_height(self.spr_info, height) }
	}

	#[cfg_attr(feature = "pyo3", getter)]
	pub fn attr(&self) -> u32 {
		unsafe { kkdlib_sprite_data_get_attr(self.sprite_data) }
	}

	#[cfg_attr(feature = "pyo3", setter)]
	pub fn set_attr(&mut self, attr: u32) {
		unsafe { kkdlib_sprite_data_set_attr(self.sprite_data, attr) }
	}

	#[cfg_attr(feature = "pyo3", getter)]
	pub fn resolution_mode(&self) -> ResolutionMode {
		unsafe { kkdlib_sprite_data_get_resolution_mode(self.sprite_data) }
	}

	#[cfg_attr(feature = "pyo3", setter)]
	pub fn set_resolution_mode(&mut self, resolution_mode: ResolutionMode) {
		unsafe { kkdlib_sprite_data_set_resolution_mode(self.sprite_data, resolution_mode) }
	}
}

impl Drop for Info {
	fn drop(&mut self) {
		unsafe {
			kkdlib_spr_info_delete(self.spr_info);
			kkdlib_sprite_data_delete(self.sprite_data)
		}
	}
}

pub struct InfoRef<'a> {
	pub(crate) _spr_info: *mut c_void,
	pub(crate) _sprite_data: *mut c_void,
	phantom: PhantomData<&'a Info>,
}

unsafe impl Send for InfoRef<'_> {}
unsafe impl Sync for InfoRef<'_> {}

impl InfoRef<'_> {
	pub fn clone(&self) -> Info {
		let mut new = Info::new();
		new.set_texid(self.texid());
		new.set_rotate(self.rotate());
		new.set_px(self.px());
		new.set_py(self.py());
		new.set_width(self.width());
		new.set_height(self.height());
		new.set_attr(self.attr());
		new.set_resolution_mode(self.resolution_mode());
		new
	}

	pub fn texid(&self) -> u32 {
		Info::texid(unsafe { std::mem::transmute(self) })
	}

	pub fn rotate(&self) -> i32 {
		Info::rotate(unsafe { std::mem::transmute(self) })
	}

	pub fn px(&self) -> f32 {
		Info::px(unsafe { std::mem::transmute(self) })
	}

	pub fn py(&self) -> f32 {
		Info::py(unsafe { std::mem::transmute(self) })
	}

	pub fn width(&self) -> f32 {
		Info::width(unsafe { std::mem::transmute(self) })
	}

	pub fn height(&self) -> f32 {
		Info::height(unsafe { std::mem::transmute(self) })
	}

	pub fn attr(&self) -> u32 {
		Info::attr(unsafe { std::mem::transmute(self) })
	}

	pub fn resolution_mode(&self) -> ResolutionMode {
		Info::resolution_mode(unsafe { std::mem::transmute(self) })
	}
}

pub struct InfoIterator<'a> {
	pub(crate) ptr: *mut c_void,
	index: i32,
	len: i32,
	phantom: PhantomData<&'a Set>,
}

impl<'a> Iterator for InfoIterator<'a> {
	type Item = (String, InfoRef<'a>);

	fn next(&mut self) -> Option<Self::Item> {
		if self.index == self.len {
			return None;
		}

		let sprinfo = unsafe { kkdlib_spr_set_get_sprinfo(self.ptr, self.index) };
		let sprname = unsafe { kkdlib_spr_set_get_sprname(self.ptr, self.index) };
		let sprdata = unsafe { kkdlib_spr_set_get_sprdata(self.ptr, self.index) };
		if sprinfo == std::ptr::null_mut()
			|| sprname == std::ptr::null()
			|| sprdata == std::ptr::null_mut()
		{
			return None;
		}

		let sprname = unsafe { CStr::from_ptr(sprname) };

		self.index += 1;
		Some((
			sprname.to_string_lossy().to_string(),
			InfoRef {
				_spr_info: sprinfo,
				_sprite_data: sprdata,
				phantom: PhantomData,
			},
		))
	}

	fn size_hint(&self) -> (usize, Option<usize>) {
		(self.len as usize, Some(self.len as usize))
	}
}

pub struct SprTexIterator<'a> {
	pub(crate) ptr: *mut c_void,
	index: i32,
	len: i32,
	phantom: PhantomData<&'a Set>,
}

impl<'a> Iterator for SprTexIterator<'a> {
	type Item = (String, crate::txp::TextureRef<'a>);

	fn next(&mut self) -> Option<Self::Item> {
		if self.index == self.len {
			return None;
		}

		let txp_set = unsafe { kkdlib_spr_set_get_txp(self.ptr) };
		let texture = unsafe {
			crate::txp::kkdlib_txp_set_get_texture_by_index(txp_set, self.index as usize)
		};
		let texname = unsafe { kkdlib_spr_set_get_texname(self.ptr, self.index) };
		if txp_set == std::ptr::null_mut() || texname == std::ptr::null() {
			return None;
		}

		let texname = unsafe { CStr::from_ptr(texname) };

		self.index += 1;
		Some((
			texname.to_string_lossy().to_string(),
			crate::txp::TextureRef {
				_ptr: texture,
				phantom: PhantomData,
			},
		))
	}

	fn size_hint(&self) -> (usize, Option<usize>) {
		(self.len as usize, Some(self.len as usize))
	}
}

unsafe extern "C" {
	fn kkdlib_spr_info_new() -> *mut c_void;
	fn kkdlib_spr_info_get_texid(info: *mut c_void) -> u32;
	fn kkdlib_spr_info_set_texid(info: *mut c_void, texid: u32);
	fn kkdlib_spr_info_get_rotate(info: *mut c_void) -> i32;
	fn kkdlib_spr_info_set_rotate(info: *mut c_void, rotate: i32);
	fn kkdlib_spr_info_get_px(info: *mut c_void) -> f32;
	fn kkdlib_spr_info_set_px(info: *mut c_void, px: f32);
	fn kkdlib_spr_info_get_py(info: *mut c_void) -> f32;
	fn kkdlib_spr_info_set_py(info: *mut c_void, py: f32);
	fn kkdlib_spr_info_get_width(info: *mut c_void) -> f32;
	fn kkdlib_spr_info_set_width(info: *mut c_void, width: f32);
	fn kkdlib_spr_info_get_height(info: *mut c_void) -> f32;
	fn kkdlib_spr_info_set_height(info: *mut c_void, height: f32);
	fn kkdlib_spr_info_delete(info: *mut c_void);

	fn kkdlib_sprite_data_new() -> *mut c_void;
	fn kkdlib_sprite_data_get_attr(data: *mut c_void) -> u32;
	fn kkdlib_sprite_data_set_attr(data: *mut c_void, attr: u32);
	fn kkdlib_sprite_data_get_resolution_mode(data: *mut c_void) -> ResolutionMode;
	fn kkdlib_sprite_data_set_resolution_mode(data: *mut c_void, resolution_mode: ResolutionMode);
	fn kkdlib_sprite_data_delete(data: *mut c_void);

	fn kkdlib_spr_set_new() -> *mut c_void;
	fn kkdlib_spr_set_get_ready(set: *mut c_void) -> bool;
	fn kkdlib_spr_set_set_ready(set: *mut c_void, ready: bool);
	fn kkdlib_spr_set_get_modern(set: *mut c_void) -> bool;
	fn kkdlib_spr_set_set_modern(set: *mut c_void, modern: bool);
	fn kkdlib_spr_set_get_big_endian(set: *mut c_void) -> bool;
	fn kkdlib_spr_set_set_big_endian(set: *mut c_void, big_endian: bool);
	fn kkdlib_spr_set_get_is_x(set: *mut c_void) -> bool;
	fn kkdlib_spr_set_set_is_x(set: *mut c_void, is_x: bool);
	fn kkdlib_spr_set_get_flag(set: *mut c_void) -> u32;
	fn kkdlib_spr_set_set_flag(set: *mut c_void, flag: u32);
	fn kkdlib_spr_set_get_num_of_texture(set: *mut c_void) -> i32;
	fn kkdlib_spr_set_get_num_of_sprite(set: *mut c_void) -> i32;
	fn kkdlib_spr_set_get_sprinfo(set: *mut c_void, index: i32) -> *mut c_void;
	fn kkdlib_spr_set_get_sprname(set: *mut c_void, index: i32) -> *const c_char;
	fn kkdlib_spr_set_get_sprdata(set: *mut c_void, index: i32) -> *mut c_void;
	fn kkdlib_spr_set_add_spr(
		set: *mut c_void,
		sprinfo: *mut c_void,
		sprname: *const c_char,
		sprdata: *mut c_void,
	);
	fn kkdlib_spr_set_get_texname(set: *mut c_void, index: i32) -> *const c_char;
	fn kkdlib_spr_set_get_txp(set: *mut c_void) -> *mut c_void;
	fn kkdlib_spr_set_set_txp(set: *mut c_void, txp: *mut c_void, texname: *const *const c_char);
	fn kkdlib_spr_set_pack_file(set: *mut c_void, data: *mut *mut c_void, size: *mut usize);
	fn kkdlib_spr_set_delete_packed_file(data: *mut c_void);
	fn kkdlib_spr_set_unpack_file(set: *mut c_void, data: *const c_void, size: usize, modern: bool);
	fn kkdlib_spr_set_delete(set: *mut c_void);
}
