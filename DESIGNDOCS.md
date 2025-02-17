
# Elo Language Design Docs
This document is meant to experiment with the language, any changes made in this file are not certain.

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

fn my_function_using(using Color) {
    # Inside this block of code, r, g, b, and a are valid variables
    # with their respective values
}

let c = Color { r: 0, g: 0, b: 0, a: 1 }
my_function_using(c)
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

Structs can also be ordered structs, with names replaced by numbers and defined in order:
```
struct Vector2(int, int)
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

Each variant can store it's own Struct, being it a named struct or an ordered struct:
```
enum Figure {
    Square(usize),
    Rectangle {
        width: usize,
        height: usize
    },
    Triangle {
        base: usize,
        height: usize
    }
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
const PI = 3.1415
```

### Memory management
Elo uses manual memory management **with assistance**.

Cases:

#### Memory not freed
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

#### Memory freed twice
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

#### Memory used after free:
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


