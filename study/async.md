# async/await

Async isn't magical. It just describes the mechanisms for changing or co-operatively scheduling a bunch of
computation by describing under which circumstances code can make progress and under which circumstances code can yield.

Tokio is a crate that abstracts over mio which abstracts over epoll, kqueue. This allows us to create operating system
event register that lets us say 'I want to go to sleep until any of these events occur'.

## decorator

The async decorator on functions is simply a transformation directive to the compiler.
In the below example, foo1 is transformed by the compiler to foo2.

```rust
use std::future::Future;

// async decorator
async fn foo1() {}

// compiler transformation
fn foo2() -> impl Future<Output = ()> {
    async {}
}
```

The output variable for the future is the same as the return type of the function

```rust
async fn foo1() -> usize {
    0
}

fn foo2() -> impl Future<Output = usize> {
    async { 0 }
}
```

## binding

In the below examples, x is a future that eventually resolves to a usize. In the first main function,
foo is never printed. In the second main function, foo is printed as the result is awaited

```rust

async fn foo1() -> usize {
    println!("foo");
    0
}

let x: usize = foo1(); // will not compile, as x cannot be of type usize (it is a future)

fn main() {
    println!("Hello World!");
    let x = foo1(); // foo never prints
}

fn main() {
    println!("Hello World!");
    let x = foo1().await(); // foo prints
}
```

## ok, but what even is a future?

Remember, async functions or indeed any async block is really just a chunked computation.

```rust
async fn foo() {
    let mut x = [0; 1024]; // we like fixed-sized byte arrays
    let n: usize = tokio::fs::read_into("file.dat", &mut [x..]).await;
    println!("{:?}", x[..n]);
}
```

The above example can also be represented in these chunks:

```rust
async fn foo() {
    // chunk 1
    {
    let mut x = [0; 1024];
    let z = vec![];
    let fut = tokio::fs::read_into("file.dat", &mut [x..]);
    }

    fut.await; // this is really a yield, which is basically a return

    // chunk 2
    {
    let n = fut.output();
    println!("{:?}", x[..n]);
    }
}
```

But where is x? Intuitively, one would think x was a stack variable. But when we await the value, x is returned, so the
stack frame should be collapsed. But our future holds a reference to x so that it can write into it. In practice, when
we write async event or an async block the compiler generates a state machine that looks similar to the below:

```rust
enum StateMachine {
    Chunk1 { x: [u8; 1024], fut: tokio::fs::ReadIntoFuture<'x> },
    Chunk2 {},
}
```

As we are required to reference x later, it is stored in the state machine. z doesn't exist outside of chunk 1 and so is
able to be dropped when the stack frame collapses. Remember how async functions are really just functions that return an
impl Future? The type is actually returns is this StateMachine type. Cool!

```rust
fn foo() -> impl Future<Output = ()> /* StateMachine */
```

This is not a perfect desugaring of what is happening, but essentially inside our async blocks, when we declare a value
x it has a type of StateMachine. When we then call await, it is essentially calling the await method on that particular
state machine. Again, not exactly what happens under the hood, but it is a good mental model. The state machine is
holding all values that are kept across await points.

```rust
#[tokio::main]
async fn main() {
    let mut x: StateMachine = foo();
    StateMachine::await(&mut x);
}
```

## desugaring await

Suppose we have a relatively large operation to be performed, like reading a file into a string buffer. the .await
basically binds the output of the read_to_string function to a future, and while that future is not ready, the thread
yields so other work can be perfomed. Upon completion, the result is taken and returned to the caller.

While we aren't awaiting a future, we can freely execute code as normal. When x is returned from
read_to_string("file2").await;, the following chunk is executed uninterrupted until we get to
read_to_string("file3").await;

```rust
fn foo2() -> impl Future<Output = usize> {
    async {
        // First time:
        println!("foo1");
        let fut = read_to_string("file1"); // desugared await model
        while !fut.is_ready() {            //
            std::thread::yield_now();      //
            fut.try_complete();            //
        }                                  //
        let result = fut.take_result();    // desugared await model
        // Second time:
        println!("foo1");
        let x = read_to_string("file2").await; // Wait here
        println!("foo1");
        some_function(x);
        other_function(x);
        read_to_string("file3").await; // wait here
        println!("foo1");
        read_to_string("file4").await;
        0
    }
}
```

