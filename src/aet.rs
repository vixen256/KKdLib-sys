use std::collections::*;
use std::ffi::*;
use std::rc::*;
use std::sync::*;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlendMode {
	None = 0,
	Copy,
	Behind,
	Normal,
	Dissolve,
	Add,
	Multiply,
	Screen,
	Overlay,
	SoftLight,
	HardLight,
	Darken,
	Lighten,
	ClassicDifference,
	Hue,
	Saturation,
	Color,
	Luminosity,
	StencilAlpha,
	StencilLuma,
	SilhouetteAlpha,
	SilhouetteLuma,
	LuminescentPremul,
	AlphaAdd,
	ClassicColorDodge,
	ClassicColorBurn,
	Exclusion,
	Difference,
	ColorDodge,
	ColorBurn,
	LinearDodge,
	LinearBurn,
	LinearLight,
	VividLight,
	PinLight,
	HardMix,
	LighterColor,
	DarkerColor,
	Subtract,
	Divide,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayerQuality {
	None = 0,
	Wireframe,
	Draft,
	Best,
}

#[bitfields::bitfield(u16)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct LayerFlags {
	pub video_active: bool,
	pub audio_active: bool,
	pub effects_active: bool,
	pub motion_blur: bool,
	pub frame_blending: bool,
	pub locked: bool,
	pub shy: bool,
	pub collapse: bool,
	pub auto_orient_rotation: bool,
	pub adjustment_layer: bool,
	pub time_remappingg: bool,
	pub is_3d: bool,
	pub look_at_camera: bool,
	pub look_at_point_of_interest: bool,
	pub solo: bool,
	pub markers_locked: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FCurveKey {
	pub frame: f32,
	pub value: f32,
	pub tangent: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FCurve {
	pub keys: Vec<FCurveKey>,
}

impl FCurve {
	pub fn interpolate(&self, frame: f32) -> f32 {
		if self.keys.is_empty() {
			0.0
		} else if self.keys.len() == 1 || frame <= self.keys[0].frame {
			self.keys[0].value
		} else if frame >= self.keys.last().unwrap().frame {
			self.keys.last().unwrap().value
		} else {
			for [cur, next] in self.keys.array_windows() {
				if next.frame >= frame {
					let (f1, p1, t1) = (cur.frame, cur.value, cur.tangent);
					let (f2, p2, t2) = (next.frame, next.value, next.tangent);

					let df = f2 - f1;
					let t = (frame - f1) / df;
					let t_2 = t * t;
					let t_3 = t_2 * t;
					let t_23 = 3.0 * t_2;
					let t_32 = 2.0 * t_3;

					let h00 = t_32 - t_23 + 1.0;
					let h01 = t_23 - t_32;
					let h10 = t_3 - 2.0 * t_2 + t;
					let h11 = t_3 - t_2;

					return (h10 * t1 + h11 * t2) * df + (h01 * p2 + h00 * p1);
				}
			}

			unreachable!()
		}
	}
}

#[derive(Debug, Clone, PartialEq)]
pub struct TransferMode {
	pub mode: BlendMode,
	pub flag: u8,
	pub matte: u8,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LayerVideo3D {
	pub anchor_z: FCurve,
	pub pos_z: FCurve,
	pub dir_x: FCurve,
	pub dir_y: FCurve,
	pub dir_z: FCurve,
	pub rot_x: FCurve,
	pub rot_y: FCurve,
	pub scale_z: FCurve,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LayerVideo {
	pub transfer_mode: TransferMode,
	pub anchor_x: FCurve,
	pub anchor_y: FCurve,
	pub pos_x: FCurve,
	pub pos_y: FCurve,
	pub rot_z: FCurve,
	pub scale_x: FCurve,
	pub scale_y: FCurve,
	pub opacity: FCurve,
	pub _3d: Option<LayerVideo3D>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LayerAudio {
	pub volume_l: FCurve,
	pub volume_r: FCurve,
	pub pan_l: FCurve,
	pub pan_r: FCurve,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Camera {
	pub eye_x: FCurve,
	pub eye_y: FCurve,
	pub eye_z: FCurve,
	pub pos_x: FCurve,
	pub pos_y: FCurve,
	pub pos_z: FCurve,
	pub dir_x: FCurve,
	pub dir_y: FCurve,
	pub dir_z: FCurve,
	pub rot_x: FCurve,
	pub rot_y: FCurve,
	pub rot_z: FCurve,
	pub zoom: FCurve,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VideoSource {
	pub name: String,
	pub id: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Video {
	pub color: [u8; 3],
	pub width: u16,
	pub height: u16,
	pub fpf: f32,
	pub sources: Vec<VideoSource>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Audio {
	pub sound_index: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Item {
	None,
	Video(Video),
	Audio(Audio),
	Composition(Composition),
}

#[derive(Debug, Clone)]
pub struct Layer {
	pub name: String,
	pub start_time: f32,
	pub end_time: f32,
	pub offset_time: f32,
	pub time_scale: f32,
	pub flags: LayerFlags,
	pub quality: LayerQuality,
	pub item: Item,
	pub markers: Vec<(String, f32)>,
	pub video: Option<LayerVideo>,
	pub audio: Option<LayerAudio>,

	// NOTE: this is NOT the layer whos item is a comp that has this as a child, this is some other thing
	// Sega pls
	pub parent: Option<Rc<Mutex<Layer>>>,
}

impl PartialEq for Layer {
	fn eq(&self, other: &Self) -> bool {
		let parent_same = if let Some(a) = &self.parent
			&& let Some(b) = &other.parent
		{
			Rc::ptr_eq(a, b)
		} else if self.parent.is_none() && other.parent.is_none() {
			true
		} else {
			false
		};
		parent_same
			&& self.name == other.name
			&& self.start_time == other.start_time
			&& self.end_time == other.end_time
			&& self.offset_time == other.offset_time
			&& self.time_scale == other.time_scale
			&& self.flags == other.flags
			&& self.quality == other.quality
			&& self.item == other.item
			&& self.markers == other.markers
			&& self.video == other.video
			&& self.audio == other.audio
	}
}

#[derive(Debug, Clone)]
pub struct Composition {
	pub layers: Vec<Rc<Mutex<Layer>>>,
}

impl PartialEq for Composition {
	fn eq(&self, other: &Self) -> bool {
		self.layers
			.iter()
			.zip(other.layers.iter())
			.all(|(a, b)| *a.lock().unwrap() == *b.lock().unwrap())
	}
}

#[derive(Debug, Clone, PartialEq)]
pub struct Scene {
	pub name: String,
	pub start_time: f32,
	pub end_time: f32,
	pub fps: f32,
	pub color: [u8; 3],
	pub width: u32,
	pub height: u32,
	pub camera: Option<Camera>,
	pub root: Composition,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Set {
	pub modern: bool,
	pub big_endian: bool,
	pub is_x: bool,
	pub scenes: Vec<Scene>,
}

#[derive(Debug, Default)]
struct SetCounts {
	audio_count: usize,
	camera_count: usize,
	comp_count: usize,
	fcurve_key_count: usize,
	layer_audio_count: usize,
	layer_count: usize,
	layer_video_3d_count: usize,
	layer_video_count: usize,
	marker_count: usize,
	name_count: usize,
	scene_count: usize,
	video_count: usize,
	video_source_count: usize,
}

fn count_fcurve(fcurve: &FCurve, count: &mut SetCounts) {
	if fcurve.keys.is_empty() || (fcurve.keys.len() == 1 && fcurve.keys[0].value == 0.0) {
	} else if fcurve.keys.len() == 1 {
		count.fcurve_key_count += 1;
	} else {
		count.fcurve_key_count += fcurve.keys.len() * 3;
	}
}

fn count_comp(comp: &Composition, count: &mut SetCounts) {
	count.comp_count += 1;
	for layer in &comp.layers {
		let layer = layer.lock().unwrap();
		count.layer_count += 1;
		count.name_count += 1;
		count.marker_count += layer.markers.len();
		count.name_count += layer.markers.len();
		if let Some(layer_video) = &layer.video {
			count.layer_video_count += 1;
			count_fcurve(&layer_video.anchor_x, count);
			count_fcurve(&layer_video.anchor_y, count);
			count_fcurve(&layer_video.pos_x, count);
			count_fcurve(&layer_video.pos_y, count);
			count_fcurve(&layer_video.rot_z, count);
			count_fcurve(&layer_video.scale_x, count);
			count_fcurve(&layer_video.scale_y, count);
			count_fcurve(&layer_video.opacity, count);

			if let Some(_3d) = &layer_video._3d {
				count.layer_video_3d_count += 1;
				count_fcurve(&_3d.anchor_z, count);
				count_fcurve(&_3d.pos_z, count);
				count_fcurve(&_3d.dir_x, count);
				count_fcurve(&_3d.dir_y, count);
				count_fcurve(&_3d.dir_z, count);
				count_fcurve(&_3d.rot_x, count);
				count_fcurve(&_3d.rot_y, count);
				count_fcurve(&_3d.scale_z, count);
			}
		}

		if let Some(layer_audio) = &layer.audio {
			count.layer_audio_count += 1;
			count_fcurve(&layer_audio.volume_l, count);
			count_fcurve(&layer_audio.volume_r, count);
			count_fcurve(&layer_audio.pan_l, count);
			count_fcurve(&layer_audio.pan_r, count);
		}

		match &layer.item {
			Item::None => {}
			Item::Video(video) => {
				count.video_count += 1;
				count.video_source_count += video.sources.len();
				count.name_count += video.sources.len();
			}
			Item::Audio(_) => count.audio_count += 1,
			Item::Composition(comp) => count_comp(comp, count),
		}
	}
}

struct SetMemory {
	audios: Vec<aet_audio>,
	cameras: Vec<aet_camera>,
	comps: Vec<aet_comp>,
	fcurves: Vec<aet_fcurve>,
	fcurve_keys: Vec<f32>,
	layer_audios: Vec<aet_layer_audio>,
	layers: Vec<aet_layer>,
	layer_ptrs: Vec<(Rc<Mutex<Layer>>, *const aet_layer)>,
	layer_video_3ds: Vec<aet_layer_video_3d>,
	layer_videos: Vec<aet_layer_video>,
	markers: Vec<aet_marker>,
	names: Vec<CString>,
	scenes: Vec<Box<aet_scene>>,
	videos: Vec<aet_video>,
	video_sources: Vec<aet_video_src>,
}

fn alloc_fcurve(in_fcurve: &FCurve, memory: &mut SetMemory) -> aet_fcurve {
	let mut out = aet_fcurve {
		keys_count: in_fcurve.keys.len() as u32,
		keys: std::ptr::null(),
	};

	if in_fcurve.keys.is_empty() || (in_fcurve.keys.len() == 1 && in_fcurve.keys[0].value == 0.0) {
		out.keys_count = 0;
		return out;
	}

	for aet_fcurve in &memory.fcurves {
		if fcurve_eq(in_fcurve, aet_fcurve) {
			out.keys = aet_fcurve.keys;
			return out;
		}
	}

	if in_fcurve.keys.len() == 1 {
		out.keys = memory.fcurve_keys.push_mut(in_fcurve.keys[0].value) as *const _;
		memory.fcurves.push(aet_fcurve {
			keys_count: in_fcurve.keys.len() as u32,
			keys: out.keys,
		});
	} else {
		out.keys = memory.fcurve_keys.push_mut(in_fcurve.keys[0].frame) as *const _;
		for key in in_fcurve.keys.iter().skip(1) {
			memory.fcurve_keys.push(key.frame);
		}
		for key in in_fcurve.keys.iter() {
			memory.fcurve_keys.push(key.value);
			memory.fcurve_keys.push(key.tangent);
		}
		memory.fcurves.push(aet_fcurve {
			keys_count: in_fcurve.keys.len() as u32,
			keys: out.keys,
		});
	}

	out
}

fn fcurve_eq(own_fcurve: &FCurve, aet_fcurve: &aet_fcurve) -> bool {
	if own_fcurve.keys.len() as u32 != aet_fcurve.keys_count {
		return false;
	}

	if own_fcurve.keys.is_empty() {
		true
	} else if own_fcurve.keys.len() == 1 {
		if unsafe { aet_fcurve.keys.read() } == own_fcurve.keys[0].value {
			true
		} else {
			false
		}
	} else {
		for i in 0..own_fcurve.keys.len() {
			if unsafe { aet_fcurve.keys.offset(i as isize).read() } != own_fcurve.keys[i].frame
				|| unsafe {
					aet_fcurve
						.keys
						.offset(own_fcurve.keys.len() as isize + i as isize * 2)
						.read()
				} != own_fcurve.keys[i].value
				|| unsafe {
					aet_fcurve
						.keys
						.offset(own_fcurve.keys.len() as isize + i as isize * 2 + 1)
						.read()
				} != own_fcurve.keys[i].tangent
			{
				return false;
			}
		}

		true
	}
}

fn layer_eq(own_layer: &Layer, aet_layer: &aet_layer) -> bool {
	let name = unsafe { CStr::from_ptr(aet_layer.name) };
	let name = name.to_string_lossy().to_string();
	if name != own_layer.name
		|| aet_layer.start_time != own_layer.start_time
		|| aet_layer.end_time != own_layer.end_time
		|| aet_layer.offset_time != own_layer.offset_time
		|| aet_layer.time_scale != own_layer.time_scale
		|| aet_layer.flags != own_layer.flags.into_bits()
		|| (aet_layer.video.is_null() && own_layer.video.is_some())
		|| (aet_layer.audio.is_null() && own_layer.audio.is_some())
		|| (!aet_layer.video.is_null() && own_layer.video.is_none())
		|| (!aet_layer.audio.is_null() && own_layer.audio.is_none())
	{
		return false;
	}

	if let Some(video) = &own_layer.video {
		let aet_layer_video = unsafe { &*aet_layer.video };
		if video.transfer_mode.mode as u8 != aet_layer_video.transfer_mode.mode
			|| video.transfer_mode.flag != aet_layer_video.transfer_mode.flag
			|| video.transfer_mode.matte != aet_layer_video.transfer_mode.matte
			|| !fcurve_eq(&video.anchor_x, &aet_layer_video.anchor_x)
			|| !fcurve_eq(&video.anchor_y, &aet_layer_video.anchor_y)
			|| !fcurve_eq(&video.pos_x, &aet_layer_video.pos_x)
			|| !fcurve_eq(&video.pos_y, &aet_layer_video.pos_y)
			|| !fcurve_eq(&video.rot_z, &aet_layer_video.rot_z)
			|| !fcurve_eq(&video.scale_x, &aet_layer_video.scale_x)
			|| !fcurve_eq(&video.scale_y, &aet_layer_video.scale_y)
			|| !fcurve_eq(&video.opacity, &aet_layer_video.opacity)
			|| (video._3d.is_some() && aet_layer_video._3d.is_null())
			|| (video._3d.is_none() && !aet_layer_video._3d.is_null())
		{
			return false;
		}

		if let Some(_3d) = &video._3d {
			let aet_layer_3d = unsafe { &*aet_layer_video._3d };
			if !fcurve_eq(&_3d.anchor_z, &aet_layer_3d.anchor_z)
				|| !fcurve_eq(&_3d.pos_z, &aet_layer_3d.pos_z)
				|| !fcurve_eq(&_3d.dir_x, &aet_layer_3d.dir_x)
				|| !fcurve_eq(&_3d.dir_y, &aet_layer_3d.dir_y)
				|| !fcurve_eq(&_3d.dir_z, &aet_layer_3d.dir_z)
				|| !fcurve_eq(&_3d.rot_x, &aet_layer_3d.rot_x)
				|| !fcurve_eq(&_3d.rot_y, &aet_layer_3d.rot_y)
				|| !fcurve_eq(&_3d.scale_z, &aet_layer_3d.scale_z)
			{
				return false;
			}
		}
	}

	if let Some(audio) = &own_layer.audio {
		let aet_layer_audio = unsafe { &*aet_layer.audio };
		if !fcurve_eq(&audio.volume_l, &aet_layer_audio.volume_l)
			|| !fcurve_eq(&audio.volume_r, &aet_layer_audio.volume_r)
			|| !fcurve_eq(&audio.pan_l, &aet_layer_audio.pan_l)
			|| !fcurve_eq(&audio.pan_r, &aet_layer_audio.pan_r)
		{
			return false;
		}
	}

	true
}

fn alloc_item(
	item: &Item,
	out_scene: &mut aet_scene,
	memory: &mut SetMemory,
) -> (u8, *const c_void) {
	match item {
		Item::None => (0, std::ptr::null()),
		Item::Video(video) => {
			'outer: for i in 0..out_scene.video_count as usize {
				if memory.videos[i].color == video.color
					&& memory.videos[i].width == video.width
					&& memory.videos[i].height == video.height
					&& memory.videos[i].fpf == video.fpf
					&& memory.videos[i].sources_count == video.sources.len() as u32
				{
					for j in 0..memory.videos[i].sources_count as usize {
						let source = unsafe { &*memory.videos[i].sources.offset(j as isize) };
						let name = unsafe { CStr::from_ptr(source.sprite_name) };
						let name = name.to_string_lossy().to_string();
						if name != video.sources[j].name
							|| source.sprite_index != video.sources[j].id
						{
							continue 'outer;
						}
					}
					return unsafe {
						(
							1,
							memory.videos.get_unchecked(i) as *const aet_video as *const c_void,
						)
					};
				}
			}

			out_scene.video_count += 1;
			let sources = memory
				.video_sources
				.last_mut()
				.map(|ptr| ptr as *const aet_video_src)
				.map(|ptr| unsafe { ptr.offset(1) })
				.unwrap_or(memory.video_sources.as_ptr());

			for source in &video.sources {
				let name = CString::new(source.name.clone()).unwrap_or_default();
				let name = memory.names.push_mut(name).as_ptr();
				let source = aet_video_src {
					sprite_name: name,
					sprite_index: source.id,
				};
				memory.video_sources.push(source);
			}

			let video = aet_video {
				color: video.color,
				width: video.width,
				height: video.height,
				fpf: video.fpf,
				sources_count: video.sources.len() as u32,
				sources,
			};
			(
				1,
				memory.videos.push_mut(video) as *const aet_video as *const c_void,
			)
		}
		Item::Audio(audio) => {
			for i in 0..out_scene.audio_count as usize {
				if memory.audios[i].sound_index == audio.sound_index {
					return unsafe {
						(
							2,
							memory.audios.get_unchecked(i) as *const aet_audio as *const c_void,
						)
					};
				}
			}

			out_scene.audio_count += 1;
			let audio = aet_audio {
				sound_index: audio.sound_index,
			};
			(
				2,
				memory.audios.push_mut(audio) as *const aet_audio as *const c_void,
			)
		}
		Item::Composition(comp) => {
			'outer: for i in 0..(out_scene.comp_count as usize - 1) {
				if memory.comps[i].layers_count != comp.layers.len() as u32 {
					continue;
				}
				for j in 0..comp.layers.len() {
					let layer = unsafe { &*memory.comps[i].layers.offset(j as isize) };
					if !layer_eq(&comp.layers[j].lock().unwrap(), layer) {
						continue 'outer;
					}
				}

				return unsafe {
					(
						3,
						memory.comps.get_unchecked(i) as *const aet_comp as *const c_void,
					)
				};
			}

			let mut out_comp = aet_comp {
				layers_count: 0,
				layers: std::ptr::null(),
			};
			alloc_comp(comp, &mut out_comp, out_scene, memory);
			out_scene.comp_count += 1;
			(
				3,
				memory.comps.push_mut(out_comp) as *const aet_comp as *const c_void,
			)
		}
	}
}

fn alloc_comp(
	in_comp: &Composition,
	out_comp: &mut aet_comp,
	out_scene: &mut aet_scene,
	memory: &mut SetMemory,
) {
	out_comp.layers_count = in_comp.layers.len() as u32;
	out_comp.layers = memory
		.layers
		.last_mut()
		.map(|ptr| ptr as *const aet_layer)
		.map(|ptr| unsafe { ptr.offset(1) })
		.unwrap_or(memory.layers.as_ptr());

	let mut aet_layers = Vec::new();

	for in_layer in &in_comp.layers {
		let layer = in_layer.lock().unwrap();
		let name = CString::new(layer.name.clone()).unwrap_or_default();
		let name = memory.names.push_mut(name).as_ptr();

		let markers = if layer.markers.is_empty() {
			std::ptr::null()
		} else {
			let markers = memory
				.markers
				.last_mut()
				.map(|ptr| ptr as *const aet_marker)
				.map(|ptr| unsafe { ptr.offset(1) })
				.unwrap_or(memory.markers.as_ptr());

			for (name, time) in &layer.markers {
				let name = CString::new(name.clone()).unwrap_or_default();
				let name = memory.names.push_mut(name).as_ptr();
				memory.markers.push(aet_marker { name, time: *time })
			}

			markers
		};

		let video = if let Some(video) = &layer.video {
			let _3d = if let Some(_3d) = &video._3d {
				let _3d = aet_layer_video_3d {
					anchor_z: alloc_fcurve(&_3d.anchor_z, memory),
					pos_z: alloc_fcurve(&_3d.pos_z, memory),
					dir_x: alloc_fcurve(&_3d.dir_x, memory),
					dir_y: alloc_fcurve(&_3d.dir_y, memory),
					dir_z: alloc_fcurve(&_3d.dir_z, memory),
					rot_x: alloc_fcurve(&_3d.rot_x, memory),
					rot_y: alloc_fcurve(&_3d.rot_y, memory),
					scale_z: alloc_fcurve(&_3d.scale_z, memory),
				};
				memory.layer_video_3ds.push_mut(_3d) as *const aet_layer_video_3d
			} else {
				std::ptr::null()
			};

			let video = aet_layer_video {
				transfer_mode: aet_transfer_mode {
					mode: unsafe { std::mem::transmute(video.transfer_mode.mode) },
					flag: video.transfer_mode.flag,
					matte: video.transfer_mode.matte,
				},
				anchor_x: alloc_fcurve(&video.anchor_x, memory),
				anchor_y: alloc_fcurve(&video.anchor_y, memory),
				pos_x: alloc_fcurve(&video.pos_x, memory),
				pos_y: alloc_fcurve(&video.pos_y, memory),
				rot_z: alloc_fcurve(&video.rot_z, memory),
				scale_x: alloc_fcurve(&video.scale_x, memory),
				scale_y: alloc_fcurve(&video.scale_y, memory),
				opacity: alloc_fcurve(&video.opacity, memory),
				_3d,
			};
			memory.layer_videos.push_mut(video) as *const aet_layer_video
		} else {
			std::ptr::null()
		};

		let audio = if let Some(audio) = &layer.audio {
			let audio = aet_layer_audio {
				volume_l: alloc_fcurve(&audio.volume_l, memory),
				volume_r: alloc_fcurve(&audio.volume_r, memory),
				pan_l: alloc_fcurve(&audio.pan_l, memory),
				pan_r: alloc_fcurve(&audio.pan_r, memory),
			};
			memory.layer_audios.push_mut(audio) as *const aet_layer_audio
		} else {
			std::ptr::null()
		};

		let aet_layer = aet_layer {
			name,
			start_time: layer.start_time,
			end_time: layer.end_time,
			offset_time: layer.offset_time,
			time_scale: layer.time_scale,
			flags: unsafe { std::mem::transmute(layer.flags) },
			quality: unsafe { std::mem::transmute(layer.quality) },
			item_type: 0,
			item: std::ptr::null(),
			parent: layer.parent.as_ref().map_or(std::ptr::null(), |parent| {
				Rc::into_raw(parent.clone()) as *const aet_layer
			}),
			markers_count: layer.markers.len() as u32,
			markers,
			video,
			audio,
		};

		let aet_layer = memory.layers.push_mut(aet_layer) as *mut aet_layer;
		aet_layers.push(aet_layer);
		memory.layer_ptrs.push((in_layer.clone(), aet_layer));
	}

	for (layer, aet_layer) in in_comp.layers.iter().zip(aet_layers.into_iter()) {
		let layer = layer.lock().unwrap();
		let (item_type, item) = alloc_item(&layer.item, out_scene, memory);
		let aet_layer = unsafe { &mut *aet_layer };
		aet_layer.item_type = item_type;
		aet_layer.item = item;
	}
}

impl Set {
	pub fn from_buf(data: &[u8], modern: bool) -> Self {
		let set = unsafe {
			let set = kkdlib_aet_set_new();
			kkdlib_aet_set_unpack_file(set, data.as_ptr() as *const c_void, data.len(), modern);
			&mut *set
		};

		let mut real = Self {
			modern: set.modern,
			big_endian: set.big_endian,
			is_x: set.big_endian,
			scenes: Vec::new(),
		};

		let scenes = std::ptr::slice_from_raw_parts(set.scenes, set.scenes_count as usize);
		let scenes = unsafe { &*scenes };
		for scene in scenes {
			let scene = unsafe { &**scene };

			if scene.comp_count == 0 || scene.comp.is_null() {
				continue;
			}

			let root = unsafe { scene.comp.offset(scene.comp_count as isize - 1).read() };
			let (root, map) = root.decode();
			for (rc, parent) in map.values() {
				let Some(parent) = parent else {
					continue;
				};

				rc.lock().unwrap().parent = map.get(parent).map(|(rc, _)| rc).cloned();
			}

			let name = unsafe { CStr::from_ptr(scene.name) };
			let camera = if !scene.camera.is_null() {
				let camera = unsafe { scene.camera.read() };
				Some(Camera {
					eye_x: camera.eye_x.into(),
					eye_y: camera.eye_y.into(),
					eye_z: camera.eye_z.into(),
					pos_x: camera.pos_x.into(),
					pos_y: camera.pos_y.into(),
					pos_z: camera.pos_z.into(),
					dir_x: camera.dir_x.into(),
					dir_y: camera.dir_y.into(),
					dir_z: camera.dir_z.into(),
					rot_x: camera.rot_x.into(),
					rot_y: camera.rot_y.into(),
					rot_z: camera.rot_z.into(),
					zoom: camera.zoom.into(),
				})
			} else {
				None
			};

			real.scenes.push(Scene {
				name: name.to_string_lossy().to_string(),
				start_time: scene.start_time,
				end_time: scene.end_time,
				fps: scene.fps,
				color: scene.color,
				width: scene.width,
				height: scene.height,
				camera,
				root,
			});
		}

		unsafe { kkdlib_aet_set_delete(set) };

		real
	}

	pub fn to_buf(&self) -> Vec<u8> {
		// Iteration one: get the amount of memory to allocate
		let mut count = SetCounts::default();

		for scene in &self.scenes {
			count.scene_count += 1;
			count.name_count += 1;
			if let Some(camera) = &scene.camera {
				count.camera_count += 1;
				count_fcurve(&camera.eye_x, &mut count);
				count_fcurve(&camera.eye_y, &mut count);
				count_fcurve(&camera.eye_z, &mut count);
				count_fcurve(&camera.pos_x, &mut count);
				count_fcurve(&camera.pos_y, &mut count);
				count_fcurve(&camera.pos_z, &mut count);
				count_fcurve(&camera.dir_x, &mut count);
				count_fcurve(&camera.dir_y, &mut count);
				count_fcurve(&camera.dir_z, &mut count);
				count_fcurve(&camera.rot_x, &mut count);
				count_fcurve(&camera.rot_y, &mut count);
				count_fcurve(&camera.rot_z, &mut count);
				count_fcurve(&camera.zoom, &mut count);
			}

			count_comp(&scene.root, &mut count);
		}

		// Iteration two: allocate memory and fill the set pointers to the correct memory
		let mut memory = SetMemory {
			audios: Vec::with_capacity(count.audio_count),
			cameras: Vec::with_capacity(count.camera_count),
			comps: Vec::with_capacity(count.comp_count),
			fcurves: Vec::new(),
			fcurve_keys: Vec::with_capacity(count.fcurve_key_count),
			layer_audios: Vec::with_capacity(count.layer_audio_count),
			layers: Vec::with_capacity(count.layer_count),
			layer_ptrs: Vec::with_capacity(count.layer_count),
			layer_video_3ds: Vec::with_capacity(count.layer_video_3d_count),
			layer_videos: Vec::with_capacity(count.layer_video_count),
			markers: Vec::with_capacity(count.marker_count),
			names: Vec::with_capacity(count.name_count),
			scenes: Vec::with_capacity(count.scene_count),
			videos: Vec::with_capacity(count.video_count),
			video_sources: Vec::with_capacity(count.video_source_count),
		};
		let set = unsafe {
			let set = kkdlib_aet_set_new();
			&mut *set
		};

		for scene in &self.scenes {
			let name = CString::new(scene.name.clone()).unwrap_or_default();
			let name = memory.names.push_mut(name).as_ptr();

			let camera = if let Some(camera) = &scene.camera {
				let camera = aet_camera {
					eye_x: alloc_fcurve(&camera.eye_x, &mut memory),
					eye_y: alloc_fcurve(&camera.eye_y, &mut memory),
					eye_z: alloc_fcurve(&camera.eye_z, &mut memory),
					pos_x: alloc_fcurve(&camera.pos_x, &mut memory),
					pos_y: alloc_fcurve(&camera.pos_y, &mut memory),
					pos_z: alloc_fcurve(&camera.pos_z, &mut memory),
					dir_x: alloc_fcurve(&camera.dir_x, &mut memory),
					dir_y: alloc_fcurve(&camera.dir_y, &mut memory),
					dir_z: alloc_fcurve(&camera.dir_z, &mut memory),
					rot_x: alloc_fcurve(&camera.rot_x, &mut memory),
					rot_y: alloc_fcurve(&camera.rot_y, &mut memory),
					rot_z: alloc_fcurve(&camera.rot_z, &mut memory),
					zoom: alloc_fcurve(&camera.zoom, &mut memory),
				};
				memory.cameras.push_mut(camera) as *const aet_camera
			} else {
				std::ptr::null()
			};

			let mut aet_scene = aet_scene {
				name,
				start_time: scene.start_time,
				end_time: scene.end_time,
				fps: scene.fps,
				color: scene.color,
				width: scene.width,
				height: scene.height,
				camera,
				comp_count: 1,
				comp: memory
					.comps
					.last_mut()
					.map(|ptr| ptr as *const aet_comp)
					.map(|ptr| unsafe { ptr.offset(1) })
					.unwrap_or(memory.comps.as_ptr()),
				video_count: 0,
				video: memory
					.videos
					.last_mut()
					.map(|ptr| ptr as *const aet_video)
					.map(|ptr| unsafe { ptr.offset(1) })
					.unwrap_or(memory.videos.as_ptr()),
				audio_count: 0,
				audio: memory
					.audios
					.last_mut()
					.map(|ptr| ptr as *const aet_audio)
					.map(|ptr| unsafe { ptr.offset(1) })
					.unwrap_or(memory.audios.as_ptr()),
			};

			let mut root_comp = aet_comp {
				layers_count: 0,
				layers: std::ptr::null(),
			};

			alloc_comp(&scene.root, &mut root_comp, &mut aet_scene, &mut memory);

			if aet_scene.video_count == 0 {
				aet_scene.video = std::ptr::null();
			}
			if aet_scene.audio_count == 0 {
				aet_scene.audio = std::ptr::null();
			}

			memory.comps.push(root_comp);
			memory.scenes.push(Box::new(aet_scene));
		}

		// Iteration three, set all the parent pointers to proper values rather than rcs
		for layer in &mut memory.layers {
			if layer.parent.is_null() {
				continue;
			}

			let parent = unsafe { Rc::from_raw(layer.parent as *const Mutex<Layer>) };
			let real_parent = memory
				.layer_ptrs
				.iter()
				.find(|(rc, _)| Rc::ptr_eq(&parent, rc))
				.map_or(std::ptr::null(), |(_, ptr)| *ptr);
			layer.parent = real_parent;
		}

		set.ready = true;
		set.modern = self.modern;
		set.big_endian = self.big_endian;
		set.is_x = self.is_x;

		let scenes = memory
			.scenes
			.iter()
			.map(|scene| scene.as_ref() as *const aet_scene)
			.collect::<Vec<_>>();
		set.scenes_count = scenes.len() as u32;
		set.scenes = scenes.as_ptr();

		let mut ptr = std::ptr::null_mut();
		let mut size = 0usize;

		unsafe {
			kkdlib_aet_set_pack_file(set, &mut ptr, &mut size);
		}

		let slice = std::ptr::slice_from_raw_parts(ptr as *const u8, size);
		let slice = unsafe { &*slice };

		let mut vec = Vec::with_capacity(size);
		vec.extend_from_slice(slice);
		unsafe {
			kkdlib_aet_set_delete_packed_file(ptr);
			kkdlib_aet_set_delete(set);
		}

		vec
	}
}

#[repr(C)]
struct aet_fcurve {
	keys_count: u32,
	keys: *const f32,
}

impl Into<FCurve> for aet_fcurve {
	fn into(self) -> FCurve {
		if self.keys_count == 0 {
			FCurve { keys: Vec::new() }
		} else if self.keys_count == 1 {
			FCurve {
				keys: vec![FCurveKey {
					frame: 0.0,
					value: unsafe { self.keys.read() },
					tangent: 0.0,
				}],
			}
		} else {
			let frames = std::ptr::slice_from_raw_parts(self.keys, self.keys_count as usize);
			let values = unsafe { self.keys.offset(self.keys_count as isize) } as *const (f32, f32);
			let values = std::ptr::slice_from_raw_parts(values, self.keys_count as usize);
			let (frames, values) = unsafe { (&*frames, &*values) };
			let keys = frames
				.iter()
				.zip(values.iter())
				.map(|(frame, (value, tangent))| FCurveKey {
					frame: *frame,
					value: *value,
					tangent: *tangent,
				})
				.collect();

			FCurve { keys }
		}
	}
}

#[repr(C)]
struct aet_layer_video_3d {
	anchor_z: aet_fcurve,
	pos_z: aet_fcurve,
	dir_x: aet_fcurve,
	dir_y: aet_fcurve,
	dir_z: aet_fcurve,
	rot_x: aet_fcurve,
	rot_y: aet_fcurve,
	scale_z: aet_fcurve,
}

#[repr(C)]
struct aet_transfer_mode {
	mode: u8,
	flag: u8,
	matte: u8,
}

#[repr(C)]
struct aet_layer_video {
	transfer_mode: aet_transfer_mode,
	anchor_x: aet_fcurve,
	anchor_y: aet_fcurve,
	pos_x: aet_fcurve,
	pos_y: aet_fcurve,
	rot_z: aet_fcurve,
	scale_x: aet_fcurve,
	scale_y: aet_fcurve,
	opacity: aet_fcurve,
	_3d: *const aet_layer_video_3d,
}

#[repr(C)]
struct aet_layer_audio {
	volume_l: aet_fcurve,
	volume_r: aet_fcurve,
	pan_l: aet_fcurve,
	pan_r: aet_fcurve,
}

#[repr(C)]
struct aet_camera {
	eye_x: aet_fcurve,
	eye_y: aet_fcurve,
	eye_z: aet_fcurve,
	pos_x: aet_fcurve,
	pos_y: aet_fcurve,
	pos_z: aet_fcurve,
	dir_x: aet_fcurve,
	dir_y: aet_fcurve,
	dir_z: aet_fcurve,
	rot_x: aet_fcurve,
	rot_y: aet_fcurve,
	rot_z: aet_fcurve,
	zoom: aet_fcurve,
}

#[repr(C)]
struct aet_video_src {
	sprite_name: *const c_char,
	sprite_index: u32,
}

#[repr(C)]
struct aet_video {
	color: [u8; 3],
	width: u16,
	height: u16,
	fpf: f32,
	sources_count: u32,
	sources: *const aet_video_src,
}

#[repr(C)]
struct aet_audio {
	sound_index: u32,
}

#[repr(C)]
struct aet_marker {
	time: f32,
	name: *const c_char,
}

#[repr(C)]
struct aet_layer {
	name: *const c_char,
	start_time: f32,
	end_time: f32,
	offset_time: f32,
	time_scale: f32,
	flags: u16,
	quality: u8,
	item_type: u8,
	item: *const c_void,
	parent: *const aet_layer,
	markers_count: u32,
	markers: *const aet_marker,
	video: *const aet_layer_video,
	audio: *const aet_layer_audio,
}

#[repr(C)]
struct aet_comp {
	layers_count: u32,
	layers: *const aet_layer,
}

impl aet_comp {
	fn decode(
		self,
	) -> (
		Composition,
		HashMap<usize, (Rc<Mutex<Layer>>, Option<usize>)>,
	) {
		let layers = std::ptr::slice_from_raw_parts(self.layers, self.layers_count as usize);
		let layers = unsafe { &*layers };

		let mut real = Composition {
			layers: Vec::with_capacity(self.layers_count as usize),
		};
		let mut map = HashMap::new();
		for (i, layer) in layers.iter().enumerate() {
			let name = unsafe { CStr::from_ptr(layer.name) };

			let markers =
				std::ptr::slice_from_raw_parts(layer.markers, layer.markers_count as usize);
			let markers = unsafe { &*markers };
			let markers = markers
				.iter()
				.map(|marker| {
					let name = unsafe { CStr::from_ptr(marker.name) };
					(name.to_string_lossy().to_string(), marker.time)
				})
				.collect::<Vec<_>>();

			let video = if !layer.video.is_null() {
				let video = unsafe { layer.video.read() };
				let _3d = if !video._3d.is_null() {
					let _3d = unsafe { video._3d.read() };
					Some(LayerVideo3D {
						anchor_z: _3d.anchor_z.into(),
						pos_z: _3d.pos_z.into(),
						dir_x: _3d.dir_x.into(),
						dir_y: _3d.dir_y.into(),
						dir_z: _3d.dir_z.into(),
						rot_x: _3d.rot_x.into(),
						rot_y: _3d.rot_y.into(),
						scale_z: _3d.scale_z.into(),
					})
				} else {
					None
				};

				Some(LayerVideo {
					transfer_mode: TransferMode {
						mode: unsafe { std::mem::transmute(video.transfer_mode.mode) },
						flag: video.transfer_mode.flag,
						matte: video.transfer_mode.matte,
					},
					anchor_x: video.anchor_x.into(),
					anchor_y: video.anchor_y.into(),
					pos_x: video.pos_x.into(),
					pos_y: video.pos_y.into(),
					rot_z: video.rot_z.into(),
					scale_x: video.scale_x.into(),
					scale_y: video.scale_y.into(),
					opacity: video.opacity.into(),
					_3d,
				})
			} else {
				None
			};

			let audio = if !layer.audio.is_null() {
				let audio = unsafe { layer.audio.read() };
				Some(LayerAudio {
					volume_l: audio.volume_l.into(),
					volume_r: audio.volume_r.into(),
					pan_l: audio.pan_l.into(),
					pan_r: audio.pan_r.into(),
				})
			} else {
				None
			};

			let item = match layer.item_type {
				0 => Item::None,
				1 => {
					let ptr = layer.item as *const aet_video;
					if ptr.is_null() {
						Item::None
					} else {
						let video = unsafe { ptr.read() };
						let sources = std::ptr::slice_from_raw_parts(
							video.sources,
							video.sources_count as usize,
						);
						let sources = unsafe { &*sources };
						let sources = sources
							.iter()
							.map(|source| {
								let name = unsafe { CStr::from_ptr(source.sprite_name) };
								VideoSource {
									name: name.to_string_lossy().to_string(),
									id: source.sprite_index,
								}
							})
							.collect();

						Item::Video(Video {
							color: video.color,
							width: video.width,
							height: video.height,
							fpf: video.fpf,
							sources,
						})
					}
				}
				2 => {
					let ptr = layer.item as *const aet_audio;
					if ptr.is_null() {
						Item::None
					} else {
						let audio = unsafe { ptr.read() };

						Item::Audio(Audio {
							sound_index: audio.sound_index,
						})
					}
				}
				3 => {
					let ptr = layer.item as *const aet_comp;
					if ptr.is_null() {
						Item::None
					} else {
						let comp = unsafe { ptr.read() };
						let (comp, inner_map) = comp.decode();
						map.extend(inner_map);
						Item::Composition(comp)
					}
				}
				_ => unreachable!(),
			};

			let rc = Rc::new(Mutex::new(Layer {
				name: name.to_string_lossy().to_string(),
				start_time: layer.start_time,
				end_time: layer.end_time,
				offset_time: layer.offset_time,
				time_scale: layer.time_scale,
				flags: unsafe { std::mem::transmute(layer.flags) },
				quality: unsafe { std::mem::transmute(layer.quality) },
				item,
				markers,
				video,
				audio,
				parent: None,
			}));

			map.insert(
				unsafe { self.layers.add(i) } as usize,
				(
					rc.clone(),
					if layer.parent.is_null() {
						None
					} else {
						Some(layer.parent as usize)
					},
				),
			);
			real.layers.push(rc);
		}

		(real, map)
	}
}

#[repr(C)]
struct aet_scene {
	name: *const c_char,
	start_time: f32,
	end_time: f32,
	fps: f32,
	color: [u8; 3],
	width: u32,
	height: u32,
	camera: *const aet_camera,
	comp_count: u32,
	comp: *const aet_comp,
	video_count: u32,
	video: *const aet_video,
	audio_count: u32,
	audio: *const aet_audio,
}

#[repr(C)]
struct aet_set {
	ready: bool,
	modern: bool,
	big_endian: bool,
	is_x: bool,
	scenes_count: u32,
	scenes: *const *const aet_scene,
}

unsafe extern "C" {
	fn kkdlib_aet_set_new() -> *mut aet_set;
	fn kkdlib_aet_set_pack_file(set: *mut aet_set, data: *mut *mut c_void, size: *mut usize);
	fn kkdlib_aet_set_delete_packed_file(data: *mut c_void);
	fn kkdlib_aet_set_unpack_file(
		set: *mut aet_set,
		data: *const c_void,
		size: usize,
		modern: bool,
	);
	fn kkdlib_aet_set_delete(set: *mut aet_set);
}
