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

Define constants using the keyword `const`:
```
const PI: float = 3.1415
```

## Memory management (not guaranteed to be something final)
Elo (may) use manual memory management **with assistance**.

Cases:

### Memory not freed
```
fn main() {
  let a = allocate(10) # Allocate 10 bytes
}
```

This code does not compile because the memory is never freed.
Error:
```
error: dynamic memory not freed 
2 |  let a = allocate(10) # Allocate 10 bytes
             ^-----------
             dynamic memory allocation here
             help: add `defer free(a)` after this line
```

### Memory freed twice
```
fn main() {
  let a = allocate(10) # Allocate 10 bytes
  defer free(a)
  defer free(a)
}
```

This code does not compile because the memory is never freed.
Error:
```
error: dynamic memory freed twice 
4 |   defer free(a)
            ^------
            dynamic memory freed here
            help: remove this line
```

### Memory used after free:
```
fn main() {
  let a = allocate(10) # Allocate 10 bytes
  free(a)
  print("{}", a)
}
```

This code does not compile because the memory is never freed.
Error:
```
error: dynamic memory freed twice 
4 |   print("{}", a)
                  ^
                  dynamic memory used here
                  help: use `free(a)` after it's used
```

## Code examples
These are some code snippets we think should be valid
in the final version of Elo. (these don't work right now)

### Hello World
```
fn main() {
    print("Hello World")
}
```

### Truth machine
```
fn main() -> int? {
    let i = input();
    while i.int()! == 1 {
        print(1)
    }
}
```