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

```rust
fn foo2() -> impl Future<Output = usize> {
    async {
        // First time:
        println!("foo1");
        let fut = read_to_string("file1");
        while !fut.is_ready() {
            std::thread::yield_now();
            fut.try_complete();
        }
        let result = fut.take_result();
        // Second time:
        println!("foo1");
        read_to_string("file2").await; // Wait here
        println!("foo1");
        read_to_string("file3").await;
        println!("foo1");
        read_to_string("file4").await;

    }
}
