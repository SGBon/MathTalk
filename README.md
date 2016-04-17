# MathTalk
Rust application intended to perform parallel computing

### Problem
CPUs have advanced greatly in the last couple of decades. Both the CPU and operating system support multithreading and by extension, parallel+concurrent computing. CPUs come with multiple cores, and operating systems deal with time sharing so that threads get ample lifetime and things appear to happen at the same time. This is useful, because time complexity for Matrix product of two NxN matrices is cubic w.r.t. N, and computing matrices is very important. As such, the problem I wish to solve is cutting out some time by computing parts of the matrix product at the same time ie. parallel computing.

The language I chose to solve this problem is Rust because it advertises itself as a low level language, and the core principle of the language is safety first; safety is very important when it comes to concurrency.

### Alternatives
Most languages have a way to do concurrency. Java and C++11 have threading libraries, Clojure has futures, and C99 has access to the POSIX thread API (on POSIX operating systems of course).

In regards to Java, safe concurrency is achieved using synchronisation blocks/functions. Code within the blocks can only be accessed by one thread, and the other threads must wait before the block before given access to the code inside.

C99 is similar. Mutexes and shared data have global visibility w.r.t. the threads. Locks on mutexes are acquired by a thread before a critical section, and released after the critical section. Threads must wait to acquire a lock on the mutex when it is currently held by another thread. There is of course a huge safety issue here: 
 critical sections are not an actual part of the language, rather just a fact of the code that the programmer must acknowledge

The way Rust handles thread safety will be explained later

### Build Tools
Just the Rust compiler.
Rust can be downloaded from https://www.rust-lang.org/downloads.html
Compile with `rustc matmult-multi.rs` in the multi-threaded/ folder
