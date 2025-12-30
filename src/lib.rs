#![feature(cfg_eval, push_mut)]

// Without mentioning the crate it will not get linked
extern crate libdeflate_sys;

pub mod database;

pub mod aet;
pub mod farc;
pub mod hash;
pub mod spr;
pub mod txp;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

#[cfg(feature = "pyo3")]
#[cfg_attr(feature = "pyo3", pymodule)]
mod kkdlib {
	#[pymodule_export]
	use super::farc::farc_module;
	#[pymodule_export]
	use super::spr::spr_module;
	#[pymodule_export]
	use super::txp::txp_module;
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn farc() {
		let farc = farc::Farc::open(
			"/games/SteamLibrary/steamapps/common/Hatsune Miku Project DIVA Mega Mix Plus/mods-loading/(1-6 Gravity) Symphony Version/rom/mod_chritm_prop.farc",
		);
		for file in farc.files() {
			let data = file.data().unwrap();
			println!("{}", file.name());
			for c in data {
				print!("{}", char::from_u32(*c as u32).unwrap());
			}
		}
	}

	#[test]
	fn txp() {
		let mut set = txp::Set::new();

		let mut pixels = Vec::with_capacity(512 * 512 * 4);
		pixels.resize(512 * 512 * 4, 0);
		for y in 0..512 {
			for x in 0..512 {
				pixels[(y * 512 + x) * 4 + 0] = (x as f32 / 512.0 * 256.0) as u8;
				pixels[(y * 512 + x) * 4 + 1] = (y as f32 / 512.0 * 256.0) as u8;
				pixels[(y * 512 + x) * 4 + 2] =
					(x as f32 / 512.0 * 126.0) as u8 + (y as f32 / 512.0 * 128.0) as u8;
				pixels[(y * 512 + x) * 4 + 3] = 255;
			}
		}

		let mip = txp::Mipmap::from_rgba(512, 512, &pixels, txp::Format::BC7).unwrap();
		let mut tex = txp::Texture::new();
		tex.set_has_cube_map(false);
		tex.set_array_size(1);
		tex.set_mipmaps_count(1);
		tex.add_mipmap(&mip);
		set.add_file(&tex);

		let mip = txp::Mipmap::from_rgba(512, 512, &pixels, txp::Format::BC6H).unwrap();
		let mut tex = txp::Texture::new();
		tex.set_has_cube_map(false);
		tex.set_array_size(1);
		tex.set_mipmaps_count(1);
		tex.add_mipmap(&mip);
		set.add_file(&tex);

		let mip = txp::Mipmap::from_rgba(512, 512, &pixels, txp::Format::L8).unwrap();
		let mut tex = txp::Texture::new();
		tex.set_has_cube_map(false);
		tex.set_array_size(1);
		tex.set_mipmaps_count(1);
		tex.add_mipmap(&mip);
		set.add_file(&tex);

		let tex = txp::Texture::encode_ycbcr(512, 512, &pixels).unwrap();
		set.add_file(&tex);

		let data = set.to_buf(false, None).unwrap();
		std::fs::write("test.txd", data).unwrap();
	}

	#[test]
	fn spr() {
		let mut set = txp::Set::new();

		let mut pixels = Vec::with_capacity(512 * 512 * 4);
		pixels.resize(512 * 512 * 4, 0);
		for y in 0..512 {
			for x in 0..512 {
				pixels[(y * 512 + x) * 4 + 0] = (x as f32 / 512.0 * 256.0) as u8;
				pixels[(y * 512 + x) * 4 + 1] = (y as f32 / 512.0 * 256.0) as u8;
				pixels[(y * 512 + x) * 4 + 2] =
					(x as f32 / 512.0 * 126.0) as u8 + (y as f32 / 512.0 * 128.0) as u8;
				pixels[(y * 512 + x) * 4 + 3] = 255;
			}
		}

		let mip = txp::Mipmap::from_rgba(512, 512, &pixels, txp::Format::BC7).unwrap();
		let mut tex = txp::Texture::new();
		tex.set_has_cube_map(false);
		tex.set_array_size(1);
		tex.set_mipmaps_count(1);
		tex.add_mipmap(&mip);
		set.add_file(&tex);

		let mip = txp::Mipmap::from_rgba(512, 512, &pixels, txp::Format::BC6H).unwrap();
		let mut tex = txp::Texture::new();
		tex.set_has_cube_map(false);
		tex.set_array_size(1);
		tex.set_mipmaps_count(1);
		tex.add_mipmap(&mip);
		set.add_file(&tex);

		let mip = txp::Mipmap::from_rgba(512, 512, &pixels, txp::Format::L8).unwrap();
		let mut tex = txp::Texture::new();
		tex.set_has_cube_map(false);
		tex.set_array_size(1);
		tex.set_mipmaps_count(1);
		tex.add_mipmap(&mip);
		set.add_file(&tex);

		let tex = txp::Texture::encode_ycbcr(512, 512, &pixels).unwrap();
		set.add_file(&tex);

		let mut spr_set = spr::Set::new();
		spr_set.set_txp(
			&set,
			vec![
				String::from("BC7"),
				String::from("BC6H"),
				String::from("L8"),
				String::from("YCbCr"),
			],
		);

		let mut spr = spr::Info::new();
		spr.set_texid(0);
		spr.set_px(0.0);
		spr.set_py(0.0);
		spr.set_width(512.0);
		spr.set_height(512.0);
		spr.set_resolution_mode(spr::ResolutionMode::FHD);
		spr_set.add_spr(&spr, "bc7");

		spr.set_texid(1);
		spr_set.add_spr(&spr, "bc6h");

		spr.set_texid(2);
		spr_set.add_spr(&spr, "l8");

		spr.set_texid(3);
		spr_set.add_spr(&spr, "ycbcr");

		let data = spr_set.to_buf().unwrap();
		let mut farc = farc::Farc::new();
		farc.add_file_data("spr_test.bin", &data);
		farc.write("test.farc", false, false);
	}

	#[test]
	fn spr_db() {
		let data = std::fs::read("/home/vixen/Desktop/mod_spr_db.bin").unwrap();
		let file = database::sprite::file::Database::from_buf(&data, false);
		for set in file.sets() {
			dbg!(set.name(), set.file_name(), set.id());
			for sprite in set.sprites() {
				dbg!(sprite.name(), sprite.id());
			}
		}

		let mut database = database::sprite::Database::new();
		database.add_file(&file);
		dbg!(database.get_spr_by_id(2966091108).is_some());
		dbg!(database.get_spr_by_id(3655173785).is_some());
		dbg!(database.get_spr_by_id(1).is_some());
		dbg!(database.get_spr_set_by_id(1).is_some());
		dbg!(database.get_spr_set_by_id(54691697).is_some());
	}

	#[test]
	fn aet() {
		let data = std::fs::read("/games/SteamLibrary/steamapps/common/Hatsune Miku Project DIVA Mega Mix Plus/mods/PS4 FT UI/rom_steam_en/rom/2d/aet_nswgam_option.bin.bak").unwrap();
		let file = aet::Set::from_buf(&data);
		for layer in &file.scenes[0].root.layers {
			if layer.name != "option_top_menu_display" {
				continue;
			}
			dbg!(layer);
		}
		let data = file.to_buf();
		dbg!(data.len());
		std::fs::write(
			"/games/SteamLibrary/steamapps/common/Hatsune Miku Project DIVA Mega Mix Plus/mods/PS4 FT UI/rom_steam_en/rom/2d/aet_nswgam_option.bin",
			&data,
		);
	}
}
