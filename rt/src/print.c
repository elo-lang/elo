#include <mem.h>
#include <str.h>
#include <typing.h>

void __elo_print_str(MemoryContext* ctx, _ELO_STR_T value) {
    char* ptr = __elo_handle_get(ctx, value.slot);
    ptr += value.offset;
    printf("%.*s\n", value.size, ptr);
}

void __elo_print_decimal(MemoryContext* ctx, _ELO_F64_T value) {
    printf("%g\n", value);
}

void __elo_print_unsigned(MemoryContext* ctx, _ELO_I64_T value) {
    printf("%llu\n", value);
}

void __elo_print_signed(MemoryContext* ctx, _ELO_U64_T value) {
    printf("%ll\n", value);
}

void __elo_print_bool(MemoryContext* ctx, _ELO_BOOL_T value) {
    printf("%s\n", value ? "true" : "false");
}

void __elo_print_char(MemoryContext* ctx, _ELO_CHAR_T value) {
    uint32_t cp = value; // CP is for CODEPOINT for fuck sake
    if (cp <= 0x7F) {
        /* 1 byte: 0xxxxxxx (Standard ASCII) */
        putchar((char)cp);
    }
    else if (cp <= 0x7FF) {
        /* 2 bytes: 110xxxxx 10xxxxxx */
        putchar((char)(0xC0 | (cp >> 6)));
        putchar((char)(0x80 | (cp & 0x3F)));
    }
    else if (cp <= 0xFFFF) {
        /* 3 bytes: 1110xxxx 10xxxxxx 10xxxxxx */
        putchar((char)(0xE0 | (cp >> 12)));
        putchar((char)(0x80 | ((cp >> 6) & 0x3F)));
        putchar((char)(0x80 | (cp & 0x3F)));
    }
    else if (cp <= 0x10FFFF) {
        /* 4 bytes: 11110xxx 10xxxxxx 10xxxxxx 10xxxxxx */
        putchar((char)(0xF0 | (cp >> 18)));
        putchar((char)(0x80 | ((cp >> 12) & 0x3F)));
        putchar((char)(0x80 | ((cp >> 6) & 0x3F)));
        putchar((char)(0x80 | (cp & 0x3F)));
    }
}
