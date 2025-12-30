#pragma once
#include <txp.hpp>

extern "C" {
txp_mipmap *kkdlib_txp_mipmap_new ();
int32_t kkdlib_txp_mipmap_get_width (txp_mipmap *mipmap);
void kkdlib_txp_mipmap_set_width (txp_mipmap *mipmap, int32_t width);
int32_t kkdlib_txp_mipmap_get_height (txp_mipmap *mipmap);
void kkdlib_txp_mipmap_set_height (txp_mipmap *mipmap, int32_t height);
txp_format kkdlib_txp_mipmap_get_format (txp_mipmap *mipmap);
void kkdlib_txp_mipmap_set_format (txp_mipmap *mipmap, txp_format format);
int32_t kkdlib_txp_mipmap_get_size (txp_mipmap *mipmap);
const void *kkdlib_txp_mipmap_get_data (txp_mipmap *mipmap);
void kkdlib_txp_mipmap_set_data (txp_mipmap *mipmap, const void *data);
void kkdlib_txp_mipmap_delete (txp_mipmap *mipmap);

txp *kkdlib_txp_new ();
bool kkdlib_txp_get_has_cube_map (txp *txp);
void kkdlib_txp_set_has_cube_map (txp *txp, bool has_cube_map);
int32_t kkdlib_txp_get_array_size (txp *txp);
void kkdlib_txp_set_array_size (txp *txp, int32_t array_size);
int32_t kkdlib_txp_get_mipmaps_count (txp *txp);
void kkdlib_txp_set_mipmaps_count (txp *txp, int32_t mipmaps_count);
txp_mipmap *kkdlib_txp_get_mipmap (txp *txp, int32_t array_index, int32_t mipmap_index);
void kkdlib_txp_add_mipmap (txp *txp, txp_mipmap *mipmap);
void kkdlib_txp_delete (txp *txp);

txp_set *kkdlib_txp_set_new ();
size_t kkdlib_txp_set_get_textures_size (txp_set *set);
txp *kkdlib_txp_set_get_texture_by_index (txp_set *set, size_t index);
void kkdlib_txp_set_add_texture (txp_set *set, txp *txp);
bool kkdlib_txp_set_pack_file (txp_set *set, void **data, size_t *size, bool big_endian);
bool kkdlib_txp_set_pack_file_modern (txp_set *set, void **data, size_t *size, bool big_endian, uint32_t signature);
void kkdlib_txp_set_delete_packed_file (void *data);
bool kkdlib_txp_set_unpack_file (txp_set *set, const void *data, bool big_endian);
bool kkdlib_txp_set_unpack_file_modern (txp_set *set, const void *data, size_t size, uint32_t signature);
void kkdlib_txp_set_delete (txp_set *set);
}