## desugaring await some more

When yielding, it is similar to a thread.yield_now(), but it bubbles up the "call stack" all the way until the point where the
thread was first awaited. Await is basically a loop that yields whenever it is unable to make progress.

```rust
fn foo2() -> impl Future<Output = usize> {
    async {
        // let x = read_to_string("file").await;

        let fut = read_to_string("file");
        let x = loop {
            if let Some(result) = fut.try_check_completed() {
                break result;
            } else {
                fut.try_make_progress();
                yield;
            }
        }
    }
}
```

## asynchronicity in practice

Suppose we have some application that reads user input from the terminal, and reads traffic over the network. Both of
these futures can reolve at any time in any order and we don't care about the order. We can handle this as per the
example below. If we can progress on the network, we will, else we attempt progress on the terminal, if we cannot
progress at all on either, we run foo2. Wen we run foo2, as soon as we hit the first yield in foo2 reading from disk, 
we bubble back up the call stack to the original select statement and check if there is anything on the network/terminal to do.

```rust
let mut network = read_from_network();
let mut terminal = read_from_terminal();
let mut foo = foo2();

loop {
    select! {
        stream <- (&mut network).await => { // must be a mutable reference or ownership transfers to await
            // do something on stream
        }
        line <- (&mut terminal).await => {
            // do something with line
        }
        foo <- (&mut foo).await => {}
    };
}
```

## cancellation

The way that cancellation is performed is that you describe the circumstances under which you should cancel the
operation.

```rust
fn foo2(cancel: tokio::sync::mpsc::Receiver<()>) -> impl Future<Output = usize> {
    async {
        println!("foo1");
        read_to_string("file1").await;
        println!("foo1");
        select! {
            done <- read_to_string("file2").await => {
                continue; // fall-through to println below
            }
            cancel <- cancel.await => {
                return 0; // break
            }
        }
        println!("foo1");
        some_function(x);
        other_function(x);
        read_to_string("file3").await;
        println!("foo1");
        read_to_string("file4").await;
        0
    }
}
```

## executor

The basic premise of an executor is that you're only allowed to await in async functions and async blocks.

```rust
async fn main() {} // top level future
```

Here, the compiler complains. This code describes a future that holds the entire control flow of the program,
with nothing to run is asynchronously and determine if we can make progress on the future, or yield. More
problematically, main cannot yield without ending progrma execution.

```rust
[#tokio::main] // executor
async fn main() {}
```

The executor crate provides low-level resources like network sockets and timers and the top-level executor loop
that allows us to await on our main function. The executor functions similarly to an the epoll API in linux.
It maintains an interest list of file descriptors on awaited futures, returning them when they change to the ready
state.

## executor macro

