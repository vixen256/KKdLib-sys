use bitflags::bitflags;
use std::collections::HashMap;
use std::ffi::*;
use std::marker::PhantomData;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

#[cfg(feature = "pyo3")]
#[cfg_attr(feature = "pyo3", pymodule(name = "farc"))]
pub(crate) mod farc_module {
	#[pymodule_export]
	use super::Farc;
	#[pymodule_export]
	use super::Flags;
	#[pymodule_export]
	use super::Signature;
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "pyo3", pyclass)]
pub enum Signature {
	Uncompressed = 0x46417263, // FArc
	Compressed = 0x46417243,   // FArC
	Encrypted = 0x46415243,    // FARC
}

bitflags! {
	#[repr(transparent)]
	#[derive(Debug, Clone, Copy, PartialEq, Eq)]
	#[cfg_attr(feature = "pyo3", pyclass)]
	pub struct Flags: u32 {
		const None = 0b00;
		const Gzip = 0b01;
		const Aes = 0b10;
	}
}

#[cfg_eval]
#[cfg_attr(feature = "pyo3", pymethods)]
impl Flags {
	#[cfg_attr(feature = "pyo3", classattr)]
	#[allow(dead_code)]
	const NONE: Self = Self::None;
	#[cfg_attr(feature = "pyo3", classattr)]
	#[allow(dead_code)]
	const GZIP: Self = Self::Gzip;
	#[cfg_attr(feature = "pyo3", classattr)]
	#[allow(dead_code)]
	const AES: Self = Self::Aes;
	#[cfg_attr(feature = "pyo3", classattr)]
	#[allow(dead_code)]
	const GZIP_AES: Self = Self::from_bits_truncate(0b11);

	fn __repr__(&self) -> &'static str {
		match *self {
			Self::NONE => "None",
			Self::GZIP => "Gzip",
			Self::AES => "Aes",
			Self::GZIP_AES => "Gzip and Aes",
			_ => "Unknown",
		}
	}
}

#[cfg_attr(feature = "pyo3", pyclass)]
pub struct Farc {
	pub(crate) ptr: *mut c_void,
}

unsafe impl Send for Farc {}
unsafe impl Sync for Farc {}

#[cfg_eval]
#[cfg_attr(feature = "pyo3", pymethods)]
impl Farc {
	#[cfg_attr(feature = "pyo3", new)]
	pub fn new() -> Self {
		Self {
			ptr: unsafe { kkdlib_farc_new(Signature::Compressed, Flags::Gzip, false) },
		}
	}

	#[cfg_attr(feature = "pyo3", staticmethod)]
	pub fn open(path: &str) -> Self {
		let mut farc = Self::new();
		farc.read(path, true, false);
		farc
	}

	#[cfg_attr(feature = "pyo3", getter)]
	pub fn flags(&self) -> Flags {
		unsafe { kkdlib_farc_get_flags(self.ptr) }
	}

	#[cfg_attr(feature = "pyo3", setter)]
	pub fn set_flags(&mut self, flags: Flags) {
		unsafe { kkdlib_farc_set_flags(self.ptr, flags) };
	}

	#[cfg_attr(feature = "pyo3", getter)]
	pub fn signature(&self) -> Signature {
		unsafe { kkdlib_farc_get_signature(self.ptr) }
	}

	#[cfg_attr(feature = "pyo3", setter)]
	pub fn set_signature(&mut self, signature: Signature) {
		unsafe { kkdlib_farc_set_signature(self.ptr, signature) };
	}

	#[cfg_attr(feature = "pyo3", getter)]
	pub fn compression_level(&self) -> i32 {
		unsafe { kkdlib_farc_get_compression_level(self.ptr) }
	}

	#[cfg_attr(feature = "pyo3", setter)]
	pub fn set_compression_level(&mut self, compression_level: i32) {
		unsafe { kkdlib_farc_set_compression_level(self.ptr, compression_level) };
	}

