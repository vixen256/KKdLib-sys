use std::ffi::*;
use std::marker::PhantomData;

pub mod file {
	use std::ffi::*;
	use std::marker::PhantomData;

	#[cfg(feature = "pyo3")]
	use pyo3::prelude::*;

	#[cfg_attr(feature = "pyo3", pyclass)]
	pub struct Entry {
		ptr: *mut c_void,
	}

	unsafe impl Send for Entry {}
	unsafe impl Sync for Entry {}

	#[cfg_eval]
	#[cfg_attr(feature = "pyo3", pymethods)]
	impl Entry {
		#[cfg_attr(feature = "pyo3", new)]
		pub fn new() -> Self {
			Self {
				ptr: unsafe { kkdlib_spr_db_spr_file_new() },
			}
		}

		#[cfg_attr(feature = "pyo3", getter)]
		pub fn id(&self) -> u32 {
			unsafe { kkdlib_spr_db_spr_file_get_id(self.ptr) }
		}

		#[cfg_attr(feature = "pyo3", setter)]
		pub fn set_id(&mut self, id: u32) {
			unsafe { kkdlib_spr_db_spr_file_set_id(self.ptr, id) };
		}

		#[cfg_attr(feature = "pyo3", getter)]
		pub fn name(&self) -> String {
			let name = unsafe { kkdlib_spr_db_spr_file_get_name(self.ptr) };
			let name = unsafe { CStr::from_ptr(name) };
			name.to_string_lossy().to_string()
		}

		#[cfg_attr(feature = "pyo3", setter)]
		pub fn set_name(&mut self, name: &str) {
			let Ok(name) = CString::new(name) else {
				return;
			};
			unsafe { kkdlib_spr_db_spr_file_set_name(self.ptr, name.as_ptr()) };
		}

		#[cfg_attr(feature = "pyo3", getter)]
		pub fn index(&self) -> u16 {
			unsafe { kkdlib_spr_db_spr_file_get_index(self.ptr) }
		}

		#[cfg_attr(feature = "pyo3", setter)]
		pub fn set_index(&mut self, index: u16) {
			unsafe { kkdlib_spr_db_spr_file_set_index(self.ptr, index) };
		}

		#[cfg_attr(feature = "pyo3", getter)]
		pub fn texture(&self) -> bool {
			unsafe { kkdlib_spr_db_spr_file_get_texture(self.ptr) }
		}

		#[cfg_attr(feature = "pyo3", setter)]
		pub fn set_texture(&mut self, texture: bool) {
			unsafe { kkdlib_spr_db_spr_file_set_texture(self.ptr, texture) };
		}
	}

	impl Drop for Entry {
		fn drop(&mut self) {
			unsafe { kkdlib_spr_db_spr_file_delete(self.ptr) };
		}
	}

