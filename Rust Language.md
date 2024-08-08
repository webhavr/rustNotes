## Links

| Description        | Link                                                         |
| ------------------ | ------------------------------------------------------------ |
| Rust Official Book | [Here](https://doc.rust-lang.org/book/title-page.html)       |
| Rust by Example    | [Here](https://doc.rust-lang.org/rust-by-example/index.html) |
| std::fmt           | [Here](https://doc.rust-lang.org/std/fmt/)                   |
| Primitives         | [Here](https://doc.rust-lang.org/std/index.html#primitives)  |
| Modules            | [Here](https://doc.rust-lang.org/std/index.html#modules)     |
| Macros             | [Here](https://doc.rust-lang.org/std/index.html#macros)      |
| Keywords           | [Here](https://doc.rust-lang.org/std/index.html#keywords)    |





## Language Features

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
