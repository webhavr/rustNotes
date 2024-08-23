- [Basics](#basics)
- [Threads](#threads)
  - [Creating a new thread](#creating-a-new-thread)
  - [Threads finish with Join](#threads-finish-with-join)
  - [Move Closures with Threads](#move-closures-with-threads)

## Basics
* [Read Ch-16: Concurrency](https://drive.google.com/file/d/1NQIZoxgzhrsCfqCUm90hZrIBBeoMxIJF/view)
* Concurrent Programming - Different parts of a program execute independently
* Parallel Programming - Different parts of a program execute at the same time
* Most of the concurrency errors in Rust are compile time errors

## Threads
* [Read Ch-16: Concurrency](https://drive.google.com/file/d/1NQIZoxgzhrsCfqCUm90hZrIBBeoMxIJF/view)
* Splitting the computation in the program into multiple threads to run multiple tasks at the same time improves performance
* Frequent Concurrency Problems
  * Race Conditions - Threads are accessing data or resources in an inconsistent order
  * Deadlocks - 2 threads are waiting for each other, preventing both the threads from continuing
  * Hard to reproduce bugs
* Rust uses a 1:1 model of thread implementation, whereby a program uses one operating system thread per one language thread

### Creating a new thread
* New thread is created with `thread::spawn` function
* We pass a closure containing the code we want to run in the new thread
* When the main thread of the Rust program completes, all spawned threads are shut down, whether or not they have finished running
* **Example-1**
    ```
    use std::thread;
    use std::time::Duration;
    fn main() {
        thread::spawn(|| {
            for i in 1..10 {
                println!("hi number {i} from the spawned thread!");
                thread::sleep(Duration::from_millis(1));
            }
        });
    
        for i in 1..5 {
        println!("hi number {i} from the main thread!");
            thread::sleep(Duration::from_millis(1));
        }
    }
    ```
### Threads finish with Join
* There is no guarantee on the order in which threads run
* We also cannot guarantee that the spawned thread will run at all
* Return type of `thread::spawn` is `JoinHandle`
* A `JoinHandle` is an owned value that, when we call the `join` method on it, will wait for the thread to finish.
* **Example-2**
    ```
    use std::thread;
    use std::time::Duration;

    fn main() {
        let handle = thread::spawn(|| {
            for i in 1..10 {
                println!("hi number {i} from the spawned thread!");
                thread::sleep(Duration::from_millis(1));
            }
        });
        for i in 1..5 {
            println!("hi number {i} from the main thread!");
            thread::sleep(Duration::from_millis(1));
        }
        handle.join().unwrap();
    }
    ```
* Calling `join` on the handle blocks the current thread until the thread represented by the handle terminates.
* Blocking the thread means that thread is prevented from performing work or exiting

### Move Closures with Threads
* `move` keyword with closures passed to the `thread::spawn` and closure will take ownership of the values it uses from the environment
* It transfers the ownership of those values from one thread to another
* **Example-3**
    ```
    use std::thread;
    fn main() {
        let v = vec![1, 2, 3];
        let handle = thread::spawn(move || {
            println!("Here's a vector: {v:?}");
        });
        handle.join().unwrap();
    }
    ```