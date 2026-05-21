#include "panic.c"
#include "mem.c"
#include "list.c"
#include "str.c"
#include "print.c"

void __elo_main(GlobalContext*);

int main(int argc, char** argv) {
    GlobalContext ctx = (GlobalContext){
        .handles = {0},
        .dead_slots = {0},
        .argc = argc,
        .argv = argv
    };
	__elo_main(&ctx);
	return 0;
}
