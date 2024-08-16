- [Keywords](#keywords)
- [Details](#details)
  - [`mut`](#mut)
  - [`const`](#const)
  - [`static`](#static)
  - [`type`](#type)
  - [`self`](#self)
  - [Unwrap Methods](#unwrap-methods)
  - [Expect](#expect)

## Keywords
| Keyword      | Link                                                      |
| ------------ | --------------------------------------------------------- |
| All Keywords | [Here](https://doc.rust-lang.org/std/index.html#keywords) |
| `trait`      | [Here](https://doc.rust-lang.org/std/keyword.trait.html)  |
| `mut`        | [Here](https://doc.rust-lang.org/std/keyword.mut.html)    |
| `const`      | [Here](https://doc.rust-lang.org/std/keyword.const.html)  |
| `static`     | [Here](https://doc.rust-lang.org/std/keyword.static.html) |
| `type`      | [Here](https://doc.rust-lang.org/std/keyword.type.html)  |
| `self`        | [Here](https://doc.rust-lang.org/std/keyword.self.html)    |
| `const`      | [Here](https://doc.rust-lang.org/std/keyword.const.html)  |
| `static`     | [Here](https://doc.rust-lang.org/std/keyword.static.html) |


## Details

### `mut`
* Can be used to represent a mutable variable, reference, or pointer.
* Mutable references can be created from mutable variables and must be unique.

### `const`
* [Rust Book Ref](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html#constants)
* 3 types:
  * Compile-time constants
  * compile-time evaluable functions
  * raw pointers.
* Constants must be explicitly typed
* Unlike with let, you can’t ignore their type and let the compiler figure it out.
* The only lifetime allowed in a constant is `static`, which is the lifetime that encompasses all others in a Rust program.
* Constants, like statics, should always be in `SCREAMING_SNAKE_CASE`
* Not allowed to use `mut` with constants. Constants aren’t just immutable by default—they’re always immutable. 
* You declare constants using the const keyword instead of the let keyword, and the type of the value must be annotated.

### `static`
* A static item is a value which is valid for the entire duration of your program (a `static` lifetime).
* Similarities to `const`
  * Both contain a value
  * Both require type annotations
  * Both can only be initialized with constant functions and values
* Differences from `const`
  * They represent a location in memory
  * That means that you can have references to static items and potentially even modify them, making them essentially global variables.
  * 2 types
    * Simple statics
    * Mutable statics

### `type`
* Define an alias for an existing type.
* `type` does not create a new type
* Similar to `typedef` in C++

### `self`
* [Link](https://doc.rust-lang.org/std/keyword.self.html)
* `self` is used in two situations:
  1. referencing the current module
  2. marking the receiver of a method.

### Unwrap Methods
* [Link](https://doc.rust-lang.org/std/option/enum.Option.html#method.unwrap)

### Expect
* [Link](https://doc.rust-lang.org/std/option/enum.Option.html#method.expect)
* Returns the contained Some value, consuming the self value.
* Panics if the value is a None with a custom panic message provided by msg.
* ```
  let x: Option<&str> = None;
  x.expect("fruits are healthy"); // panics with `fruits are healthy`
  ```


