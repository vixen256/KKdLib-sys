#pragma once
#include <spr.hpp>

struct SprSet {
	spr_set set;
	prj::shared_ptr<prj::stack_allocator> alloc;

	SprSet () : set (), alloc () {}
};

extern "C" {
spr::SprInfo *kkdlib_spr_info_new ();
uint32_t kkdlib_spr_info_get_texid (spr::SprInfo *info);
void kkdlib_spr_info_set_texid (spr::SprInfo *info, uint32_t texid);
int32_t kkdlib_spr_info_get_rotate (spr::SprInfo *info);
void kkdlib_spr_info_set_rotate (spr::SprInfo *info, int32_t rotate);
float_t kkdlib_spr_info_get_px (spr::SprInfo *info);
void kkdlib_spr_info_set_px (spr::SprInfo *info, float_t px);
float_t kkdlib_spr_info_get_py (spr::SprInfo *info);
void kkdlib_spr_info_set_py (spr::SprInfo *info, float_t py);
float_t kkdlib_spr_info_get_width (spr::SprInfo *info);
void kkdlib_spr_info_set_width (spr::SprInfo *info, float_t width);
float_t kkdlib_spr_info_get_height (spr::SprInfo *info);
void kkdlib_spr_info_set_height (spr::SprInfo *info, float_t height);
void kkdlib_spr_info_delete (spr::SprInfo *info);

SpriteData *kkdlib_sprite_data_new ();
uint32_t kkdlib_sprite_data_get_attr (SpriteData *data);
void kkdlib_sprite_data_set_attr (SpriteData *data, uint32_t attr);
resolution_mode kkdlib_sprite_data_get_resolution_mode (SpriteData *data);
void kkdlib_sprite_data_set_resolution_mode (SpriteData *data, enum resolution_mode resolution_mode);
void kkdlib_sprite_data_delete (SpriteData *data);

SprSet *kkdlib_spr_set_new ();
bool kkdlib_spr_set_get_ready (SprSet *set);
void kkdlib_spr_set_set_ready (SprSet *set, bool ready);
bool kkdlib_spr_set_get_modern (SprSet *set);
void kkdlib_spr_set_set_modern (SprSet *set, bool modern);
bool kkdlib_spr_set_get_big_endian (SprSet *set);
void kkdlib_spr_set_set_big_endian (SprSet *set, bool big_endian);
bool kkdlib_spr_set_get_is_x (SprSet *set);
void kkdlib_spr_set_set_is_x (SprSet *set, bool is_x);
uint32_t kkdlib_spr_set_get_flag (SprSet *set);
void kkdlib_spr_set_set_flag (SprSet *set, uint32_t flag);
int32_t kkdlib_spr_set_get_num_of_texture (SprSet *set);
int32_t kkdlib_spr_set_get_num_of_sprite (SprSet *set);
spr::SprInfo *kkdlib_spr_set_get_sprinfo (SprSet *set, int32_t index);
const char *kkdlib_spr_set_get_sprname (SprSet *set, int32_t index);
SpriteData *kkdlib_spr_set_get_sprdata (SprSet *set, int32_t index);
void kkdlib_spr_set_add_spr (SprSet *set, spr::SprInfo *sprinfo, const char *sprname, SpriteData *sprdata);
const char *kkdlib_spr_set_get_texname (SprSet *set, int32_t index);
txp_set *kkdlib_spr_set_get_txp (SprSet *set);
void kkdlib_spr_set_set_txp (SprSet *set, txp_set *txp, const char **texname);
void kkdlib_spr_set_pack_file (SprSet *set, void **data, size_t *size);
void kkdlib_spr_set_delete_packed_file (void *data);
void kkdlib_spr_set_unpack_file (SprSet *set, const void *data, size_t size, bool modern);
void kkdlib_spr_set_delete (SprSet *set);
}
