#include <mem.h>
#include <str.h>

void __elo_print_str(MemoryContext* ctx, Str str) {
    char* ptr = __elo_handle_get(ctx, str.slot);
    ptr += str.offset;
    printf("%.*s\n", str.size, ptr);
}
