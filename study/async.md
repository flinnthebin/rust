# async/await

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

## mental model

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
let network = read_from_network();
let terminal = read_from_terminal();
let mut foo = foo2();

loop {
    select! {
        stream <- network.await => {
            // do something on stream
        }
        line <- terminal.await => {
            // do something with line
        }
        foo <- foo.await => {}
    };
}
```

## cancellation

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