The [#tokio::main] macro that allows us to write async fn main is really just a procedural macro

```rust
[#tokio::main]
async fn main() {
    // some code
}
```

That rewires our main code to something like this, because the operating system expects all binaries to have a
main. It doesn't understand async runtimes, which is why we need to create a runtime event loop to run our async code.

```rust
fn main() {
    let runtime = tokio::runtime::Runtime::new();
    runtime.block_on(async {
        // some code
    });
}
```

## abandoned select arm side effects

Select tries all branches until one succeeds. In this example, suppose it tries all arms and then begins copying. It
copies part of the file over, but then the disk is saturated, so it yields and waits on the disk. Then, stream
completes. As we are not looping, we exit the select. Some bytes are copied from foo to bar, but the future isn't
complete. This issue only occurs with selects and can be solved by using non-selecting awaits.

```rust
fn main() {
    let runtime = tokio::runtime::Runtime::new();
    runtime.block_on(async {
        let mut network = read_from_network();
        let mut terminal = read_from_terminal();
        let mut foo = foo2();
        let mut f1 = tokio::fs::File::open("foo"); // async version of I/O operation for async runtime
        let mut f2 = tokio::fs::File::create("bar");
        let copy = tokio::io::copy(&mut f1, &mut f2); // async read from f1, async write to f2

        select! {
            stream <- (&mut network).await => { // must be a mutable reference or ownership transfers to await
                // do something on stream
            }
            line <- (&mut terminal).await => {
                // do something with line
            }
            foo <- (&mut foo).await => {}
            _ <- copy.await => {}
        };

        (&mut foo).await // awaiting after loop exit on a mutable reference prevents dropping/move semantics
    });
}
```

## fuse

Fuse is a way to describe that it is safe to pull a future, even if that future has been completed in the past. Suppose
in the loop below, network and terminal are both complete. The select will pull the network future, then in the second
iteration of the loop, network has been pulled but select does not remember this. The Fuse trait makes the network
variable safe to pull again, even though it has already yielded its value.

```rust
loop {
    select! {
        stream <- (&mut network).await => { // must be a mutable reference or ownership transfers to await
            // do something on stream
        }
        line <- (&mut terminal).await => {
            // do something with line
        }
        foo <- (&mut foo).await => {}
        _ <- copy.await => {}
    };
}
```

## join

Below is an iterator of futurers that we want to wait for all of them to complete before continuing. In this case, there
is the join operation. Join and join_all use FuturesOrdered under the hood, FuturesUnordered can be faster if the use
case is permissive.

```rust
let files: Vec<_> = (0..3)
    .map(|i| tokio::fs::File::read_to_string(format!("file{}", i)))
    .collect();
let (file1, file2, file3) = join!(files[0], files[1], files[2]); // explicitly name when you want output to mirror input
// or
let file_bytes = join_all(files) // here, file_bytes[0] == file[0]
```

## parallelism

Join and Select are tools that allow for concurrency, not parallelism. The issue here is that the top-level async block
is effectively one top-level future, which can only run on one thread.

```rust
fn main {
    let runtime = tokio::runtime::Runtime::new();
    runtime.block_on(async {
        println!("Hello World!");

        let accept = tokio::net::TcpListener::bind("0.0.0.0:8080");
        let mut connections = futures::future::FuturesUnordered::new();
        loop {
            select! {
                stream <- (&mut accept).await => {
                    connections.push(handle_connection(stream));
                }
                _ <- (&mut connections).await => {} // this needs to be here, awaiting all the futures.
                // if nothing awaits the FuturesUnordered, nothing awaits the futures inside there, nothing
                // is awaiting the client connections, so no client connections are served.
            }
        }
    }
}

async fn handle_connection(_: TcpStream) { todo!() }
```

There is a function provided by every executor, spawn. Spawn is a hook into the executor (tokio here) which you pass a
future and spawn moves it into the executor. Now it is as if you have given the future directly to the runtime, meaning
the runtime has both the top-level async block future and the future passed to it by spawn. It is important to note that
spawn is not a thread spawn, there are a fixed number of threads in the runtime thread pool.
Here we are simply passing futures to this thread pool.

```rust
fn main {
    let runtime = tokio::runtime::Runtime::new();
    runtime.block_on(async {
        println!("Hello World!");

        let accept = tokio::net::TcpListener::bind("0.0.0.0:8080");
        while let Ok(stream) = accept.wait {
            tokio::spawn(handle_connection(stream));
        }
    }
}

async fn handle_connection(_: TcpStream) { todo!() }
```

Spawn generally requires 'static lifetimes, as in the example below, the handle_connection future could complete before
the spawned future within completed, meaning that that thread would still be trying to access x after it was dropped,
whihc we solve with 'static. If an operation is particularly greedy, we can use spawn_blocking to perform the
computation in such a way that permits some asynchonicity where otherwise they would be none.

```rust
async fn handle_connection(_: TcpStream {
    let x = vec![];
    tokio::spawn_blocking(async {
        &x;
    }
}
```

When spawning two things and having them access the same data structure, we need to use this type of pattern so that
x1 and x2 each have their own atomic reference count, a mutex that guards the underlying value & thread locking. Similar
style to C++ multi-threaded programming.

```rust
async fn handle_connection(_: TcpStream) {
    let x = Arc::new(Mutex::new(vec![]));
    let x1 = Arc::clone(&x);
    tokio::spawn(async move {
        x1.lock()
    });
    let x2 = Arc::clone(&x);
    tokio::spawn(async move {
        x2.lock()
    });
}
```

In order to communicate the value of a spawned operation back to the caller, we need to await the value of the spawned
operation. If we do not await the join_handle, the value is dropped.

```rust
async fn handle_connection(_: TcpStream) {
    let x = Arc::new(Mutex::new(vec![]));
    let x1 = Arc::clone(&x);
    let join_handle = tokio::spawn(async move {
        x1.lock();
        0
    });
    assert_eq!(join_handle.await(), 0);
}
```

Errors in async can be tricky, because there is no guarantee that the caller is awaiting the value of the operation.
Even though we return the error to handle_connection, there is no guarantee it bubbles up to main.

```rust
async fn handle_connection(_: TcpStream) {
    let x = Arc::new(Mutex::new(vec![]));
    let join_handle = tokio::spawn(async move {
        let y: Result<_, _> = definitely_errors();
    });
    assert_eq!(join_handle.await(), Err);
}
```

## future sizes

Recall our earlier example. Here, when we call bar, we need to pass the entire state machine into the function. This
requires a memcopy of the entire state machine. Because all futures hold all of their sub futures, as you go up the
stack, it can get extremely expensive to pass these futures around.

```rust
#[tokio::main]
async fn main() {
    let mut x: StateMachine = foo();
    StateMachine::await(&mut x);
    bar(x);
}

fn bar(_: impl Future){}

enum StateMachine {
    Chunk1 { x: [u8; 1024], fut: tokio::fs::ReadIntoFuture<'x> },
    Chunk2 {},
}
```

This can be solved by boxing out futures, which creates a heap allocation that we can then simply pass to bar as a boxed
value that it is allowed to treat as a future

```rust
#[tokio::main]
async fn main() {
    let mut x = Box::pin(foo());
    bar(x);
}
```

Spawn is another good way to reduce memory overhead. Spawn holds a pointer to the original future, so rather than
growing the onion by consuming the future, it simply holds a reference.

```rust
#[tokio::main]
async fn main() {
    let mut x = [0; 1024]; // we like fixed-sized byte arrays
    let n: usize = tokio::fs::read_into("file.dat", &mut [x..]).await;
    println!("{:?}", x[..n]);
    some_library::execute()).await // here, the caller needs to store the whole future
    tokio::spawn(some_library::execute()).await // here, spawn stores a pointer to the value with no memcopy
}
```

## mutex

Below we have two asynchronous threads, both of which are accessing the same Arc using the std library Mutex.
In a runtime with only 1 thread, say our first loop runs first, x locks. Then we await on the file read. The await
yields when it can no longer read and we move to the second loop, but the lock is still held by x in the first loop.
Becuase we are using the standard library mutex, it blocks the thread. It doesn't know anything about asynchrony. So the
single thread of the runtime is blocked. This means the first future never drops its lock guard, so the lock is never
released and program execution is deadlocked.

```rust
async fn main() {
    let x = Arc::new(Mutex::new(0));
    let x1 = Arc::clone(&x);
    tokio::spawn(async move {
        loop {
            let x = x1.lock();
            tokio::fs::read_to_string("file").await;
            *x += 1;
        }
    });
    let x2 = Arc::clone(&x);
    tokio::spawn(async move {
        loop {
            *x2.lock() -= 1;
        }
    });
}
```

This is still strange, even if we use the tokio mutex. When we move to the second future, it still can't complete until
the lock guard is dropped and the lock is released. So the second future will just repeatedly yield until the first
future released the lock. Async mutexes are also much slower then std library mutexes, which is a consideration. In
general, we want to use standard library mutexes unless there is a likelihood that yielding will create a deadlock. If
we are awaiting any value, we should use an asynchronous mutex.

```rust
async fn main() {
    let x = Arc::new(tokio::sync::Mutex::new(0));
    let x1 = Arc::clone(&x);
    tokio::spawn(async move {
        loop {
            let x = x1.lock();
            tokio::fs::read_to_string("file").await;
            *x += 1;
        }
    });
    let x2 = Arc::clone(&x);
    tokio::spawn(async move {
        loop {
            *x2.lock() -= 1;
        }
    });
}
```

In the below example, we can use a standard library mutex as there is no await that may cause a deadlock, so the
standard library mutex will run faster and better serve our purposes.

```rust
async fn main() {
    let x = Arc::new(Mutex::new(0));
    let x1 = Arc::clone(&x);
    tokio::spawn(async move {
        loop {
            let x = x1.lock();
            *x += 1;
        }
    });
    let x2 = Arc::clone(&x);
    tokio::spawn(async move {
        loop {
            *x2.lock() -= 1;
        }
    });
}
```

