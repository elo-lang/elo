# Elo's pattern matching

---

## Destructuring
- Destructure struct fields by separating each field by `,` (comma) in `let`, `var` or `for` statements:
```
struct Vector2 { x: int, y: int };

fn main() {
    var a = Vector2 { x: 10, y: 7 };
    let b = Vector2 { x: 56, y: 73 };
    var { x: x1, y: y1 } = a;
    let { x: x2, y: y2 } = b;

    let array = { a, b };
    for { x, y } in array {
        print(x)
        print(y)
    }
}
```

- Destructure tuples by order of appearance using the same tuple syntax in `let`, `var` or `for` statements:
```
fn main() {
    var xs = (1, 2, 3);
    let (a, b, c) = xs;

    let array = { xs };
    for (x, y, z) in array {
        print(x)
        print(y)
    }
}
```

## General

### `match` statement
- Check for pattern matchings using the `match` statemnt:
```
fn foo(): int!str { ... }

fn main() {
    match foo() {
        ok x {
            print('ok with {x}')
        }
        fail x => print('failed with {x}')
    }
}
```

### `if-match` statement
- Use `if-match` statements to match only one pattern variant:
```
fn foo(): int!str { ... }

if foo() match ok o {
  print("ok with {o}");
} else => print("something else");
```

```
fn bar(): int? { ... }

if 78 match x? {
    print("optional is present: {o}");
}
```
