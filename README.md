# Data Race in Rust

This repository demonstrates a simple example of a data race in Rust using mutable references.  Data races occur when multiple threads or parts of code access and modify the same memory location concurrently without proper synchronization.  This can lead to unpredictable and incorrect program behavior.

The `bug.rs` file contains the buggy code, while `bugSolution.rs` shows how to fix it using techniques like mutexes or other synchronization primitives.  The solution focuses on ensuring that only one part of the code modifies the shared variable at any given time.