#include "txp.hpp"

extern "C" {
txp_mipmap *
kkdlib_txp_mipmap_new () {
	return new txp_mipmap ();
}

int32_t
kkdlib_txp_mipmap_get_width (txp_mipmap *mipmap) {
	return mipmap->width;
}

void
kkdlib_txp_mipmap_set_width (txp_mipmap *mipmap, int32_t width) {
	mipmap->width = width;
}

int32_t
kkdlib_txp_mipmap_get_height (txp_mipmap *mipmap) {
	return mipmap->height;
}

void
kkdlib_txp_mipmap_set_height (txp_mipmap *mipmap, int32_t height) {
	mipmap->height = height;
}

txp_format
kkdlib_txp_mipmap_get_format (txp_mipmap *mipmap) {
	return mipmap->format;
}

void
kkdlib_txp_mipmap_set_format (txp_mipmap *mipmap, txp_format format) {
	mipmap->format = format;
}

int32_t
kkdlib_txp_mipmap_get_size (txp_mipmap *mipmap) {
	return mipmap->get_size ();
}

const void *
kkdlib_txp_mipmap_get_data (txp_mipmap *mipmap) {
	if (mipmap->data.size () != mipmap->get_size ()) return nullptr;
	return mipmap->data.data ();
}

void
kkdlib_txp_mipmap_set_data (txp_mipmap *mipmap, const void *data) {
	mipmap->size = mipmap->get_size ();
	mipmap->data.resize (mipmap->size);
	memcpy (mipmap->data.data (), data, mipmap->size);
}

void
kkdlib_txp_mipmap_delete (txp_mipmap *mipmap) {
	delete mipmap;
}

txp *
kkdlib_txp_new () {
	return new txp ();
}

bool
kkdlib_txp_get_has_cube_map (txp *txp) {
	return txp->has_cube_map;
}

void
kkdlib_txp_set_has_cube_map (txp *txp, bool has_cube_map) {
	txp->has_cube_map = has_cube_map;
}

int32_t
kkdlib_txp_get_array_size (txp *txp) {
	return txp->array_size;
}

void
kkdlib_txp_set_array_size (txp *txp, int32_t array_size) {
	txp->array_size = array_size;
}

int32_t
kkdlib_txp_get_mipmaps_count (txp *txp) {
	return txp->mipmaps_count;
}

void
kkdlib_txp_set_mipmaps_count (txp *txp, int32_t mipmaps_count) {
	txp->mipmaps_count = mipmaps_count;
}

txp_mipmap *
kkdlib_txp_get_mipmap (txp *txp, int32_t array_index, int32_t mipmap_index) {
	if (array_index >= txp->array_size || mipmap_index >= txp->mipmaps_count) return nullptr;
	return txp->mipmaps.data () + array_index * txp->mipmaps_count + mipmap_index;
}

void
kkdlib_txp_add_mipmap (txp *txp, txp_mipmap *mipmap) {
	txp->mipmaps.push_back (*mipmap);
}

void
kkdlib_txp_delete (txp *txp) {
	delete txp;
}

txp_set *
kkdlib_txp_set_new () {
	return new txp_set ();
}

size_t
kkdlib_txp_set_get_textures_size (txp_set *set) {
	return set->textures.size ();
}

txp *
kkdlib_txp_set_get_texture_by_index (txp_set *set, size_t index) {
	return set->textures.data () + index;
}

void
kkdlib_txp_set_add_texture (txp_set *set, txp *txp) {
	set->textures.push_back (*txp);
}

bool
kkdlib_txp_set_pack_file (txp_set *set, void **data, size_t *size, bool big_endian) {
	return set->pack_file (data, size, big_endian);
}

bool
kkdlib_txp_set_pack_file_modern (txp_set *set, void **data, size_t *size, bool big_endian, uint32_t signature) {
	return set->pack_file_modern (data, size, big_endian, signature);
}

void
kkdlib_txp_set_delete_packed_file (void *data) {
	if (data != nullptr) free_def (data);
}

bool
kkdlib_txp_set_unpack_file (txp_set *set, const void *data, bool big_endian) {
	return set->unpack_file (data, big_endian);
}

bool
kkdlib_txp_set_unpack_file_modern (txp_set *set, const void *data, size_t size, uint32_t signature) {
	return set->unpack_file_modern (data, size, signature);
}

void
kkdlib_txp_set_delete (txp_set *set) {
	delete set;
}
}
