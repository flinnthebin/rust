/// Declarative Macros

/// All content is stolen from the following address:
/// https://www.youtube.com/watch?v=q6paRBbLgNw
/// These are my personal notes, a pale reflection of the original source
/// that is filled with far more insight, compiler error walkthroughs, thorough
/// analysis of concepts, etc. This simply exists to refresh my mind.
/// In the spirit of sharing and collaboration, I welcome you to read my notes.

/// 
/// expr: an expression is (almost) anything that can be terminated with a semicolon
/// ty: a type is a type
///

/// typical macro definition
macro_rules! f{
    () => {}
}

/// these are all valid macro invocations
/// there is no ability for the author to impose a syntactic requirement
/// on the calling sytax
f!()
f![]
f!{}

/// typical syntax with single brackets evaluates to an expression
#[macro_export]     // basically 'pub', allows for calling the macro in other files
macro_rules! foo {
    () => {
        Vec::new() // this evaluates to a valid rust expression
    }
}

/// blocks {{ }}
#[macro_export]
macro_rules! bar {
    ($e:expr) => {{                 // this block is required
        let mut vs = Vec::new();    // as what is in the scope
        vs.push($e);                // doesn't evaluate to a
        vs                          // valid rust expression
    }}
}

/// blocks are required where you want to do something like this:
let x: Vec<u32> = bar![42];
/// without the block to scope the macro, it would expand to:
let x: Vec<u32> = let mut vs = Vec::new();
/// losing all the work after the semicolon, not returning vs

#[macro_export]
macro_rules! bar {
    ($e:expr) => [{ // this block scoping is also valid
        let mut vs = Vec::new();
        vs.push($e);
        vs
        }]
}

#[macro_export]
macro_rules! baz {
    ($e:expr) => ({ // so is this
        let mut vs = Vec::new();
        vs.push($e);
        vs
        })
}

/// scaling macros. * = 0 or more, + = 1 or more, ? = 0 or 1
/// * is used here because it means that the 0 case returns an empty vector
#[macro_export]
macro_rules! bar {
    #[allow(unused_mut]                     // required for the 0 case
    ($($e:expr),*) => {{                    // this allows us to express multiple expressions
        let mut vs = Vec::new();            // separated by commas (any delimiter can be used)
        w(vs.push($e);)*                    // matching the syntax here, we can repeat operations
        vs                                  // the same number of times as the pattern that had $e
    }}                                      // in it
}

/// trailing comma pattern
#[macro_export]
macro_rules! bar {
    #[allow(unused_mut]
    ($($e:expr),* $(,)?) => {{              // the ? operator allows 0 or 1 trailing commas
        let mut vs = Vec::new();            // following the pattern of comma separatede expressions
        $(vs.push($e);)*
        vs                                  // the same number of times as the pattern that had $e
    }}                                      // in it
}


/// utility of macros
trait MaxValue {
    fn max_value() -> Self;
}

impl MaxValue for i32 {
    fn max_value() -> Self {
        i32::MAX
    }
}

impl MaxValue for u32 {
    fn max_value() -> Self {
        u32::MAX
    }
}

impl MaxValue for i64 {
    fn max_value() -> Self {
        i64::MAX
    }
}

impl MaxValue for u64 {
    fn max_value() -> Self {
        u64::MAX
    }
}

/// this can be handled with a macro
#[macro_export]
macro_rules! max_impl {
    ($t:ty) => {
        impl $crate::MaxValue for $t {
            fn max_value() -> Self {
                <$t>::MAX
            }
        }
    };
}

/// rather than define the implementation multiple times
/// on the same pattern, the pattern can be defined and called
/// with different types
max_impl!(i32);
max_impl!(u32);
max_impl!(i64);
max_impl!(u64);

/// vec fill to count
#[macro_export]
macro_rules! bar {
    #[allow(unused_mut)]
    ($e:expr; $c:expr) => {{
        let mut vs = Vec::new();
        for _ in 0..$c {
          vs.push($e);                              // this only works naively with integer values
        }                                           // if we pass a Some(val), it will break the macro
        vs                                          // as the first time val is evaluated, it works, then
    }}                                              // val becomes none
}

