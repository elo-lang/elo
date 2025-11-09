
## Elo's memory management

Elo provides **manual memory management** with **compiler-assisted safety**.
The goal is to give programmers full control over dynamic memory while
preventing common mistakes like leaks, double-frees, and
use-after-free — all without introducing complex logic or implicit deallocation.

### Core Principles

In Elo, you never directly allocate memory. You create data.
Memory is a consequence.

Elo treats heap values as constructed, not allocated — every heap
value begins life fully formed, and ends only when the programmer says so.

- **Manual allocation and deallocation**:
  All dynamic memory must be explicitly freed by the programmer.

- **Heap is constructed, not allocated**:
  Heap memory is created automatically when a value of a dynamic
  type is instantiated. There is no `malloc()` in Elo.

- **No implicit deallocations**:
  The compiler never inserts automatic deallocation. Every drop is explicit.

- **Safety enforcement**:
  The compiler tracks heap allocations and their aliases,
  issuing errors if misuse is detected.

This is possible because all possible dynamic types are tracked by the compiler all the time.
You can't create custom dynamic types without using the base ones. This way the compiler always knows
about all the dynamic allocation in the program.

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

### Function Parameters and Drop Permission
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
