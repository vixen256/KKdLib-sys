#include "aet.hpp"

extern "C" {
AetSet *
kkdlib_aet_set_new () {
	AetSet *set = new AetSet ();
	set->alloc  = prj::shared_ptr<prj::stack_allocator> (new prj::stack_allocator);
	return set;
}

void
kkdlib_aet_set_pack_file (AetSet *set, void **data, size_t *size) {
	return set->set.pack_file (data, size);
}

void
kkdlib_aet_set_delete_packed_file (void *data) {
	if (data) free_def (data);
}

void
kkdlib_aet_set_unpack_file (AetSet *set, const void *data, size_t size, bool modern) {
	set->set.unpack_file (set->alloc, data, size, modern);
}

void
kkdlib_aet_set_delete (AetSet *set) {
	delete set;
}
}
