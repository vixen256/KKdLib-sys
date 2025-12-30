fn main() {
	let xxhash_files = ["xxHash/xxhash.c"];

	let files = [
		"database/aet.cpp",
		"database/sprite.cpp",
		"f2/enrs.cpp",
		"f2/header.cpp",
		"f2/pof.cpp",
		"f2/struct.cpp",
		"io/file_stream.cpp",
		"io/memory_stream.cpp",
		"io/path.cpp",
		"io/stream.cpp",
		"prj/stack_allocator.cpp",
		"prj/time.cpp",
		"aes.cpp",
		"aet.cpp",
		"default.cpp",
		"deflate.cpp",
		"divafile.cpp",
		"farc.cpp",
		"half_t.cpp",
		"hash.cpp",
		"image.cpp",
		"interpolation.cpp",
		"key_val.cpp",
		"kf.cpp",
		"mat.cpp",
		"quat.cpp",
		"sort.cpp",
		"spr.cpp",
		"str_utils.cpp",
		"time.cpp",
		"txp.cpp",
		"vec.cpp",
	];

	let kkdlib_files = files
		.iter()
		.map(|file| format!("KKdLib/{file}"))
		.filter(|file| std::fs::exists(file).map_or(false, |r| r))
		.collect::<Vec<_>>();

	let bridge_files = files
		.iter()
		.map(|file| format!("bridge/{file}"))
		.filter(|file| std::fs::exists(file).map_or(false, |r| r))
		.collect::<Vec<_>>();

	for file in &kkdlib_files {
		println!("cargo:rerun-if-changed={}", file);
	}

	for file in &bridge_files {
		println!("cargo:rerun-if-changed={}", file);
	}

	cc::Build::new()
		.include("xxHash/")
		.files(xxhash_files)
		.static_crt(true)
		.warnings(false)
		.compile("xxhash");

	let mut build = cc::Build::new();
	build
		.include("KKdLib/")
		.include("xxHash/")
		.files(kkdlib_files)
		.files(bridge_files)
		.static_crt(true)
		.cpp(true)
		.std("c++20")
		.warnings(false)
		.extra_warnings(false);

	if let Ok(include) = std::env::var("DEP_LIBDEFLATE_INCLUDE") {
		build.include(include);
	}

	build.compile("kkdlib-bridge");
}
