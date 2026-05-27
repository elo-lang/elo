#ifndef SLICE_H
#define SLICE_H

typedef size_t Slot;
typedef struct { Slot data; size_t len; } _ELO_SLICE_T;

#define __elo_slice_get(ctx, T, slice, index) \
    ((T*)__elo_handle_get(ctx, (slice).data))[(index)]

#define __elo_slice_new(ctx, ptr, len) \
    { .data = __elo_handle_add((ctx), (ptr)), .len = (len) }

#endif
