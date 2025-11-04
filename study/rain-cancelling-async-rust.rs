/// This creates a state machine
let future = async {
    let data = request.await();
    process(data).await
};
/// Nothing executes yet, `future` is just a struct/enum in memory

/// The compiler generates something like
enum MyFuture {
    Start,
    WaitingForNetwork(NetworkFuture),
    WaitingForProcess(ProcessFuture, Data),
    Done(Result),
}
/// It's just data, no running code!

/// The compiler also generates an implementation of the future trait
impl Future for MyFuture {
    fn poll(/*...*/) -> Poll<Self::Output> {
        match self {
            Start { /* ... */ } => { /* ... */ }
            WaitingForNetwork(nfut) => { /* ... */ }
            WaitingForProcess(pfut, d) => { /* ... */ }
            Done(res) =>{ /* ... */ }
        }
    }
}
/// when await is called, it is translated to this poll function
/// so it is only when await or the poll function is called that something
/// actually happens in async rust

/// this is just data, it does nothing
let future = request::get("/api/data/");
/// only now does the work happen
future.await;
/// futures are passive: inert until await or poll fn called

/// the universal protocol - drop the future or
let future = some_async_work();
drop(future); // cancelled
/// or just don't await the future

/// you can simply do nothing with the future
/// and that is the universal canncellation protocol
/// they are passive.
/// you can throw it away at any time
/// and in particular you can throw it away at any time
/// that the poll function is not being called.
/// effectively, this means that any rust future can be
/// cancelled at any await point.
/// at any point it is suspended or poll is not being called
/// it can be cancelled. this is not easily done in
/// synchronous rust.

/// cancel safety
/// the property of a future that can be dropped without side-effects
example: tokio::time::sleep()
/// a cancel unsafe future (sending a message over a channel) will
/// have a side effect (message lost forever)
counterexample: tokio::mspc::Sender::send()
/// cancel safety is a local property of a future and you can analyze
/// that without necessarily thinking about the larger system

/// cancel safety is not all that one needs to care about. what matters
/// is the context the cancellation happens in. in other words, whether this
/// cancellation causes some kind of property in the system to be violated.
///
/// for example, if you drop a future, which sends a message, but, for whatever
/// reason you dont care about that data anymore, right, maybe you just don't care
/// about it, then its not really a bug, right?

/// this is defined as `cancel correctness`. system correctness despite cancellations.
/// it is not mainstream

/// when is cancel correctness violated?
/// the three requirements to catch a `cancel correctness` bug
/// all 3 must exist together in order to have a cancellation bug
/// address one of the three prongs and the bug will go away
///
///    (1) cancel-unsafe future exists
///    (2) cancel-unsafe future is cancelled
///    (3) cancellation violates sytem property
///     - data loss (message lost forever)
///     - invariant cleanup (data that should be Some is None)
///     - missing cleanup (future cancelled prior to cleanup routine)

/// tokio mutexes are an API prone to cancel correctness issues
/// you create a mutual exclusion (mutex), you lock it, and that
/// gives you mutable access to the data underneath and then you
/// unlock it, by releasing the mutex. if you look at the lock functions
/// documentation there is a small cancel safety section and what it says
/// is that this method uses a queue to fairly distribute locks in the order
/// they were requested, cancelling a call makes you lose your place in line.
/// its not totally cancel safe but its a fairness cancel safety issue, right?
/// you lose your place in line. but it's not the whole story. the problems here
/// lie in what you actually do with the mutex. in practice. most uses of mutexes
/// are in order to temporarily violate invariants that are otherwise upheld when
/// a lock isn't held.

let guard = mutex.lock().await;
/// guard.data is Option<T>, Some to begin with
let data = guard.data.take(); // guard.data is now None

let new_data = process_data(data);
guard.data = Some(new_data); // guard.data is Some again

/// if process data contains an await point
let guard = mutex.lock().await;
let data = guard.data.take(); // guard.data is now None

