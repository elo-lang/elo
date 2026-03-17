<p align=center><img width="260" src="docs/assets/elo-icon-text-bw.png"></p>
<h4 align=center><strong><em>Building software intuitively.</em></strong></h4>

**Elo** is a compiled systems programming language focused on developer experience
without sacrificing performance or control. It features a simple syntax, a lean
standard library, and **Assisted Memory Management (AMM)** — a memory model that
gives you manual control while the compiler ensures you never misuse memory.
```elo
fn main() {
    print('Elo, world!')
}
```

## Highlights

- **Assisted Memory Management** — manual memory management with compiler-enforced safety
- **Simple, expressive syntax** — designed to be readable and writable without ceremony
- **Efficient compilation** — [TCC](https://bellard.org/tcc/) backend for fast compilation, [Clang/LLVM](https://clang.llvm.org/) for high-performance optimized release builds
- **C interoperability** — seamless interop with C libraries

> **NOTE**: This language is not finished. Use this piece of software at your own risk. This software offers no warranty over itself. Read [LICENSE](./LICENSE) for more information.

## Documentation
Since this project is still a work in progress, we are still working on how the language will look and feel to the user. For now, we have a simple **design documentation** (design docs) meant to showcase what we expect to have implemented for Elo.

A lot of what is in the Design Docs is already working perfectly fine, but others are not even close to being done. Please read it with caution and be aware that this is still a preview. 

- Read the design docs [here](./docs/design). 

### Building from source-code

#### Compiler build instructions
The compiler is implemented in [**Rust**](https://rust-lang.org/).
You need to use [**cargo**](https://doc.rust-lang.org/stable/cargo/) to build the project.

- Windows
  * Run cargo to compile from source
    ```console
    > cd path\to\elo
    > cargo build --release
    ```

- Linux/MacOS
  * Run cargo to compile from source
    ```console
    $ cd ./path/to/elo
    $ cargo build --release
    ```

#### Runtime library build instructions
Read them [here](./rt/README.md).

---

> Copyright(C) 2026 Igor Ferreira, Marcio Dantas

> Licensed under [MIT License](https://choosealicense.com/licenses/mit/). Read [LICENSE](./LICENSE) for more information.
