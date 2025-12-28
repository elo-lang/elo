## Elo's memory management

Elo provides **manual memory management** with **compiler-assisted safety**.
The goal is to give programmers full control over dynamic memory while
preventing common mistakes like leaks, double-frees, and
use-after-free — all without introducing complex logic or implicit deallocation.

### Core Principles

- **Manual deallocation**:
  All dynamic memory must be explicitly freed by the programmer.

- **Heap is constructed, not allocated**:
  Heap memory is created automatically when a value of a dynamic
  type is instantiated. There is no `malloc()` in Elo.

- **No implicit dynamic deallocations**:
  The compiler never inserts automatic deallocation. Every "free" is explicit.

- **Safety enforcement**:
  The compiler tracks heap allocations and their aliases,
  issuing errors if misuse is detected.

This is possible because you are limited to built-in dynamic types that are tracked by the compiler all the time.
You can't create custom dynamic types without using the built-in ones. This way the compiler always knows
about all the dynamic allocation in the program.

### Dynamic Types & Heap construction
As mentioned before, you are limited to a list of useful dynamic types that come with the language:

|Name|Type syntax|Construction syntax (example)|
|---|---|---|
|Dynamic Array|`[T]`|`[a, b, c, ...]`|
|Dynamic Hashmap|`[K:V]`|`[k: v, k: v, ...]`|
|Dynamic String|`string`|`"lorem ipsum"` *|

> \* Do not confuse the double quotes (`"`) and the single quotes (`'`). In Elo, double quoted string means a dynamic string and single quoted strings mean static strings.

When you construct any dynamic type, for example:
```
let x = [1, 2, 3]
```

What is happening is a heap allocation for the dynamic array (or other dynamic type) you're creating.

If you just instantiate this array and do nothing else. The compiler should raise an error telling you something like this:
```
dynamic instance 'x' is never dropped.
please deallocate the memory using `drop` keyword.
```

Then, to make the code compile (with the _safecheck_ enabled), you must 'drop' the dynamic array:
```
let x = [1, 2, 3]
drop x;
```

But if, after you used `drop`, you try to refer to that same variable:
```
let x = [1, 2, 3]
drop x;
print(x) # possible dangling pointer
```

the compiler should raise another error telling you something like this:
```
dynamic instance 'x' referenced after being dropped.
please modify instance's lifetime (delay the `drop`).
```

This is the main concept around AMM.

### Main Rules

1. Any dynamic instance created inside a scope must be 'dropped' in that same scope.

1. Dynamic instance referenced after being 'dropped' is not allowed.

1. 'Dropping' a dynamic instance for the second time is not allowed.

1. Composite types containing fields with dynamic types are themselves dynamic types as well.
    ```
    struct Person { name: string, age: int }
    //                    ^^^^^^ dynamic type
    ...
    let p = Person { name: "John", age: 30 }  // Person is a dynamic type
    ```

1. Any reference or direct copy creates a shallow alias; all copies refer to the same underlying allocation. ([**shallow copy**](https://en.wikipedia.org/wiki/Object_copying#Shallow_copy))

1. Dropping aliases to dynamic instance is not allowed, but dropping the original object invalidates all aliases:
    ```
    let a = "foo"
    let b = a
    drop a       // invalidates both a and b
    print(b)     // error
    ```

1. Pointers to dynamic instances are allowed:
    ```
    fn add_number_and_print(xs: *mut [int], number: int) {
        xs.push(number);
        print(number);
    }

    fn main() {
      var xs = [1, 2, 3];
      add_number_and_print(&xs, 10);
      drop xs;
    }
    ```

1. Pointers to dynamic types aren't considered dynamic types themselves. That's why they can't be dropped even when dereferenced.

1. When dereferencing pointers to dynamic instances, the operation returns a **shallow copy** of the original instance.

### Drop permission

Elo introduces an analogue to the **ownership** model. But instead of ownership of the object itself, it's the ownership of the raw memory allocation.

Owning a dynamic object in Elo means you have **drop permission**, that means you are able to use the `drop` keyword on it (deallocate the underlying heap memory).

Before anything else, let's undestand the possible states of a dynamic instance in Elo:
- **Valid**: Normal dynamic value, can be used and abused.
- **Dropped**: Invalid dynamic value, can't be used in any way.

To get a **valid** dynamic instance, you just have to create it:
```
let valid = [1, 2, 3];
```

To turn that **valid** instance into an **invalid** instance, it's very easy as well:
```
drop valid; // from now on, `valid` is invalid.
```

Now that you understand the basics of the possible states, let's introduce a new state for the **drop permission transfer**:

- **Escaped**: Dynamic instance that got its drop permission transferred somewhere else.

Now, to transfer the drop permission to another reference (escape a variable), you use **functions**.

To get a variable's permission to get transferred into another variable, use the function arguments with the `!` syntax:
```
fn transfer_into_param(!param: [int]) {
    print(param);
    drop param;
}

fn main() {
    let x = [1, 2, 3];
    // Here I lose the drop permision to the original 'x'
    transfer_into_param(x);
    // And transfer it to the 'param' parameter inside the function.
}
```

After a permission transfer into a function, you don't need to 'care' about the variable anymore. The function is now in charge of handling it.

Following the normal AMM rules, inside the function you must deallocate it at some point of the execution of the function or you will potentially cause a memory leak.

But instead of 'dropping' the transferred variable you can escape it again into another function as well:

```
fn transfer_into_param2(!param: [int]) {
    print(param);
    drop param;
}

fn transfer_into_param(!param: [int]) {
    transfer_into_param2(param);
    // now I don't need to do "drop" here anymore
}

fn main() {
    let x = [1, 2, 3];
    transfer_into_param(x);
    // Here i still don't care anymore, permission was transferred.
}
```

Since at some point after the transfer the `drop` must happen, it's safe to assume that after the function call to `transfer_into_param`, the variable x is not valid anymore:

```
fn main() {
    let x = [1, 2, 3];
    transfer_into_param(x);
    print(x) // error: at this point, 'x' must be dropped
}
```

To transfer an object created inside a function to its outer context, you just return it:
```

fn get_array(): [int] {
    let a = [1, 2, 3]; // for now, 'a' owns the memory.
    ret a;
    // after the return, 'a' gets its drop permission transferred to the caller
}

fn main() {
    let x = get_array();
    print(x)
    // must do the `drop` now because permission was transferred upward toward main function ('x' variable)
    drop x;
}
```
