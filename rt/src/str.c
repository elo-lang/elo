#include <panic.h>
#include <mem.h>
#include <str.h>
#include <string.h>

Str __elo_str_new(MemoryContext* ctx, const char* cstr) {
    Slot slot = __elo_handle_add(ctx, cstr);
    return (Str) {
        .slot = slot,
        .offset = 0,
        .size = strlen(cstr)
    };
}

char __elo_str_get(Pos pos, MemoryContext* ctx, Str str, size_t index) {
    char* cstr = __elo_handle_get(ctx, str.slot);
    char* start = cstr + str.offset;
    if (index >= str.size)
        __elo_panic(pos, "index %zu is out of bounds for str of length %zu", index, str.size);
    return *(start + index);
}

Str __elo_str_slice(Pos pos, MemoryContext *ctx, Str str, size_t start, size_t end) {
    if (start > end)
        __elo_panic(pos, "slice start %zu is greater than slice end %zu for str of length %zu", start, end, str.size);
    if (start >= str.size)
        __elo_panic(pos, "slice start %zu is out of bounds for str of length %zu", start, str.size);
    if (end >= str.size)
        __elo_panic(pos, "slice end %zu is out of bounds for str of length %zu", end, str.size);

    size_t offset = str.offset + start;
    size_t size = end - offset;

    return (Str) {
        .slot = str.slot,
        .offset = offset,
        .size = size
    };
}
