#include "database/sprite.hpp"

extern "C" {
spr_db_spr_file *
kkdlib_spr_db_spr_file_new () {
	return new spr_db_spr_file ();
}

uint32_t
kkdlib_spr_db_spr_file_get_id (spr_db_spr_file *spr_file) {
	return spr_file->id;
}

void
kkdlib_spr_db_spr_file_set_id (spr_db_spr_file *spr_file, uint32_t id) {
	spr_file->id = id;
}

const char *
kkdlib_spr_db_spr_file_get_name (spr_db_spr_file *spr_file) {
	return spr_file->name.c_str ();
}

void
kkdlib_spr_db_spr_file_set_name (spr_db_spr_file *spr_file, const char *name) {
	spr_file->name.assign (name);
}

uint16_t
kkdlib_spr_db_spr_file_get_index (spr_db_spr_file *spr_file) {
	return spr_file->index;
}

void
kkdlib_spr_db_spr_file_set_index (spr_db_spr_file *spr_file, uint16_t index) {
	spr_file->index = index;
}

bool
kkdlib_spr_db_spr_file_get_texture (spr_db_spr_file *spr_file) {
	return spr_file->texture;
}

void
kkdlib_spr_db_spr_file_set_texture (spr_db_spr_file *spr_file, bool texture) {
	spr_file->texture = texture;
}

void
kkdlib_spr_db_spr_file_delete (spr_db_spr_file *spr_file) {
	delete spr_file;
}

spr_db_spr_set_file *
kkdlib_spr_db_spr_set_file_new () {
	return new spr_db_spr_set_file ();
}

uint32_t
kkdlib_spr_db_spr_set_file_get_id (spr_db_spr_set_file *set_file) {
	return set_file->id;
}

void
kkdlib_spr_db_spr_set_file_set_id (spr_db_spr_set_file *set_file, uint32_t id) {
	set_file->id = id;
}

const char *
kkdlib_spr_db_spr_set_file_get_name (spr_db_spr_set_file *set_file) {
	return set_file->name.c_str ();
}

void
kkdlib_spr_db_spr_set_file_set_name (spr_db_spr_set_file *set_file, const char *name) {
	set_file->name.assign (name);
}

const char *
kkdlib_spr_db_spr_set_file_get_file_name (spr_db_spr_set_file *set_file) {
	return set_file->file_name.c_str ();
}

void
kkdlib_spr_db_spr_set_file_set_file_name (spr_db_spr_set_file *set_file, const char *name) {
	set_file->file_name.assign (name);
}

size_t
kkdlib_spr_db_spr_set_file_get_sprite_size (spr_db_spr_set_file *set_file) {
	return set_file->sprite.size ();
}

spr_db_spr_file *
kkdlib_spr_db_spr_set_file_get_sprite (spr_db_spr_set_file *set_file, size_t index) {
	return set_file->sprite.data () + index;
}

void
kkdlib_spr_db_spr_set_file_add_sprite (spr_db_spr_set_file *set_file, spr_db_spr_file *spr_file) {
	set_file->sprite.push_back (*spr_file);
}

void
kkdlib_spr_db_spr_set_file_delete (spr_db_spr_set_file *set_file) {
	delete set_file;
}

sprite_database_file *
kkdlib_sprite_database_file_new () {
	return new sprite_database_file ();
}

bool
kkdlib_sprite_database_file_get_ready (sprite_database_file *database_file) {
	return database_file->ready;
}

void
kkdlib_sprite_database_file_set_ready (sprite_database_file *database_file, bool ready) {
	database_file->ready = ready;
}

bool
kkdlib_sprite_database_file_get_modern (sprite_database_file *database_file) {
	return database_file->modern;
}

void
kkdlib_sprite_database_file_set_modern (sprite_database_file *database_file, bool modern) {
	database_file->modern = modern;
}

bool
kkdlib_sprite_database_file_get_big_endian (sprite_database_file *database_file) {
	return database_file->big_endian;
}

void
kkdlib_sprite_database_file_set_big_endian (sprite_database_file *database_file, bool big_endian) {
	database_file->big_endian = big_endian;
}

bool
kkdlib_sprite_database_file_get_is_x (sprite_database_file *database_file) {
	return database_file->is_x;
}

void
kkdlib_sprite_database_file_set_is_x (sprite_database_file *database_file, bool is_x) {
	database_file->is_x = is_x;
}
size_t
kkdlib_sprite_database_file_get_sprite_set_size (sprite_database_file *database_file) {
	return database_file->sprite_set.size ();
}

spr_db_spr_set_file *
kkdlib_sprite_database_file_get_sprite_set (sprite_database_file *database_file, size_t index) {
	return database_file->sprite_set.data () + index;
}

void
kkdlib_sprite_database_file_add_sprite_set (sprite_database_file *database_file, spr_db_spr_set_file *set_file) {
	set_file->index = database_file->sprite_set.size ();
	database_file->sprite_set.push_back (*set_file);
}

void
kkdlib_sprite_database_file_read_file (sprite_database_file *database_file, const char *path, bool modern) {
	database_file->read (path, modern);
}

void
kkdlib_sprite_database_file_read_data (sprite_database_file *database_file, void *data, size_t size, bool modern) {
	database_file->read (data, size, modern);
}

void
kkdlib_sprite_database_file_write_file (sprite_database_file *database_file, const char *path) {
	database_file->write (path);
}

void
kkdlib_sprite_database_file_write_data (sprite_database_file *database_file, void **data, size_t *size) {
	database_file->write (data, size);
}

void
kkdlib_sprite_databse_file_delete_packed_data (void *data) {
	if (data) free_def (data);
}

void
kkdlib_sprite_database_file_delete (sprite_database_file *database_file) {
	delete database_file;
}

uint32_t
kkdlib_spr_db_spr_get_id (const spr_db_spr *spr) {
	return spr->id;
}

const char *
kkdlib_spr_db_spr_get_name (const spr_db_spr *spr) {
	return spr->name.c_str ();
}

uint16_t
kkdlib_spr_db_spr_get_index (const spr_db_spr *spr) {
	return spr->info.index;
}

uint16_t
kkdlib_spr_db_spr_get_set_index (const spr_db_spr *spr) {
	return spr->info.set_index;
}

uint32_t
kkdlib_spr_db_spr_set_get_id (const spr_db_spr_set *set) {
	return set->id;
}

const char *
kkdlib_spr_db_spr_set_get_name (const spr_db_spr_set *set) {
	return set->name.c_str ();
}

const char *
kkdlib_spr_db_spr_set_get_file_name (const spr_db_spr_set *set) {
	return set->file_name.c_str ();
}

uint32_t
kkdlib_spr_db_spr_set_get_index (const spr_db_spr_set *set) {
	return set->index;
}

sprite_database *
kkdlib_sprite_database_new () {
	return new sprite_database ();
}

void
kkdlib_sprite_database_add_file (sprite_database *database, sprite_database_file *file) {
	database->add (file);
}

const spr_db_spr_set *
kkdlib_sprite_database_get_spr_set_by_name (sprite_database *database, const char *name) {
	return database->get_spr_set_by_name (name);
}

const spr_db_spr_set *
kkdlib_sprite_database_get_spr_set_by_id (sprite_database *database, uint32_t set_id) {
	return database->get_spr_set_by_id (set_id);
}

const spr_db_spr_set *
kkdlib_sprite_database_get_spr_set_by_index (sprite_database *database, uint32_t index) {
	return database->get_spr_set_by_index (index);
}

const spr_db_spr *
kkdlib_sprite_database_get_spr_by_name (sprite_database *database, const char *name) {
	return database->get_spr_by_name (name);
}

const spr_db_spr *
kkdlib_sprite_database_get_spr_by_id (sprite_database *database, uint32_t id) {
	return database->get_spr_by_id (id);
}

void
kkdlib_sprite_database_delete (sprite_database *database) {
	delete database;
}
}
