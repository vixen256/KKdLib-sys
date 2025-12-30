#pragma once
#include <aet.cpp>

extern "C" {
struct AetSet {
	aet_set set;
	prj::shared_ptr<prj::stack_allocator> alloc;

	AetSet () : set (), alloc () {}
};

AetSet *kkdlib_aet_set_new ();
void kkdlib_aet_set_pack_file (AetSet *set, void **data, size_t *size);
void kkdlib_aet_set_delete_packed_file (void *data);
void kkdlib_aet_set_unpack_file (AetSet *set, const void *data, size_t size, bool modern);
void kkdlib_aet_set_delete (AetSet *set);
}
