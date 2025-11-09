# Elo's basic features
and differences from other programming languages

---

## Functions
> **NOTE**: Use the `//` syntax to create comments in Elo.
> (there are no multiline comments in Elo)

### Entry-point
The entry-point function (or "main function") is the function that starts the execution of a program.
A program in Elo should have a function called `main`. It should not return anything or accept any arguments.
```
fn main() {
    // code ...
}
```

### Usage
- Functions can be defined using the keyword `fn`:
```
fn foo(arguments) {
    code
}
```

- The arguments are expressed by the syntax `name: type` and separated by comma:
```
fn foo(bar: eggs, baz: spam) {
    code
}
```

- Arguments that have the same type may be expressed as `a, b, ...: type`, this way you don't repeat the type of the arguments for each name:
```
fn foo(bar, baz: spam) {
    code
}
```

- Return statements can be expressed using both `return` and `ret` keywords. Both options are valid Elo code:
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
Struct is a compound structure that has named fields. Each field must have a type.

- Define a structure using the keyword `struct`, according to this model:
```
struct Person (
    name: str,
    age: int
)
```

- You may use the same syntax of function arguments to compress fields that have the same type:
```
struct Vector2 (
    x, y: int
)
```

### Enumerations
Enumeration is a compound structure that contains variants that may hold a specific state or kind of something.

Define enumerations using the keyword `enum`:
```
enum Week (
    Sun,
    Mon,
    Tue,
    Wed,
    Thu,
    Fri,
    Sat,
)
```

### Usage
- Initialization of a struct is done following this model:
```
MyStruct { field: value, field: value }
```

- Access fields of an initialized struct like this:
```
instance.field
```

- Usage of an enum variant is done following this model:
```
Enum.Variant
```

## Variables and constants

### Variables
Variable is a named binding to a value in runtime.

- Define local **immutable** variables using the keyword `let`:
```
let x = 10
let y = 20
```

> **NOTE**: In Elo, writing a semicolon (`;`) after a statement is optional, unless you want 2 statements in the same line.

- Define local **mutable** variables using the keyword `var`:
```
var x = 10
var y = 20
```

### Constants
Constant is a named binding to a constant value, known at compile time.

- Define global constants using the keyword `const`:
```
const PI: float = 3.1415
```