	pub struct EntryRef<'a> {
		_ptr: *mut c_void,
		phantom: PhantomData<&'a Entry>,
	}

	unsafe impl Send for EntryRef<'_> {}
	unsafe impl Sync for EntryRef<'_> {}

	impl EntryRef<'_> {
		pub fn id(&self) -> u32 {
			Entry::id(unsafe { std::mem::transmute(self) })
		}

		pub fn name(&self) -> String {
			Entry::name(unsafe { std::mem::transmute(self) })
		}

		pub fn index(&self) -> u16 {
			Entry::index(unsafe { std::mem::transmute(self) })
		}

		pub fn texture(&self) -> bool {
			Entry::texture(unsafe { std::mem::transmute(self) })
		}
	}

	pub struct EntryIter<'a> {
		ptr: *mut c_void,
		index: usize,
		len: usize,
		phantom: PhantomData<&'a Set>,
	}

	impl<'a> Iterator for EntryIter<'a> {
		type Item = EntryRef<'a>;

		fn next(&mut self) -> Option<Self::Item> {
			if self.index == self.len {
				return None;
			}

			let entry = unsafe { kkdlib_spr_db_spr_set_file_get_sprite(self.ptr, self.index) };
			if entry.is_null() {
				return None;
			}

			self.index += 1;
			Some(EntryRef {
				_ptr: entry,
				phantom: PhantomData,
			})
		}

		fn size_hint(&self) -> (usize, Option<usize>) {
			(self.len, Some(self.len))
		}
	}

	#[cfg_attr(feature = "pyo3", pyclass)]
	pub struct Set {
		ptr: *mut c_void,
	}

	unsafe impl Send for Set {}
	unsafe impl Sync for Set {}

	#[cfg_eval]
	#[cfg_attr(feature = "pyo3", pymethods)]
	impl Set {
		#[cfg_attr(feature = "pyo3", new)]
		pub fn new() -> Self {
			Self {
				ptr: unsafe { kkdlib_spr_db_spr_set_file_new() },
			}
		}

		#[cfg_attr(feature = "pyo3", getter)]
		pub fn id(&self) -> u32 {
			unsafe { kkdlib_spr_db_spr_set_file_get_id(self.ptr) }
		}

		#[cfg_attr(feature = "pyo3", setter)]
		pub fn set_id(&mut self, id: u32) {
			unsafe { kkdlib_spr_db_spr_set_file_set_id(self.ptr, id) };
		}

		#[cfg_attr(feature = "pyo3", getter)]
		pub fn name(&self) -> String {
			let name = unsafe { kkdlib_spr_db_spr_set_file_get_name(self.ptr) };
			let name = unsafe { CStr::from_ptr(name) };
			name.to_string_lossy().to_string()
		}

		#[cfg_attr(feature = "pyo3", setter)]
		pub fn set_name(&mut self, name: &str) {
			let Ok(name) = CString::new(name) else {
				return;
			};
			unsafe { kkdlib_spr_db_spr_set_file_set_name(self.ptr, name.as_ptr()) };
		}

		#[cfg_attr(feature = "pyo3", getter)]
		pub fn file_name(&self) -> String {
			let name = unsafe { kkdlib_spr_db_spr_set_file_get_file_name(self.ptr) };
			let name = unsafe { CStr::from_ptr(name) };
			name.to_string_lossy().to_string()
		}

		#[cfg_attr(feature = "pyo3", setter)]
		pub fn set_file_name(&mut self, file_name: &str) {
			let Ok(file_name) = CString::new(file_name) else {
				return;
			};
			unsafe { kkdlib_spr_db_spr_set_file_set_file_name(self.ptr, file_name.as_ptr()) };
		}

		pub fn add_sprite(&self, entry: &Entry) {
			unsafe {
				kkdlib_spr_db_spr_set_file_add_sprite(self.ptr, entry.ptr);
			}
		}
	}

	impl<'a> Set {
		pub fn sprites(&'a self) -> EntryIter<'a> {
			EntryIter {
				ptr: self.ptr,
				index: 0,
				len: unsafe { kkdlib_spr_db_spr_set_file_get_sprite_size(self.ptr) },
				phantom: PhantomData,
			}
		}
	}

	impl Drop for Set {
		fn drop(&mut self) {
			unsafe { kkdlib_spr_db_spr_set_file_delete(self.ptr) };
		}
	}

	pub struct SetRef<'a> {
		_ptr: *mut c_void,
		phantom: PhantomData<&'a Set>,
	}

	unsafe impl Send for SetRef<'_> {}
	unsafe impl Sync for SetRef<'_> {}

	impl<'a> SetRef<'a> {
		pub fn id(&self) -> u32 {
			Set::id(unsafe { std::mem::transmute(self) })
		}

		pub fn name(&self) -> String {
			Set::name(unsafe { std::mem::transmute(self) })
		}

		pub fn file_name(&self) -> String {
			Set::file_name(unsafe { std::mem::transmute(self) })
		}

		pub fn sprites(&'a self) -> EntryIter<'a> {
			Set::sprites(unsafe { std::mem::transmute(self) })
		}
	}

	pub struct SetIter<'a> {
		ptr: *mut c_void,
		index: usize,
		len: usize,
		phantom: PhantomData<&'a Database>,
	}

	impl<'a> Iterator for SetIter<'a> {
		type Item = SetRef<'a>;

		fn next(&mut self) -> Option<Self::Item> {
			if self.index == self.len {
				return None;
			}

			let set = unsafe { kkdlib_sprite_database_file_get_sprite_set(self.ptr, self.index) };
			if set.is_null() {
				return None;
			}

			self.index += 1;
			Some(SetRef {
				_ptr: set,
				phantom: PhantomData,
			})
		}

		fn size_hint(&self) -> (usize, Option<usize>) {
			(self.len, Some(self.len))
		}
	}

	#[cfg_attr(feature = "pyo3", pyclass)]
	pub struct Database {
		pub(crate) ptr: *mut c_void,
	}

	unsafe impl Send for Database {}
	unsafe impl Sync for Database {}

	#[cfg_eval]
	#[cfg_attr(feature = "pyo3", pymethods)]
	impl Database {
		#[cfg_attr(feature = "pyo3", new)]
		pub fn new() -> Self {
			Self {
				ptr: unsafe { kkdlib_sprite_database_file_new() },
			}
		}

		#[cfg_attr(feature = "pyo3", getter)]
		pub fn ready(&self) -> bool {
			unsafe { kkdlib_sprite_database_file_get_ready(self.ptr) }
		}

		#[cfg_attr(feature = "pyo3", setter)]
		pub fn set_ready(&mut self, ready: bool) {
			unsafe { kkdlib_sprite_database_file_set_ready(self.ptr, ready) };
		}

		#[cfg_attr(feature = "pyo3", getter)]
		pub fn modern(&self) -> bool {
			unsafe { kkdlib_sprite_database_file_get_modern(self.ptr) }
		}

		#[cfg_attr(feature = "pyo3", setter)]
		pub fn set_modern(&mut self, modern: bool) {
			unsafe { kkdlib_sprite_database_file_set_modern(self.ptr, modern) };
		}

		#[cfg_attr(feature = "pyo3", getter)]
		pub fn big_endian(&self) -> bool {
			unsafe { kkdlib_sprite_database_file_get_big_endian(self.ptr) }
		}

		#[cfg_attr(feature = "pyo3", setter)]
		pub fn set_big_endian(&mut self, big_endian: bool) {
			unsafe { kkdlib_sprite_database_file_set_big_endian(self.ptr, big_endian) };
		}

		#[cfg_attr(feature = "pyo3", getter)]
		pub fn is_x(&self) -> bool {
			unsafe { kkdlib_sprite_database_file_get_is_x(self.ptr) }
		}

		#[cfg_attr(feature = "pyo3", setter)]
		pub fn set_is_x(&mut self, is_x: bool) {
			unsafe { kkdlib_sprite_database_file_set_is_x(self.ptr, is_x) };
		}

		pub fn add_set(&mut self, set: &Set) {
			unsafe { kkdlib_sprite_database_file_add_sprite_set(self.ptr, set.ptr) };
		}

		#[cfg_attr(feature = "pyo3", staticmethod)]
		pub fn from_buf(data: &[u8], modern: bool) -> Self {
			let database = Self::new();
			unsafe {
				kkdlib_sprite_database_file_read_data(
					database.ptr,
					data.as_ptr() as *const c_void,
					data.len(),
					modern,
				)
			};
			database
		}

		pub fn to_buf(&self) -> Option<Vec<u8>> {
			let mut ptr = std::ptr::null_mut();
			let mut size = 0usize;
			unsafe {
				kkdlib_sprite_database_file_write_data(self.ptr, &mut ptr, &mut size);
			}

			if ptr.is_null() || size == 0 {
				return None;
			}

			let slice = std::ptr::slice_from_raw_parts(ptr as *const u8, size);
			let slice = unsafe { slice.as_ref()? };

			let mut vec = Vec::with_capacity(size);
			vec.extend_from_slice(slice);

			unsafe { kkdlib_sprite_databse_file_delete_packed_data(ptr) };

			Some(vec)
		}
	}

	impl<'a> Database {
		pub fn sets(&'a self) -> SetIter<'a> {
			SetIter {
				ptr: self.ptr,
				index: 0,
				len: unsafe { kkdlib_sprite_database_file_get_sprite_set_size(self.ptr) },
				phantom: PhantomData,
			}
		}
	}

	impl Drop for Database {
		fn drop(&mut self) {
			unsafe { kkdlib_sprite_database_file_delete(self.ptr) };
		}
	}

	unsafe extern "C" {
		fn kkdlib_spr_db_spr_file_new() -> *mut c_void;
		fn kkdlib_spr_db_spr_file_get_id(spr_file: *mut c_void) -> u32;
		fn kkdlib_spr_db_spr_file_set_id(spr_file: *mut c_void, id: u32);
		fn kkdlib_spr_db_spr_file_get_name(spr_file: *mut c_void) -> *const c_char;
		fn kkdlib_spr_db_spr_file_set_name(spr_file: *mut c_void, name: *const c_char);
		fn kkdlib_spr_db_spr_file_get_index(spr_file: *mut c_void) -> u16;
		fn kkdlib_spr_db_spr_file_set_index(spr_file: *mut c_void, index: u16);
		fn kkdlib_spr_db_spr_file_get_texture(spr_file: *mut c_void) -> bool;
		fn kkdlib_spr_db_spr_file_set_texture(spr_file: *mut c_void, texture: bool);
		fn kkdlib_spr_db_spr_file_delete(spr_file: *mut c_void);

		fn kkdlib_spr_db_spr_set_file_new() -> *mut c_void;
		fn kkdlib_spr_db_spr_set_file_get_id(set_file: *mut c_void) -> u32;
		fn kkdlib_spr_db_spr_set_file_set_id(set_file: *mut c_void, id: u32);
		fn kkdlib_spr_db_spr_set_file_get_name(set_file: *mut c_void) -> *const c_char;
		fn kkdlib_spr_db_spr_set_file_set_name(set_file: *mut c_void, name: *const c_char);
		fn kkdlib_spr_db_spr_set_file_get_file_name(set_file: *mut c_void) -> *const c_char;
		fn kkdlib_spr_db_spr_set_file_set_file_name(set_file: *mut c_void, name: *const c_char);
		fn kkdlib_spr_db_spr_set_file_get_sprite_size(set_file: *mut c_void) -> usize;
		fn kkdlib_spr_db_spr_set_file_get_sprite(
			set_file: *mut c_void,
			index: usize,
		) -> *mut c_void;
		fn kkdlib_spr_db_spr_set_file_add_sprite(set_file: *mut c_void, spr_file: *mut c_void);
		fn kkdlib_spr_db_spr_set_file_delete(set_file: *mut c_void);

		fn kkdlib_sprite_database_file_new() -> *mut c_void;
		fn kkdlib_sprite_database_file_get_ready(database_file: *mut c_void) -> bool;
		fn kkdlib_sprite_database_file_set_ready(database_file: *mut c_void, ready: bool);
		fn kkdlib_sprite_database_file_get_modern(database_file: *mut c_void) -> bool;
		fn kkdlib_sprite_database_file_set_modern(database_file: *mut c_void, modern: bool);
		fn kkdlib_sprite_database_file_get_big_endian(database_file: *mut c_void) -> bool;
		fn kkdlib_sprite_database_file_set_big_endian(database_file: *mut c_void, big_endian: bool);
		fn kkdlib_sprite_database_file_get_is_x(database_file: *mut c_void) -> bool;
		fn kkdlib_sprite_database_file_set_is_x(database_file: *mut c_void, is_x: bool);
		fn kkdlib_sprite_database_file_get_sprite_set_size(database_file: *mut c_void) -> usize;
		fn kkdlib_sprite_database_file_get_sprite_set(
			database_file: *mut c_void,
			index: usize,
		) -> *mut c_void;
		fn kkdlib_sprite_database_file_add_sprite_set(
			database_file: *mut c_void,
			set_file: *mut c_void,
		);
		fn kkdlib_sprite_database_file_read_data(
			database_file: *mut c_void,
			data: *const c_void,
			size: usize,
			modern: bool,
		);
		fn kkdlib_sprite_database_file_write_data(
			database_file: *mut c_void,
			data: *mut *mut c_void,
			size: *mut usize,
		);
		fn kkdlib_sprite_databse_file_delete_packed_data(data: *mut c_void);
		fn kkdlib_sprite_database_file_delete(database_file: *mut c_void);
	}
}