	#[cfg_attr(feature = "pyo3", getter)]
	pub fn alignment(&self) -> u32 {
		unsafe { kkdlib_farc_get_alignment(self.ptr) }
	}

	#[cfg_attr(feature = "pyo3", setter)]
	pub fn set_alignment(&mut self, alignment: u32) {
		unsafe { kkdlib_farc_set_alignment(self.ptr, alignment) };
	}

	#[cfg_attr(feature = "pyo3", getter)]
	pub fn ft(&self) -> bool {
		unsafe { kkdlib_farc_get_ft(self.ptr) }
	}

	pub fn set_ft(&mut self, ft: bool) {
		unsafe { kkdlib_farc_set_ft(self.ptr, ft) };
	}

	pub fn read(&mut self, path: &str, unpack: bool, save: bool) {
		let Ok(c) = CString::new(path) else {
			return;
		};

		unsafe { kkdlib_farc_read_file(self.ptr, c.as_ptr(), unpack, save) };
	}

	pub fn write(&self, path: &str, add_extension: bool, get_files: bool) {
		let Ok(c) = CString::new(path) else {
			return;
		};

		unsafe {
			kkdlib_farc_write_file(
				self.ptr,
				c.as_ptr(),
				self.signature(),
				self.flags(),
				add_extension,
				get_files,
			)
		};
	}

	#[cfg_attr(feature = "pyo3", getter)]
	pub fn get_files(&self) -> HashMap<String, Vec<u8>> {
		self.files()
			.filter(|file| file.size() > 0 && file.data().is_some())
			.map(|file| (file.name(), file.data().unwrap().to_vec()))
			.collect()
	}

	pub fn add_file_data(&mut self, name: &str, data: &[u8]) {
		let Some(mut file) = self.add_file(name) else {
			return;
		};
		file.set_data(data);
	}

	#[cfg_attr(feature = "pyo3", staticmethod)]
	pub fn from_buf(data: &[u8], unpack: bool) -> Self {
		let set = Self::new();
		unsafe {
			kkdlib_farc_read_data(set.ptr, data.as_ptr() as *const c_void, data.len(), unpack)
		};
		set
	}

	pub fn to_buf(&self) -> Option<Vec<u8>> {
		let mut ptr = std::ptr::null_mut();
		let mut size = 0usize;
		unsafe {
			kkdlib_farc_write_data(
				self.ptr,
				&mut ptr,
				&mut size,
				self.signature(),
				self.flags(),
			);
		}

		if ptr.is_null() || size == 0 {
			return None;
		}

		let slice = std::ptr::slice_from_raw_parts(ptr as *const u8, size);
		let slice = unsafe { slice.as_ref()? };

		let mut vec = Vec::with_capacity(size);
		vec.extend_from_slice(slice);

		unsafe { kkdlib_farc_delete_packed_file(ptr) };

		Some(vec)
	}
}

// These funcs cannot be used as pymethods
impl Farc {
	pub fn files<'a>(&'a self) -> FarcFileIterator<'a> {
		FarcFileIterator {
			ptr: self.ptr,
			current_index: 0,
			len: unsafe { kkdlib_farc_get_files_size(self.ptr) },
			phantom: PhantomData,
		}
	}

	pub fn get_file<'a>(&'a self, name: &str) -> Option<FarcFile<'a>> {
		let c = CString::new(name).ok()?;
		let ptr = unsafe { kkdlib_farc_get_file_by_name(self.ptr, c.as_ptr()) };
		if ptr.is_null() {
			None
		} else {
			Some(FarcFile {
				ptr,
				phantom: PhantomData,
			})
		}
	}

	pub fn get_file_mut<'a>(&'a mut self, name: &str) -> Option<FarcFileMut<'a>> {
		let c = CString::new(name).ok()?;
		let ptr = unsafe { kkdlib_farc_get_file_by_name(self.ptr, c.as_ptr()) };
		if ptr.is_null() {
			None
		} else {
			Some(FarcFileMut {
				ptr,
				phantom: PhantomData,
			})
		}
	}

	pub fn add_file<'a>(&'a mut self, name: &str) -> Option<FarcFileMut<'a>> {
		let c = CString::new(name).ok()?;
		let ptr = unsafe { kkdlib_farc_add_file(self.ptr, c.as_ptr()) };
		if (self.flags() & Flags::Gzip) == Flags::Gzip {
			unsafe { kkdlib_farc_file_set_compressed(ptr, true) };
		}
		if (self.flags() & Flags::Aes) == Flags::Aes {
			unsafe { kkdlib_farc_file_set_encrypted(ptr, true) };
		}
		Some(FarcFileMut {
			ptr,
			phantom: PhantomData,
		})
	}
}

