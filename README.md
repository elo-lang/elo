<h1 align="center">Elo</h1>

<p align=center><strong><em>Building software intuitively.</em></strong></p>

> **Elo** is a systems compiled programming language
> designed to allow developers enjoy software development
> through a **simple syntax**, **debloated standard library**
> and an **intuitive experience** in general.

## Getting started

### Development
Elo is in its early stages of development, we are still brainstorming a lot.
This repository contains the source code of Elo's compiler.

### Design docs
In this repository, there is a Design documentation which specifies
Elo programming language in a simple way. If you're interested, consider [reading it](./docs/design).

### Building from source
> **WARNING**: Elo is not a finished language. There's no warranty of this piece of software. **Use it at your own risk**.

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

---

> By [Igor Ferreira](https://github.com/igotfr), [Marcio Dantas](https://github.com/marc-dantas)

> Licensed under MIT License. Read [LICENSE](./LICENSE) for more information.
