### Statically typed and Dynamically Typed Languages

*   A statically typed language is one where the data type of a variable is checked and verified at compile time.

* Key characteristics of statically typed languages:
    * **Type declaration**: You explicitly declare the data type of a variable when you define it.   
    * **Type checking**: The compiler verifies that you're using variables correctly based on their declared types.   
    * **Early error detection**: Many potential errors are caught during compilation, leading to more reliable code.   
    * **Improved performance**: Due to early type checking, the compiler can often optimize code better.

* Examples of statically typed languages - Java, C++, C#, Rust, Go, Swift

* In contrast to dynamically typed languages, where type checking happens at runtime, static typing offers a higher level of safety and predictability.

### LLVM

* LLVM is a collection of modular and reusable compiler and toolchain technologies. Think of it as a powerful toolkit for building compilers.

* Core Components
    * **Intermediate Representation (IR)**: This is a low-level language-independent representation of code. It's like a common ground for different programming languages.
    * **Optimizer**: LLVM includes various optimization passes to improve code performance and efficiency.
    * **Code Generator**: It translates the optimized IR into machine code for specific architectures.

* Key Features
    * **Portability**: LLVM can target multiple architectures, making it suitable for cross-platform development.
    * **Flexibility**: It can be used for both static and dynamic compilation.
    * **Efficiency**: Its optimization capabilities lead to high-performance code generation.
    * **Modularity**: Its components can be easily combined and customized.


