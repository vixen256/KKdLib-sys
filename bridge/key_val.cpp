#include "key_val.hpp"

extern "C" {
key_val *
kkdlib_key_val_new () {
	return new key_val ();
}

void
kkdlib_key_val_close_scope (key_val *kv) {
	kv->close_scope ();
}

bool
kkdlib_key_val_has_key (key_val *kv, const char *str) {
	return kv->has_key (str);
}

bool
kkdlib_key_val_open_scope (key_val *kv, const char *str) {
	return kv->open_scope (str);
}

bool
kkdlib_key_val_open_scope_uint32 (key_val *kv, uint32_t i) {
	return kv->open_scope_fmt (i);
}

void
kkdlib_key_val_parse (key_val *kv, const void *data, size_t size) {
	kv->parse (data, size);
}

bool
kkdlib_key_val_read_bool (key_val *kv, const char *key, bool *value) {
	return kv->read (key, *value);
}

bool
kkdlib_key_val_read_float (key_val *kv, const char *key, float_t *value) {
    return kv->read (key, *value);
}

bool
kkdlib_key_val_read_int32 (key_val *kv, const char *key, int32_t *value) {
    return kv->read (key, *value);
}

bool
kkdlib_key_val_read_uint32 (key_val *kv, const char *key, uint32_t *value) {
    return kv->read (key, *value);
}

bool
kkdlib_key_val_read_str (key_val *kv, const char *key, const char **value) {
    return kv->read (key, *value);
}

void
kkdlib_key_val_delete (key_val *kv) {
	delete kv;
}
}
