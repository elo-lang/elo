#include <stdio.h>
#include <stdlib.h>
#include <stdarg.h>
#include <panic.h>

void __elo_panic(Pos pos, const char *fmt, ...) {
    va_list args;
    fprintf(stderr, "%s:%zu:%zu: PANIC: ", pos.filename, pos.line, pos.col);
    va_start(args, fmt);
    vfprintf(stderr, fmt, args);
    va_end(args);
    fputc('\n', stderr);
    exit(1);
}

void __elo_index_check(Pos pos, size_t len, size_t index) {
    if (index >= len) {
        __elo_panic(pos, "index %zu is out of bounds for length %zu", index, len);
    }
}
