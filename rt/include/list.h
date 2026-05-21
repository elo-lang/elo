#include <panic.h>
#include <mem.h>

#ifndef LIST_H
#define LIST_H

typedef struct {
	Slot slot;
	size_t len;
	size_t capacity;
	size_t elem;
} List;

List __elo_list_new(GlobalContext* ctx, size_t elem);
void __elo_list_append(GlobalContext* ctx, List* list, void* x);
void __elo_list_drop(GlobalContext* ctx, List list);
void* __elo_list_get(Pos pos, const GlobalContext* ctx, List list, size_t index);

#endif
