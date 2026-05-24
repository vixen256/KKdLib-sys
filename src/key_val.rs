use std::ffi::*;

pub struct KeyVal {
	pub(crate) ptr: *mut c_void,
}

impl KeyVal {
	pub fn new() -> Self {
		Self {
			ptr: unsafe { kkdlib_key_val_new() },
		}
	}

	pub fn parse(&mut self, data: &str) {
		unsafe { kkdlib_key_val_parse(self.ptr, data.as_ptr() as *const c_void, data.len()) };
	}

	pub fn from_data(data: &str) -> Self {
		let mut kv = Self::new();
		kv.parse(data);
		kv
	}

	pub fn open_scope<'a>(&'a self, key: &str) -> Option<ScopeGuard<'a>> {
		let Ok(c) = CString::new(key) else {
			return None;
		};
		if unsafe { kkdlib_key_val_open_scope(self.ptr, c.as_ptr()) } {
			Some(ScopeGuard { kv: self })
		} else {
			None
		}
	}

	pub fn open_scope_num<'a>(&'a self, i: u32) -> Option<ScopeGuard<'a>> {
		if unsafe { kkdlib_key_val_open_scope_uint32(self.ptr, i) } {
			Some(ScopeGuard { kv: self })
		} else {
			None
		}
	}

	pub fn has_key(&self, key: &str) -> bool {
		let Ok(c) = CString::new(key) else {
			return false;
		};
		unsafe { kkdlib_key_val_has_key(self.ptr, c.as_ptr()) }
	}

	pub fn read_bool(&self, key: &str) -> Option<bool> {
		let mut out = false;
		let c = CString::new(key).ok()?;
		if unsafe { kkdlib_key_val_read_bool(self.ptr, c.as_ptr(), &mut out) } {
			Some(out)
		} else {
			None
		}
	}

	pub fn read_f32(&self, key: &str) -> Option<f32> {
		let mut out = 0f32;
		let c = CString::new(key).ok()?;
		if unsafe { kkdlib_key_val_read_float(self.ptr, c.as_ptr(), &mut out) } {
			Some(out)
		} else {
			None
		}
	}

	pub fn read_i32(&self, key: &str) -> Option<i32> {
		let mut out = 0i32;
		let c = CString::new(key).ok()?;
		if unsafe { kkdlib_key_val_read_int32(self.ptr, c.as_ptr(), &mut out) } {
			Some(out)
		} else {
			None
		}
	}

	pub fn read_u32(&self, key: &str) -> Option<u32> {
		let mut out = 0u32;
		let c = CString::new(key).ok()?;
		if unsafe { kkdlib_key_val_read_uint32(self.ptr, c.as_ptr(), &mut out) } {
			Some(out)
		} else {
			None
		}
	}

	pub fn read_str(&self, key: &str) -> Option<&str> {
		let mut out = std::ptr::null();
		let c = CString::new(key).ok()?;
		if unsafe { kkdlib_key_val_read_str(self.ptr, c.as_ptr(), &mut out) } {
			if out.is_null() {
				return None;
			}
			let c = unsafe { CStr::from_ptr(out) };
			c.to_str().ok()
		} else {
			None
		}
	}
}

impl Drop for KeyVal {
	fn drop(&mut self) {
		unsafe { kkdlib_key_val_delete(self.ptr) };
	}
}

pub struct ScopeGuard<'a> {
	pub(crate) kv: &'a KeyVal,
}

impl ScopeGuard<'_> {
	pub fn open_scope<'a>(&'a self, key: &str) -> Option<ScopeGuard<'a>> {
		let Ok(c) = CString::new(key) else {
			return None;
		};
		if unsafe { kkdlib_key_val_open_scope(self.kv.ptr, c.as_ptr()) } {
			Some(ScopeGuard { kv: self.kv })
		} else {
			None
		}
	}

	pub fn open_scope_num<'a>(&'a self, i: u32) -> Option<ScopeGuard<'a>> {
		if unsafe { kkdlib_key_val_open_scope_uint32(self.kv.ptr, i) } {
			Some(ScopeGuard { kv: self.kv })
		} else {
			None
		}
	}

	pub fn has_key(&self, key: &str) -> bool {
		let Ok(c) = CString::new(key) else {
			return false;
		};
		unsafe { kkdlib_key_val_has_key(self.kv.ptr, c.as_ptr()) }
	}

	pub fn read_bool(&self, key: &str) -> Option<bool> {
		let mut out = false;
		let c = CString::new(key).ok()?;
		if unsafe { kkdlib_key_val_read_bool(self.kv.ptr, c.as_ptr(), &mut out) } {
			Some(out)
		} else {
			None
		}
	}

	pub fn read_f32(&self, key: &str) -> Option<f32> {
		let mut out = 0f32;
		let c = CString::new(key).ok()?;
		if unsafe { kkdlib_key_val_read_float(self.kv.ptr, c.as_ptr(), &mut out) } {
			Some(out)
		} else {
			None
		}
	}

	pub fn read_i32(&self, key: &str) -> Option<i32> {
		let mut out = 0i32;
		let c = CString::new(key).ok()?;
		if unsafe { kkdlib_key_val_read_int32(self.kv.ptr, c.as_ptr(), &mut out) } {
			Some(out)
		} else {
			None
		}
	}

	pub fn read_u32(&self, key: &str) -> Option<u32> {
		let mut out = 0u32;
		let c = CString::new(key).ok()?;
		if unsafe { kkdlib_key_val_read_uint32(self.kv.ptr, c.as_ptr(), &mut out) } {
			Some(out)
		} else {
			None
		}
	}

	pub fn read_str(&self, key: &str) -> Option<&str> {
		let mut out = std::ptr::null();
		let c = CString::new(key).ok()?;
		if unsafe { kkdlib_key_val_read_str(self.kv.ptr, c.as_ptr(), &mut out) } {
			if out.is_null() {
				return None;
			}
			let c = unsafe { CStr::from_ptr(out) };
			c.to_str().ok()
		} else {
			None
		}
	}
}

impl Drop for ScopeGuard<'_> {
	fn drop(&mut self) {
		unsafe { kkdlib_key_val_close_scope(self.kv.ptr) };
	}
}

unsafe extern "C" {
	fn kkdlib_key_val_new() -> *mut c_void;
	fn kkdlib_key_val_close_scope(kv: *mut c_void);
	fn kkdlib_key_val_has_key(kv: *mut c_void, key: *const c_char) -> bool;
	fn kkdlib_key_val_open_scope(kv: *mut c_void, key: *const c_char) -> bool;
	fn kkdlib_key_val_open_scope_uint32(kv: *mut c_void, i: u32) -> bool;
	fn kkdlib_key_val_parse(kv: *mut c_void, data: *const c_void, size: usize);
	fn kkdlib_key_val_read_bool(kv: *mut c_void, key: *const c_char, value: *mut bool) -> bool;
	fn kkdlib_key_val_read_float(kv: *mut c_void, key: *const c_char, value: *mut f32) -> bool;
	fn kkdlib_key_val_read_int32(kv: *mut c_void, key: *const c_char, value: *mut i32) -> bool;
	fn kkdlib_key_val_read_uint32(kv: *mut c_void, key: *const c_char, value: *mut u32) -> bool;
	fn kkdlib_key_val_read_str(
		kv: *mut c_void,
		key: *const c_char,
		value: *mut *const c_char,
	) -> bool;
	fn kkdlib_key_val_delete(kv: *mut c_void);
}
