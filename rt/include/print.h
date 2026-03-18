#include <mem.h>
#include <str.h>

#ifndef PRINT_H
#define PRINT_H

void __elo_print_str(MemoryContext* ctx, _ELO_STR_T value);
void __elo_print_decimal(MemoryContext* ctx, _ELO_F64_T value);
void __elo_print_unsigned(MemoryContext* ctx, _ELO_U64_T value);
void __elo_print_signed(MemoryContext* ctx, _ELO_I64_T value);
void __elo_print_bool(MemoryContext* ctx, _ELO_BOOL_T value);
void __elo_print_char(MemoryContext* ctx, _ELO_CHAR_T value);

#endif
