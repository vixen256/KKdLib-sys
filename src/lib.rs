#![feature(cfg_eval, push_mut, array_windows)]

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
		let data = std::fs::read("/games/SteamLibrary/steamapps/common/Hatsune Miku Project DIVA Mega Mix Plus/mods-testing/Template Mod/rom_steam/rom/2d/aet_gam_cmn.bak.bin").unwrap();
		let file = aet::Set::from_buf(&data, false);

		fn print_audios(comp: &aet::Composition) {
			for layer in &comp.layers {
				let layer = layer.try_lock().unwrap();
				match &layer.item {
					aet::Item::None => {}
					aet::Item::Audio(audio) => {
						dbg!(audio);
					}
					aet::Item::Video(_) => {}
					aet::Item::Composition(comp) => print_audios(comp),
				}
			}
		}
		print_audios(&file.scenes[0].root);
		/*
		let data = file.to_buf();
		dbg!(data.len());
		std::fs::write(
			"/games/SteamLibrary/steamapps/common/Hatsune Miku Project DIVA Mega Mix Plus/mods/PS4 FT UI/rom_steam_en/rom/2d/aet_gam_cmn_new2.bin",
			&data,
		);
		*/
	}
}