impl Drop for Farc {
	fn drop(&mut self) {
		unsafe { kkdlib_farc_delete(self.ptr) };
	}
}

pub struct FarcFile<'a> {
	pub(crate) ptr: *mut c_void,
	phantom: PhantomData<&'a Farc>,
}

unsafe impl Send for FarcFile<'_> {}
unsafe impl Sync for FarcFile<'_> {}

impl<'a> FarcFile<'a> {
	pub fn name(&self) -> String {
		let c = unsafe { kkdlib_farc_file_get_name(self.ptr) };
		let ffi = unsafe { CStr::from_ptr(c) };
		ffi.to_string_lossy().to_string()
	}

	pub fn size(&self) -> usize {
		unsafe { kkdlib_farc_file_get_size(self.ptr) }
	}

	pub fn data(&self) -> Option<&'a [u8]> {
		let ptr = unsafe { kkdlib_farc_file_get_data(self.ptr) };
		if ptr.is_null() {
			return None;
		}
		let slice = std::ptr::slice_from_raw_parts(ptr as *const u8, self.size());
		unsafe { slice.as_ref() }
	}

	pub fn compressed(&self) -> bool {
		unsafe { kkdlib_farc_file_get_compressed(self.ptr) }
	}

	pub fn encrypted(&self) -> bool {
		unsafe { kkdlib_farc_file_get_encrypted(self.ptr) }
	}
}

pub struct FarcFileMut<'a> {
	pub(crate) ptr: *mut c_void,
	phantom: PhantomData<&'a mut Farc>,
}

unsafe impl Send for FarcFileMut<'_> {}
unsafe impl Sync for FarcFileMut<'_> {}

impl<'a> FarcFileMut<'a> {
	pub fn name(&self) -> String {
		let c = unsafe { kkdlib_farc_file_get_name(self.ptr) };
		let ffi = unsafe { CStr::from_ptr(c) };
		ffi.to_string_lossy().to_string()
	}

	pub fn set_name(&mut self, name: &str) {
		let Ok(c) = CString::new(name) else {
			return;
		};
		unsafe { kkdlib_farc_file_set_name(self.ptr, c.as_ptr()) };
	}

	pub fn size(&self) -> usize {
		unsafe { kkdlib_farc_file_get_size(self.ptr) }
	}

	pub fn data(&self) -> Option<&'a [u8]> {
		let ptr = unsafe { kkdlib_farc_file_get_data(self.ptr) };
		if ptr.is_null() {
			return None;
		}
		let slice = std::ptr::slice_from_raw_parts(ptr as *const u8, self.size());
		unsafe { slice.as_ref() }
	}

	pub fn set_data(&mut self, data: &[u8]) {
		unsafe { kkdlib_farc_file_set_data(self.ptr, data.as_ptr() as *const c_void, data.len()) };
	}

	pub fn compressed(&self) -> bool {
		unsafe { kkdlib_farc_file_get_compressed(self.ptr) }
	}

	pub fn set_compressed(&mut self, compressed: bool) {
		unsafe { kkdlib_farc_file_set_compressed(self.ptr, compressed) };
	}

	pub fn encrypted(&self) -> bool {
		unsafe { kkdlib_farc_file_get_encrypted(self.ptr) }
	}

	pub fn set_encrypted(&mut self, encrypted: bool) {
		unsafe { kkdlib_farc_file_set_encrypted(self.ptr, encrypted) };
	}
}

