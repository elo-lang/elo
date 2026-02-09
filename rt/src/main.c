#include "panic.c"
#include "mem.c"
#include "list.c"

void __elo_main(MemoryContext*);

int main(int argc, char** argv) {
	MemoryContext ctx = {0};
	__elo_main(&ctx);
	return 0;
}
