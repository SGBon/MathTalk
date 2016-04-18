# MathTalk
Rust application intended to perform parallel computing

### Problem
CPUs have advanced greatly in the last couple of decades. Both the CPU and operating system support multithreading and by extension, parallel+concurrent computing. CPUs come with multiple cores, and operating systems deal with time sharing so that threads get ample lifetime and things appear to happen at the same time. This is useful, because time complexity for Matrix product of two NxN matrices is cubic w.r.t. N, and computing matrices is very important. As such, the problem I wish to solve is cutting out some time by computing parts of the matrix product at the same time ie. parallel computing.

The language I chose to solve this problem is Rust because it advertises itself as a low level language, and the core principle of the language is safety first; safety is very important when it comes to concurrency.

### Alternatives
Most languages have a way to do concurrency. Java and C++11 have threading libraries, Clojure has futures, and C99 has access to the POSIX thread API (on POSIX operating systems of course).

In regards to Java, safe concurrency is achieved using synchronisation blocks/functions. Code within the blocks can only be accessed by one thread, and the other threads must wait before the block before given access to the code inside.

C99 is similar. Mutexes and shared data have global visibility w.r.t. the threads. Locks on mutexes are acquired by a thread before a critical section, and released after the critical section. Threads must wait to acquire a lock on the mutex when it is currently held by another thread. There is of course a huge safety issue here:
* critical sections must be acknowledged by the programmer, and is not the business of the compiler
* mutexes are independent of the data they are intended to protect

Rust doesn't agree with that.

The way Rust handles thread safety will be explained later

### Build Tools
Just the Rust compiler.
Rust can be downloaded from https://www.rust-lang.org/downloads.html
Compile with `rustc matmult-multi.rs` in the multi-threaded/ folder
Or run `make` with the supplied makefile

### Language Features

#### Basics
Rust is really big on safety. I hinted at thread safety earlier, but that is just a subset of the entire safety model by which Rust is built upon.
Basics first, the C influence is very clear in Rust. Structs are *the* way of defining data structures in rust, much like C. Within my application I have defined two structs:
- Vec3, a struct representing a 3-component vector
- Mat3, a struct representing a 3x3 matrix

Futhermore I have a handful of methods for the two structs. All of the methods are essentially constructors for initializing, which are simply static methods invoked in a c++ style of [Structname]::[Methodname]. These are implemented on structs using
<pre><code>
impl [Structname]{
  pub foo() -> bar{
    bar
  }

}
</code></pre>

an example in my code is:
<pre><code>
impl Vec3{    
  // used for constructing Vec3 nicely
  pub fn new(x:f32,y:f32,z:f32) -> Vec3 {
  Vec3{x: x, y: y, z: z}
  }
/*...*/
}</code></pre>


This is a constructor for Vec3, in order to call it we would do:
`let some_vector = Vec3::new(1.0,2.0,3.0);`

Futhermore, there are also non-static methods. These have (&self) as a first parameter and they are called on objects. There are 2 within my code, they are Vec3::length() and Vec3::normalize():
<pre><code>impl Vec3{

  /*...*/

  // return the magnitude of the vector
  pub fn length(&self) -> f32{
    ((self.x*self.x)+(self.y+self.y)+(self.z+self.z)).sqrt()
  }

  // return a normalized form of the vector
  pub fn normalize(&self) -> Vec3{
    let len = &self.length();
    Vec3{x:self.x/len,y:self.y/len,z:self.z/len}
  }
}
</code></pre>

Using the `some_vector` above, calling these methods would take the form of:
<pre><code>
let len = some_vector.length();
let normalized = some_vector.normalize();
</code></pre>
Worth to note now is that there is no semi colon when returning the new vector. It's not actually ommitted, it's actually required to *not* be there. Similar to Scala, Rust has a concept of expressions as well as statements. The rule of thumb is: If there is a semi-colon, it's a statement, if there isn't, it's an expression. Functions return expressions, not statements. The function signature declares what the return type is (eg. `-> Vec3`) and the last line per *control-path* is required to be some expression which results in that return type.

#### Ownership and Thread Communication
The real meat of Rust is the concept of Ownership. All variables in Rust are pointers, and those pointers are owned by the scope they are within. When variables are passed to functions, the ownership of that variable passes on to the function, and unless it was explicitly defined to simply 'borrow' the variable (by using & operator much like C) then the ownership will die at the end of the scope of the function. In other terms, the function 'consumes' the variable. Likewise, ownership of a variable can only be held by one scope. This isn't a very significant feature when calling functions back to back, but very effective when implementing concurrency.

In a previous iteration of my application, I utilized Mutual Exclusion (as a mutex) in order to protect critical sections from data races. furthermore, Mutexes in Rust work with a structure Arc<T>, Atomic Reference Counter. Arcs keep track of how many scopes own a piece of data, and insures that all operations on the data is atomic. A thread clones Arc<T> in order to claim a shared ownership of the data inside. Mutexes insure that only 1 thread has access to the data inside at a time. Without using either of these constructs, the program will simply not compile, as ownership would not be able to be held by all threads at once.

In the current version of the application, instead of cloning mutexes, we elect to clone parts of inter-thread communication channels. The master thread creates a transmitter/receiver pair - `let (tx, rx) = mpsc::channel();`, and each worker thread creates a copy of the transmitter defined within their scope - `let tx = tx.clone();`. The transmitters use send() to send any data through the channel, and the receiver uses recv() to pick up any data on the channels. Within my application, the transmitter sends the computed value at ij in the new matrix as well as the indices i and j, and the receiver receives it as an implicit tuple:

<pre><code>
let mut result = Mat3::new_empty();
/*...*/
for _ in 0..9{
  let pair = rx.recv().unwrap();
  // put the result into i,j of the result matrix
  result.values[pair.2][pair.1] = pair.0;
}
</code></pre>

rx.recv() blocks until data that it can receive comes through on the channel, so this block of code will finish when all 9 threads have finished

Then of course we return the resulting matrix, whose ownership will now be given to the scope calling this function.

#### Concepts Covered by the Course
We have already seen how Rust handles Lexical Scoping with its cool ownership model. The type system is very nice. Unlike C,C++, or Java - Rust gives a more cleaner and human readable approach to specifying data size. C,C++ and Java have ints, longs, doubles, floats, char, shorts, long long, etc. Even worse, for the former two languages the memory size of those variables are implementation dependent. Rust skips the pulp and gets straight to the juice:
- i8,i16,i32,i64
- u8,u16,u32,u64
- f32,f64

Clean and simple; you know how many bits you are allocating and how the data is represented.
Type inferencing is also present much in the same sense as Scala, as are closures.
Scala style traits are also present within the language. There are traits which designate data types which are safe to be used with concurrency, as well as simply traits for deciding copying behaviour of structs.
