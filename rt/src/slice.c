#include <slice.h>
#include <mem.h>

void* __elo_slice_get(void* ctx, Pos pos, size_t elem, _ELO_SLICE_T slice, size_t index) {
    uint8_t* start = __elo_handle_get((GlobalContext*)ctx, slice.data);
    if (index >= slice.len)
        __elo_panic(pos, "index %zu is out of bounds for slice of length %zu", index, slice.len);
    return (void*)(start + (index*elem));
}