pub struct EntryRef<'a> {
	ptr: *const c_void,
	phantom: PhantomData<&'a Database>,
}

unsafe impl Send for EntryRef<'_> {}
unsafe impl Sync for EntryRef<'_> {}

impl EntryRef<'_> {
	pub fn id(&self) -> u32 {
		unsafe { kkdlib_spr_db_spr_get_id(self.ptr) }
	}

	pub fn name(&self) -> String {
		let ptr = unsafe { kkdlib_spr_db_spr_get_name(self.ptr) };
		let cstr = unsafe { CStr::from_ptr(ptr) };
		cstr.to_string_lossy().to_string()
	}

	pub fn index(&self) -> u16 {
		unsafe { kkdlib_spr_db_spr_get_index(self.ptr) }
	}

	pub fn set_index(&self) -> u16 {
		unsafe { kkdlib_spr_db_spr_get_set_index(self.ptr) }
	}
}

pub struct SetRef<'a> {
	ptr: *const c_void,
	phantom: PhantomData<&'a Database>,
}

unsafe impl Send for SetRef<'_> {}
unsafe impl Sync for SetRef<'_> {}

impl SetRef<'_> {
	pub fn id(&self) -> u32 {
		unsafe { kkdlib_spr_db_spr_set_get_id(self.ptr) }
	}

	pub fn name(&self) -> String {
		let ptr = unsafe { kkdlib_spr_db_spr_set_get_name(self.ptr) };
		let cstr = unsafe { CStr::from_ptr(ptr) };
		cstr.to_string_lossy().to_string()
	}

	pub fn file_name(&self) -> String {
		let ptr = unsafe { kkdlib_spr_db_spr_set_get_file_name(self.ptr) };
		let cstr = unsafe { CStr::from_ptr(ptr) };
		cstr.to_string_lossy().to_string()
	}

	pub fn index(&self) -> u32 {
		unsafe { kkdlib_spr_db_spr_set_get_index(self.ptr) }
	}
}

