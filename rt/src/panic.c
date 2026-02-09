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
