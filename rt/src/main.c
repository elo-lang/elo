#include <prelude.h>

#include "panic.c"
#include "mem.c"
#include "list.c"
#include "str.c"
#include "builtins.c"

void __elo_main(GlobalContext*);

int main(int argc, char** argv) {
    GlobalContext ctx = (GlobalContext){
        .handles = {0},
        .dead_slots = {0},
        .args = {0},
    };
    size_t size = sizeof(_ELO_STR_T)*argc;
    _ELO_SLICE_T args = { .data = __elo_handle_add(&ctx, malloc(size)), .len = argc };
    _ELO_STR_T* p = __elo_handle_get(&ctx, args.data);
    for (size_t i = 0; i < argc; i++, p++) {
        *p = __elo_str_new(&ctx, argv[i]);
    }

	__elo_main(&ctx);
	return 0;
}
