### Ownership
* [Read Chapter - Ownership](https://drive.google.com/file/d/1unEsGBAMhBZHX3FUKQQhlNobcuRXSeaU/view)
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

