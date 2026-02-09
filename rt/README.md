# rt
Elo programming language runtime implementation

> **WARNING**: Elo programming language is **NOT** finished.
> Use this at your own risk.

This project consists of the implementation of the underlying
software that will run with the compiled Elo program you create
to manage program initialization, memory and special types.

## Structure
- `src`: Base runtime implementation source-code
  * `main.c`: Entry-point of the entire program and is bundled with the generated C
- `include`: Base runtime header source-code

These folders compose to a library that will be linked with the
compiled program by Elo's compiler (`elort`).

## Building
To build the library, use [`nob`](https://github.com/tsoding/nob.h).

### Instructions
Compile `nob.c` with any standard C compiler and run the build:
```console
$ cc -o nob nob.c
$ ./nob
```

This will create the static library `libelort.a` and a test program in the `bin/` folder.

---

> Copyright (c) 2026 Igor Ferreira, Marcio Dantas 

> Licensed under [MIT License](https://choosealicense.com/licenses/mit/)