pub struct FarcFileIterator<'a> {
	pub(crate) ptr: *mut c_void,
	current_index: usize,
	len: usize,
	phantom: PhantomData<&'a Farc>,
}

impl<'a> Iterator for FarcFileIterator<'a> {
	type Item = FarcFile<'a>;
	fn next(&mut self) -> Option<Self::Item> {
		if self.current_index == self.len {
			return None;
		}

		let ptr = unsafe { kkdlib_farc_get_file_by_index(self.ptr, self.current_index) };
		if ptr.is_null() {
			return None;
		}

		self.current_index += 1;
		Some(Self::Item {
			ptr,
			phantom: PhantomData,
		})
	}

	fn size_hint(&self) -> (usize, Option<usize>) {
		(self.len, Some(self.len))
	}
}

unsafe extern "C" {
	fn kkdlib_farc_new(signature: Signature, flags: Flags, ft: bool) -> *mut c_void;
	fn kkdlib_farc_get_files_size(farc: *mut c_void) -> usize;
	fn kkdlib_farc_get_file_by_index(farc: *mut c_void, index: usize) -> *mut c_void;
	fn kkdlib_farc_get_file_by_name(farc: *mut c_void, name: *const c_char) -> *mut c_void;
	fn kkdlib_farc_add_file(farc: *mut c_void, name: *const c_char) -> *mut c_void;
	fn kkdlib_farc_get_signature(farc: *mut c_void) -> Signature;
	fn kkdlib_farc_set_signature(farc: *mut c_void, signature: Signature);
	fn kkdlib_farc_get_flags(farc: *mut c_void) -> Flags;
	fn kkdlib_farc_set_flags(farc: *mut c_void, flags: Flags);
	fn kkdlib_farc_get_compression_level(farc: *mut c_void) -> i32;
	fn kkdlib_farc_set_compression_level(farc: *mut c_void, compression_level: i32);
	fn kkdlib_farc_get_alignment(farc: *mut c_void) -> u32;
	fn kkdlib_farc_set_alignment(farc: *mut c_void, alignment: u32);
	fn kkdlib_farc_get_ft(farc: *mut c_void) -> bool;
	fn kkdlib_farc_set_ft(farc: *mut c_void, ft: bool);
	fn kkdlib_farc_read_file(farc: *mut c_void, path: *const c_char, unpack: bool, save: bool);
	fn kkdlib_farc_read_data(farc: *mut c_void, data: *const c_void, size: usize, unpack: bool);
	fn kkdlib_farc_write_file(
		farc: *mut c_void,
		path: *const c_char,
		signature: Signature,
		flags: Flags,
		add_extension: bool,
		get_files: bool,
	);
	fn kkdlib_farc_write_data(
		farc: *mut c_void,
		data: *mut *mut c_void,
		size: *mut usize,
		signature: Signature,
		flags: Flags,
	);
	fn kkdlib_farc_delete_packed_file(data: *mut c_void);
	fn kkdlib_farc_delete(farc: *mut c_void);

	fn kkdlib_farc_file_get_name(file: *mut c_void) -> *const c_char;
	fn kkdlib_farc_file_set_name(file: *mut c_void, name: *const c_char);
	fn kkdlib_farc_file_get_size(file: *mut c_void) -> usize;
	fn kkdlib_farc_file_get_data(file: *mut c_void) -> *mut c_void;
	fn kkdlib_farc_file_set_data(file: *mut c_void, data: *const c_void, size: usize);
	fn kkdlib_farc_file_get_compressed(file: *mut c_void) -> bool;
	fn kkdlib_farc_file_set_compressed(file: *mut c_void, compressed: bool);
	fn kkdlib_farc_file_get_encrypted(file: *mut c_void) -> bool;
	fn kkdlib_farc_file_set_encrypted(file: *mut c_void, encrypted: bool);
}
