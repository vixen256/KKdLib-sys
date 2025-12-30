#pragma once
#include <database/sprite.cpp>

extern "C" {
spr_db_spr_file *kkdlib_spr_db_spr_file_new ();
uint32_t kkdlib_spr_db_spr_file_get_id (spr_db_spr_file *spr_file);
void kkdlib_spr_db_spr_file_set_id (spr_db_spr_file *spr_file, uint32_t id);
const char *kkdlib_spr_db_spr_file_get_name (spr_db_spr_file *spr_file);
void kkdlib_spr_db_spr_file_set_name (spr_db_spr_file *spr_file, const char *name);
uint16_t kkdlib_spr_db_spr_file_get_index (spr_db_spr_file *spr_file);
void kkdlib_spr_db_spr_file_set_index (spr_db_spr_file *spr_file, uint16_t index);
bool kkdlib_spr_db_spr_file_get_texture (spr_db_spr_file *spr_file);
void kkdlib_spr_db_spr_file_set_texture (spr_db_spr_file *spr_file, bool texture);
void kkdlib_spr_db_spr_file_delete (spr_db_spr_file *spr_file);

spr_db_spr_set_file *kkdlib_spr_db_spr_set_file_new ();
uint32_t kkdlib_spr_db_spr_set_file_get_id (spr_db_spr_set_file *set_file);
void kkdlib_spr_db_spr_set_file_set_id (spr_db_spr_set_file *set_file, uint32_t id);
const char *kkdlib_spr_db_spr_set_file_get_name (spr_db_spr_set_file *set_file);
void kkdlib_spr_db_spr_set_file_set_name (spr_db_spr_set_file *set_file, const char *name);
const char *kkdlib_spr_db_spr_set_file_get_file_name (spr_db_spr_set_file *set_file);
void kkdlib_spr_db_spr_set_file_set_file_name (spr_db_spr_set_file *set_file, const char *name);
size_t kkdlib_spr_db_spr_set_file_get_sprite_size (spr_db_spr_set_file *set_file);
spr_db_spr_file *kkdlib_spr_db_spr_set_file_get_sprite (spr_db_spr_set_file *set_file, size_t index);
void kkdlib_spr_db_spr_set_file_add_sprite (spr_db_spr_set_file *set_file, spr_db_spr_file *spr_file);
void kkdlib_spr_db_spr_set_file_delete (spr_db_spr_set_file *set_file);

sprite_database_file *kkdlib_sprite_database_file_new ();
bool kkdlib_sprite_database_file_get_ready (sprite_database_file *database_file);
void kkdlib_sprite_database_file_set_ready (sprite_database_file *database_file, bool ready);
bool kkdlib_sprite_database_file_get_modern (sprite_database_file *database_file);
void kkdlib_sprite_database_file_set_modern (sprite_database_file *database_file, bool modern);
bool kkdlib_sprite_database_file_get_big_endian (sprite_database_file *database_file);
void kkdlib_sprite_database_file_set_big_endian (sprite_database_file *database_file, bool big_endian);
bool kkdlib_sprite_database_file_get_is_x (sprite_database_file *database_file);
void kkdlib_sprite_database_file_set_is_x (sprite_database_file *database_file, bool is_x);
size_t kkdlib_sprite_database_file_get_sprite_set_size (sprite_database_file *database_file);
spr_db_spr_set_file *kkdlib_sprite_database_file_get_sprite_set (sprite_database_file *database_file, size_t index);
void kkdlib_sprite_database_file_add_sprite_set (sprite_database_file *database_file, spr_db_spr_set_file *set_file);
void kkdlib_sprite_database_file_read_file (sprite_database_file *database_file, const char *path, bool modern);
void kkdlib_sprite_database_file_read_data (sprite_database_file *database_file, void *data, size_t size, bool modern);
void kkdlib_sprite_database_file_write_file (sprite_database_file *database_file, const char *path);
void kkdlib_sprite_database_file_write_data (sprite_database_file *database_file, void **data, size_t *size);
void kkdlib_sprite_databse_file_delete_packed_data (void *data);
void kkdlib_sprite_database_file_delete (sprite_database_file *database_file);

uint32_t kkdlib_spr_db_spr_get_id (const spr_db_spr *spr);
const char *kkdlib_spr_db_spr_get_name (const spr_db_spr *spr);
uint16_t kkdlib_spr_db_spr_get_index (const spr_db_spr *spr);
uint16_t kkdlib_spr_db_spr_get_set_index (const spr_db_spr *spr);

uint32_t kkdlib_spr_db_spr_set_get_id (const spr_db_spr_set *set);
const char *kkdlib_spr_db_spr_set_get_name (const spr_db_spr_set *set);
const char *kkdlib_spr_db_spr_set_get_file_name (const spr_db_spr_set *set);
uint32_t kkdlib_spr_db_spr_set_get_index (const spr_db_spr_set *set);

sprite_database *kkdlib_sprite_database_new ();
void kkdlib_sprite_database_add_file (sprite_database *database, sprite_database_file *file);
const spr_db_spr_set *kkdlib_sprite_database_get_spr_set_by_name (sprite_database *database, const char *name);
const spr_db_spr_set *kkdlib_sprite_database_get_spr_set_by_id (sprite_database *database, uint32_t set_id);
const spr_db_spr_set *kkdlib_sprite_database_get_spr_set_by_index (sprite_database *database, uint32_t index);
const spr_db_spr *kkdlib_sprite_database_get_spr_by_name (sprite_database *database, const char *name);
const spr_db_spr *kkdlib_sprite_database_get_spr_by_id (sprite_database *database, uint32_t id);
void kkdlib_sprite_database_delete (sprite_database *database);
}
