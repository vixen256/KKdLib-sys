use std::ffi::*;

pub fn fnv1a64m<T: Into<Vec<u8>>>(data: T) -> u64 {
	let data = data.into();
	unsafe { kkdlib_hash_fnv1a64m(data.as_ptr() as *const c_void, data.len()) }
}

pub fn murmurhash<T: Into<Vec<u8>>>(data: T) -> u32 {
	let data = data.into();
	unsafe { kkdlib_hash_murmurhash(data.as_ptr() as *const c_void, data.len()) }
}

pub fn crc16_ccitt<T: Into<Vec<u8>>>(data: T) -> u16 {
	let data = data.into();
	unsafe { kkdlib_hash_crc16_ccitt(data.as_ptr() as *const c_void, data.len()) }
}

pub fn xxh3_64bits<T: Into<Vec<u8>>>(data: T) -> u64 {
	let data = data.into();
	unsafe { kkdlib_hash_xxh3_64bits(data.as_ptr() as *const c_void, data.len()) }
}

pub fn adler32<T: Into<Vec<u8>>>(data: T, adler: u32) -> u32 {
	let data = data.into();
	unsafe { kkdlib_hash_adler32(adler, data.as_ptr() as *const c_void, data.len()) }
}

unsafe extern "C" {
	fn kkdlib_hash_fnv1a64m(data: *const c_void, size: usize) -> u64;
	fn kkdlib_hash_murmurhash(data: *const c_void, size: usize) -> u32;
	fn kkdlib_hash_crc16_ccitt(data: *const c_void, size: usize) -> u16;
	fn kkdlib_hash_xxh3_64bits(data: *const c_void, size: usize) -> u64;
	fn kkdlib_hash_adler32(adler: u32, data: *const c_void, size: usize) -> u32;
}
