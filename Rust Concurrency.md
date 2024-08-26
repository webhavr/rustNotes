- [Basics](#basics)
- [Threads](#threads)
  - [Creating a new thread](#creating-a-new-thread)
  - [Threads finish with Join](#threads-finish-with-join)
  - [Move Closures with Threads](#move-closures-with-threads)
- [Message Passing among Threads](#message-passing-among-threads)
  - [Channels](#channels)

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
## Message Passing among Threads
* In Message passing, threads communicate by sending each other messages containing data
* "Do not communicate by sharing memory; instead; share memory by communicating"

### Channels
* A channel is a general programming concept by which data is sent from one thread to another
* **2 Halves** 
  * **Transmitter**
    * Upstream location where data is sent
    * One part of the code calls methods on the trasmitter with the data you want to send
  * **Receiver**
    * Downstream location where data is received
    * Another part of the code checks the receiving end for arriving messages
* A channel is said to be closed if either the transmitter or receiver half is dropped
* In Rust library, we create `mpsc::channel` - meaning Multiple Producer, Single Consumer channel
* In short, Rust implements channels means a channel can have multiple sending ends that produce values but only 1 receving end
* `mpsc::channel` function returns a tuple, the first element of which is the sending end - transmitter, and the second element is the receiving end - the receiver
* `thread::spawn` is used to create a new thread and then using `move`, `tx` is moved into the closure so the spawned thread owns the `tx`
* **Example**
  ```
    use std::sync::mpsc;
    use std::thread;
    fn main() {
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            let val = String::from("hi");
            tx.send(val).unwrap();
        });
        let received = rx.recv().unwrap();
        println!("Got: {received}");
    }
  ```
* **2 Receive methods**
  * `recv`
    * Blocking
    * Short for receive, which will block the main thread's execution and wait until the value is sent down the channel
    * Once a value is sent, `recv` will return `Result<T,E>`
    * When the transmitter closes, `recv` will return an error to signal that no more values will be coming
  * `try_recv`
    * Non-Blocking
    * Doesn't block but returns `Result<T,E>`
    * `Ok` value if the message is available
    * `Err` value if the message is not available
* **Example**
  * Attempting to use a value that is already sent will result in a compiler error
  ```
    use std::sync::mpsc;
    use std::thread;
    fn main() {
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            let val = String::from("hi");
            tx.send(val).unwrap();
            println!("val is {val}"); // Will give a compiler error, as after send, reciever takes its ownership
        });
        let received = rx.recv().unwrap();
        println!("Got: {received}");
    }
  ```
* We can clone the transmitter which will result in an entirely new transmitter
* **Example**
  ```
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    for received in rx {
        println!("Got: {received}");
    }
  ```