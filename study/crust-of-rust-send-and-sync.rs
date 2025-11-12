/// Crust of Rust - Send and Sync!

/// Send and Sync describe thread safety in the language
/// They are used to represent thread safety at the type level
/// They describe using a given value or a give type to be used
/// either across thread boundaries (send) or by multiple threads
/// at once (sync).

/// Send and sync are both marker traits. std::marker traits are traits
/// with no methods associated with them. They mark that the type meets
/// a given property or has a particular guarantee about it, but it does
/// not confer any additional behaviour.

pub unsafe auto trait Send {}

/// The auto trait means that the compiler will automatically implement
/// this trait for you if all of the members of a type are themselves that
/// same trait. All auto traits are marker traits but not all marker traits
/// are auto traits.

/// They are primarily used in trait boundsm where a given value or a give type
/// is required to be used either across thread boundaries (send) or by multiple threads
/// at once (sync).


