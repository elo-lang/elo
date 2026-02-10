#ifndef PANIC_H
#define PANIC_H

typedef struct {
	const char* filename;
	size_t line, col;
} Pos;

void __elo_panic(Pos pos, const char* message, ...);

#endif
