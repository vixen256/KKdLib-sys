#include "spr.hpp"

extern "C" {
spr::SprInfo *
kkdlib_spr_info_new () {
	return new spr::SprInfo ();
}

uint32_t
kkdlib_spr_info_get_texid (spr::SprInfo *info) {
	return info->texid;
}

void
kkdlib_spr_info_set_texid (spr::SprInfo *info, uint32_t texid) {
	info->texid = texid;
}

int32_t
kkdlib_spr_info_get_rotate (spr::SprInfo *info) {
	return info->rotate;
}

void
kkdlib_spr_info_set_rotate (spr::SprInfo *info, int32_t rotate) {
	info->rotate = rotate;
}

float_t
kkdlib_spr_info_get_px (spr::SprInfo *info) {
	return info->px;
}

void
kkdlib_spr_info_set_px (spr::SprInfo *info, float_t px) {
	info->px = px;
}

float_t
kkdlib_spr_info_get_py (spr::SprInfo *info) {
	return info->py;
}

void
kkdlib_spr_info_set_py (spr::SprInfo *info, float_t py) {
	info->py = py;
}

float_t
kkdlib_spr_info_get_width (spr::SprInfo *info) {
	return info->width;
}

void
kkdlib_spr_info_set_width (spr::SprInfo *info, float_t width) {
	info->width = width;
}

float_t
kkdlib_spr_info_get_height (spr::SprInfo *info) {
	return info->height;
}

void
kkdlib_spr_info_set_height (spr::SprInfo *info, float_t height) {
	info->height = height;
}

void
kkdlib_spr_info_delete (spr::SprInfo *info) {
	delete info;
}

SpriteData *
kkdlib_sprite_data_new () {
	return new SpriteData ();
}

uint32_t
kkdlib_sprite_data_get_attr (SpriteData *data) {
	return data->attr;
}
void
kkdlib_sprite_data_set_attr (SpriteData *data, uint32_t attr) {
	data->attr = attr;
}

resolution_mode
kkdlib_sprite_data_get_resolution_mode (SpriteData *data) {
	return data->resolution_mode;
}

void
kkdlib_sprite_data_set_resolution_mode (SpriteData *data, enum resolution_mode resolution_mode) {
	data->resolution_mode = resolution_mode;
}

void
kkdlib_sprite_data_delete (SpriteData *data) {
	delete data;
}

SprSet *
kkdlib_spr_set_new () {
	SprSet *set = new SprSet ();
	set->alloc  = prj::shared_ptr<prj::stack_allocator> (new prj::stack_allocator);
	return set;
}

bool
kkdlib_spr_set_get_ready (SprSet *set) {
	return set->set.ready;
}

void
kkdlib_spr_set_set_ready (SprSet *set, bool ready) {
	set->set.ready = ready;
}

bool
kkdlib_spr_set_get_modern (SprSet *set) {
	return set->set.modern;
}

void
kkdlib_spr_set_set_modern (SprSet *set, bool modern) {
	set->set.modern = modern;
}

bool
kkdlib_spr_set_get_big_endian (SprSet *set) {
	return set->set.big_endian;
}

void
kkdlib_spr_set_set_big_endian (SprSet *set, bool big_endian) {
	set->set.big_endian = big_endian;
}

bool
kkdlib_spr_set_get_is_x (SprSet *set) {
	return set->set.is_x;
}

void
kkdlib_spr_set_set_is_x (SprSet *set, bool is_x) {
	set->set.is_x = is_x;
}

uint32_t
kkdlib_spr_set_get_flag (SprSet *set) {
	return set->set.flag;
}

void
kkdlib_spr_set_set_flag (SprSet *set, uint32_t flag) {
	set->set.flag = flag;
}

int32_t
kkdlib_spr_set_get_num_of_texture (SprSet *set) {
	return set->set.num_of_texture;
}

int32_t
kkdlib_spr_set_get_num_of_sprite (SprSet *set) {
	return set->set.num_of_sprite;
}

spr::SprInfo *
kkdlib_spr_set_get_sprinfo (SprSet *set, int32_t index) {
	return set->set.sprinfo + index;
}

const char *
kkdlib_spr_set_get_sprname (SprSet *set, int32_t index) {
	return set->set.sprname[index];
}

SpriteData *
kkdlib_spr_set_get_sprdata (SprSet *set, int32_t index) {
	return set->set.sprdata + index;
}

void
kkdlib_spr_set_add_spr (SprSet *set, spr::SprInfo *sprinfo, const char *sprname, SpriteData *sprdata) {
	spr::SprInfo *new_sprinfo = set->alloc->allocate<spr::SprInfo> (set->set.num_of_sprite + 1);
	const char **new_sprname  = set->alloc->allocate<const char *> (set->set.num_of_sprite + 1);
	SpriteData *new_sprdata   = set->alloc->allocate<SpriteData> (set->set.num_of_sprite + 1);

	memcpy (new_sprinfo, set->set.sprinfo, sizeof (spr::SprInfo) * set->set.num_of_sprite);
	memcpy (new_sprname, set->set.sprname, sizeof (const char *) * set->set.num_of_sprite);
	memcpy (new_sprdata, set->set.sprdata, sizeof (SpriteData) * set->set.num_of_sprite);
	new_sprinfo[set->set.num_of_sprite] = *sprinfo;
	new_sprname[set->set.num_of_sprite] = set->alloc->allocate<char> (strlen (sprname) + 1);
	memcpy ((void *)new_sprname[set->set.num_of_sprite], sprname, strlen (sprname) + 1);
	new_sprdata[set->set.num_of_sprite] = *sprdata;

	set->set.num_of_sprite++;
	set->set.sprinfo = new_sprinfo;
	set->set.sprname = new_sprname;
	set->set.sprdata = new_sprdata;
}

const char *
kkdlib_spr_set_get_texname (SprSet *set, int32_t index) {
	return set->set.texname[index];
}

txp_set *
kkdlib_spr_set_get_txp (SprSet *set) {
	return set->set.txp;
}

void
kkdlib_spr_set_set_txp (SprSet *set, txp_set *txp, const char **texname) {
	if (set->set.txp) delete txp;
	set->set.txp            = new txp_set ();
	*set->set.txp           = *txp;
	set->set.num_of_texture = txp->textures.size ();

	set->set.texname = set->alloc->allocate<const char *> (txp->textures.size ());
	for (size_t i = 0; i < txp->textures.size (); i++) {
		set->set.texname[i] = set->alloc->allocate<char> (strlen (texname[i]) + 1);
		memcpy ((void *)set->set.texname[i], texname[i], strlen (texname[i]) + 1);
	}
}

void
kkdlib_spr_set_pack_file (SprSet *set, void **data, size_t *size) {
	if (set->set.txp == nullptr || set->set.num_of_sprite == 0) return;
	for (int32_t i = 0; i < set->set.num_of_sprite; i++) {
		auto texture           = set->set.txp->textures.at (set->set.sprinfo[i].texid);
		set->set.sprinfo[i].su = set->set.sprinfo[i].px / texture.mipmaps[0].width;
		set->set.sprinfo[i].sv = set->set.sprinfo[i].py / texture.mipmaps[0].height;
		set->set.sprinfo[i].eu = (set->set.sprinfo[i].px + set->set.sprinfo[i].width) / texture.mipmaps[0].width;
		set->set.sprinfo[i].ev = (set->set.sprinfo[i].py + set->set.sprinfo[i].height) / texture.mipmaps[0].height;
	}

	set->set.ready = true;
	set->set.pack_file (data, size);
}

void
kkdlib_spr_set_delete_packed_file (void *data) {
	if (data != nullptr) free_def (data);
}

void
kkdlib_spr_set_unpack_file (SprSet *set, const void *data, size_t size, bool modern) {
	set->set.unpack_file (set->alloc, data, size, modern);
}

void
kkdlib_spr_set_delete (SprSet *set) {
	delete set;
}
}
