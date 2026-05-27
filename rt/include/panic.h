#ifndef PANIC_H
#define PANIC_H

typedef struct {
	const char* filename;
	size_t line, col;
} Pos;

void __elo_panic(Pos pos, const char* message, ...);
void __elo_index_check(Pos pos, size_t len, size_t index);

#endif
