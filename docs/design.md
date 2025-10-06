# Elo Language Design Docs
This document is meant to be a brainstorming whiteboard, any information in this document are not certain.

This file is not meant to be any kind of documentation or reference to the language.

This file has the only goal to show its "features" or at least its differences from other programming languages.

## Functions

Functions can be defined using the keyword `fn`:

```
fn my_function(...) {
    ...
}
```

The arguments are expressed by the syntax `name: type` and separated by comma:
```
fn my_function_with_arguments(foo: int, bar: float) {
    ...
}
``` 

Arguments that have the same type may be expressed as `a, b, ...: type`, this way you don't repeat the type of the arguments for each name:
```
fn my_function_with_arguments(foo, bar: float) {
    ...
}
```

Compound structures such as `struct` can be destructured as function arguments using the keyword `using`:
```
struct Color {
    r, g, b, a: float
}

fn my_function_using(using c: Color) {
    # Inside this block of code, c, r, g, b, and a are valid variables
    # with their respective values
    # c: Color
    # r: float
    # g: float
    # b: float
    # a: float
}

# Instantiate structures using this syntax:
let c = Color { r: 0, g: 0, b: 0, a: 1 }
my_function_using(c)
```

Return statements can be expressed using both `return` and `ret` keywords. Both options are valid Elo code:
```
fn foo(): int {
    ret 0
}

fn foo(): int {
    return 0
}
```

## Compound Structures

### Structs
Struct is a compound structure that has named (or not) fields. Each field must have a type.

Define a structure using the keyword `struct`:
```
struct Vector2 {
    x: int,
    y: int
}
```

You may use the same syntax of function arguments to compress fields that have the same type:
```
struct Vector2 {
    x, y: int
}
```

### Enums
Enumeration is a compound structure that contains variants that may hold a specific state or kind of anything.

Define enumerations using the keyword `enum`:
```
enum Week {
    Sun,
    Mon,
    Tue,
    Wed,
    Thu,
    Fri,
    Sat,
}
```

## Typing
Elo has 2 different kinds of types that can be used arbitrarily by the user:
1. Static types (stack-allocated)
1. Dynamic types (head-allocated)

The list of all static types is the following:
- Signed integers: `int`, `i8`, `i16`, `i32`, `i64`
- Unsigned integers: `uint`, `u8`, `u16`, `u32`, `u64`
- Boolean: `bool` 
- Function-pointer: `R fn(A, ...)`
- Floating-point: `float`, `f32`, `f64`
- Character: `char`
- Sequences: `(T, ...)`, `str`, `{T; N}`
- Pointer and slice: `*T`, `*{T}`

The list of all dynamic types is the following:
- Dynamic array: `[T]`
- Growable and mutable string: `string`
- Hashmap: `[K:V]`
- _We plan to add more types to this list, but for now this is what
  we see as the most needed ones_.

Static types may also be called "primitive types", since they
do not need special memory management or advanced checking by
the Elo's compiler.

## Variables and constants
Variable is a named binding to a value in runtime.

Constant is a named binding to a constant value, known at compile time.

Define local immutable variables using the keyword `let`:
```
let x = 10
let y = 20
```

Define local mutable variables using the keyword `var`:
```
var x = 10
var y = 20
```

Define global constants using the keyword `const`:
```
const PI: float = 3.1415
```

## Memory management (not guaranteed to be something final)
Elo (may) use manual memory management **with assistance**.

Cases:

### Memory not freed
```
fn main() {
  let a = [1, 2, 3] # Allocate simple dynamic array
}
```

This code does not compile because the memory is never freed.
Error:
```
error: dynamic memory not freed 
2 |  let a = [1, 2, 3] # Allocate simple dynamic array
             ^-----------
             dynamic memory allocation here
             help: add `defer { drop a }` after this line
```

### Memory freed twice
```
fn main() {
  let a = "Hello world" # dynamic string
  drop a
  drop a
}
```

This code does not compile because the memory is freed twice.
Error:
```
error: dynamic memory freed twice 
4 |   drop a
      ^-----
      dynamic memory freed here
      help: remove this statement
```

### Memory used after free:
```
fn main() {
  let ages = ["john": 21, "mary": 19] # Simple hashmap
  drop ages
  print('{a}')
}
```

This code does not compile because the memory is used after it's been freed.
Error:
```
error: dynamic memory used after deallocation 
4 |   print('{a}')
              ^
              dynamic memory used here
              help: use `drop` only after all uses
```

## Helper Types *

Here's a simple table comparing the **helper types** \*
syntax between different programming languages and the
syntax we're willing to implement in Elo.

| Description          | Zig       | Swift              | Rust           | Elo   |
|----------------------|-----------|--------------------|----------------|-------|
| Optional type        | `?T`      | `optional<T>`      | `Option<T>`    | `T?`  |
| success/fail wrapper | `E!O` **  | `throws(E) -> O`   | `Result<O, E>` | `O!E` |

> \* 'Helper type' is a name we came up with to
> describe this specific set of types to "help"
> the user design their data architecture more
> clearly and efficiently, such as using success/error
> wrappers or optional types. They are often part of a
> group called [Container Types](https://en.wikipedia.org/wiki/Container_(abstract_data_type))

> ** In Zig you can't put types that are different from `anyerror`, which is inherently different from the concept in other languages including Elo.

## Code examples
These are some code snippets we think should be valid
in the final version of Elo. (these don't work right now)

### Hello World
```
fn main() {
    print('Hello World')
}
```

### Truth machine
```
fn main() {
    let i = input();
    while i.int()! == 1 {
        print(1)
    }
}
```
