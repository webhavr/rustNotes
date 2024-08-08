## Links

| Description     | Link                                                         |
| --------------- | ------------------------------------------------------------ |
| std::fmt        | [Here](https://doc.rust-lang.org/std/fmt/)                   |
| Rust by Example | [Here](https://doc.rust-lang.org/rust-by-example/index.html) |




## Language Features

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