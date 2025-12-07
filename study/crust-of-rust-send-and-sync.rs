// Crust of Rust - Send and Sync!

// Send and Sync describe thread safety in the language
// They are used to represent thread safety at the type level
// They describe using a given value or a give type to be used
// either across thread boundaries (send) or by multiple threads
// at once (sync).

// Send and sync are both marker traits. std::marker traits are traits
// with no methods associated with them. They mark that the type meets
// a given property or has a particular guarantee about it, but it does
// not confer any additional behaviour.

pub unsafe auto trait Send {}

// The auto trait means that the compiler will automatically implement
// this trait for you if all of the members of a type are themselves that
// same trait. All auto traits are marker traits but not all marker traits
// are auto traits.

// They are primarily used in trait bounds where a given value or a give type
// is required to be used either across thread boundaries (send) or by multiple threads
// at once (sync).

// The send trait allows for ownership of a type/value to be transferred across
// the thread boundary. Most primitives are send, but for things like Rc or MutexGuard,
// if they were `Send`, it would violate some invariant of the type. Rc, is a
// single-threaded reference counting pointer, so making it `Send` violates that invariant.
// For MutexGuard, on some operating systems the thread that `locks` has to be the thread
// that `unlocks`, so making it `Send` would violate that invariant.

// This is Johns `super basic implementation of Rc. All of the clones you have of an Rc point
// to the same heap-allocated value,
struct Rc<T> {
    inner: *mut Inner<T>,
}

struct Inner<T> {
    count: usize,
    value: T,
}

impl<T> Rc<T> {
    pub fn new(v: T) -> Self {
        Rc {
            inner: Box::into_raw(Box::new(Inner { count: 1, value: v })),
        }
    }
}

/// When you clone some Rc<T>, the inner count is incremented and the pointer to the inner value
/// on the heap is cloned, which is very cheap.
impl<T> Clone for Rc<T> {
    fn clone(&self) -> Self {
        // === single-threaded lack of concurrency ===
        // the reason this is generally considered safe is that Rc is !Send and !Sync. Despite
        // clone taking an immutable reference to itself, it can mutate the state of self.inner
        // because it is operating on a single thread, meaning any value is effectively mutexed
        // by virtue of the only thread being limited in its potential concurrent operations to 1.
        unsafe { &mut *self.inner }.count += 1;
        Rc { inner: self.inner }
    }
}

// When you drop some Rc<T>, the inner count is decremented and in the event that the inner count
// drops from 1 to 0, the inner heap value is dropped along with the last pointer.
impl<T> Drop for Rc<T> {
    fn drop(&mut self) {
        // also safe due to single-threaded lack of concurrency
        let cnt = &mut unsafe { &mut *self.inner }.count;
        if *cnt == 1 {
            // because self.inner is a raw pointer, if we drop it nothing happens. So we drop it by
            // boxing it into a value, and dropping it from scope by allocating it to _. Box::Drop
            // then runs to deallocate the heap allocation and runs the destructor for the inner
            // type T
            // also safe due to single-threaded lack of concurrency
            let _ = unsafe {
                Box::from_raw(self.inner);
            };
        } else {
            *cnt -= 1;
        }
    }
}

impl<T> std::ops::Deref for Rc<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        // also safe due to single-threaded lack of concurrency
        &unsafe { &*self.inner }.value
    }
}

// For some reason, the compiler won't really complain about these definitions and whatnot until
// you try and do something with them. So a good way to debug whether something does or does not
// work with respect to the type inference is this

// So we have a function foo that holds the trait bound for any some generic type and does nothing
fn foo<T: Send>(_: T) {}

// and we have a function bar that takes a param of our custom Rc type and calls foo with it
fn bar(x: Rc<()>) {
    foo(x)
}
// The compiler will complain that our unit type Rc doesn't implement send and now we know the invisible trait bound!

// Sync is defined in terms of Send. A type T is Sync if and only if a immutable reference to T is
// Send. If you have some type where a reference to that type is allowed to be shared across
// threads, then that type is Sync, even if the type itself cannot be passed to another thead. A
// mutex guard can't be passed to another thread, but a immutable reference to that mutex guard CAN be
// passed and therefore it is Sync but not Send.

// Rc is !Sync. In the same kind of way, if Rc was sync then we could take a reference to an Rc,
// give that reference to another thread and that other thread just calls clone on it. From the
// implementation above, we know that the invariant that defines the safety on the unsafe calls in
// Rc is that the operations are designed to work on a single thread, not concurrently.
