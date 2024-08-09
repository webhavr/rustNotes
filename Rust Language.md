## Links

- [Links](#links)
- [Pending](#pending)
- [Rust Benefits](#rust-benefits)
  - [Compile time memory safety](#compile-time-memory-safety)
  - [No undefined runtime behavior](#no-undefined-runtime-behavior)
  - [Modern language features](#modern-language-features)
- [Language Basics](#language-basics)
  - [Rust Primitives](#rust-primitives)
  - [Crates](#crates)
  - [Item](#item)
  - [Module](#module)
  - [Macros](#macros)
  - [Panic](#panic)
  - [References](#references)
  - [Traits](#traits)
    - [Debug Trait](#debug-trait)


| Description        | Link                                                         |
| ------------------ | ------------------------------------------------------------ |
| Rust Official Book | [Here](https://doc.rust-lang.org/book/title-page.html)       |
| Rust by Example    | [Here](https://doc.rust-lang.org/rust-by-example/index.html) |
| std::fmt           | [Here](https://doc.rust-lang.org/std/fmt/)                   |
| Primitives         | [Here](https://doc.rust-lang.org/std/index.html#primitives)  |
| Modules            | [Here](https://doc.rust-lang.org/std/index.html#modules)     |
| Macros             | [Here](https://doc.rust-lang.org/std/index.html#macros)      |
| Keywords           | [Here](https://doc.rust-lang.org/std/index.html#keywords)    |
| Macros             | [Here](https://doc.rust-lang.org/book/ch19-06-macros.html)   |


## Pending
* [ ] unsafe
* [ ] Defined behavior for everything
* [ ] Array Data Type
* [ ] Vector Data Type

## Rust Benefits

### Compile time memory safety
* Whole classes of memory bugs are prevented at compile time
* No uninitialized variables.
* No double-frees.
* No use-after-free.
* No NULL pointers.
* No forgotten locked mutexes.
* No data races between threads.
* No iterator invalidation.

### No undefined runtime behavior
* What a Rust statement does is never left unspecified
* Array access is bounds checked.
* Integer overflow is defined (panic or wrap-around).

### Modern language features
* Enums and pattern matching.
* Generics.
* No overhead FFI.
* Zero-cost abstractions.
* Great compiler errors.
* Built-in dependency manager.
* Built-in support for testing.
* Excellent Language Server Protocol support


## Language Basics
### Rust Primitives
* [Link](https://doc.rust-lang.org/std/index.html#primitives)
* ![Summary Image](https://drive.google.com/file/d/1JBkldbsViCtjVAqQ9zvPAvvKFlYSdIgv/view?usp=sharing)

### Crates
* [Rust Book Ref](https://doc.rust-lang.org/reference/crates-and-source-files.html)
* A crate is a unit of compilation and linking, as well as versioning, distribution, and runtime loading.
* Each compilation processes a single crate in source form, and if successful, produces a single crate in binary form: either an executable or some sort of library.
* The Rust compiler is always invoked with a single source file as input, and always produces a single output crate
* A Rust source file describes a module
* Each source file contains a sequence of zero or more Item definitions

### Item
* [Rust Book Ref](https://doc.rust-lang.org/reference/items.html)
* An item is a component of a crate.
* Items are organized within a crate by a nested set of modules.
* Every crate has a single "outermost" anonymous module; all further items within the crate have paths within the module tree of the crate.
* Items are entirely determined at compile-time, generally remain fixed during execution, and may reside in read-only memory.
  
### Module
* [Rust Book Ref](https://doc.rust-lang.org/reference/items/modules.html)
* A module is a container for zero or more items.
* A module item is a module, surrounded in braces, named, and prefixed with the keyword `mod`

### Macros
* [Rust Book Ref](https://doc.rust-lang.org/book/ch19-06-macros.html)
* A function signature must declare the number and type of parameters the function has. 
* Macros, on the other hand, can take a variable number of parameters: we can call `println!("hello")` with one argument or `println!("hello {}", name)` with two arguments.
* The downside to implementing a macro instead of a function is that macro definitions are more complex than function definitions because you’re writing Rust code that writes Rust code
* Similar to Templates in C++.

### Panic
* [Link](https://doc.rust-lang.org/std/macro.panic.html)
* Panics the current thread.
* This allows a program to terminate immediately and provide feedback to the caller of the program.

### References
* [Link](https://doc.rust-lang.org/std/primitive.reference.html)
* [Rust Book Ref](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)
* A reference represents a borrow of some owned value. You can get one by using the `&` or `&mut` operators on a value, or by using a `ref` or `ref mut` pattern.
* A reference is just a pointer that is assumed to be aligned, not null, and pointing to memory containing a valid value of `T`
* We call the action of creating a reference borrowing. As in real life, if a person owns something, you can borrow it from them. When you’re done, you have to give it back. You don’t own it.

### Traits
* [Link](https://doc.rust-lang.org/rust-by-example/trait.html)
* A trait is like an interface that data types can implement. 
* When a type implements a trait it can be treated abstractly as that trait using generics or trait objects.
* A trait is a collection of methods defined for an unknown type: Self. They can access other methods declared in the same trait.
* Traits can be implemented for any data type
* Analogous to Base class in C++.

#### Debug Trait
* [Link](https://doc.rust-lang.org/std/fmt/trait.Debug.html)
* This is a standard library trait used for formatting values in a human-readable way, primarily for debugging purposes.
* It allows you to print the contents of a value to the console using the `{:?}` format specifier with `println!`.
