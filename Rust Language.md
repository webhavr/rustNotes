## Links

- [Links](#links)
- [Pending](#pending)
- [Rust Benefits](#rust-benefits)
  - [Compile time memory safety](#compile-time-memory-safety)
  - [No undefined runtime behavior](#no-undefined-runtime-behavior)
  - [Modern language features](#modern-language-features)
- [Rust Structs](#rust-structs)
- [Rust Primitives](#rust-primitives)
  - [Slice](#slice)
  - [str](#str)
  - [String](#string)
  - [References](#references)
  - [Pointer](#pointer)
    - [Pointer Alignment in Rust](#pointer-alignment-in-rust)
  - [Tuple](#tuple)
  - [usize \& isize](#usize--isize)
  - [fn](#fn)
  - [unit](#unit)
- [Language Basics](#language-basics)
  - [Crates](#crates)
  - [Item](#item)
  - [Module](#module)
  - [Macros](#macros)
  - [Panic](#panic)
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

## Rust Structs

| Struct          | Link                                                         |
| --------------- | ------------------------------------------------------------ |
| Vector          | [Here](https://doc.rust-lang.org/std/vec/struct.Vec.html)    |
| Rust by Example | [Here](https://doc.rust-lang.org/rust-by-example/index.html) |
| std::fmt        | [Here](https://doc.rust-lang.org/std/fmt/)                   |
| Primitives      | [Here](https://doc.rust-lang.org/std/index.html#primitives)  |
| Modules         | [Here](https://doc.rust-lang.org/std/index.html#modules)     |
| Macros          | [Here](https://doc.rust-lang.org/std/index.html#macros)      |
| Keywords        | [Here](https://doc.rust-lang.org/std/index.html#keywords)    |
| Macros          | [Here](https://doc.rust-lang.org/book/ch19-06-macros.html)   |


## Rust Primitives
* [Link](https://doc.rust-lang.org/std/index.html#primitives)
* ![Primitives](https://drive.google.com/uc?id=1JBkldbsViCtjVAqQ9zvPAvvKFlYSdIgv)

### Slice
* [Link](https://doc.rust-lang.org/std/primitive.slice.html)
* A dynamically-sized view into a contiguous sequence, `[T]`
* Since they are just a view, they don't own the data and borrow it from the sliced type.
* Slices are a view into a block of memory represented as a pointer and a length.
* Slices are either mutable or shared. 
  * The shared slice type is `&[T]`
  * The mutable slice type is &mut `[T]`

### str
* [Link](https://doc.rust-lang.org/std/primitive.str.html)
* The str type, also called a ‘string slice’, is the most primitive string type
* ![String Slice](https://drive.google.com/uc?id=1tDhRkXwiMjMbueeu93_YXhsiiY5WgZgO)
* A `&str` is made up of two components: a pointer to some bytes, and a length.

### String
* [Link](https://doc.rust-lang.org/std/string/struct.String.html)
* ![String](https://drive.google.com/uc?id=1MJp-FO8Jk9vG0m4xR-vsTcrDc2JfY4vz)
* `String` has ownership over contents of string stored in a heap allocated buffer, while primitive `str` has borrowed it.
  
### References
* [Link](https://doc.rust-lang.org/std/primitive.reference.html)
* [Rust Book Ref](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)
* A reference represents a borrow of some owned value. You can get one by using the `&` or `&mut` operators on a value, or by using a `ref` or `ref mut` pattern.
* A reference is just a pointer that is assumed to be aligned, not null, and pointing to memory containing a valid value of `T`
* We call the action of creating a reference borrowing. As in real life, if a person owns something, you can borrow it from them. When you’re done, you have to give it back. You don’t own it.

### Pointer
* [Link](https://doc.rust-lang.org/std/ptr/index.html)
* [Primitive](https://doc.rust-lang.org/std/primitive.pointer.html)
* Raw pointers can be unaligned or null. 
* When a raw pointer is dereferenced (using the * operator), it must be non-null and aligned
* [Box](https://doc.rust-lang.org/std/boxed/struct.Box.html) is similar to a C++ unique pointer.

#### Pointer Alignment in Rust
Pointer alignment in Rust refers to the memory address where a value can be safely stored.

* **Key Points**:
  * Alignment Requirement: Each data type has an associated alignment, specifying the address offsets at which it can be safely placed.   
  * Pointer Validity: A pointer is considered aligned if it points to a memory address that satisfies the alignment requirement of the pointed-to type.
  * Undefined Behavior: Accessing a value through a misaligned pointer results in undefined behavior, leading to crashes or unpredictable results.
  * Compiler Guarantees: Rust's compiler ensures that all pointers created through normal means are properly aligned.


* **Basic Types**:
  * `u8`: Can be placed at any address (alignment of 1).
  * `u32`: Must be placed at an address divisible by 4 (alignment of 4).
  * `u64`: Must be placed at an address divisible by 8 (alignment of 8).
* **Structs**:
    * The alignment of a struct is the maximum alignment of its fields.

* **Automatic Alignment**: The compiler automatically handles alignment for most cases, ensuring that data is placed at correct memory addresses.

* **Why Alignment Matters**:
  * **Performance**: Many hardware architectures have performance optimizations for aligned data access.
  * **Correctness**: Incorrect alignment can lead to unexpected behavior or crashes.

### Tuple
* [Link](https://doc.rust-lang.org/std/primitive.tuple.html)
* A finite heterogeneous sequence
* Tuples are finite. In other words, a tuple has a length.
* Tuples are heterogeneous. This means that each element of the tuple can have a different type.
* Tuples are a sequence. This means that they can be accessed by position; this is called ‘tuple indexing’
 
### usize & isize
* Pointer size for unsigned and signed integer types
* The size of this primitive is how many bytes it takes to reference any location in memory.
* For example:
  * On a `32 bit` target, this is `4 bytes`
  * On a `64 bit` target, this is `8 bytes`

### fn
* [Link](https://doc.rust-lang.org/std/primitive.fn.html)
 
### unit
* [Link](https://doc.rust-lang.org/std/primitive.unit.html)
* ![Unit Primitive](https://drive.google.com/uc?id=19fdHDBwGRa5VcKRHEO1j5TFkoldSlqXv)

## Language Basics
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
* This allows a program to terminate immediately and provide feedback to the caller of the program
* This is a run-time check

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
