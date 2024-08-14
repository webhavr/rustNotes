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

### Pending
* [ ] Ch-3: Programming Concepts
* [x] Ch-4: Ownership
* [ ] Ch-5: Structs - 24
* [x] Ch-6: Enums & Pattern Matching
* [ ] Ch-7: Packages, Crates, Modules - 28
* [x] Ch-8: Common Collections
* [ ] Ch-9: Error Handling - 24
* [x] Ch-10: Generics, Traits, Lifetimes
* [ ] Ch-11: Tests - 39
* [ ] Ch-13: Iterators and Closures - 30
* [ ] Ch-14: Cargo & Crates - 25
* [ ] Ch-15: Smart Pointers - 49
* [ ] Ch-16: Concurrency - 29
* [ ] Ch-17: OOPS - 27
* [ ] Ch-18: Patterns and Matching - 27
* [ ] Ch-19: Advanced Features - 50
  
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
   
### Match
* [Read Ch-6: - Enum Patterns](https://drive.google.com/file/d/1MPidpIiuC5oYjitT9yJgHTSJo0pkWcDT/view)
* Similar to switch but with enhanced capabilities
* Separated by the `=>` arm
* Match and Enums are combined a lot in Rust
* Matches are exhaustive - i.e. the arms must cover all the possibilities
* If any possible case is left out in match, Rust code will not compile
* We could use `other` for left out cases, but `other` is binding
* We could also use `_`, which is non-binding
