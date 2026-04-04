# Elo's pattern matching

---

## Destructuring

Destructuring allows you to unpack values from structs, tuples and sequences directly into named bindings.

### Structs

Destructure struct fields using `{ field: binding, ... }` syntax in `let`, `var` or `for` statements:

```
struct Vector2 { x, y: int }

fn main() {
    var a = Vector2 { x: 10, y: 7 }
    let b = Vector2 { x: 56, y: 73 }

    var { x: x1, y: y1 } = a
    let { x: x2, y: y2 } = b

    let array = { a, b }
    for { x, y } in array {
        print('\(x)')
        print('\(y)')
    }
}
```

> **NOTE**: If the binding name matches the field name, you can use the shorthand `{ x, y }` instead of `{ x: x, y: y }`.

### Tuples

Destructure tuples by position using `(a, b, ...)` syntax in `let`, `var` or `for` statements:

```
fn main() {
    var xs = (1, 2, 3)
    var ys = (2, 3, 4)

    let (a, b, c) = xs

    let array = { xs, ys }
    for (x, y, z) in array {
        print('\(x)')
        print('\(y)')
    }
}
```

---

## Match statement

Use `match` to exhaustively check all possible patterns of a value. Every possible variant must be handled, either explicitly or with an `else` catch-all arm.

### Matching result state values (`O!E`)

```
fn foo(): int!str { ... }

fn main() {
    match foo() {
        ok x {
            print('ok with \(x)')
        }
        fail e => print('failed with \(e)')
    }
}
```

- `ok x` binds the success value to `x`
- `fail e` binds any error value to `e`
- `fail Error.Variant` matches a specific error variant

Example with a custom error enum:

```
enum Error { NotFound, Unauthorized, Timeout }

fn fetch(): str!Error { ... }

fn main() {
    match fetch() {
        ok x                => print('got: \(x)')
        fail Error.NotFound => print('not found')
        else                => print('some other error')
    }
}
```

### Matching optional values (`T?`)

```
fn bar(): int? { ... }

fn main() {
    match bar() {
        some x => print('value is \(x)')
        none => print('no value')
    }
}
```

### Matching enums

```
enum Direction { North, South, East, West }

fn main() {
    let d = Direction.North

    match d {
        Direction.North => print('going north')
        Direction.South => print('going south')
        Direction.East  => print('going east')
        Direction.West  => print('going west')
    }
}
```

### The `else` arm

Use `else` as a catch-all for any unhandled patterns:

```
match foo() {
    ok x => print('ok: \(x)')
    else => print('something else')
}
```

> **NOTE**: `else` must always be the last arm in a `match` block.

---

## If-match statement

Use `if-match` when you only care about one specific pattern variant, without needing to handle all cases.

### With result state values:

```
fn foo(): int!str { ... }

if foo() match ok o {
    print('ok with \(o)')
} else => print('something else')
```

### With optional values:

```
fn bar(): int? { ... }

if bar() match some o {
    print('optional is present: \(o)')
}
```

> **NOTE**: The `else` branch in `if-match` is optional. If omitted, unmatched cases are simply skipped.
