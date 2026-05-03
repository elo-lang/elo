#define NOB_IMPLEMENTATION
#include "nob.h"

#define BUILD_DIR "bin/"
#define RT_NATIVE  "libelort.a"
#define RT_WIN     "elort.lib"
#define ENTRY      "src/main.c"

static Cmd cmd = {0};

bool build_native(const char *source) {
    cmd_append(&cmd, "cc", "-c", "-o", BUILD_DIR RT_NATIVE, source);
    cmd_append(&cmd, "-I", "include");
    return cmd_run(&cmd);
}

bool build_mingw(const char *source) {
    cmd_append(&cmd, "x86_64-w64-mingw32-gcc", "-c", "-o", BUILD_DIR RT_WIN, source);
    cmd_append(&cmd, "-I", "include");
    return cmd_run(&cmd);
}

int main(int argc, char **argv) {
    NOB_GO_REBUILD_URSELF(argc, argv);

    if (!mkdir_if_not_exists(BUILD_DIR)) return 1;
    if (!build_native(ENTRY)) return 1;
    if (!build_mingw(ENTRY)) return 1;

    return 0;
}
