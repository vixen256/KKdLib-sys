#include "farc.hpp"

extern "C" {
farc *
kkdlib_farc_new (farc_signature signature, farc_flags flags, bool ft) {
	return new farc (signature, flags, ft);
}

size_t
kkdlib_farc_get_files_size (farc *farc) {
	return farc->files.size ();
}

farc_file *
kkdlib_farc_get_file_by_index (farc *farc, size_t index) {
	return farc->files.data () + index;
}

farc_file *
kkdlib_farc_get_file_by_name (farc *farc, const char *name) {
	return farc->read_file (name);
}

farc_file *
kkdlib_farc_add_file (farc *farc, const char *name) {
	return farc->add_file (name);
}

farc_signature
kkdlib_farc_get_signature (farc *farc) {
	return farc->signature;
}

void
kkdlib_farc_set_signature (farc *farc, farc_signature signature) {
	farc->signature = signature;
}

farc_flags
kkdlib_farc_get_flags (farc *farc) {
	return farc->flags;
}

void
kkdlib_farc_set_flags (farc *farc, farc_flags flags) {
	farc->flags = flags;
}

int32_t
kkdlib_farc_get_compression_level (farc *farc) {
	return farc->compression_level;
}

void
kkdlib_farc_set_compression_level (farc *farc, int32_t compression_level) {
	farc->compression_level = compression_level;
}

uint32_t
kkdlib_farc_get_alignment (farc *farc) {
	return farc->alignment;
}

void
kkdlib_farc_set_alignment (farc *farc, uint32_t alignment) {
	farc->alignment = alignment;
}

bool
kkdlib_farc_get_ft (farc *farc) {
	return farc->ft;
}

void
kkdlib_farc_set_ft (farc *farc, bool ft) {
	farc->ft = ft;
}

void
kkdlib_farc_read_file (farc *farc, const char *path, bool unpack, bool save) {
	farc->read (path, unpack, save);
}

void
kkdlib_farc_read_data (farc *farc, const void *data, size_t size, bool unpack) {
	farc->read (data, size, unpack);
}

void
kkdlib_farc_write_file (farc *farc, const char *path, farc_signature signature, farc_flags flags, bool add_extension, bool get_files) {
	return farc->write (path, signature, flags, add_extension, get_files);
}

void
kkdlib_farc_write_data (farc *farc, void **data, size_t *size, farc_signature signature, farc_flags flags) {
	return farc->write (data, size, signature, flags);
}

void
kkdlib_farc_delete_packed_file (void *data) {
	if (data) free_def (data);
}

void
kkdlib_farc_delete (farc *farc) {
	delete farc;
}

const char *
kkdlib_farc_file_get_name (farc_file *file) {
	return file->name.c_str ();
}

void
kkdlib_farc_file_set_name (farc_file *file, const char *name) {
	file->name.assign (name);
}

size_t
kkdlib_farc_file_get_size (farc_file *file) {
	return file->size;
}

void *
kkdlib_farc_file_get_data (farc_file *file) {
	return file->data;
}

void
kkdlib_farc_file_set_data (farc_file *file, const void *data, size_t size) {
	if (file->data) free (file->data);
	file->size = size;
	file->data = malloc (size);
	memcpy (file->data, data, size);
	file->data_changed = true;
}

bool
kkdlib_farc_file_get_compressed (farc_file *file) {
	return file->compressed;
}

void
kkdlib_farc_file_set_compressed (farc_file *file, bool compressed) {
	file->compressed = compressed;
}

bool
kkdlib_farc_file_get_encrypted (farc_file *file) {
	return file->encrypted;
}

void
kkdlib_farc_file_set_encrypted (farc_file *file, bool encrypted) {
	file->encrypted = encrypted;
}
}
