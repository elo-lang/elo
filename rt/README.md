# rt
Elo programming language runtime implementation

> **WARNING**: Elo programming language is **NOT** finished.
> Use this at your own risk.

This project consists of the implementation of the underlying
software that will run with the compiled Elo program you create
to manage program initialization, memory and special types.

## How it works

The runtime library has this structure:
  - **`src` folder**: Runtime implementation source-code, meant to be pre-compiled to a library file somewhere in your compiler installation.
  - **`include` folder**: Base runtime headers, also meant to be located in the compiler's installation. Crucial for the C backend generation.

The final library is `libelort.a` / `elort.lib`.

## Building
To build the library, use [`nob`](https://github.com/tsoding/nob.h).

### Instructions
Compile `nob.c` with any standard C compiler and run the build:
```console
$ cc -o nob nob.c
$ ./nob
```

This will create the static libraries for all platforms in the `bin/` folder.

---

> Copyright(C) 2026 Igor Ferreira, Marcio Dantas 

> Licensed under [MIT License](https://choosealicense.com/licenses/mit/). Read [LICENSE](../LICENSE) for more information.
