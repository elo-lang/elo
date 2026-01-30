## Introduction

<p align="center">
    <img src="../assets/elo-logo-improved-contrast.png" width="400" />
</p>

Elo is a compiled systems programming language for computers.

The language is called Elo because it means "chain link" in Portuguese which is
closely related to this language's purpose as a tool. A strong and reliable piece
of your project, but still intentional and reliable.

We chose the slogan "_Building software intuitively_" to reflect the clear intent
to be a robust but intuitive programming language.

## History and goals
I have been creating this language with my friend Igor Ferreira for more than a year now.
It is far from being finished but we already have a prototype that works and we are building from there.

We started this project when we saw about the Dada programming language,
which is (or at least was) maintained by some Rust developers trying to make a language as safe
as Rust, but with a DX (developer experience) better than Rust's, which is pretty bad.

But since Dada is not really a working language and it seems like people stopped updating
and/or contributing to it, it's safe to consider it as a practically abandoned project (for now).

We also noticed that Dada's developers recently changed the direction the language was going:
from a "general purpose systems language" to
"an experimental new programming language for building WebAssembly components"
(taken from their website)

With that, we decided that we were going to make a language of our own with similar
principles but with our own taste and thoughts.

Let's take the DX more seriously than a lot of modern languages do.

The thing is that in most cases, an "easy" or "intuitive" language comes with a lot of problems,
like speed in interpreted and/or garbage-collected languages and safety like in Zig
or C/C++.

We aim to make a simple, DX-focused language with safety and speed.

But this is not our slogan. The DX is a consequence of all the design choices
we make while developing this language. When we propose a design choice, we
try to always think how the developer will reason about that in a real-world
code and how it will work with all other features and quirks of the language.

By being careful when introducing systems and choices in any part of the language,
it's "automatically" making this language intuitive.
