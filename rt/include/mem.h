// All memory-related constructs in Elo's runtime environment
// Copyright (c) 2026 Igor Ferreira, Marcio Dantas

#include <stddef.h>
#include <slice.h>

#ifndef MEM_H
#define MEM_H

// Terminology
//    handle - A heap address located in the handles table
//    slot   -

// Index in the handle table containing the slot's heap address
typedef size_t Slot;

// Handles table
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

Slot __elo_handle_add(GlobalContext* ctx, void* ptr);
Slot __elo_handle_new(GlobalContext* ctx, size_t size);
void __elo_handle_resize(GlobalContext* ctx, Slot slot, size_t size);
void __elo_handle_drop(GlobalContext* ctx, Slot slot);
void* __elo_handle_get(GlobalContext* ctx, Slot slot);

#endif
