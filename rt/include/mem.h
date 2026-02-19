// All memory-related constructs in Elo's runtime environment
// Copyright (c) 2026 Igor Ferreira, Marcio Dantas

#include <stddef.h>

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
} MemoryContext;

Slot __elo_handle_add(MemoryContext* ctx, void* ptr);
Slot __elo_handle_new(MemoryContext* ctx, size_t size);
void __elo_handle_resize(MemoryContext* ctx, Slot slot, size_t size);
void __elo_handle_drop(MemoryContext* ctx, Slot slot);
void* __elo_handle_get(MemoryContext* ctx, Slot slot);

#endif
