#pragma once
#include <key_val.hpp>

extern "C" {
key_val *kkdlib_key_val_new ();
void kkdlib_key_val_close_scope (key_val *kv);
bool kkdlib_key_val_has_key (key_val *kv, const char *str);
bool kkdlib_key_val_open_scope (key_val *kv, const char *str);
bool kkdlib_key_val_open_scope_uint32 (key_val *kv, uint32_t i);
void kkdlib_key_val_parse (key_val *kv, const void *data, size_t size);
bool kkdlib_key_val_read_bool (key_val *kv, const char *key, bool *value);
bool kkdlib_key_val_read_float (key_val *kv, const char *key, float_t *value);
bool kkdlib_key_val_read_int32 (key_val *kv, const char *key, int32_t *value);
bool kkdlib_key_val_read_uint32 (key_val *kv, const char *key, uint32_t *value);
bool kkdlib_key_val_read_str (key_val *kv, const char *key, const char **value);
void kkdlib_key_val_delete (key_val *kv);
}
