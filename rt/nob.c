#define NOB_IMPLEMENTATION
#include "nob.h"

#define BUILD_DIR "bin/"
#define RT "libelort.a"
#define ENTRY "src/main.c"

static Cmd cmd = {0};

bool build_rt(const char* source) {
	cmd_append(&cmd, "cc", "-c", "-o", BUILD_DIR RT, source);
	cmd_append(&cmd, "-I", "include");
	return cmd_run(&cmd);
}

int main(int argc, char** argv) {
	NOB_GO_REBUILD_URSELF(argc, argv);
	if (!mkdir_if_not_exists(BUILD_DIR)) return 1;
	if (!build_rt(ENTRY)) return 1;
	return 0;
}
