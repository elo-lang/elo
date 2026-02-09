// All memory-related constructs in Elo's runtime environment
// Copyright (c) 2026 Igor Ferreira, Marcio Dantas

#include <stdio.h>
#include <stdlib.h>
#include <mem.h>

#define da_append(xs, x)                                                             \
    do {                                                                             \
        if ((xs)->count >= (xs)->capacity) {                                         \
            if ((xs)->capacity == 0) (xs)->capacity = 256;                           \
            else (xs)->capacity *= 2;                                                \
            (xs)->items = realloc((xs)->items, (xs)->capacity*sizeof(*(xs)->items)); \
        }                                                                            \
        (xs)->items[(xs)->count++] = (x);                                            \
    } while (0)

Slot __elo_handle_new(MemoryContext* ctx, size_t size) {
	void* ptr = malloc(size);
	if (ptr == NULL) {
		abort();
	}
	Slot s;
	if (ctx->dead_slots.count > 0) {
		s = ctx->dead_slots.items[--(ctx->dead_slots.count)];
		ctx->handles.items[s] = ptr;
		return s;
	}
	s = ctx->handles.count;
	da_append(&ctx->handles, ptr);
	return s;
}

void __elo_handle_resize(MemoryContext* ctx, Slot slot, size_t size) {
	void* old_ptr = ctx->handles.items[slot];
	void* new_ptr = realloc(old_ptr, size);
	if (new_ptr == NULL) abort();
	ctx->handles.items[slot] = new_ptr;
}

void __elo_handle_drop(MemoryContext* ctx, Slot slot) {
	free(ctx->handles.items[slot]);
	ctx->handles.items[slot] = NULL;
	da_append(&ctx->dead_slots, slot);
}
