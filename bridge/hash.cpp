#include "hash.hpp"

extern "C" {
uint64_t
kkdlib_hash_fnv1a64m (const void *data, size_t size) {
	return hash_fnv1a64m (data, size);
}

uint32_t
kkdlib_hash_murmurhash (const void *data, size_t size) {
	return hash_murmurhash (data, size);
}

uint16_t
kkdlib_hash_crc16_ccitt (const void *data, size_t size) {
	return hash_crc16_ccitt (data, size);
}

uint64_t
kkdlib_hash_xxh3_64bits (const void *data, size_t size) {
	return hash_xxh3_64bits (data, size);
}

uint32_t
kkdlib_hash_adler32 (uint32_t adler, const void *buf, size_t len) {
	return hash_adler32 (adler, buf, len);
}
}