pub struct Database {
	ptr: *mut c_void,
}

unsafe impl Send for Database {}
unsafe impl Sync for Database {}

impl<'a> Database {
	pub fn new() -> Self {
		Self {
			ptr: unsafe { kkdlib_sprite_database_new() },
		}
	}

	pub fn add_file(&mut self, file: &file::Database) {
		unsafe { kkdlib_sprite_database_add_file(self.ptr, file.ptr) };
	}

	pub fn get_spr_set_by_name(&'a self, name: &str) -> Option<SetRef<'a>> {
		let cstring = CString::new(name).ok()?;
		let ptr = unsafe { kkdlib_sprite_database_get_spr_set_by_name(self.ptr, cstring.as_ptr()) };
		if ptr.is_null() || unsafe { kkdlib_spr_db_spr_set_get_id(ptr) } == u32::MAX {
			None
		} else {
			Some(SetRef {
				ptr,
				phantom: PhantomData,
			})
		}
	}

	pub fn get_spr_set_by_id(&'a self, id: u32) -> Option<SetRef<'a>> {
		let ptr = unsafe { kkdlib_sprite_database_get_spr_set_by_id(self.ptr, id) };
		if ptr.is_null() || unsafe { kkdlib_spr_db_spr_set_get_id(ptr) } == u32::MAX {
			None
		} else {
			Some(SetRef {
				ptr,
				phantom: PhantomData,
			})
		}
	}

	pub fn get_spr_set_by_index(&'a self, index: u32) -> Option<SetRef<'a>> {
		let ptr = unsafe { kkdlib_sprite_database_get_spr_set_by_index(self.ptr, index) };
		if ptr.is_null() || unsafe { kkdlib_spr_db_spr_set_get_id(ptr) } == u32::MAX {
			None
		} else {
			Some(SetRef {
				ptr,
				phantom: PhantomData,
			})
		}
	}

	pub fn get_spr_by_name(&'a self, name: &str) -> Option<EntryRef<'a>> {
		let cstring = CString::new(name).ok()?;
		let ptr = unsafe { kkdlib_sprite_database_get_spr_by_name(self.ptr, cstring.as_ptr()) };
		if ptr.is_null() || unsafe { kkdlib_spr_db_spr_get_id(ptr) } == u32::MAX {
			None
		} else {
			Some(EntryRef {
				ptr,
				phantom: PhantomData,
			})
		}
	}

	pub fn get_spr_by_id(&'a self, id: u32) -> Option<EntryRef<'a>> {
		let ptr = unsafe { kkdlib_sprite_database_get_spr_by_id(self.ptr, id) };
		if ptr.is_null() || unsafe { kkdlib_spr_db_spr_get_id(ptr) } == u32::MAX {
			None
		} else {
			Some(EntryRef {
				ptr,
				phantom: PhantomData,
			})
		}
	}
}

impl Drop for Database {
	fn drop(&mut self) {
		unsafe { kkdlib_sprite_database_delete(self.ptr) };
	}
}

unsafe extern "C" {
	fn kkdlib_spr_db_spr_get_id(spr: *const c_void) -> u32;
	fn kkdlib_spr_db_spr_get_name(spr: *const c_void) -> *const c_char;
	fn kkdlib_spr_db_spr_get_index(spr: *const c_void) -> u16;
	fn kkdlib_spr_db_spr_get_set_index(spr: *const c_void) -> u16;

	fn kkdlib_spr_db_spr_set_get_id(set: *const c_void) -> u32;
	fn kkdlib_spr_db_spr_set_get_name(set: *const c_void) -> *const c_char;
	fn kkdlib_spr_db_spr_set_get_file_name(set: *const c_void) -> *const c_char;
	fn kkdlib_spr_db_spr_set_get_index(set: *const c_void) -> u32;

	fn kkdlib_sprite_database_new() -> *mut c_void;
	fn kkdlib_sprite_database_add_file(database: *mut c_void, file: *mut c_void);
	fn kkdlib_sprite_database_get_spr_set_by_name(
		database: *mut c_void,
		name: *const c_char,
	) -> *const c_void;
	fn kkdlib_sprite_database_get_spr_set_by_id(
		database: *mut c_void,
		set_id: u32,
	) -> *const c_void;
	fn kkdlib_sprite_database_get_spr_set_by_index(
		database: *mut c_void,
		index: u32,
	) -> *const c_void;
	fn kkdlib_sprite_database_get_spr_by_name(
		database: *mut c_void,
		name: *const c_char,
	) -> *const c_void;
	fn kkdlib_sprite_database_get_spr_by_id(database: *mut c_void, id: u32) -> *const c_void;
	fn kkdlib_sprite_database_delete(database: *mut c_void);
}
