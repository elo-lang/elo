#include <stdlib.h>
#include <string.h>
#include <mem.h>
#include <panic.h>
#include <list.h>

#define ELO_LIST_INITIAL_CAPACITY 1024

List __elo_list_new(MemoryContext* ctx, size_t elem) {
	Slot s = __elo_handle_new(ctx, ELO_LIST_INITIAL_CAPACITY);
	List list = {
		.slot = s,
		.len = 0,
		.capacity = ELO_LIST_INITIAL_CAPACITY,
		.elem = elem,
	};
	return list;
}

void __elo_list_append(MemoryContext* ctx, List* list, void* x) {
	size_t new_length = list->len + 1;
	if (list->capacity < new_length) {
		__elo_handle_resize(ctx, list->slot, list->capacity*2);
	}
	char* dest = ctx->handles.items[list->slot];
	memcpy(dest+(list->len*list->elem), x, list->elem);
	list->len = new_length;
}

void* __elo_list_get(Pos pos, const MemoryContext* ctx, List list, size_t index) {
	if (index >= list.len) {
		__elo_panic(pos, "index %zu is out of bounds for list of length %zu", index, list.len);
	}
	char* ptr = (char*)ctx->handles.items[list.slot];
	return (void*)(ptr+index*list.elem);
}

void __elo_list_drop(MemoryContext* ctx, List list) {
	__elo_handle_drop(ctx, list.slot);
}
