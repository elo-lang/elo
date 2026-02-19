#include <mem.h>

#ifndef STR_H
#define STR_H

typedef struct {
    Slot slot;
    size_t offset;
    size_t size;
} Str;

Str __elo_str_new(MemoryContext* ctx, const char* cstr);
Str __elo_str_slice(MemoryContext* ctx, Str str, size_t start, size_t end);
char __elo_str_get(Pos pos, MemoryContext* ctx, Str str, size_t index);

#endif
