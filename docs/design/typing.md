# Elo's typing specification

---

## Categorization and casting

### Categories
Types in Elo are categorized into 2 main categories and their corresponding subcategories:

1. **Compound types**
- pointer: `*T`, `*mut T`
- sequence: `{T}`, `{T; n}`
- sum type: `T?`, `O!E`
- dynamic: `[T]`, `[K:V]`, `string`
- function: `fn R(A, ...)`

1. **Primitive types**
- signed integer: `int` `i8` `i16` `i32` `i64`
- unsigned integer: `uint` `u8` `u16` `u32` `u64`
- floating-point: `float` `f32` `f64`
- textual: `char` `str`
- logic: `bool`

### Type casting
Elo's type-checker is meant to automatically infer implicit casts between types that are safe to cast automatically.

Automatic cast is allowed when converting:

1. Any unsigned with size X into signed with size >=2X
1. Any signed of size X into a floating-point with
   size >=X
1. Any unsigned of size X into any floating-point with
   size >=2X
1. Any logic into any integer
1. `char` into `u32`
1. `{char}` or `{char; _}` into `str` and vice-versa
1. `*mut T` into `*T`

Otherwise, you must use explicit cast or the cast is just not possible.

To make an explicit cast in Elo, use the `as` keyword:
```
foo as i32
bar as str
baz as u8
```


Automatic casting examples:
```
struct Person {
  name: str, age: i64
}

let x: u32 = 50
let p = Person { name: 'bob', age: x }; // auto cast rule #1
//                                 ^
```

```
struct Vector2 { x, y: f64 }

let x: u16 = 69
let y: u16 = 420
let v2 = Vector2 { x: x, y: y } // auto cast rule #3
//                    ^     ^
```

## Static types VS Dynamic types
Static types are all stack-allocated. That means their lifetime is defined by their scope and have fixed size.
1. Dynamic types are heap-allocated. Their lifetimes are arbitrary and may have different size during runtime.

All the static types are the primitives.

The list of all dynamic types is the following:
  * Dynamic array: `[T]`
  * Growable and mutable string (string builder): `string`
  * Hash map: `[K:V]`
  * _We plan to add more types to this list, but for now this is what
    we see as the most needed ones_.

## Pointers
In Elo, pointers are raw addresses that point to specific part of memory, containing a value of a specified type (`*T`).

- You dereference a pointer using the unary operator `*`:
```
*pointer
```

- Get the pointer from a value at runtime using the `&` unary operator:
```
&value
```

Learn more about how Elo manages memory in [this document](./amm.md).

## Helper Types / Sum types

### Definitions
'Helper type' is a name we came up with to
describe this specific set of types to "help"
the user design their data architecture more
clearly and efficiently, such as using success/error
wrappers or optional types.

This kind of type is often part of a group called [Container Types](https://en.wikipedia.org/wiki/Container_(abstract_data_type)).
They are also called [Sum Type](https://en.wikipedia.org/wiki/Algebraic_data_type) as a broader definition.

---

Here's a simple table comparing the helper types
syntax between different programming languages and the
syntax we're willing to implement in Elo.

Legend:
- `O`: Type when OK
- `E`: Type when error
- `T`: Any type

| Description          | Zig       | Swift              | Rust           | Elo   |
|----------------------|-----------|--------------------|----------------|-------|
| Optional type        | `?T`      | `optional<T>`      | `Option<T>`    | `T?`  |
| success/fail wrapper | `E!O` **  | `throws(E) -> O`   | `Result<O, E>` | `O!E` |


> ** In Zig you can't put types that are different from `anyerror`, which is inherently different from the concept in other languages including Elo.

### Usage
Elo's Helper types/Sum types have a pretty straigh-forward way to be used. The next topics will describe each one of them in more detail.

### Optional value
Optional values (`T?`) are basically invisible in terms of how they appear in code as values.

- Create a variable that contains an optional type:
```
let o: int? = 10;
```

> The value `10` in this variable is not an `int`. Actually it is an `int?`. But syntactically they appear the same.

> It is like that to avoid the overhead of always specifying that it is an optional variant everytime you want to use it.

- To specify the absence of the optional value, use the keyword `none`:
```
let o: int? = none;
```


- Passing an optional value into something that expects the inner type will not work:
```
fn foo(a: int) { ... }

let x: int? = 10;
foo(x) // error
```

- To get the inner type from the optional value, you can use the `!` operator.
```
fn foo(a: int) { ... }

let x: int? = 10;
foo(x!) // fine if x is not none
```

> The quirk about the `!` operator is that if the optional value being used is actually `none`, the program will crash.

> It is similar to the [`.unwrap()` function in Rust programming language](https://doc.rust-lang.org/rust-by-example/error/option_unwrap.html).

- To safely handle the possibilities of the optional value, use [pattern matching](./patmatching.md).

### Result state value
Result state values (`O!E`) are used to describe a value that means the result of an operation that can fail or not.

Normally, you would use this type as the return of a function that may fail.

- Create a function that returns a result state value:
```
fn is_10(i: int): int!str {
    if i == 10 {
        ret i
    } else {
        ret 'sorry, i is not 10'
    }
}
```

> The compiler automatically detects which type you are returning and assigns it to the designated variant of the result state.

> If the type after `ret`/`return` is the one specified by the OK variant of the result state, it assigns it to the OK variant and returns. The same for the error variant. Otherwise it is an error.

- Extract the inner OK variant of the result state using the `!` operator.
```
let x = 10;
let r = is_10(x)!; // fine if the return is OK variant
```
> The quirk about the `!` operator is that if the result state value being used is the error variant, the program will crash.

> It is similar to the [`.unwrap()` function in Rust programming language](https://doc.rust-lang.org/rust-by-example/error/option_unwrap.html).
