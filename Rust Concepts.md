- [Pending](#pending)
- [Ownership](#ownership)
- [Borrowing](#borrowing)
- [Memory Management Comparison](#memory-management-comparison)
- [Generics](#generics)
- [Traits](#traits)
- [Lifetimes](#lifetimes)
- [Vectors](#vectors)
- [Strings](#strings)
- [Hash Maps](#hash-maps)
- [Enums](#enums)
- [Match](#match)
- [Structs](#structs)
- [Methods](#methods)
- [Packages](#packages)
- [Crates](#crates)
- [Modules](#modules)
- [Use](#use)
- [Errors](#errors)
- [Panic](#panic)
- [Recoverable Errors](#recoverable-errors)
- [Error handling guidelines](#error-handling-guidelines)
- [Tests](#tests)
- [Test Run Control](#test-run-control)
- [Test Organization](#test-organization)
- [Closures](#closures)

### Pending
* [x] Ch-4: Ownership
* [x] Ch-5: Structs
* [x] Ch-6: Enums & Pattern Matching
* [x] Ch-7: Packages, Crates, Modules
* [x] Ch-8: Common Collections
* [x] Ch-9: Error Handling
* [x] Ch-10: Generics, Traits, Lifetimes
* [x] Ch-11: Tests 
* [ ] Ch-13: Iterators and Closures - 30 - Sat
* [ ] Ch-14: Cargo & Crates - 25 - Sat
* [ ] Ch-15: Smart Pointers - 49 - Sat
* [ ] Ch-16: Concurrency - 29 - Sun
* [ ] Ch-17: OOPS - 27 - Sun
* [ ] Ch-18: Patterns and Matching - 27 - Sun
* [ ] Ch-19: Advanced Features - 50 - Mon
  
### Ownership
* [Read Ch-4: - Ownership](https://drive.google.com/file/d/1unEsGBAMhBZHX3FUKQQhlNobcuRXSeaU/view)
* Memory is managed through a system of ownership rules that compiler checks
* Program won't compile if rules violated
* **Ownership Rules**
  * Each value in Rust has an owner
  * There can only be one owner at a time
  * When the owner goes out of scope, value will be dropped
* Rust calls a special function `drop`, which returns the heap allocated memory when the variable owning it goes out of scope
* With `clone` method, we can explicitly create a deep copy of a heap allocated memory
* For some data types, Rust implements a `Copy` trait. Rust will not let us annotate a type with `Copy` trait if any of its parts has implemented a `drop` trait
* `Copy` trait implemented by
  * All integer types
  * All Boolean types
  * All Floating types
  * Char type
  * Typles only if they contain above types
* If we pass a variable to another function, it takes the ownership function of that variable. This is similar to C++ `std::move()`
* If we want to transfer a variable without ownership, we need to use References
* ![Reference Representation](https://drive.google.com/uc?id=1kzmbGbBA0W7ZKiMOGSdWkmaATM5H6CCz)
* Reference is `s`, but ownership is still with `s1`

### Borrowing
* One way we can borrow is through creating a reference
* For any variable, we can have
  * Only 1 mutable reference. No other reference - either mutable or immutable now
  * Multiple immutable references
* **Data Race**
  * Two or more pointers access the same data at the same time.
  * At least one of the pointers is being used to write to the data.
  * There’s no mechanism being used to synchronize access to the data.
* **Benefits of Restriction**
  * Data Races can be prevented at compile time
* Scope of a reference is from the time it is created until it is last used
* References cannot be dangling and must always be valid

### Memory Management Comparison
* **Pros**
 
| Manual like C       | Automatic Like Java | Scope based like C++ | Rust                 |
| ------------------- | ------------------- | -------------------- | -------------------- |
| No runtime overhead | Fully automatic     | Partially automatic  | Enforced by compiler |
|                     | Safe & Correct      | No runtime overhead  | No runtime overhead  |
|                     |                     |                      | Safe & Correct       |

* **Cons**
 
| Manual like C  | Automatic Like Java | Scope based like C++          | Rust    |
| -------------- | ------------------- | ----------------------------- | ------- |
| Use after free | Garbage Collection  | Complex, opt-in by programmer | Complex |
| Double free    | Destructor delays   | Potential for use-after free  |         |
| Memory leaks   |                     |                               |         |

### Generics
*  [Read Ch-10: - G T Lifetime](https://drive.google.com/file/d/1FJ6g33pGUuDInt2b3fqjWDRCKwGpteYR/view)
*  Allow us to replace specific types with placeholder that represents multiple types to avoid code duplication
*  Using generics is a compile-time cost rather than a run-time cost.
*  Monomorphization is the process of turning generic code into specific code by filing in concrete types that are used when compiled

### Traits
*  [Read Ch-10: - G T Lifetime](https://drive.google.com/file/d/1FJ6g33pGUuDInt2b3fqjWDRCKwGpteYR/view)
*  Traits are similar to a feature called interfaces in other languages, with some differences
*  Trait can be thought of as a virtual class with virtual methods. If any class wants to implement a trait, it needs to implement the virtual methods required under the virtual class
*  We could also use default implementations for a trait
*  We can only implement a trait on a type only if either the trait or the type, or both are local to the crate.

### Lifetimes
*  [Read Ch-10: - G T Lifetime](https://drive.google.com/file/d/1FJ6g33pGUuDInt2b3fqjWDRCKwGpteYR/view)
*  Lifetime ensures that references are valid as long as we need them to be.
*  Every reference in Rust has a lifetime, which is the scope for which the reference is valid
*  We must annotate the lifetimes when lifetimes of references could be related in a few different ways
*  Rust has a Borrow Checker that compares scopes to determine whether all borrows are valid
*  `Static` lifetime denotes the associated reference can live for the entire duration of the program
*  All string literals have a static lifetime

### Vectors
*  [Read Ch-8: - Collections](https://drive.google.com/file/d/1TRfVi795ALHgfyvjDjuA7xgU7Ye3KB34/view)
* For vectors, strings, and hash maps, data is stored on the heap
* A vector allows to store a variable number of values next to each other
* Can only store values of the same type
* Can be created using the `vec!` macro or `Vec::new()` function
* We can get the value at an index using `&` or `[]`
* We can also use the `get` method with index as an argument. It returns `Option<T>`
* If we use vector in combination with Enum, we can use the vector to store multiple types
* At any time, if the vector's reference is used, the borrow checker would ensure the lifetime of the reference and the borrow checker rules 
  

### Strings
*  [Read Ch-8: - Collections](https://drive.google.com/file/d/1TRfVi795ALHgfyvjDjuA7xgU7Ye3KB34/view)
* String literals are stored in program's binary or Stack
* String type is growable, and stored on the heap
* V IMP - Rust Strings do not support indexing
* If we want to iterate over strings:
  * We could use the `chars()` method -> For unicode scalar values
  * Or, we could use the `bytes()` method -> For raw bytes
  * Unicode scalar values may be made up of more than 1 byte

### Hash Maps
*  [Read Ch-8: - Collections](https://drive.google.com/file/d/1TRfVi795ALHgfyvjDjuA7xgU7Ye3KB34/view)
*  In hash maps, ownership is decided by if the data type implements the Copy trait or not
*  For `i32` which implement the `Copy` trait, the values are copied into the hash map
*  For owned values like `String`, the values will be moved into the hash map, which will become the owner of those values

### Enums
* [Read Ch-6: - Enum Patterns](https://drive.google.com/file/d/1MPidpIiuC5oYjitT9yJgHTSJo0pkWcDT/view)
* Allow to define a type by enumerating its possible variations
* `Option` is an enum that can be `Some(*)` or `None`
* Enums and struct can be combined in Rust
* Example
    ```
    num IpAddr {
      V4(u8, u8, u8, u8),
      V6(String),
    }
    ```
* We can also define methods on `enums` like structs using the `impl` keyword
* Option Enum
    ```
    enum Option<T> {
      None,
      Some(T),
    }
    ```
* Before using, if we want to use `4i32`, we need to convert it from `Some(4i32)` explicitly
* If we use the `pub` keyword before an enum, all the fields of enum become public by default
   
### Match
* [Read Ch-6: - Enum Patterns](https://drive.google.com/file/d/1MPidpIiuC5oYjitT9yJgHTSJo0pkWcDT/view)
* Similar to switch but with enhanced capabilities
* Separated by the `=>` arm
* Match and Enums are combined a lot in Rust
* Matches are exhaustive - i.e. the arms must cover all the possibilities
* If any possible case is left out in match, Rust code will not compile
* We could use `other` for left out cases, but `other` is binding
* We could also use `_`, which is non-binding

### Structs
* [Read Ch-5: - Structs](https://drive.google.com/file/d/15HWha7dNxTZtl1ywXNsJWrbjsdK_zXdo/view)
* Structs help to hold multiple related values together
* Fields are names of types in structs
* If we want to change a struct, the entire instance must be mutable. Rust doesn't allow to mark only certain fields as mutable
* We can create struct instances from other instances using Struct Update Syntax. This way, we can create a new instance which differs in say only 1-2 fields
* We can also create Tuple Structs like - `struct Color(i32, i32, i32)`
* Structs can also carry references to data owned by something else, but to do so requires the use of explicitly specifying reference lifetimes
* If we use `pub` keyword before Struct, only the Struct is public
* All of the fields inside Struct need to be explicitly made `pub` one by one

### Methods
*  [Read Ch-5: - Structs](https://drive.google.com/file/d/15HWha7dNxTZtl1ywXNsJWrbjsdK_zXdo/view)
*  Methods are similar to functions and defined with the `fn` keyword
*  They are defined within the context of a struct or an enum using an `impl` keyword
*  Example - an area method inside the Rectangle struct
*  Within an `impl` block, the type Self is an alias for the type that the `impl` block is for
*  Methods must have a parameter named `self` of type `Self` for their first parameter
*  Depending on the type of borrow, this could be prefixed with simply `&` or `&mut`
  
### Packages
* [Read Ch-7: Packages Crates Modules](https://drive.google.com/file/d/10XSj62Q5d9Z8H8-j3EFyTcI6Zebwy0BU/view)
* A Cargo feature that allows to build, test, and share crates
* A bundle of one or more crates that provides a set of functionality
* Package contains a `Cargo.toml` file that describes how to build those crates
* A package must contain at least 1 crate - either binary crate or library crate 
* Packages added in `Cargo.toml` tell Cargo to download the specific package and its dependencies from `crates.io` and make it available to the project
* To use any external package into our current package, we need to use the `use` keyword
* Package -> Crates -> Modules

### Crates
* [Read Ch-7: Packages Crates Modules](https://drive.google.com/file/d/10XSj62Q5d9Z8H8-j3EFyTcI6Zebwy0BU/view)
* A Tree of modules that produces a library or executable
* The smallest amount of code that the rust compiler considers at a time
* **Library Crate**
  * Don't have a `main` function
  * They don't compile to an executable
  * A package can contain at most 1 library crate
* **Binary Crate**
  * A package may contain as many binary traits as possible
  * Must have a `main` function
  * These can be compiled to an executable that can be run
* **Crate Compilation**
  * Crate Root is a source file that the Rust compiler starts from and makes up the root module of the crate
  * When compiling a crate, the compiler first looks in the crate root file
    * `src/lib.rs` for a library crate
    * `src/main.rs` for a binary crate

### Modules
* [Read Ch-7: Packages Crates Modules](https://drive.google.com/file/d/10XSj62Q5d9Z8H8-j3EFyTcI6Zebwy0BU/view)
* Help to control the organization, scope, and privacy of paths
* `use` keyword brings a path into scope
* `pub` keyword is used to make things public
* `mod` keyword is used to decalre modules
* `super` keyword helps to access relative paths in the ancestor modules
* All items within a module is private from its parent modules by default
* To make a module public, we need to use `pub mod`
* A module can contain actual functions with the `fn` keyword
* Items in a parent module can't use the private items inside child modules, but items in a child module can use the items in their ancestor modules.


### Use
* [Read Ch-7: Packages Crates Modules](https://drive.google.com/file/d/10XSj62Q5d9Z8H8-j3EFyTcI6Zebwy0BU/view)
* Adding `use` and a path in a scope is similar to creating a symbolic link in the filesystem
* Paths brought into scope with `use` also check for privacy of modules

### Errors
* [Read Ch-9: Errors](https://drive.google.com/file/d/1jwHZr5HfQfiwfjrfptblDtWWkBTEs9QZ/view)
* 2 types of errors
  * **Recoverable Errors**
  * **Unrecoverable Errors**
    * These are dealt with `panic`
    * Panic could be implicit - like calling an array out of bounds index
    * Explicitly calling the panic command

### Panic
* [Read Ch-9: Errors](https://drive.google.com/file/d/1jwHZr5HfQfiwfjrfptblDtWWkBTEs9QZ/view)
* Unrecoverable errors are dealt with `panic`
* Panic could be implicit - like calling an array out of bounds index
* Explicitly calling the panic command
* **Panic Response**
  * **Unwinding**
    * Rust walks back up the stack and cleans data from each function it encounters
    * Run rust code with `RUST_BACKTRACE=1`
    * We can also enable debug symbols in Rust
  * **Aborting**
    * Immediately aborting
    * Memory that the program was using will then need to be cleaned up by the operating system
    * To enable, `panic = 'abort'` could be added to the profile section in `Cargo.toml` file

### Recoverable Errors
* [Read Ch-9: Errors](https://drive.google.com/file/d/1jwHZr5HfQfiwfjrfptblDtWWkBTEs9QZ/view)
* Result Enum
  ```
  enum Result<T, E> {
    Ok(T),
    Err(E),
  }
  ``` 
* We could use the `match` with `Result` enums to combine variety of errors
* Alternatives or Combinations - `unwrap_or_else`, `unwrap`, `expect`
* Errors can also be propagated using using the `?` operator, so the calling code can decide what to do with them

### Error handling guidelines
* It's advisable to get code panic when it's possible that code could end up in a bad state
* A bad state is when some assumption, guarantee, contract, or invariant
has been broken, such as when invalid values, contradictory values, or missing values are passed to your code—plus one or more of the following:
  * The bad state is something that is unexpected, as opposed to something that will likely happen occasionally, like a user entering data in the wrong format.
  * Your code after this point needs to rely on not being in this bad state, rather than checking for the problem at every step.
* If someone calls your code and passes in values that don’t make sense, it’s best to return an
error if you can so the user of the library can decide what they want to do in that case.
* However, in cases where continuing could be insecure or harmful, the best choice might be to call panic! and alert the person using your library to the bug in their code so they can fix it during development.
* However, when failure is expected, it’s more appropriate to return a Result than to make a panic! call.
* When your code performs an operation that could put a user at risk if it’s called using invalid values, your code should verify the values are valid first and panic if the values aren’t valid.

### Tests
* [Read Ch-11: Tests](https://drive.google.com/file/d/1vx_dHJUt00o1J1S901uCFIO431WD8JhG/view)
* Tests are best written in 3 sections
  * Set Up
  * Run the code to test
  * Assert the results
* Rust annotates test by adding the `#[test]` before the fn body of the test
* When tests are run with `cargo test`, Rust builds a test runner binary that runs the annotated functions and reports on whether each test function passes or fails
* Each test by default runs in its own thread. When the main thread sees that a test thread has died, the test is marked as failed
* **Assert Macros**
  * `assert!` - Evaluate if a statement is true or false
  * `assert_eq` - Evaluate if 2 arguments are equal
  * `assert_ne` - Evaluate if 2 arguments are not equal
  * Optionally, with macro, we can add a custome message to be printed in case of failure
* `should_panic`
  * The test passes if the code inside the function panics
  * The test fails if the code inside the function does not panic
  * `#[should_panic]` attribute is placed after the `#[test]` attribute
  * `should_panic` can also be combined with `expected` attribute to check if the same error message actually matches in case of failure
* We can also use the `Result<T, E>` option in place of panics

### Test Run Control
* [Read Ch-11: Tests](https://drive.google.com/file/d/1vx_dHJUt00o1J1S901uCFIO431WD8JhG/view)
* `cargo test` compiles the code in test mode and runs the resulting binary
* If we want to make sure that tests don't run in parallel because of possible inter-shared states, we could use specify the number of threads using `cargo test --test-threads=1`
* **Ignoring Specific Tests**
  * We can add the `#[ignore]` line after the `#[test]` if we want to exclude any test
  * If we need to run ignored tests, we need to run `cargo test -- --include-ignored`

### Test Organization
* [Read Ch-11: Tests](https://drive.google.com/file/d/1vx_dHJUt00o1J1S901uCFIO431WD8JhG/view)
* Mainly 2 categories of tests:
  * **Unit-Tests:**
    * Test one specific module in isolation and can test private interfaces
    * Unit tests are added by adding them in the `tests` module in each file to contain the test functions along with the actual implementation code
    * The `#[cfg(test)]` tells the Rust compiler to compile and run these codes only when `cargo --test` is invoked
  * **Integration Tests:**
    * Tests which are external to the library and test the overall public interface
    * Generally there is a separate directory for integration tests - `tests`
    * Each file in the `tests` directory is a separate crate, so we need to bring our library into each test crates' scope 
    * We don't annotate integration tests with `#[cfg(test)]`. They run automatically when we invoke `cargo --test`
* If any test in a section fails, the following sections will not run

### Closures
*  [Read Ch-13: Tests](https://drive.google.com/file/d/19DzJZpzyLzEm59r0t8hzbLF1wtm5Jltu/view)
*  A function like construct that can be stored in a variable
*  Rust Closures are anonymous functions you can save in a variable or pass as arguments to other functions
*  Closure can be created in one context and called in a totally different context
*  Closures can capture values from the scope in which they are defined
*  Differences between Closures and Functions
   *  Closures can capture their environment where they are defined. This cannot be done by functions
* **Syntax:**
  *  ```
      fn add_one_v1 (x: u32) -> u32 { x + 1 } // Normal Function
      let add_one_v2 = |x: u32| -> u32 { x + 1 }; // Closure
      let add_one_v3 = |x| { x + 1 }; // Closure does not need to know variable type
      let add_one_v4 = |x| x + 1 ; // Closure does not even need curly braces
      ```
  *   Compiler can infer and then will record only type of parameters for their types
      ```
      add_one_v3(4i32);
      add_one_v3(4u32); // Will give compiler error now
      ```
* **Closure capturing values from environment**
  * **Borrowing Immutably**
    * As with functions, we can call closure multiple times with captured varibales from environment
    * Because we can have multiple immutable references to a variable
  * **Borrowing Mutably**
    * If a closure is capturing a variable in the environment mutably
    * Then, there cannot be any other reference to that variable
    * Here, we cannot even call the closure more than once, as it cannot capture mutably a reference more than once
  * **Taking Ownership**
    * While working with threads and closures, we would want to move the ownership of the variable to the closure itself
    * We can do this by explicitly mentioning `move` before the closure `thread::spawn(move || println!("From thread: {list:?}"))`=
* **Moving captured values out of the closure**
  * A closure body can do any of the following
    * Move a captured body out of the closure
    * Mutate the captured value
    * Neither move, nor mutate the value
    * Capture nothing from the environment to begin with 