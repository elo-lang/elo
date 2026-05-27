#include <mem.h>
#include <str.h>
#include <typing.h>

#ifndef BUILTINS_H
#define BUILTINS_H

// Print
void __elo_print_str(GlobalContext* ctx, _ELO_STR_T value);
void __elo_print_decimal(GlobalContext* ctx, _ELO_F64_T value);
void __elo_print_unsigned(GlobalContext* ctx, _ELO_U64_T value);
void __elo_print_signed(GlobalContext* ctx, _ELO_I64_T value);
void __elo_print_bool(GlobalContext* ctx, _ELO_BOOL_T value);
void __elo_print_char(GlobalContext* ctx, _ELO_CHAR_T value);

// Args
_ELO_SLICE_T __elo_args(GlobalContext* ctx);

#endif
