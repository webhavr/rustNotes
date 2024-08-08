### Links

| Description     | Link                                                         |
| --------------- | ------------------------------------------------------------ |
| std::fmt        | [Here](https://doc.rust-lang.org/std/fmt/)                   |
| Rust by Example | [Here](https://doc.rust-lang.org/rust-by-example/index.html) |

## Language Features

### Traits
* [Link](https://doc.rust-lang.org/rust-by-example/trait.html#:~:text=A%20trait%20is%20a%20collection,declared%20in%20the%20same%20trait.)
* A trait is a collection of methods defined for an unknown type: Self. They can access other methods declared in the same trait.
* Traits can be implemented for any data type
* Analogous to Base class in C++.

#### Debug Trait
* [Link](https://doc.rust-lang.org/std/fmt/trait.Debug.html)
* This is a standard library trait used for formatting values in a human-readable way, primarily for debugging purposes.
* It allows you to print the contents of a value to the console using the `{:?}` format specifier with `println!`.