#[macro_export]
macro_rules! bar {
    #[allow(unused_mut)]
    ($e:expr; $c:expr) => {{
        let mut vs = Vec::new();
        let x = $e;
        for _ in 0..$c {
          vs.push(x.clone());                       // here, the expression is passed once to x, then the
        }                                           // cloned value is pushed, which makes the compiler happy
        vs
    }}
}

let y = 42;
bar![y; 2]; // this is fine

bar![y; "twenty times"];                            // this will break because the line 'for _ in 0..$c' requires an integer
                                                    // so the semantic error on that line will bubble up at compilation time

/// How to write failing compilations such that doctest
/// will run them and confirm that they do in fact fail to compile
/// be careful that you actually understand the semantic reasoning behind
/// the failing compilation or you could get a false positive

///```compile_fail
/// let x: Vec<u32> = cratename::bar![42; "foo"];
///
[#allow(dead_code)]
struct CompileFailTest;

/// So this sucks because it requires a bunch of resizing, which is `silly` because we can simply
/// give the capacity of the vector as we have count!
#[macro_export]
#[allow(unused_mut)]
macro_rules! bar {
    ($e:expr; $c:expr) => {{
        let mut vs = Vec::with_capacity($c);         // init a vec with size count
        let x = $e;
        for _ in 0..$c {                             // but we call count here, count has moved/ownership
          vs.push(x.clone());                        // is transferred
        }
        vs
    }}
}

/// So any time you want to use a token parameter more than once, it needs to become a variable
#[macro_export]
#[allow(unused_mut)]
macro_rules! baz {
    ($e:expr; $c: expr) => {{
        let count = $c;
        let mut vs = Vec::with_capacity(count);
        let x = $;
        for _ in 0..count {                         // this is still a busy, noisy loop though.
            vs.push(x.clone());                     // it's $c cpu instructions. with bounds checking!
        }                                           // still allocation checking on every push!
        vs                                          // this is unnecessary!
    }};
}

/// He calls this section "Reallocations for the repetition constructor
/// Sexy Gjengset Wizardry                          // std::iter::repeat is really handy, it yields
#[macro_export]                                     // clones of the element you give for as long
#[allow(unused_mut)]                                // as you take from the iterator, `take` says
macro_rules! baz {                                  // "only take this many things"
    ($e:expr; $c: expr) => {{                       // extend knows to keep adding to the vector
        let count = $c;                             // inside the set bounds. in theory this can be
        let mut vs = vec::with_capacity(count);     // made better if we use `exact_size iterator`
        vs.extend(std::iter::repeat($e).take(count) // inside the set bounds. in theory this can be
        vs                                          // made better if we use `exact_size iterator`
        }};                                         // but its tricky because `repeat + take`
}                                                   // doesn't implement `exact_size`

/// std::iter::repeat has a bound that the type of the element implements clone.
/// the rust compiler can't simply optimize a Vec::with_capacity(count) followed by a series
/// of calls to push because it's too sophisticated. it would require the compiler to know the
/// semantics of `Vec`, that it would have logic describing some relationship between `Vec::new(),
/// Vec::push, and Vec::with_capacity()`. Vec would be very heavily bounded if all behaviour
/// programmers used a vec for could be described in the compiler. It will, however, do this IFF
/// we use `Vec::from_iterator() and managed to produce an iterator that implemented
/// `exact_sized_iterator`


/// Macros don't have trait bounds. They can't, for example, enforce that the type of $e:expr needs
/// to implement Clone. Macros in general cannot express that. Instead, what will happen if you try
/// and use something that doesn't implement Clone, the compiler will still generate the code from
/// the macro. As you can imagine, the error simply bubbles up from the scope of the block and into
/// the compiler output.

/// One thing to be careful of here is that the macro is defined by the caller, so if they have
/// modules such as std, iter, their calls will override the standard library. so the correct
/// syntax would be
#[macro_export]
#[allow(unused_mut)]
macro_rules! baz {
    ($e:expr; $c: expr) => {{
        let count = $c;
        let mut vs = vec::with_capacity(count);
        vs.extend(::std::iter::repeat($e).take(count)
        vs
        }};
}

/// This is probably easier, but std::iter::repeat is fancy
#[macro_export]
#[allow(unused_mut)]
macro_rules! baz {
    ($e:expr; $c: expr) => {{                       // this is also fancier
        let mut vs = Vec::new();                    // because no bounds checking
        vs.resize($c, $e);                          // and resize will handle the
        vs                                          // size in a single allocation
        }};
}

/// The standard library likely does this
/// jon does not know why it needs to be in this order
#[macro_export]
macro_rules! foo {
    ($($e:expr),*) => {{
        let mut vs = Vec::new();
        $(vs.push($e);)*
        vs
    }};
    ($($e:expr,)*) => {{
        $crate::foo![$($e),*]
    }};
    }
    /// so that no invocation can exist like foo![,]
    /// which is permissible with:
    /// ($($e:expr),* $(,)?) 

/// this triggers a recursion limit?
/// Chris edit: this feels like conditional branching
/// you want to stack the fizzbuzz on top of the fizz
/// and the buzz?
#[macro_export]
macro_rules! foo {
    ($($e:expr),*) => {{
    [#allow(unused_mut)]
    // let count = [$($e),*].len();                 // this doesn't work as all $e are consumed
    let mut vs = Vec::new();                        // this should really be Vec::with_capacity()
    $(vs.push($e);)*                                // but how can we know the capacity without
    vs                                              // count being passed in?
    }};
    ($($e:expr,)*) => {{
        $crate::foo![$($e),*]
    }};
}

/// So one way we can handle this is with a `private`
/// method on our macro that can be passed without consuming
/// the expressions

/// <[()]>::len([$($crate::foo![@SUBST; $e]),*])
/// this nightmare hellscape says the following:
/// take a reference to this array: &[$($crate::foo![@SUBST; $e]),*]
/// then call the implementation of `len` for slices of unit
/// which is this part: <[()]>::len()
/// this works because arrays implement AsRef<[T]> (AsRef Slice), we can call
/// any method that exists on a slice on the array, by calling the AsRef trait
/// (&[] aka pass by reference to len). So we are calling slice.len(&[])
#[macro_export]
macro_rules! foo {
        ($($e:expr),*) => {{
        [#allow(unused_mut)]
        let mut vs = Vec::with_capacity($crate::foo![@COUNT; $($e),*);
        $(vs.push($e);)*
        vs
    }};
    ($($e:expr,)*) => {{
        $crate::foo![$($e),*]
    }};
    ($e:expr; $c: expr) => {{
        let mut vs = vec::with_capacity(count);
        vs.resize($c,$e);
        vs
    }};
    (@COUNT; $($e:expr),*) => {                     // when this expands the macro in its body
        <[()]>::len(&[$($crate::foo![@SUBST; $e]),*])// it doesn't consume $e, it uses unit to get
                                                    // the length, preserving $e for the main macro
    };
    (@SUBST; $_e:expr) => { ()                      // this takes an expression, but doesn't
    };                                              // consume it
}

// When this expands to let mut vs = Vec::with_capacity(<[()]>::len(&[(), ()...]));
// Unit is a zero-sized type, so this calculation doesn't actually take up any memory
// on the stack, the computation is performed arithmetically (basically)
// NB: this produces a non-contant expression

/// Final
#[macro_export]
macro_rules! foo {
    ($($e:expr),*) => {{
        [#allow(unused_mut)]
        let mut vs = Vec::with_capacity($crate::count![@COUNT; $($e),*);
        $(vs.push($e);)*
        vs
    }};
    ($($e:expr,)*) => {{
        $crate::foo![$($e),*]
    }};
    ($e:expr; $c: expr) => {{
        let mut vs = vec::with_capacity(count);
        vs.resize($c,$e);
        vs
    }};
}
#[macro_export]
#[doc(hidden)]
macro_roles! count {
    (@COUNT; $($e:expr),*) => {
    <[()]>::len(&[$($crate::count![@SUBST; $e]),*])
    };
    (@SUBST; $_e:expr) => { ()
    };
}

