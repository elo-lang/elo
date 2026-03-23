## Elo's memory management

### Introduction

In this language, we are introducing a new memory management system called
AMM as an attempt to minimize the DX harms that the Ownership model from Rust
and the new programming language
[Dada](https://dada-lang.org) (which are one of the inspirations for Elo, see the [Introduction](./introduction.md)).

AMM stands for Assisted Memory Management.

The core idea is simple: dynamic instances are automatically freed at the end of
the scope they were created in. The compiler tracks all dynamic allocations and
ensures memory is always cleaned up — without requiring the programmer to manually
call any deallocation function, and without the rigidity of an ownership system.

### Problems with existing approaches

**Garbage collection** solves memory safety but introduces runtime overhead and
non-deterministic collection times, making it unsuitable for systems programming.

**Rust's ownership model** is powerful but too rigid — it restricts how programs
can naturally flow their logic to enforce ownership rules, even in cases where
the programmer's intent is obviously safe.

**Manual memory management (C/C++)** gives full control but offers no safety net,
leading to memory leaks, use-after-free bugs, and double frees.

AMM sits between these approaches: the control of manual memory management, the
safety of a compiler-enforced model, without the rigidity of ownership systems
or the overhead of a garbage collector.

### How it works

AMM is possible because Elo limits heap allocation to a fixed set of built-in
dynamic types. You cannot create custom heap-allocated types without using the
built-in ones. This way the compiler always knows exactly where every dynamic
allocation is in the program.

The built-in dynamic types are:

| Name | Type syntax | Initialization syntax | Description |
|---|---|---|---|
| List | `[T]` | `[a, b, c, ...]` | Growable array |
| Map | `[K:V]` | `[k: v, k: v, ...]` | Growable dynamic hash-map |
| String | `string` | `"lorem ipsum"` * | Dynamic string |

> \* Do not confuse double quotes (`"`) and single quotes (`'`). In Elo, double-quoted strings are dynamic `string` types and single-quoted strings are static `str` slices.

This is similar to how scripting languages like Lua or JavaScript treat types like
`table` or `Object` as first-class primitives, allowing the runtime to track them.
The difference is that Elo does this at compile time with zero runtime overhead.

### Automatic deallocation

When a dynamic instance goes out of scope, the compiler automatically inserts
the deallocation. The programmer never has to think about freeing memory:

```
fn main() {
    let xs = [1, 2, 3]
    print(xs[0])
} // xs is automatically freed here
```

This also applies to structs that contain dynamic fields — they are themselves
considered dynamic types and are freed when they go out of scope:

```
struct Person { name: string, age: int }
//                    ^^^^^^ dynamic field makes Person a dynamic type

fn main() {
    let p = Person { name: "John", age: 30 }
    print(p.age)
} // p.name (string) and p itself are freed here
```

### Shallow copies

Any assignment or passing of a dynamic type makes a **shallow copy** — both
variables refer to the same underlying allocation. The original scope is still
responsible for freeing the memory:

```
fn main() {
    let a = "Hello"
    let b = a        // b is a shallow copy, same memory as a
    print(b)         // fine
} // a is freed here, b was just a reference to it
```

Pointers to dynamic instances follow the same rule — they reference the original
memory and are never responsible for freeing it:

```
fn add_and_print(xs: *mut [int], number: int) {
    xs.push(number)
    print(number)
}

fn main() {
    var xs = [1, 2, 3]
    add_and_print(&xs, 10)
} // xs is freed here
```

### Extending lifetimes with `give`

By default, a dynamic instance lives until the end of the scope it was created in.
If you need it to live longer — for example, to assign it to something in an outer
scope — you use the `give` keyword:

```
fn main() {
    var g = Greeting { value: "placeholder" }
    if true {
        let s = "Hello world"
        g.value = give s   // s's lifetime is extended to match g's scope
    }
    print(g.value)         // fine, s lives until main exits
} // g (and its value) freed here
```

Without `give`, assigning a dynamic instance created in an inner scope to
something in an outer scope is a compile error — the compiler knows the instance
would be freed before it could be safely used:

```
fn main() {
    var g = Greeting { value: "placeholder" }
    if true {
        let s = "Hello world"
        g.value = s   // error: 's' does not live long enough
    }
}
```

`give` is cheap — it does not copy the data, it simply extends the lifetime of
the original instance. Both the inner and outer scopes can use it freely.

### Returning dynamic instances

Returning a dynamic instance from a function naturally extends its lifetime to
the caller's scope — no special syntax needed:

```
fn get_array(): [int] {
    let a = [1, 2, 3]
    ret a   // a's lifetime is transferred to the caller
}

fn main() {
    let x = get_array()
    print(x[0])
} // x is freed here
```

This can chain across multiple functions:

```
fn get_array(): [int] {
    ret [1, 2, 3]
}

fn wrap(): [int] {
    let a = get_array()
    ret a   // lifetime transferred again, to wrap's caller
}

fn main() {
    let x = wrap()
    print(x[0])
} // x is freed here
```

### Deep copies with `clone`

When you need a fully independent copy of a dynamic instance — one that can
outlive the original or be mutated independently — use `clone`:

```
fn main() {
    let a = "Hello"
    if true {
        let b = clone a   // b is a deep copy, independent from a
        b.push('!')
        print(b)          // "Hello!"
    }
    print(a)              // "Hello", unchanged
}
```

Unlike `give`, `clone` allocates new memory. Use it when you specifically need
two independent instances. It's usually preferrable to just use `give` when you need to extend a lifetime.
