#pragma once
#include <farc.hpp>

extern "C" {
farc *kkdlib_farc_new (farc_signature signature, farc_flags flags, bool ft);
size_t kkdlib_farc_get_files_size (farc *farc);
farc_file *kkdlib_farc_get_file_by_index (farc *farc, size_t index);
farc_file *kkdlib_farc_get_file_by_name (farc *farc, const char *name);
farc_file *kkdlib_farc_add_file (farc *farc, const char *name);
farc_signature kkdlib_farc_get_signature (farc *farc);
void kkdlib_farc_set_signature (farc *farc, farc_signature signature);
farc_flags kkdlib_farc_get_flags (farc *farc);
void kkdlib_farc_set_flags (farc *farc, farc_flags flags);
int32_t kkdlib_farc_get_compression_level (farc *farc);
void kkdlib_farc_set_compression_level (farc *farc, int32_t compression_level);
uint32_t kkdlib_farc_get_alignment (farc *farc);
void kkdlib_farc_set_alignment (farc *farc, uint32_t alignment);
bool kkdlib_farc_get_ft (farc *farc);
void kkdlib_farc_set_ft (farc *farc, bool ft);
void kkdlib_farc_read_file (farc *farc, const char *path, bool unpack, bool save);
void kkdlib_farc_read_data (farc *farc, const void *data, size_t size, bool unpack);
void kkdlib_farc_write_file (farc *farc, const char *path, farc_signature signature, farc_flags flags, bool add_extension, bool get_files);
void kkdlib_farc_write_data (farc *farc, void **data, size_t *size, farc_signature signature, farc_flags flags);
void kkdlib_farc_delete_packed_file (void *data);
void kkdlib_farc_delete (farc *farc);

const char *kkdlib_farc_file_get_name (farc_file *file);
void kkdlib_farc_file_set_name (farc_file *file, const char *name);
size_t kkdlib_farc_file_get_size (farc_file *file);
void *kkdlib_farc_file_get_data (farc_file *file);
void kkdlib_farc_file_set_data (farc_file *file, const void *data, size_t size);
bool kkdlib_farc_file_get_compressed (farc_file *file);
void kkdlib_farc_file_set_compressed (farc_file *file, bool compressed);
bool kkdlib_farc_file_get_encrypted (farc_file *file);
void kkdlib_farc_file_set_encrypted (farc_file *file, bool encrypted);
}
