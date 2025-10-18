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
    // Inside this block of code, c, r, g, b, and a are valid variables
    // with their respective values
    // c: Color
    // r: float
    // g: float
    // b: float
    // a: float
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

## Memory management

Elo provides **manual memory management** with **compiler-assisted safety**.
The goal is to give programmers full control over dynamic memory while
preventing common mistakes like leaks, double-frees, and
use-after-free — all without introducing ownership complexity
or implicit drops.

### Core Principles

In Elo, you never directly allocate memory. You create data.
Memory is a consequence.

Elo treats heap values as constructed, not allocated — every heap
value begins life fully formed, and ends only when the programmer says so.

- **Manual allocation and deallocation**:
  All dynamic memory must be explicitly freed by the programmer.

- **Heap is constructed, not allocated**: 
  Heap memory is created automatically when a value of a dynamic
  type is instantiated. There is no alloc() or pointer arithmetic.
  
- **No implicit deallocations**:
  The compiler never inserts automatic deallocation. Every drop is explicit.

- **Safety enforcement**:
  The compiler tracks heap allocations and their aliases,
  issuing errors if misuse is detected.

### Heap construction
Dynamic types are automatically placed on the heap when instantiated. Examples:

```
let s = "hello"       // dynamic string
let arr = [1, 2, 3]   // dynamic array
let map = ["a": 1]    // dynamic hashmap
```
* The heap is not exposed; you cannot manually allocate memory.

Composite types containing dynamic fields are themselves heap-allocated.

```
struct Person { name: string, age: int }
...
let p = Person { name: "John", age: 30 }  // heap allocation
```

### Aliasing and Shallow Copies
* Copying a heap variable creates a shallow alias; all copies refer to the same underlying allocation.
* Dropping any alias invalidates all others.

Example:
```
let a = "foo"
let b = a
drop a       // invalidates both a and b
print(b)     // compile-time error: use after drop
```

* The compiler tracks alias sets to enforce this rule.

### Function Parameters and Drop Permission (`!` operator)
Elo introduces a function-level syntax for granting **drop permission**:
* `!` before a parameter name marks it as dropped after the function.
* Only parameters marked with `!` can be dropped by the callee.
* After the call, the caller’s variable and all its aliases are invalidated.

Example:
```
fn consume(!s: string) {
    drop s   // allowed because ! grants permission
}

fn main() {
    let x = "hello"
    consume(x)    // x invalid after call
    print(x)      // error: invalid after permission transfer
}
```

#### Rules
1. **Explicit drop required**: Every path through a function must either `drop`
   or forward the `!` parameter to another `!` parameter.
2. **Compiler error if unused**:
   If a `!` parameter reaches the end of a function without
   being dropped, the compiler emits an error.
3. Multiple consumable parameters are supported:
   ```
   fn consume(!a: string, !b: [int]) {
       drop a
       drop b
   }
   ```
4. The return value of any function **always** escapes
   the permission to the caller, unless the value is a `!` parameter.

### Return Values and Escaping
* Any non-`!` dynamic type returned from a function automatically transfers drop permission to the caller.

Example:
```
fn make_string(): string {
    let s = "hello"
    ret s       # permission escapes to caller
}

fn main() {
    let x = make_string()
    drop x      # caller now responsible
}
```

### Flow and Lifecycle
Heap variables exist in one of these states:
* Valid: Can be read, copied, or dropped.
* Dropped: Deallocated; any use is an error.
* Escaped: Returned or passed to another ! parameter; caller becomes responsible for dropping.

### Summary
In Elo, dynamic memory is constructed, not allocated.
Programmers are fully responsible for freeing memory.
Aliases are automatically invalidated on drop,
and consumable parameters (`!`) make explicit
which values a function may deallocate.

The system combines manual control with compiler-assisted safety,
preserving predictability, stack-like usage, and minimal syntax complexity.

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