let new_data = process_data(data).await; // if cancelled now, stuck in invalid state (None)
guard.data = Some(new_data); // guard.data is Some again
/// this is hard to reason about, because you need to think in the global context
/// ie is the parent/grandparent cancelling this?
/// solution: avoid tokio mutexes or at least don't await them across invariant violations

/// cancellation patterns - missing await
some_async_work(); // missing .await
/// fogetting await
let _  = some_async_work(); // future returns result

/// cancellation patterns - try_join
try_join!(
    stop_service_a(),
    stop_service_b(),
    stop_service_c(),
)?;
/// bug: if one service fails to stop, the other futures are cancelled

/// cancellation patterns - select
tokio::select! {
    result1 = future1 => handle_result1(result1),
    result2 = future2 => handle_result2(result2),
}
/// when one future completes, the others are always cancelled. Select loops are particularly dangerous

/// select is called with a set of futures and drives them all forwards concurrently.
/// each future has a code block associated with it.
/// if one of those futures completes, that corresponding code block is called.
/// but also, all of those other futures are immediately dropped. they are always cancelled.

/// making futures cancel safe
loop {
    let msg = next_msg();
    match timeout(Duration::from_secs(5), tx.send(msg)).await {
        Ok(Ok(_)) => println!("sent successfully"),
        Ok(Err(_)) => return,
        Err(_) => println!("no space for 5 seconds"),
    }
}
/// this code is not cancel safe because if it is cancelled at the await,
/// the message will be dropped entirely

/// breaking up operations
/// this doesn't work for all cases, but works for most.
/// break the loop up into a first component, reserving a permit/slot
/// in the channel that is waiting for you to send a message
loop {
    let msg = next_msg();
    loop {
        match timeout(Duration::from_secs(5), tx.reserve()).await {
            Ok(Ok(permit)) => { permit.send(msg); break; },
            Ok(Err(_)) => return,
            Err(_) println!("no space for 5 seconds"),
        }
    }
}
/// the second operation is actually sending the message, while you
/// have hold of the permit and there is a slot.
/// note: reserve isn't 100% cancel-safe, channels in tokio follow a
/// First-In-First-Out pattern where dropping the future makes you
/// lose your place in line, similar to tokio mutexes

/// in synchronous rust, the write_all API allows us to write an entire
/// buffer out to a writer
use std::io::Write;

let buffer: &[u8] = /* ... */;
writer.write_all(buffer)?;
/// this is fine in synchronous rust

/// in async rust, the equivalent is not cancel safe
use tokio::io::AsyncWriteExt;

let buffer: &[u8] = /* ... */;
writer.write_all(buffer).await?;
/// if this is a cancelled at the await point, there is no way to tell
/// how much of the buffer has been written out

/// use the right tooling!
use tokio::io::AsyncWriteExt;

let mut buffer: io::Cursor<&[u8]> = /* ... */;
writer.write_all_buf(&mut buffer).await?;
/// this allows recording of partial progress in the cursor

/// not cancelling futures - resuming instead of restarting
let mut future = Box::pin(channel.reserve());
    loop {
    tokio::select! {
        result = &mut future => break result,
        _ = other_condition => continue,
    }
}
/// by pinning the future and polling a mutable
/// reference to that future, you can resume the future
/// instead of cancelling it each time
/// while reserve isn't 100% cancel safe, cancelling a reserve
/// makes you lose your place in line, by pinning the reserve future
/// and polling a mutable reference to it, you don't lose your place in line

/// not cancelling futures - using tasks
/// before: future cancelled on TCP close
handle_request(req).await; // non-local reasoning extends over the network
/// after: task runs to completion
tokio::spawn(handle_request(req));

/// Summary:
/// Futures are passive
/// Understand cancel safety vs cancel correctness
/// Think of the three prongs of cancel correctness bugs
/// There are no great systematic solutions yet
/// Recommendations:
/// Avoid tokio mutexes - or at least, don't await across invariant violations
/// Find ways to make futures cancel-safe
/// To handle cancel-unsafe futures, pin and resume, or spawn tasks
///
/// github.com/sunshowers/cancelling-async-rust
