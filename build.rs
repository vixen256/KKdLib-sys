fn main() {
	let xxhash_files = ["xxHash/xxhash.c"];

	let files = [
		// "database/aet.cpp",
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
		// "key_val.cpp",
		// "kf.cpp",
		"mat.cpp",
		"quat.cpp",
		"sort.cpp",
		"spr.cpp",
		"str_utils.cpp",
		// "time.cpp",
		"txp.cpp",
		"vec.cpp",
	];

	let kkdlib_files = files
		.iter()
		.map(|file| format!("KKdLib/{file}"))
		.filter(|file| std::fs::exists(file).is_ok_and(|r| r))
		.collect::<Vec<_>>();

	let bridge_files = files
		.iter()
		.map(|file| format!("bridge/{file}"))
		.filter(|file| std::fs::exists(file).is_ok_and(|r| r))
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
		.warnings(false)
		.compile("xxhash");

	let mut build = cc::Build::new();
	build
		.include("KKdLib/")
		.include("xxHash/")
		.files(kkdlib_files)
		.files(bridge_files)
		.cpp(true)
		.std("c++17")
		.warnings(false)
		.extra_warnings(false);

	if let Ok(include) = std::env::var("DEP_LIBDEFLATE_INCLUDE") {
		build.include(include);
	}

	build.compile("kkdlib-bridge");

	let mut config = ispc::Config::new();
	config
		.file("bc7enc_rdo/bc7e.ispc")
		.add_define("SUPPORT_BC7E", Some("1"))
		.optimization_opt(ispc::OptimizationOpt::DisableAssertions)
		.optimization_opt(ispc::OptimizationOpt::FastMath);

	if cfg!(target_arch = "x86_64") {
		config.target_isas(vec![
			ispc::TargetISA::SSE2i32x4,
			ispc::TargetISA::SSE4i32x4,
			ispc::TargetISA::AVX1i32x8,
			ispc::TargetISA::AVX2i32x8,
		]);
	}

	config.compile("bc7e");

	println!("cargo:rerun-if-changed=bridge/bc7enc_rdo/rdo_bc_encoder.cpp");

	let mut build = cc::Build::new();
	get_openmp(&mut build);

	build
		.include(std::env::var_os("OUT_DIR").unwrap())
		.include("bc7enc_rdo/")
		.flag("--include=cstdint")
		.define("SUPPORT_BC7E", "1")
		.files([
			"bc7enc_rdo/bc7enc.cpp",
			"bc7enc_rdo/bc7decomp.cpp",
			"bc7enc_rdo/bc7decomp_ref.cpp",
			"bc7enc_rdo/ert.cpp",
			"bc7enc_rdo/lodepng.cpp",
			"bc7enc_rdo/rdo_bc_encoder.cpp",
			"bc7enc_rdo/rgbcx.cpp",
			"bc7enc_rdo/utils.cpp",
		])
		.files(["bridge/bc7enc_rdo/rdo_bc_encoder.cpp"])
		.cpp(true)
		.std("c++17")
		.warnings(false)
		.extra_warnings(false);

	build.compile("bc7enc_rdo");
}

// Based on https://gitlab.com/kornelski/openmp-rs
fn get_openmp(build: &mut cc::Build) {
	let wants_static = std::env::var_os("OPENMP_STATIC").is_some();
	let comp = build.get_compiler();

	if comp.is_like_clang() && cfg!(target_vendor = "apple") {
		build.flags(["-Xpreprocessor", "-fopenmp"]);
		if wants_static {
			build.flag("-static-openmp");
		}
	} else if comp.is_like_clang_cl() {
		build.flag("/openmp");
	} else if comp.is_like_msvc() {
		build.flag("/openmp");
		println!("cargo:rustc-link-lib=vcomp");
		if wants_static {
			println!("cargo:error=Visual Studio doesn't support static OpenMP. Ship vcomp.dll");
		}
		return;
	} else {
		build.flag("-fopenmp");
		if wants_static {
			build.flag("-static-openmp");
		}
	};

	if wants_static && comp.is_like_gnu() {
		find_and_link(&["gcc_eh"], true);
	}

	let lib_names = if comp.is_like_clang() || comp.is_like_clang_cl() {
		&["omp", "iomp", "gomp"][..]
	} else {
		&["gomp"]
	};
	find_and_link(lib_names, wants_static);
}

fn find_and_link(lib_names: &[&str], statik: bool) {
	let names = lib_names
		.iter()
		.copied()
		.map(|lib_name| {
			if statik {
				(lib_name, format!("lib{}.a", lib_name))
			} else {
				(
					lib_name,
					format!(
						"{}{}{}",
						std::env::consts::DLL_PREFIX,
						lib_name,
						std::env::consts::DLL_SUFFIX
					),
				)
			}
		})
		.collect::<Vec<_>>();

	let mut compiler_libs = Vec::new();
	if let Ok(library_path) = std::env::var("LIBRARY_PATH") {
		for lib_dir in std::env::split_paths(&library_path) {
			compiler_libs.push(std::path::Path::new(&lib_dir).to_path_buf());
		}
	}

	let comp = cc::Build::new()
		.flag("-v")
		.flag("-print-search-dirs")
		.get_compiler();

	if cfg!(target_vendor = "apple") && comp.is_like_clang() {
		if let Ok(output) = std::process::Command::new("brew")
			.arg("--prefix")
			.arg("libomp")
			.output() && output.status.success()
		{
			let prefix_str = std::str::from_utf8(&output.stdout).unwrap_or_default();
			let prefix = std::path::PathBuf::from(prefix_str.trim());
			let lib_dir = prefix.join("lib");
			if lib_dir.exists() {
				compiler_libs.push(lib_dir);
			}
		}
	};

	let mut cmd = comp.to_command();
	let output = cmd.output();
	let output = match &output {
		Ok(output) => std::str::from_utf8(&output.stdout).unwrap_or_default(),
		Err(_) => "",
	};

	for line in output
		.split('\n')
		.filter_map(|l| l.strip_prefix("libraries: ="))
	{
		compiler_libs.extend(std::env::split_paths(line));
	}

	for path in &compiler_libs {
		for (name, file) in &names {
			if path.join(file).exists() {
				println!("cargo:rustc-link-search=native={}", path.display());
				println!(
					"cargo:rustc-link-lib{}={}",
					if statik { "=static" } else { "" },
					name
				);
				return;
			}
		}
	}

	let cc = std::env::var("CC").unwrap_or_else(|_| "cc".to_owned());
	println!(
		"cargo:error=openmp-sys is unable to find library {} for {} in {:?}",
		names[0].1, cc, compiler_libs
	);
}
