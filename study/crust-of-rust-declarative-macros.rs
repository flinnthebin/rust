/// Declarative Macros

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
#[macro_export]
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
    #[allow(unused_mut]             // required for the 0 case
    ($($e:expr),*) => {{            // this allows us to express multiple expressions
        let mut vs = Vec::new();    // separated by commas (any delimiter can be used)
        w(vs.push($e);)*            // matching the syntax here, we can repeat operations
        vs                          // the same number of times as the pattern that had $e
    }}                              // in it
}

/// trailing comma pattern
#[macro_export]
macro_rules! bar {
    #[allow(unused_mut]
    ($($e:expr),* $(,)?) => {{      // the ? operator allows 0 or 1 trailing commas
        let mut vs = Vec::new();    // following the pattern of comma separatede expressions
        $(vs.push($e);)*
        vs                          // the same number of times as the pattern that had $e
    }}                              // in it
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
          vs.push($e);          // this only works naively with constant values
        }                       // if we pass a Some(val), it will break the macro
        vs                      // as the first time val is evaluated, it works, then
    }}                          // val becomes none
}

#[macro_export]
macro_rules! bar {
    #[allow(unused_mut)]
    ($e:expr; $c:expr) => {{
        let mut vs = Vec::new();
        let x = $e;
        for _ in 0..$c {
          vs.push(x.clone());   // here, the expression is passed once to x, then the
        }                       // cloned value is pushed, which makes the compiler happy
        vs
    }}
}

let y = 42;
bar![y; 2]; // this is fine

bar![y; "twenty times"];    // this will break because the line 'for _ in 0..$c' requires an integer
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

