## Elo's memory management

### Introduction
In this language, we are introducing a new memory management system called
AMM as an attempt to minimize the DX harms that the Ownership model from Rust
and the new programming language
[Dada](https://dada-lang.org) (which are one of the inspirations for Elo, see the [Introduction](./introduction.md)).

AMM stands for Assisted Memory Management.

Basically, AMM is manual memory management. But the compiler assists you
to not commit any harmful mistakes.

The compiler keeps track of all the dynamic memory you allocate in the code
and enforces you to get rid of its allocation at some point.

### Problems
But this system comes with a cost, since the compiler should know what is dynamic memory
so it keeps track of it properly, that means the language itself should contain
the necessary information about if the value you're creating is dynamically allocated
or static (stack-allocated).

In contrast, Ownership model is more generic. Every value contains an ownership
that is passed or borrowed. Then finally deleted at the end of its scope. And can only be
fully shallow-copied with an implementation of `Clone` and `Copy` in case of Rust.
This way it's very simple to know which values are borrowed or moved where since
the rule is applied to every type and with these special `trait`s that allow
full cheap copy or not.

This is why Rust nor Dada needs information about if that value
has ownership or not. The compiler determines that all values have ownership
and what makes them copiable is just a `trait`.

Clearly, this system is not intuitive in a lot of ways for being too generic and
restricting how the program's logic should flow to ensure the ownership is not
violated. Even in the most stupid and nonsensical cases.

Dada tried to solve this trying to make these rules less rigid and changing
how the ownership is passed to be clearer for the user to understand what is
happening. For example:
- In Rust, move is done by default when passed by value
- In Dada, move is only done with a specific operator.
  Otherwise, it's always shallow copy.

Dada got really close with these solutions to ownership, but there are still
some flaws and it seems like this part of Dada's compiler is not even implemented
yet...

### How we try to solve them
Anyways, the way we figured out (for now) how the AMM system would work reasonably
better in terms of DX than Ownership was making this "flag" for dynamic
memory the types themselves, as part of the language's syntax.
The language comes with types like `Vec`, `String`, or even `HashMap`
as part of its core type system, not as an external structure implemented
somewhere else.

This is similar to scripting languages like Lua or JavaScript that have similar
types like `table` or `Object` in the same "primitive type" scope as
integers, floats, or strings so the garbage collector is able
to track them properly.

If this was implemented in C, we would have to track what pointers are pointing
to Heap and where they came from so the compiler knows you need to call `free()`
to this pointer. To do that we would have to mark `malloc` and all other functions
that return a pointer to heap as "special" with a magical check by the compiler,
which clearly is very inconsistent and counter-intuitive.  

The `free()` is also another problem. Just like `malloc`, we can't just make `free` a special function
that the compiler considers as a deallocator for the dynamic memory or that would
destroy the consistency of a normal function call.

To the `free` problem, we propose a new keyword/statement that has a specific behavior.
We called this `drop`, just like Rust. 

So, in a nutshell, the types like dynamic arrays, string builders or dynamic hashmaps are
just like normal types like `int`, `i32`, `size_t`, `double` etc. But they are tracked by the
compiler so you use this specific statement with the `drop` keyword so you ensure the
dynamic memory allocated for that type is deallocated.

### Core Principles
Now that you understand what this system is about and what it is trying to solve. Let's go through the details
of how this system is going to work.

AMM is basically RAII, but with manual lifetime control.

As described earlier, this is possible because you are limited to built-in dynamic types
that are tracked by the compiler all the time. You can't create custom dynamic types
without using the built-in ones. This way the compiler always knows about all the
dynamic allocation in the program.

### Dynamic Types and Initialization
As mentioned before, you are limited to a list of useful dynamic types that come with the language:

|Name|Type syntax|Initialization syntax (example)|Description|
|---|---|---|---|
|List|`[T]`|`[a, b, c, ...]`|Growable array|
|Map|`[K:V]`|`[k: v, k: v, ...]`|Growable dynamic hash-map|
|String|`string`|`"lorem ipsum"` *|String builder|

> \* Do not confuse the double quotes (`"`) and the single quotes (`'`). In Elo, double quoted string means a dynamic string and single quoted strings mean static strings.

When you construct any dynamic type, for example:
```
let x = [1, 2, 3]
```

What is happening is an allocation for the list (or other dynamic type) you're creating.

If you just instantiate this array and do nothing else. The compiler should raise an error telling you something like this:
```
list 'x' is never dropped.
please drop 'x' using `drop` statement.
```

Then, to make the code compile (with the _safecheck_ enabled), you must 'drop' the dynamic array:
```
let x = [1, 2, 3]
drop x;
```

But if, after you used `drop`, you try to refer to that same variable:
```
let x = [1, 2, 3]
drop x; // error: x is used after drop
print(x) // possible dangling pointer
```

the compiler should raise another error telling you something like this:
```
dynamic instance 'x' used after being dropped.
please modify instance's lifetime (delay the `drop`).
```

If you want to guarantee this dynamic instance is dropped at the exit of the scope,
use the `defer` keyword with the drop statement:

```
let list = [1, 2, 3]; // initialization
defer drop list; // delays the deallocation to the scope's exit
```

### Main Rules

1. Any dynamic instance created inside a scope must be 'dropped' in that same scope.

1. Dynamic instance referenced after being 'dropped' is not allowed.

1. 'Dropping' a dynamic instance for the second time is not allowed.

1. Aggregate types containing fields with dynamic types are themselves dynamic types as well.
    ```
    struct Person { name: string, age: int }
    //                    ^^^^^^ dynamic type
    ...
    let p = Person { name: "John", age: 30 }  // Person is a dynamic type
    ```

1. Any pointer to or direct assignment to a dynamic type makes a shallow copy;
   all copies refer to the same underlying allocation. ([**shallow copy**](https://en.wikipedia.org/wiki/Object_copying#Shallow_copy))

1. Dropping aliases to dynamic instance is not allowed, but dropping the original object must guarantee their references are never used after that:
    ```
    let a = "foo"
    let b = a
    drop a       // error, b is used after this drop
    print(b)     // here
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

1. Pointers to dynamic types do not own the original memory. That's why they can't be dropped even when dereferenced.

1. When dereferencing pointers to dynamic instances, the operation returns a **shallow copy** of the original instance.

### Permission
Owning a dynamic object in Elo means you have **permission**, that means the
compiler is going to check if the owner scope handles the dynamic object until the end of it.

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

Now that you understand the basics of the possible states, let's introduce a new state for the **permission transfer**:

- **Escaped**: Dynamic instance that got its permission transferred somewhere else.

Now, to transfer the drop permission to another reference (escape a variable), you use the **return** statement.

To transfer an object created inside a function to its outer context, you just return it:
```
fn get_array(): [int] {
    let a = [1, 2, 3]; // for now, 'a' owns the memory.
    return a;
    // after the return, 'a' gets its drop permission transferred to the caller
}

fn main() {
    let x = get_array();
    print(x)
    // must do the `drop` now because permission was transferred upward toward main function ('x' variable)
    drop x;
}
```

Permission transfer only happens by extending the dynamic object's lifetime.
This is why you cannot transfer permission **into** something, or that would
cause its lifetime to implicitly get shorter, which may cause confusion in most cases.

Following the normal AMM rules, outside the function you must deallocate it at
some point of the owner scope or the compilation will not succeed.

But instead of 'dropping' the transferred variable you can escape it again:

```
fn get_array(): [int] {
    let a = [1, 2, 3]; // for now, 'a' owns the memory.
    return a;
    // after the return, 'a' gets its drop permission transferred to get_array2
}

fn get_array2(): [int] {
    let a = get_array(); // the original array gets transferred to this scope
    return a;
    // after the return, the original gets its drop permission transferred again,
    // but now to main
}


fn main() {
    let x = get_array();
    // Must handle it somehow
    drop x;
}
```
