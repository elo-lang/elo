// All memory-related constructs in Elo's runtime environment
// Copyright (c) 2026 Igor Ferreira, Marcio Dantas

#include <stddef.h>
#include <slice.h>

#ifndef MEM_H
#define MEM_H

// Memory management terminology
//    handle       - An arbitrary pointer address (most of the time on heap) created during program execution
//    handle table - Sequential list of all handles stored in the runtime
//    slot         - An unsigned integer value which is an index for a handle in the handle table
//    slot table   - Sequential list of different slots
//    dead slots   - Slot table containing all slots that correspond to freed or unreachable addresses in the handle table

// Memory management 'layout' (example values only):
//  Handle Table (dynamic array)
//  | Slot (index #) | Value (memory address) |
//  |----------------|------------------------|
//  | 0              | 0xcafebabe             |
//  | 1              | 0xbabecafe             |
//  | ...            | ...                    | and so on...
// Dead Slots (dynamic array
//  | Index | Value (slot referencing a freed/unreachable handle) |
//  |-------|-----------------------------------------------------|
//  | 0     | 25                                                  |
//  | 1     | 34                                                  |
//  | ...   | ...                                                 | and so on...

// NOTE: When creating new handles, the system will prioritize using the dead slots first to save space.

typedef struct {
	void** items;
	size_t count;
	size_t capacity;
} HandleTable;

typedef struct {
	Slot* items;
	size_t count;
	size_t capacity;
} SlotTable;

typedef struct {
	HandleTable handles;
	SlotTable   dead_slots;
	_ELO_SLICE_T args;
} GlobalContext;

// General operations
Slot __elo_handle_add(GlobalContext* ctx, void* ptr);
void* __elo_handle_get(GlobalContext* ctx, Slot slot);

// Heap-only operations
Slot __elo_handle_new(GlobalContext* ctx, size_t size); // allocates heap
void __elo_handle_resize(GlobalContext* ctx, Slot slot, size_t size); // allocates heap
void __elo_handle_drop(GlobalContext* ctx, Slot slot); // frees heap

#endif
